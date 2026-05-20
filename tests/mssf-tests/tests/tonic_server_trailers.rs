// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Pins down what a tonic-generated server, called through the
//! tonic-generated client, looks like to
//! [`ResolveStatusMiddleware`] — and where the metadata
//! actually lives on the wire.
//!
//! ## Why this exists
//!
//! - [`crates/libs/util/tests/tonic_middleware.rs`](../../crates/libs/util/tests/tonic_middleware.rs)
//!   uses scripted `http_body::Frame::trailers` and only proves
//!   the middleware's dedup behaviour **given** a trailers frame.
//! - [`tonic_failover.rs`](tonic_failover.rs) uses raw hyper
//!   servers that emit a trailers frame by hand.
//!
//! Neither exercises the **tonic codegen path** a real adopter
//! takes (`#[tonic::async_trait] impl Foo for MySvc` →
//! `FooServer::new(MySvc)` served via
//! `tonic::transport::Server`, called via `FooClient::new(...)`).
//! That's what this file does: a `.proto` ships in
//! [`proto/testsvc.proto`](../../tests/mssf-tests/proto/testsvc.proto)
//! and [`build.rs`](../../tests/mssf-tests/build.rs) drives
//! `tonic-prost-build` to emit `TestSvcServer<T>` /
//! `TestSvcClient<T>`. The server is wired through
//! `serve_with_incoming_shutdown` exactly like
//! [`crates/samples/reflection/src/main.rs`](../../crates/samples/reflection/src/main.rs).
//!
//! ## Test layout
//!
//! Three tests use the **generated client** so the call site
//! matches what every adopter will write:
//!
//! - [`ok_without_metadata_does_not_rebuild`]
//! - [`ok_with_metadata_round_trips_and_middleware_catches_it`]
//! - [`err_with_metadata_round_trips_and_middleware_catches_it`]
//!
//! One test uses a **raw HTTP/2 send** to capture body frames
//! and prove *which* HTTP/2 frame carries each piece of
//! metadata — that's the wire-level evidence behind why we
//! must inspect *both* initial headers and the trailers frame.
//! The generated client decodes metadata into typed accessors
//! and erases this distinction, so we keep one raw-send test
//! as the single source of truth:
//!
//! - [`wire_level_placement_diagnostic`]
//!
//! ## Finding (2026-05)
//!
//! Tonic 0.14 surfaces user metadata at the **client API
//! level** in both paths — `Response::metadata()` on the `Ok`
//! path and `Status::metadata()` on the `Err` path. **But**
//! the on-the-wire placement is HEADERS, not trailers:
//!
//! - `Response::metadata_mut().insert(...)` on `Ok` → metadata
//!   in the **initial HEADERS** frame; trailers frame carries
//!   only `grpc-status: 0`.
//! - `Status::with_metadata(...)` on `Err` → **trailers-only**
//!   response: a single HEADERS frame (END_STREAM) carrying
//!   `grpc-status`, `grpc-message`, **and** the user metadata.
//!   No DATA, no separate trailers frame.
//!
//! [`ResolveStatusMiddleware`] therefore classifies the
//! configured header on **both** the initial response
//! `HeaderMap` and any trailers frame, firing at most once
//! per response. Both `*_round_trips_and_middleware_catches_it`
//! tests assert `counter == 1`.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use arc_swap::ArcSwap;
use bytes::Bytes;
use http::{HeaderName, Method, Request, Uri};
use http_body::Body as _;
use http_body_util::{BodyExt as _, Full};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tonic::body::Body;
use tonic::transport::server::TcpIncoming;
use tonic::transport::{Channel, Endpoint};
use tonic::{Code, Status};
use tower::{Service, ServiceExt as _};

use mssf_util::tonic::ResolveStatusMiddleware;

// ---------------------------------------------------------------
// Generated tonic code (from proto/testsvc.proto via build.rs).
// ---------------------------------------------------------------

mod testsvc {
    tonic::include_proto!("testsvc");
}

use testsvc::{
    PingReply, PingRequest,
    test_svc_client::TestSvcClient,
    test_svc_server::{TestSvc, TestSvcServer},
};

/// Trailer header configured on the middleware. Mirrors the SDK
/// convention from `TonicConnectorDesign.md`.
const TRAILER: &str = "mssf-status";

// ---------------------------------------------------------------
// Server-side behaviour — switchable per test so a single server
// can drive multiple variants in sequence.
// ---------------------------------------------------------------

