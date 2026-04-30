// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! End-to-end failover test for `mssf_util::tonic::TargetChannel`.
//!
//! Spins up two real HTTP/2 servers on ephemeral ports. A custom
//! [`TargetResolver`] backed by [`ArcSwap`] starts pointing at
//! server A; server A always attaches `mssf-status: not-primary`
//! to its responses; server B attaches no trailer.
//!
//! Sequence of events:
//!
//! 1. Resolver returns A.
//! 2. First request lands on A. Server A responds with
//!    `mssf-status: not-primary` trailer. The middleware fires
//!    `rebuild()` on the [`SwapChannel`]; the new inner
//!    `tonic::Channel` has an empty hyper pool.
//! 3. Test flips the resolver to point at B.
//! 4. Second request through the freshly-rebuilt Channel triggers
//!    a pool miss → the connector calls the resolver → B is
//!    dialed → server B responds with no trailer.
//!
//! Assertions verify the request landed on the expected server and
//! the rebuild signal fired exactly once.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use arc_swap::ArcSwap;
use bytes::Bytes;
use futures::future::BoxFuture;
use http::{HeaderMap, HeaderValue, Method, Request, Response, Uri};
use http_body::Frame;
use http_body_util::BodyExt as _;
use hyper::body::Incoming;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tonic::body::Body;
use tower::{Service, ServiceExt as _};

use mssf_util::tonic::{BoxError, DialTarget, TargetChannel, TargetChannelBuilder, TargetResolver};

// ---------------------------------------------------------------
// Switchable resolver: ArcSwap<DialTarget>; tests flip it at will.
// ---------------------------------------------------------------

#[derive(Clone)]
struct SwitchableResolver {
    target: Arc<ArcSwap<DialTarget>>,
    calls: Arc<AtomicUsize>,
}

impl SwitchableResolver {
    fn new(initial: DialTarget) -> Self {
        Self {
            target: Arc::new(ArcSwap::from_pointee(initial)),
            calls: Arc::new(AtomicUsize::new(0)),
        }
    }
    fn point_at(&self, t: DialTarget) {
        self.target.store(Arc::new(t));
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl TargetResolver for SwitchableResolver {
    fn resolve(&self) -> BoxFuture<'_, Result<DialTarget, BoxError>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok((**self.target.load()).clone())
        })
    }
}

// ---------------------------------------------------------------
// Tiny HTTP/2 server. Each accepted connection responds to every
// request with 200 + `server-id` header + (optionally) the
// trailer currently configured on the shared `TrailerSwitch`.
// Tests flip the switch between calls to drive the dedup state
// machine into the stable / unstable cases at will.
// ---------------------------------------------------------------

/// Shared mutable trailer policy. `Some("v")` → attach
/// `mssf-status: v`. `None` → omit the trailer (steady state).
#[derive(Clone, Default)]
struct TrailerSwitch(Arc<arc_swap::ArcSwapOption<String>>);

impl TrailerSwitch {
    fn set<S: Into<String>>(&self, value: Option<S>) {
        match value {
            Some(v) => self.0.store(Some(Arc::new(v.into()))),
            None => self.0.store(None),
        }
    }
    fn current(&self) -> Option<Arc<String>> {
        self.0.load_full()
    }
}

#[derive(Clone)]
struct ServerCfg {
    id: &'static str,
    /// Switchable trailer policy (shared with the test).
    trailer: TrailerSwitch,
    /// Counts requests served, for assertions.
    hits: Arc<AtomicUsize>,
}

/// Body that emits a tiny DATA frame and then a trailers frame.
/// Real HTTP/2 transports require the data frame to land before
/// trailing HEADERS — a trailer-only body can be collapsed into
/// the initial HEADERS with END_STREAM, hiding our trailer.
struct DataThenTrailerBody {
    state: u8, // 0 = data, 1 = trailers, 2 = done
    trailers: Option<HeaderMap>,
}

impl DataThenTrailerBody {
    fn new(trailers: Option<HeaderMap>) -> Self {
        Self { state: 0, trailers }
    }
}

impl http_body::Body for DataThenTrailerBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.state {
            0 => {
                self.state = 1;
                Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(
                    b"\x00\x00\x00\x00\x00",
                )))))
            }
            1 => {
                self.state = 2;
                match self.trailers.take() {
                    Some(map) => Poll::Ready(Some(Ok(Frame::trailers(map)))),
                    None => Poll::Ready(None),
                }
            }
            _ => Poll::Ready(None),
        }
    }

    fn is_end_stream(&self) -> bool {
        self.state == 2
    }
}

