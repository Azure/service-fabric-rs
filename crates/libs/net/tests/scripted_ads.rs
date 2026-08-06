// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Cluster-free end-to-end proof.
//!
//! Drives a **stock, unmodified** `tonic-xds` client against `mssf-net`'s ADS
//! server and asserts that it routes to the endpoint the source publishes, and
//! follows that endpoint when it relocates.
//!
//! **Requires no Service Fabric cluster, no SF runtime, and constructs no
//! `FabricClient`.** This is what makes it runnable in CI before any cluster
//! exists.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use mssf_net::ads::{AdsService, ServerHandle, bootstrap_json};
use mssf_net::{
    EndpointSnapshot, HostPort, ScriptedEndpointSource, XdsMapping, host_port_interpreter,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};
use tonic_xds::{BootstrapConfig, XdsChannelBuilder, XdsChannelConfig, XdsUri};

pub mod testsvc {
    tonic::include_proto!("testsvc");
}

use testsvc::identity_client::IdentityClient;
use testsvc::identity_server::{Identity, IdentityServer};
use testsvc::{WhoAmIReply, WhoAmIRequest};

/// A stand-in backend that reports the name it was started with.
struct NamedIdentity {
    name: String,
}

#[tonic::async_trait]
impl Identity for NamedIdentity {
    async fn who_am_i(
        &self,
        _request: Request<WhoAmIRequest>,
    ) -> Result<Response<WhoAmIReply>, Status> {
        Ok(Response::new(WhoAmIReply {
            name: self.name.clone(),
        }))
    }
}

/// A stand-in backend and its serving task.
///
/// Mirrors [`mssf_net::ads::ServerHandle`]: the task is owned, stopped by an
/// explicit signal, and **awaited**, so a test never leaves a detached server
/// running and a serving failure is not swallowed.
#[must_use = "dropping the handle aborts the backend; call shutdown() to stop it"]
struct BackendHandle {
    addr: SocketAddr,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<Result<(), tonic::transport::Error>>>,
}

impl BackendHandle {
    async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(task) = self.task.take() {
            task.await
                .expect("backend task panicked")
                .expect("backend server failed");
        }
    }
}

impl Drop for BackendHandle {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

/// Start a stand-in backend on an ephemeral loopback port.
async fn start_backend(name: &str) -> BackendHandle {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = IdentityServer::new(NamedIdentity {
        name: name.to_string(),
    });
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(svc)
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                let _ = rx.await;
            })
            .await
    });
    BackendHandle {
        addr,
        shutdown: Some(tx),
        task: Some(task),
    }
}

/// Start the ADS server on an ephemeral loopback port.
async fn start_ads(mapping: XdsMapping, source: Arc<ScriptedEndpointSource>) -> ServerHandle {
    AdsService::new(mapping, source)
        .serve_on_ephemeral_loopback()
        .await
        .expect("failed to start the ADS server")
}

fn mapping() -> XdsMapping {
    XdsMapping::new("reflection", "fabric:/App/Svc", host_port_interpreter()).unwrap()
}

