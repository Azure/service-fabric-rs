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

use envoy_types::pb::envoy::config::listener::v3::Listener;
use envoy_types::pb::envoy::service::discovery::v3::aggregated_discovery_service_client::AggregatedDiscoveryServiceClient;
use envoy_types::pb::envoy::service::discovery::v3::{DiscoveryRequest, DiscoveryResponse};
use mssf_net::ads::{AdsService, ServerHandle, bootstrap_json};
use mssf_net::resources::{CLUSTER_TYPE_URL, LISTENER_TYPE_URL};
use mssf_net::{
    EndpointSnapshot, EndpointSource, HostPort, ScriptedEndpointHandle, ScriptedEndpointSource,
    ServiceRegistry, XdsMapping, host_port_interpreter,
};
use prost::Message;
use tokio::net::TcpListener;
use tokio_stream::wrappers::{ReceiverStream, TcpListenerStream};
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

/// The xDS resource name can be the SF service URI itself.
///
/// This is the shape the proposal wants — `xds:///fabric:/App/Service` — and it
/// matters because it removes the need for a separate alias: the name a client
/// targets *is* the service it wants, so there is no mapping table to keep in
/// sync. Verified end-to-end rather than assumed: the Listener/Cluster names
/// carry `:` and `/`, and virtual-host matching sees the whole string as the
/// authority (which is why the vhost domain is `*`).
#[tokio::test]
#[test_log::test]
async fn xds_name_can_be_the_fabric_service_uri() {
    const SERVICE_URI: &str = "fabric:/MyApp/MyService";

    let a = start_backend("backend-a").await;
    let (source, _handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.addr.ip().to_string(),
        a.addr.port(),
    )));

    // Name the xDS resource after the SF service URI verbatim.
    let mapping = XdsMapping::new(SERVICE_URI, SERVICE_URI, host_port_interpreter()).unwrap();
    let ads = AdsService::new(mapping, source)
        .serve_on_ephemeral_loopback()
        .await
        .expect("failed to start the ADS server");

    let mut client = xds_client(ads.addr(), &format!("xds:///{SERVICE_URI}"));
    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    drop(client);
    ads.shutdown().await.expect("ads server failed");
    a.shutdown().await;
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

// ---- multi-service ---------------------------------------------------------

/// A mapping for one entry of a multi-service registry.
fn mapping_named(name: &str) -> XdsMapping {
    XdsMapping::new(name, format!("fabric:/App/{name}"), host_port_interpreter()).unwrap()
}

fn primary(addr: SocketAddr) -> EndpointSnapshot {
    EndpointSnapshot::Primary(HostPort::new(addr.ip().to_string(), addr.port()))
}

/// One ADS server publishing two services, each with its own scripted source
/// and its own stand-in backend.
struct TwoServices {
    ads: ServerHandle,
    backend_a: BackendHandle,
    backend_b: BackendHandle,
    source_a: Arc<ScriptedEndpointSource>,
    source_b: Arc<ScriptedEndpointSource>,
    /// `Option` so a test can drop the only sender and close `svc-a`'s channel.
    handle_a: Option<ScriptedEndpointHandle>,
    /// Held rather than used: the handle owns the watch sender, so dropping the
    /// last one closes the channel.
    _handle_b: ScriptedEndpointHandle,
}

impl TwoServices {
    async fn start() -> Self {
        let backend_a = start_backend("backend-a").await;
        let backend_b = start_backend("backend-b").await;

        let (source_a, handle_a) = ScriptedEndpointSource::new(primary(backend_a.addr));
        let (source_b, _handle_b) = ScriptedEndpointSource::new(primary(backend_b.addr));

        let registry = ServiceRegistry::builder()
            .add(mapping_named("svc-a"), source_a.clone())
            .unwrap()
            .add(mapping_named("svc-b"), source_b.clone())
            .unwrap()
            .build()
            .unwrap();

        let ads = AdsService::from_registry(registry)
            .serve_on_ephemeral_loopback()
            .await
            .expect("failed to start the ADS server");

        Self {
            ads,
            backend_a,
            backend_b,
            source_a,
            source_b,
            handle_a: Some(handle_a),
            _handle_b,
        }
    }