/// What the unary handler should return for the next request.
#[derive(Clone, Copy, Debug)]
enum Behavior {
    /// `Ok(Response::new(PingReply {}))` with no metadata.
    /// Baseline "still the right one" case.
    Ok,
    /// `Ok(Response::new(PingReply {}))` with
    /// `response.metadata_mut().insert(TRAILER, value)`. Tonic
    /// emits this metadata in the **initial response headers**,
    /// not in the trailers frame.
    OkWithMeta(&'static str),
    /// `Err(Status::with_metadata(Code::Unavailable, "...", md))`
    /// where `md` carries `mssf-status: value`. Tonic emits this
    /// as a **trailers-only** response (single HEADERS frame
    /// with END_STREAM).
    ErrWithMeta(&'static str),
}

/// Test implementation of the generated `TestSvc` trait.
#[derive(Clone)]
struct MyTestSvc {
    behavior: Arc<ArcSwap<Behavior>>,
    hits: Arc<AtomicUsize>,
}

#[tonic::async_trait]
impl TestSvc for MyTestSvc {
    async fn ping(
        &self,
        _req: tonic::Request<PingRequest>,
    ) -> Result<tonic::Response<PingReply>, Status> {
        self.hits.fetch_add(1, Ordering::SeqCst);
        match **self.behavior.load() {
            Behavior::Ok => Ok(tonic::Response::new(PingReply {})),
            Behavior::OkWithMeta(v) => {
                let mut r = tonic::Response::new(PingReply {});
                r.metadata_mut().insert(TRAILER, v.parse().unwrap());
                Ok(r)
            }
            Behavior::ErrWithMeta(v) => {
                let mut md = tonic::metadata::MetadataMap::new();
                md.insert(TRAILER, v.parse().unwrap());
                Err(Status::with_metadata(Code::Unavailable, "not primary", md))
            }
        }
    }
}

/// Spawn `tonic::transport::Server` on an ephemeral port
/// serving the generated `TestSvcServer`. Same
/// `serve_with_incoming_shutdown` pattern
/// [`crates/samples/reflection/src/main.rs`](../../crates/samples/reflection/src/main.rs)
/// uses in production.
async fn spawn_server(
    behavior: Arc<ArcSwap<Behavior>>,
    hits: Arc<AtomicUsize>,
) -> (SocketAddr, ServerHandle) {
    let svc = MyTestSvc { behavior, hits };
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    let incoming = TcpIncoming::from(listener);
    let cancel = CancellationToken::new();
    let cancel_for_server = cancel.clone();
    let task = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(TestSvcServer::new(svc))
            .serve_with_incoming_shutdown(incoming, async move {
                cancel_for_server.cancelled().await;
            })
            .await
            .expect("tonic server task");
    });
    (
        addr,
        ServerHandle {
            cancel,
            task: Some(task),
        },
    )
}

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
        self.cancel.cancel();
        if let Some(t) = self.task.take() {
            t.abort();
        }
    }
}

// ---------------------------------------------------------------
// Client-side helpers
// ---------------------------------------------------------------