/// Build a stock xDS channel client pointed at our ADS server.
fn xds_client(ads_addr: SocketAddr, target: &str) -> IdentityClient<tonic_xds::XdsChannelGrpc> {
    let bootstrap = BootstrapConfig::from_json(&bootstrap_json(ads_addr, "mssf-net-test")).unwrap();
    let uri = XdsUri::parse(target).unwrap();
    let channel = XdsChannelBuilder::new(XdsChannelConfig::new(uri).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .unwrap();
    IdentityClient::new(channel)
}

/// Per-RPC timeout. An xDS call can block indefinitely while discovery has no
/// usable endpoint, so every call is bounded rather than relying on the outer
/// loop's deadline (which would only be checked after the call returned).
const CALL_TIMEOUT: Duration = Duration::from_secs(3);

/// Make one bounded `WhoAmI` call.
async fn who_am_i_once(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
) -> Result<String, String> {
    match tokio::time::timeout(
        CALL_TIMEOUT,
        client.who_am_i(Request::new(WhoAmIRequest {})),
    )
    .await
    {
        Ok(Ok(r)) => Ok(r.into_inner().name),
        Ok(Err(status)) => Err(format!("status: {status}")),
        Err(_) => Err(format!("call exceeded {CALL_TIMEOUT:?}")),
    }
}

/// Call `WhoAmI`, retrying transient discovery errors until `timeout`.
async fn who_am_i_until_ok(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    timeout: Duration,
) -> String {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let err = match who_am_i_once(client).await {
            Ok(name) => return name,
            Err(e) => e,
        };
        if tokio::time::Instant::now() >= deadline {
            panic!("WhoAmI did not succeed within {timeout:?}; last error: {err}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Call `WhoAmI` until it returns `expected`, or panic.
async fn who_am_i_until(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    expected: &str,
    timeout: Duration,
) {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let last = match who_am_i_once(client).await {
            Ok(name) if name == expected => return,
            Ok(name) => format!("reached {name:?}"),
            Err(e) => e,
        };
        if tokio::time::Instant::now() >= deadline {
            panic!("expected to reach {expected:?} within {timeout:?}; last: {last}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Call `WhoAmI` until it fails, or panic.
async fn who_am_i_until_err(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    timeout: Duration,
) -> String {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if let Err(e) = who_am_i_once(client).await {
            return e;
        }
        if tokio::time::Instant::now() >= deadline {
            panic!("expected a failure within {timeout:?}, but calls kept succeeding");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// SC-001 / SC-005: a stock xDS client reaches the published endpoint with no
/// SF-specific code, and SC-003: it follows the endpoint when it relocates.
#[tokio::test]
#[test_log::test]
async fn stock_xds_client_routes_and_follows_relocation() {
    let a = start_backend("backend-a").await;
    let b = start_backend("backend-b").await;

    let (source, handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    let mut client = xds_client(ads.addr(), "xds:///reflection");

    // Initial routing.
    let first = who_am_i_until_ok(&mut client, Duration::from_secs(20)).await;
    assert_eq!(first, "backend-a");

    // Relocate. The already-created client must follow, with neither the
    // client nor the ADS server restarted.
    handle.set(EndpointSnapshot::Primary(HostPort::new(
        b.addr.ip().to_string(),
        b.addr.port(),
    )));
    who_am_i_until(&mut client, "backend-b", Duration::from_secs(20)).await;

    drop(client);
    ads.shutdown().await.expect("ads server failed");
    a.shutdown().await;
    b.shutdown().await;
}

/// Regression: an ADS stream is long-lived, and graceful shutdown waits for
/// open connections to drain. If the service does not end its streams on
/// shutdown, `ServerHandle::shutdown` blocks forever.
#[tokio::test]
#[test_log::test]
async fn ads_server_shuts_down_with_a_live_client_attached() {
    let a = start_backend("backend-a").await;
    let (source, _handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    // Establish a real ADS stream before stopping.
    let mut client = xds_client(ads.addr(), "xds:///reflection");
    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    // Deliberately do NOT drop the client first: shutting down while a client
    // still holds the stream is the case that used to hang.
    tokio::time::timeout(Duration::from_secs(15), ads.shutdown())
        .await
        .expect("ads shutdown timed out with a client attached")
        .expect("ads server failed");

    drop(client);
    a.shutdown().await;
}

/// A caller-supplied cancellation token stops the server, so an ADS server can
/// be tied to an existing scope (e.g. an SF replica's `close`) rather than
/// needing its own bespoke shutdown plumbing.
#[tokio::test]
#[test_log::test]
async fn caller_supplied_cancellation_token_stops_the_server() {
    let a = start_backend("backend-a").await;
    let (source, _handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));

    let token = CancellationToken::new();
    let ads = AdsService::with_cancellation(mapping(), source, token.clone())
        .serve_on_ephemeral_loopback()
        .await
        .expect("failed to start the ADS server");

    let mut client = xds_client(ads.addr(), "xds:///reflection");
    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    // Cancel the caller's token rather than calling shutdown().
    token.cancel();

    tokio::time::timeout(Duration::from_secs(15), ads.shutdown())
        .await
        .expect("server did not stop after the caller's token was cancelled")
        .expect("ads server failed");

    drop(client);
    a.shutdown().await;
}

/// Shutdown completes promptly while the stream is under continuous load.
///
/// Note this is a smoke test, not a proof of the `biased;` ordering: with
/// unbiased `select!` the cancellation branch still wins with probability 1
/// (it loses n polls with probability ~0.5^n), so a black-box test cannot
/// distinguish the two. `biased;` is there for *determinism* — bounded
/// shutdown latency and no responses emitted after cancellation — rather than
/// to convert a hang into a non-hang. This test guards the coarser property
/// that shutdown is not blocked by ongoing work.
#[tokio::test]
#[test_log::test]
async fn shutdown_completes_under_continuous_endpoint_churn() {
    let a = start_backend("backend-a").await;
    let b = start_backend("backend-b").await;

    let (source, handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    let mut client = xds_client(ads.addr(), "xds:///reflection");
    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    // Keep the watch branch permanently ready by flipping the endpoint in a
    // tight loop for the duration of the shutdown.
    let churn_stop = CancellationToken::new();
    let churn_guard = churn_stop.clone();
    let (a_addr, b_addr) = (a.addr, b.addr);
    let churn = tokio::spawn(async move {
        let mut flip = false;
        while !churn_guard.is_cancelled() {
            let addr = if flip { a_addr } else { b_addr };
            handle.set(EndpointSnapshot::Primary(HostPort::new(
                addr.ip().to_string(),
                addr.port(),
            )));
            flip = !flip;
            tokio::task::yield_now().await;
        }
    });

    tokio::time::timeout(Duration::from_secs(15), ads.shutdown())
        .await
        .expect("shutdown did not complete while the stream was under load")
        .expect("ads server failed");

    churn_stop.cancel();
    churn.await.expect("churn task panicked");

    drop(client);
    a.shutdown().await;
    b.shutdown().await;
}

/// SC-004: the transient no-primary window fails calls in a bounded way
/// (rather than hanging or silently succeeding against a stale address), and
/// recovers once an endpoint is published again.
#[tokio::test]
#[test_log::test]
async fn no_primary_fails_bounded_then_recovers() {
    let a = start_backend("backend-a").await;

    let (source, handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    let ads = start_ads(mapping(), source).await;
    let mut client = xds_client(ads.addr(), "xds:///reflection");

    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    // No primary: calls must fail, bounded.
    handle.set(EndpointSnapshot::NoPrimary);
    let failure = who_am_i_until_err(&mut client, Duration::from_secs(20)).await;
    tracing::info!(%failure, "observed no-primary failure");

    // Recovery.
    handle.set(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    who_am_i_until(&mut client, "backend-a", Duration::from_secs(20)).await;

    drop(client);
    ads.shutdown().await.expect("ads server failed");
    a.shutdown().await;
}

/// SC-007: a name this ADS server has no mapping for fails in a bounded way
/// rather than hanging.
#[tokio::test]
#[test_log::test]
async fn unknown_resource_name_fails_bounded() {
    let a = start_backend("backend-a").await;
    let (source, _handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    let mut client = xds_client(ads.addr(), "xds:///no-such-service");
    let failure = who_am_i_until_err(&mut client, Duration::from_secs(20)).await;
    tracing::info!(%failure, "observed unknown-resource failure");

    drop(client);
    ads.shutdown().await.expect("ads server failed");
    a.shutdown().await;
}