    /// Publish a new endpoint for `svc-a`.
    fn set_a(&self, snapshot: EndpointSnapshot) {
        self.handle_a
            .as_ref()
            .expect("svc-a's handle was dropped")
            .set(snapshot);
    }

    /// Drop the only sender for `svc-a`, closing its watch channel.
    fn drop_handle_a(&mut self) {
        self.handle_a = None;
    }

    /// Clients for both services, built from the **same** bootstrap — i.e. the
    /// same ADS address — so the only thing that can distinguish them is the
    /// xDS resource name each one targets.
    fn clients(
        &self,
    ) -> (
        IdentityClient<tonic_xds::XdsChannelGrpc>,
        IdentityClient<tonic_xds::XdsChannelGrpc>,
    ) {
        (
            xds_client(self.ads.addr(), "xds:///svc-a"),
            xds_client(self.ads.addr(), "xds:///svc-b"),
        )
    }

    /// Stop everything this fixture owns, in dependency order, so nothing is
    /// left detached and a stuck stream fails loudly instead of hanging.
    async fn shutdown(self) {
        tokio::time::timeout(Duration::from_secs(15), self.ads.shutdown())
            .await
            .expect("ads shutdown timed out")
            .expect("ads server failed");
        self.source_a.shutdown().await;
        self.source_b.shutdown().await;
        self.backend_a.shutdown().await;
        self.backend_b.shutdown().await;
    }
}

/// One ADS server, two services: each client reaches its own backend.
///
/// Both channels share a bootstrap, so a registry lookup that ignored the
/// requested resource name — or keyed it by the wrong type URL — would show up
/// here as one client landing on the other's backend.
#[tokio::test]
#[test_log::test]
async fn two_services_on_one_ads_server_route_independently() {
    let env = TwoServices::start().await;
    let (mut a, mut b) = env.clients();

    assert_eq!(
        who_am_i_until_ok(&mut a, Duration::from_secs(20)).await,
        "backend-a"
    );
    assert_eq!(
        who_am_i_until_ok(&mut b, Duration::from_secs(20)).await,
        "backend-b"
    );

    drop(a);
    drop(b);
    env.shutdown().await;
}

/// Relocating one service must not disturb any other service on the same
/// server.
///
/// Note this is an **isolation** test, not a proof of the
/// ALL_RESOURCES_REQUIRED rule: each `tonic-xds` client opens its own ADS
/// stream with its own single-resource subscription, so a subset push on one
/// stream cannot delete the other's Listener. What it does prove is that the
/// per-entry fan-in routes a change to the right entry and leaves the rest
/// alone. `a_change_repeats_every_subscribed_listener` below is the actual
/// guard for the deletion trap, on a single shared stream.
#[tokio::test]
#[test_log::test]
async fn relocating_one_service_leaves_the_other_routing() {
    let env = TwoServices::start().await;
    let (mut a, mut b) = env.clients();

    assert_eq!(
        who_am_i_until_ok(&mut a, Duration::from_secs(20)).await,
        "backend-a"
    );
    assert_eq!(
        who_am_i_until_ok(&mut b, Duration::from_secs(20)).await,
        "backend-b"
    );

    // Move only svc-a, on the live streams: neither client nor server restarts.
    let c = start_backend("backend-c").await;
    env.set_a(primary(c.addr));

    who_am_i_until(&mut a, "backend-c", Duration::from_secs(20)).await;
    who_am_i_until(&mut b, "backend-b", Duration::from_secs(20)).await;

    drop(a);
    drop(b);
    env.shutdown().await;
    c.shutdown().await;
}