async fn handle(
    cfg: ServerCfg,
    _req: Request<Incoming>,
) -> Result<Response<DataThenTrailerBody>, Infallible> {
    cfg.hits.fetch_add(1, Ordering::SeqCst);
    let trailers = cfg.trailer.current().map(|v| {
        let mut m = HeaderMap::new();
        m.insert("mssf-status", HeaderValue::from_str(v.as_str()).unwrap());
        m
    });
    let resp = Response::builder()
        .status(200)
        .header("server-id", cfg.id)
        .header("content-type", "application/grpc")
        .body(DataThenTrailerBody::new(trailers))
        .unwrap();
    Ok(resp)
}

/// Spawn an HTTP/2 server bound to an ephemeral port. Returns
/// the bound address and a [`ServerHandle`] whose [`shutdown`]
/// method triggers a graceful shutdown via cancellation token
/// and awaits the listener task plus every accepted connection.
async fn spawn_server(cfg: ServerCfg) -> (SocketAddr, ServerHandle) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    let cancel = CancellationToken::new();
    let cancel_listener = cancel.clone();
    let listener_task = tokio::spawn(async move {
        let mut conns: JoinSet<()> = JoinSet::new();
        loop {
            tokio::select! {
                _ = cancel_listener.cancelled() => break,
                accept = listener.accept() => {
                    let (sock, _) = match accept {
                        Ok(x) => x,
                        Err(_) => break,
                    };
                    let cfg2 = cfg.clone();
                    let cancel_conn = cancel_listener.clone();
                    conns.spawn(async move {
                        let svc = service_fn(move |req| handle(cfg2.clone(), req));
                        let mut conn = std::pin::pin!(
                            http2::Builder::new(TokioExecutor::new())
                                .serve_connection(TokioIo::new(sock), svc)
                        );
                        tokio::select! {
                            _ = cancel_conn.cancelled() => {
                                conn.as_mut().graceful_shutdown();
                                let _ = conn.await;
                            }
                            res = conn.as_mut() => { let _ = res; }
                        }
                    });
                }
            }
        }
        // Drain any in-flight connections so the test waits for
        // their `graceful_shutdown` to complete.
        while conns.join_next().await.is_some() {}
    });
    let handle = ServerHandle {
        cancel,
        task: Some(listener_task),
    };
    (addr, handle)
}

/// Owns the listener task + cancellation token. Call
/// [`shutdown`](Self::shutdown) (or drop) to stop the server.
struct ServerHandle {
    cancel: CancellationToken,
    task: Option<tokio::task::JoinHandle<()>>,
}

impl ServerHandle {
    async fn shutdown(mut self) {
        self.cancel.cancel();
        if let Some(t) = self.task.take() {
            let _ = t.await;
        }
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        // Best-effort cancel on drop so a panicking test doesn't
        // leak the listener task; the explicit `shutdown().await`
        // is what gives deterministic ordering.
        self.cancel.cancel();
        if let Some(t) = self.task.take() {
            t.abort();
        }
    }
}

// ---------------------------------------------------------------
// Test driver
// ---------------------------------------------------------------