/// Counts how many times the middleware fires `rebuild`.
#[derive(Clone, Default)]
struct RebuildCounter(Arc<AtomicUsize>);
impl RebuildCounter {
    fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
    fn fire(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Build a tonic `Channel` pointed at `addr`, wrap it in
/// `ResolveStatusMiddleware`, return both the middleware and the
/// counter the test will assert on.
fn build_middleware(addr: SocketAddr) -> (ResolveStatusMiddleware<Channel>, RebuildCounter) {
    let endpoint = Endpoint::from_shared(format!("http://{addr}")).expect("valid endpoint");
    let channel = endpoint.connect_lazy();
    let counter = RebuildCounter::default();
    let counter_for_rebuild = counter.clone();
    let mw = ResolveStatusMiddleware::new(channel, HeaderName::from_static(TRAILER), move || {
        counter_for_rebuild.fire()
    });
    (mw, counter)
}

/// Build a generated `TestSvcClient` whose transport is the
/// middleware-wrapped channel. This is the realistic call site:
/// adopters write `TestSvcClient::new(middleware_wrapped_channel)`
/// and use the typed API.
fn build_client(
    addr: SocketAddr,
) -> (
    TestSvcClient<ResolveStatusMiddleware<Channel>>,
    RebuildCounter,
) {
    let (mw, counter) = build_middleware(addr);
    (TestSvcClient::new(mw), counter)
}

// ---------------------------------------------------------------
// Tests using the generated client (the realistic call site)
// ---------------------------------------------------------------

/// `Ok` with no metadata: clean steady-state path. Generated
/// client returns a `Response` with empty metadata; middleware
/// sees a trailers frame carrying only `grpc-status: 0` and
/// does not rebuild.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ok_without_metadata_does_not_rebuild() {
    let behavior = Arc::new(ArcSwap::from_pointee(Behavior::Ok));
    let hits = Arc::new(AtomicUsize::new(0));
    let (addr, srv) = spawn_server(behavior, hits.clone()).await;
    let (mut client, counter) = build_client(addr);

    let resp = client.ping(PingRequest {}).await.expect("ok response");
    assert!(
        resp.metadata().get(TRAILER).is_none(),
        "no server metadata configured → client sees none",
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(counter.count(), 0);

    drop(client);
    srv.shutdown().await;
}

/// `Response::metadata_mut().insert(TRAILER, v)` on the server
/// round-trips to the client via `Response::metadata()`, **and**
/// the middleware observes it via the initial HEADERS frame.
/// Tonic places this metadata in the initial HEADERS frame (not
/// the trailers frame), so the middleware's initial-headers
/// classification path — not its body-frame inspection — is
/// what fires. See [`wire_level_placement_diagnostic`] for the
/// wire-level evidence behind the placement claim.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ok_with_metadata_round_trips_and_middleware_catches_it() {
    let behavior = Arc::new(ArcSwap::from_pointee(Behavior::OkWithMeta("not-primary")));
    let hits = Arc::new(AtomicUsize::new(0));
    let (addr, srv) = spawn_server(behavior, hits.clone()).await;
    let (mut client, counter) = build_client(addr);

    let resp = client.ping(PingRequest {}).await.expect("ok response");

    // Server → client API round-trip works.
    assert_eq!(
        resp.metadata()
            .get(TRAILER)
            .expect("client sees mssf-status via Response::metadata()")
            .to_str()
            .unwrap(),
        "not-primary",
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    // Middleware in between also catches it via initial HEADERS.
    assert_eq!(
        counter.count(),
        1,
        "middleware must inspect initial HEADERS to catch mssf-status set \
         via Response::metadata_mut(); see wire_level_placement_diagnostic.",
    );

    drop(client);
    srv.shutdown().await;
}

/// `Status::with_metadata(...)` on the server round-trips to
/// the client via `Status::metadata()`, **and** the middleware
/// catches it. Tonic emits a trailers-only HEADERS frame for
/// this case (single HEADERS + END_STREAM carrying
/// `grpc-status`, `grpc-message`, and the user metadata; no
/// DATA frame, no separate trailers frame), which the
/// middleware classifies via its initial-headers path. See
/// [`wire_level_placement_diagnostic`].
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn err_with_metadata_round_trips_and_middleware_catches_it() {
    let behavior = Arc::new(ArcSwap::from_pointee(Behavior::ErrWithMeta("not-primary")));
    let hits = Arc::new(AtomicUsize::new(0));
    let (addr, srv) = spawn_server(behavior, hits.clone()).await;
    let (mut client, counter) = build_client(addr);

    let status = client
        .ping(PingRequest {})
        .await
        .expect_err("server returned Err");

    // Server → client API round-trip works.
    assert_eq!(status.code(), Code::Unavailable);
    assert_eq!(
        status
            .metadata()
            .get(TRAILER)
            .expect("client sees mssf-status via Status::metadata()")
            .to_str()
            .unwrap(),
        "not-primary",
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    // Middleware in between also catches it via initial HEADERS
    // (trailers-only response).
    assert_eq!(
        counter.count(),
        1,
        "middleware must inspect initial HEADERS to catch mssf-status set \
         via Status::with_metadata() in a trailers-only response; same \
         path as ok_with_metadata_round_trips_and_middleware_catches_it.",
    );

    drop(client);
    srv.shutdown().await;
}

// ---------------------------------------------------------------
// Wire-level diagnostic — single source of truth for *why*
// the tripwire tests above hold.
// ---------------------------------------------------------------

/// Frame the test captured off the wire — used by
/// [`wire_level_placement_diagnostic`].
#[derive(Debug)]
#[allow(dead_code)] // `len` is captured for diagnostic prints only.
enum CapturedFrame {
    Data { len: usize },
    Trailers(http::HeaderMap),
}

#[derive(Debug)]
struct Captured {
    headers: http::HeaderMap,
    frames: Vec<CapturedFrame>,
}

impl Captured {
    fn trailer_frame(&self) -> Option<&http::HeaderMap> {
        self.frames.iter().find_map(|f| match f {
            CapturedFrame::Trailers(h) => Some(h),
            _ => None,
        })
    }
}

/// Send one valid (empty-message) gRPC request through the
/// middleware-wrapped channel and capture the response headers
/// + every body frame.
///
/// We bypass the generated client here precisely so we can see
/// *which frame* each piece of response metadata lands on — the
/// generated client decodes metadata into typed `metadata()`
/// accessors and erases that information.
async fn send_one_raw(mw: &mut ResolveStatusMiddleware<Channel>) -> Captured {
    // gRPC framing for one empty `PingRequest`:
    //   1 byte  compression flag (0 = uncompressed)
    //   4 bytes big-endian length (0)
    //   0 message bytes
    // Total: 5 zero bytes.
    let body =
        Body::new(Full::new(Bytes::from_static(&[0u8; 5])).map_err(|i: Infallible| match i {}));
    let req = Request::builder()
        .method(Method::POST)
        // Authority is overridden by the Channel; the path is
        // what `TestSvcServer` routes on.
        .uri(Uri::from_static("http://invalid.test/testsvc.TestSvc/Ping"))
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .body(body)
        .unwrap();
    let svc = mw.ready().await.expect("middleware ready");
    let resp = svc.call(req).await.expect("server responded");
    let (parts, mut body) = resp.into_parts();

    let mut frames = Vec::new();
    use std::future::poll_fn;
    loop {
        let frame = poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        match frame {
            None => break,
            Some(Ok(f)) => {
                if let Some(map) = f.trailers_ref() {
                    frames.push(CapturedFrame::Trailers(map.clone()));
                } else if let Some(d) = f.data_ref() {
                    frames.push(CapturedFrame::Data { len: d.len() });
                }
            }
            Some(Err(_)) => break,
        }
    }
    Captured {
        headers: parts.headers,
        frames,
    }
}

/// Single wire-level test that captures HTTP/2 frames for both
/// `OkWithMeta` and `ErrWithMeta` paths and asserts exactly
/// where tonic puts the metadata. This is the source of truth
/// for the placement claim that drives the middleware's
/// inspect-initial-headers branch: tonic emits user metadata on
/// the initial HEADERS frame for both `Ok` (alongside DATA +
/// real trailers) and `Err` (trailers-only HEADERS, no DATA, no
/// trailers frame). The corresponding
/// `*_round_trips_and_middleware_catches_it` tests assert that
/// the middleware classifies that initial-headers placement.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wire_level_placement_diagnostic() {
    let behavior = Arc::new(ArcSwap::from_pointee(Behavior::OkWithMeta("not-primary")));
    let hits = Arc::new(AtomicUsize::new(0));
    let (addr, srv) = spawn_server(behavior.clone(), hits.clone()).await;
    let (mut mw, _counter) = build_middleware(addr);

    // --- Ok + metadata: HEADERS carries mssf-status, trailers frame doesn't. ---
    let captured = send_one_raw(&mut mw).await;
    assert_eq!(
        captured
            .headers
            .get(TRAILER)
            .expect("Ok+meta: mssf-status in initial HEADERS")
            .to_str()
            .unwrap(),
        "not-primary",
        "tonic puts Response::metadata_mut() in initial HEADERS",
    );
    let trailers = captured
        .trailer_frame()
        .expect("Ok responses end with a trailers frame");
    assert!(
        trailers.get(TRAILER).is_none(),
        "Ok+meta: mssf-status is NOT mirrored into the trailers frame",
    );
    assert_eq!(trailers.get("grpc-status").unwrap(), "0");

    // --- Switch to Err + metadata: trailers-only mode (single HEADERS, END_STREAM). ---
    behavior.store(Arc::new(Behavior::ErrWithMeta("not-primary")));
    let captured = send_one_raw(&mut mw).await;
    assert_eq!(
        captured
            .headers
            .get(TRAILER)
            .expect("Err+meta: mssf-status in trailers-only HEADERS")
            .to_str()
            .unwrap(),
        "not-primary",
    );
    assert_eq!(captured.headers.get("grpc-status").unwrap(), "14");
    assert!(
        captured.headers.get("grpc-message").is_some(),
        "trailers-only response carries grpc-message in HEADERS",
    );
    assert!(
        captured.trailer_frame().is_none(),
        "trailers-only response has no separate trailers frame",
    );
    assert!(
        !captured
            .frames
            .iter()
            .any(|f| matches!(f, CapturedFrame::Data { .. })),
        "trailers-only response has no DATA frame",
    );

    assert_eq!(hits.load(Ordering::SeqCst), 2);

    drop(mw);
    srv.shutdown().await;
}