/// `NotFound` withholds only its own entry's Listener and Cluster.
///
/// The unaffected service must keep routing; the missing one must fail in a
/// bounded way. The failure is asserted on failure-versus-success and timing
/// only — `tonic-xds` is `0.1.0-alpha.2` and its error text is not a stable
/// contract.
#[tokio::test]
#[test_log::test]
async fn not_found_for_one_service_leaves_the_other_routing() {
    let env = TwoServices::start().await;
    let (mut a, mut b) = env.clients();

    assert_eq!(
        who_am_i_until_ok(&mut a, Duration::from_secs(20)).await,
        "backend-a"
    );
    assert_eq!(
        who_am_i_until_ok(&mut b, Duration::from_secs(20)).await,
        "backend-b"
    );

    env.set_a(EndpointSnapshot::NotFound);

    let failure = who_am_i_until_err(&mut a, Duration::from_secs(20)).await;
    tracing::info!(%failure, "observed not-found failure for svc-a");

    who_am_i_until(&mut b, "backend-b", Duration::from_secs(20)).await;

    drop(a);
    drop(b);
    env.shutdown().await;
}

/// A server built from a registry reports exactly what it publishes, so a
/// caller does not have to keep a parallel copy of the configuration to know
/// which names are served.
#[test]
fn from_registry_round_trips_the_registered_services() {
    let (source_a, _handle_a) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
    let (source_b, _handle_b) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);

    let registry = ServiceRegistry::builder()
        .add(mapping_named("svc-a"), source_a)
        .unwrap()
        .add(mapping_named("svc-b"), source_b)
        .unwrap()
        .build()
        .unwrap();

    let svc = AdsService::from_registry(registry);
    let registry = svc.registry();

    assert_eq!(registry.len(), 2);
    let names: Vec<&str> = registry
        .entries()
        .iter()
        .map(|e| e.mapping().xds_name())
        .collect();
    assert_eq!(names, ["svc-a", "svc-b"]);
    assert_eq!(
        registry.entries()[0].mapping().cluster_name(),
        "svc-a-primary"
    );
    assert_eq!(registry.index_of(LISTENER_TYPE_URL, "svc-b"), Some(1));
    assert_eq!(
        registry.index_of(CLUSTER_TYPE_URL, "svc-a-primary"),
        Some(0)
    );
}

// ---- protocol-level: the ALL_RESOURCES_REQUIRED rule -----------------------

/// Drive a raw ADS stream, subscribed to both Listeners at once.
///
/// The `tonic-xds` client opens one stream per channel and subscribes only to
/// its own target, so it structurally cannot observe a cross-service deletion.
/// Speaking the protocol directly is the only way to put two subscriptions on
/// one stream and see what a change actually pushes.
async fn open_ads_stream(
    ads_addr: SocketAddr,
    requests: tokio::sync::mpsc::Receiver<DiscoveryRequest>,
) -> tonic::Streaming<DiscoveryResponse> {
    let channel = tonic::transport::Channel::from_shared(format!("http://{ads_addr}"))
        .unwrap()
        .connect()
        .await
        .expect("failed to connect to the ADS server");

    AggregatedDiscoveryServiceClient::new(channel)
        .stream_aggregated_resources(ReceiverStream::new(requests))
        .await
        .expect("failed to open the ADS stream")
        .into_inner()
}

/// Read responses until one carries `type_url`, or panic.
async fn next_response_of_type(
    stream: &mut tonic::Streaming<DiscoveryResponse>,
    type_url: &str,
    timeout: Duration,
) -> DiscoveryResponse {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        let next = tokio::time::timeout(remaining, stream.message())
            .await
            .unwrap_or_else(|_| panic!("no {type_url} response within {timeout:?}"))
            .expect("ads stream failed");
        match next {
            Some(resp) if resp.type_url == type_url => return resp,
            Some(_) => continue,
            None => panic!("ads stream closed while waiting for {type_url}"),
        }
    }
}