/// Send one HTTP request through the channel and return the
/// `server-id` response header (so the test knows which backend
/// served it).
async fn send_one(channel: &mut TargetChannel) -> String {
    let req: Request<Body> = Request::builder()
        .method(Method::POST)
        .uri(Uri::from_static("http://fabric.invalid/test/Method"))
        .header("content-type", "application/grpc")
        .body(Body::empty())
        .unwrap();
    let svc = channel.ready().await.expect("ready");
    let resp = svc.call(req).await.expect("response");
    let (parts, body) = resp.into_parts();
    let server_id = parts
        .headers
        .get("server-id")
        .map(|v| v.to_str().unwrap().to_string())
        .unwrap_or_default();
    // Drain the body so the trailer observer fires before we
    // dispatch the next request.
    let _ = body.collect().await.unwrap();
    server_id
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failover_via_trailer_and_resolver_flip() {
    // -- Server A: starts attaching mssf-status: not-primary --
    let trailer_a = TrailerSwitch::default();
    trailer_a.set(Some("not-primary"));
    let hits_a = Arc::new(AtomicUsize::new(0));
    let cfg_a = ServerCfg {
        id: "A",
        trailer: trailer_a.clone(),
        hits: hits_a.clone(),
    };
    let (addr_a, srv_a) = spawn_server(cfg_a).await;

    // -- Server B: no trailer --
    let trailer_b = TrailerSwitch::default();
    let hits_b = Arc::new(AtomicUsize::new(0));
    let cfg_b = ServerCfg {
        id: "B",
        trailer: trailer_b.clone(),
        hits: hits_b.clone(),
    };
    let (addr_b, srv_b) = spawn_server(cfg_b).await;

    // -- Switchable resolver, pointing at A initially --
    let resolver = Arc::new(SwitchableResolver::new(DialTarget {
        host: addr_a.ip().to_string(),
        port: addr_a.port(),
    }));

    // -- TargetChannel via the convenience builder --
    let mut channel = TargetChannelBuilder::new()
        .resolver(resolver.clone())
        .trailer_header("mssf-status")
        .build();

    // 1. First request: lands on A, sees the trailer, middleware
    //    fires rebuild(). Response delivered to caller as-is.
    let id1 = send_one(&mut channel).await;
    assert_eq!(id1, "A");
    assert_eq!(hits_a.load(Ordering::SeqCst), 1);
    assert_eq!(hits_b.load(Ordering::SeqCst), 0);

    // 2. Flip the resolver to point at B (simulating an SF
    //    primary move). Subsequent dials go to B.
    resolver.point_at(DialTarget {
        host: addr_b.ip().to_string(),
        port: addr_b.port(),
    });

    // 3. Second request: the rebuilt Channel has an empty pool,
    //    so the connector dials → resolver returns B → lands
    //    on B (no trailer).
    let id2 = send_one(&mut channel).await;
    assert_eq!(id2, "B");
    assert_eq!(
        hits_a.load(Ordering::SeqCst),
        1,
        "A should not be hit again"
    );
    assert_eq!(hits_b.load(Ordering::SeqCst), 1);

    // 4. Resolver call count: 1 cold dial against A, 1 cold dial
    //    against B (the rebuilt Channel's first request).
    assert_eq!(resolver.calls(), 2);

    // 5. Third request reuses B's hyper pool — no new dial, no
    //    new resolve, server B's hits increments.
    let id3 = send_one(&mut channel).await;
    assert_eq!(id3, "B");
    assert_eq!(hits_b.load(Ordering::SeqCst), 2);
    assert_eq!(
        resolver.calls(),
        2,
        "no extra resolve when reusing the live HTTP/2 connection"
    );

    // Drop the client first so its connections close, then
    // gracefully drain both servers.
    drop(channel);
    srv_a.shutdown().await;
    srv_b.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn no_trailer_no_rebuild_pool_reused() {
    // Server B: no trailer, ever. Multiple requests should reuse
    // the same HTTP/2 connection — only one resolver call total.
    let hits = Arc::new(AtomicUsize::new(0));
    let cfg = ServerCfg {
        id: "B",
        trailer: TrailerSwitch::default(),
        hits: hits.clone(),
    };
    let (addr, srv) = spawn_server(cfg).await;

    let resolver = Arc::new(SwitchableResolver::new(DialTarget {
        host: addr.ip().to_string(),
        port: addr.port(),
    }));

    let mut channel = TargetChannelBuilder::new()
        .resolver(resolver.clone())
        .trailer_header("mssf-status")
        .build();

    for _ in 0..3 {
        assert_eq!(send_one(&mut channel).await, "B");
    }
    assert_eq!(hits.load(Ordering::SeqCst), 3);
    assert_eq!(
        resolver.calls(),
        1,
        "steady-state should reuse the pooled connection"
    );

    drop(channel);
    srv.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn server_becomes_stable_after_one_trailer() {
    // Single server. Starts attaching `mssf-status: not-primary`.
    // After the test flips its trailer off, subsequent requests
    // run cleanly. We expect: 1 rebuild, then steady state with
    // the SAME server (same `host:port`, but a fresh hyper pool
    // because rebuild() dropped the old Channel).
    let trailer = TrailerSwitch::default();
    trailer.set(Some("not-primary"));
    let hits = Arc::new(AtomicUsize::new(0));
    let cfg = ServerCfg {
        id: "S",
        trailer: trailer.clone(),
        hits: hits.clone(),
    };
    let (addr, srv) = spawn_server(cfg).await;

    let resolver = Arc::new(SwitchableResolver::new(DialTarget {
        host: addr.ip().to_string(),
        port: addr.port(),
    }));
    let mut channel = TargetChannelBuilder::new()
        .resolver(resolver.clone())
        .trailer_header("mssf-status")
        .build();

    // 1. First call: server is "unstable" → trailer fires once
    //    → rebuild scheduled. last_seen = Some("not-primary").
    assert_eq!(send_one(&mut channel).await, "S");
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(resolver.calls(), 1);

    // 2. Server switches to stable (no trailer).
    trailer.set::<String>(None);

    // 3. Second call: rebuilt Channel has empty pool → fresh
    //    dial → resolver returns same target → server now
    //    serves cleanly → dedup `Reset`, last_seen = None.
    assert_eq!(send_one(&mut channel).await, "S");
    assert_eq!(hits.load(Ordering::SeqCst), 2);
    assert_eq!(resolver.calls(), 2, "rebuild caused one new dial");

    // 4. Third call: pool reused, no resolve, no trailer, no
    //    rebuild. Steady state.
    assert_eq!(send_one(&mut channel).await, "S");
    assert_eq!(hits.load(Ordering::SeqCst), 3);
    assert_eq!(resolver.calls(), 2, "steady state should not re-dial");

    drop(channel);
    srv.shutdown().await;
}