fn listener_names(resp: &DiscoveryResponse) -> Vec<String> {
    let mut names: Vec<String> = resp
        .resources
        .iter()
        .map(|r| Listener::decode(r.value.as_slice()).unwrap().name)
        .collect();
    names.sort();
    names
}

/// **The** regression guard for the ALL_RESOURCES_REQUIRED trap.
///
/// One stream subscribes to both `svc-a` and `svc-b`. Only `svc-a` then moves.
/// Listeners are ALL_RESOURCES_REQUIRED_IN_SOTW, so the resulting LDS response
/// must still enumerate BOTH: a response carrying only `svc-a` would not be a
/// partial update, it would silently DELETE `svc-b`'s Listener on the client
/// and break a service that never changed.
///
/// Verified to fail as intended by narrowing the LDS push to the changed entry,
/// which makes the assertion below report `["svc-a"]`.
#[tokio::test]
#[test_log::test]
async fn a_change_repeats_every_subscribed_listener() {
    let env = TwoServices::start().await;

    let (tx, rx) = tokio::sync::mpsc::channel::<DiscoveryRequest>(8);
    let mut stream = open_ads_stream(env.ads.addr(), rx).await;

    // Subscribe to both Listeners on this one stream. An empty nonce marks it
    // as a new subscription rather than an ACK.
    tx.send(DiscoveryRequest {
        type_url: LISTENER_TYPE_URL.to_string(),
        resource_names: vec!["svc-a".to_string(), "svc-b".to_string()],
        ..Default::default()
    })
    .await
    .unwrap();

    let initial =
        next_response_of_type(&mut stream, LISTENER_TYPE_URL, Duration::from_secs(10)).await;
    assert_eq!(
        listener_names(&initial),
        ["svc-a", "svc-b"],
        "the initial subscription must be answered with both Listeners"
    );

    // ACK, so the server does not treat the next request as a re-subscription.
    tx.send(DiscoveryRequest {
        type_url: LISTENER_TYPE_URL.to_string(),
        resource_names: vec!["svc-a".to_string(), "svc-b".to_string()],
        version_info: initial.version_info.clone(),
        response_nonce: initial.nonce.clone(),
        ..Default::default()
    })
    .await
    .unwrap();

    // Move only svc-a.
    let c = start_backend("backend-c").await;
    env.set_a(primary(c.addr));

    let pushed =
        next_response_of_type(&mut stream, LISTENER_TYPE_URL, Duration::from_secs(10)).await;
    assert_eq!(
        listener_names(&pushed),
        ["svc-a", "svc-b"],
        "a change to svc-a must repeat svc-b's Listener; omitting it deletes it",
    );

    drop(tx);
    env.shutdown().await;
    c.shutdown().await;
}

/// A source torn down must not take the rest of the server with it.
///
/// A closed `watch` channel reports "changed" immediately and forever, and
/// every reconnecting stream re-subscribes to it — so a source that ended the
/// stream would break every *other* service on the server, permanently. The
/// retired entry keeps serving its last known state.
#[tokio::test]
#[test_log::test]
async fn dropping_one_source_leaves_the_other_serving() {
    let mut env = TwoServices::start().await;
    let (mut a, mut b) = env.clients();

    assert_eq!(
        who_am_i_until_ok(&mut a, Duration::from_secs(20)).await,
        "backend-a"
    );
    assert_eq!(
        who_am_i_until_ok(&mut b, Duration::from_secs(20)).await,
        "backend-b"
    );

    // Drop svc-a's sender: its channel closes while the streams are live.
    env.drop_handle_a();

    // svc-b is unaffected, and svc-a still serves its last known endpoint.
    who_am_i_until(&mut b, "backend-b", Duration::from_secs(20)).await;
    who_am_i_until(&mut a, "backend-a", Duration::from_secs(20)).await;

    drop(a);
    drop(b);
    env.shutdown().await;
}
