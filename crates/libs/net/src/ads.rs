// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! State-of-the-World Aggregated Discovery Service (ADS).
//!
//! Serves the resource graph built by [`crate::resources`] and *pushes* a fresh
//! `ClusterLoadAssignment` to every connected stream whenever a mapped
//! service's authoritative endpoint changes.
//!
//! Delta/incremental xDS is not implemented; State-of-the-World is sufficient
//! for the gRPC xDS clients this targets.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use envoy_types::pb::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
    aggregated_discovery_service_server::{
        AggregatedDiscoveryService, AggregatedDiscoveryServiceServer,
    },
};
use envoy_types::pb::google::protobuf::Any;
use futures::Stream;
use futures::future::select_all;
use prost::Message;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};

use crate::config::XdsMapping;
use crate::endpoint::{EndpointSnapshot, EndpointSource};
use crate::registry::ServiceRegistry;
use crate::resources::{
    CLUSTER_TYPE_URL, ENDPOINT_TYPE_URL, LISTENER_TYPE_URL, build_cluster, build_endpoints,
    build_listener,
};

/// Serves a [`ServiceRegistry`] over ADS, backed by each entry's
/// [`EndpointSource`].
///
/// One server publishes any number of services; clients subscribe by resource
/// name. Use [`AdsService::new`] for the single-service case, or
/// [`AdsService::from_registry`] to serve several.
pub struct AdsService {
    registry: Arc<ServiceRegistry>,
    /// Cancelling ends every open ADS stream and stops the server.
    ///
    /// An ADS stream is long-lived by design, and `tonic`'s graceful shutdown
    /// waits for open connections to drain — so without a way to end the
    /// streams, a graceful stop would block forever. One token drives both:
    /// the streams observe it and finish, and it doubles as the server's
    /// shutdown signal.
    token: CancellationToken,
}

impl AdsService {
    /// Create a single-service instance with its own cancellation token.
    pub fn new(mapping: XdsMapping, source: Arc<dyn EndpointSource>) -> Self {
        Self::with_cancellation(mapping, source, CancellationToken::new())
    }

    /// Create a single-service instance driven by a caller-supplied token.
    ///
    /// Use this to tie the server's lifetime to an existing scope — for
    /// example an SF service's `close(cancellation_token)`, so the ADS server
    /// stops when the replica does. To adapt from this crate's
    /// [`mssf_core::runtime::executor::BoxedCancelToken`], see
    /// `mssf_util::tokio::TokioCancelToken`, which wraps the same underlying
    /// [`CancellationToken`] and can convert in both directions.
    pub fn with_cancellation(
        mapping: XdsMapping,
        source: Arc<dyn EndpointSource>,
        token: CancellationToken,
    ) -> Self {
        Self::from_registry_with_cancellation(ServiceRegistry::single(mapping, source), token)
    }

    /// Serve every service in `registry`, with a fresh cancellation token.
    pub fn from_registry(registry: ServiceRegistry) -> Self {
        Self::from_registry_with_cancellation(registry, CancellationToken::new())
    }

    /// Serve every service in `registry`, driven by a caller-supplied token.
    pub fn from_registry_with_cancellation(
        registry: ServiceRegistry,
        token: CancellationToken,
    ) -> Self {
        Self {
            registry: Arc::new(registry),
            token,
        }
    }

    /// The services this server publishes.
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// A clone of the token that stops this service.
    ///
    /// Take this *before* [`AdsService::into_server`] if you are mounting the
    /// service on your own `tonic` server, and cancel it as part of your
    /// shutdown signal — otherwise graceful shutdown will hang on the open
    /// ADS streams.
    pub fn cancellation_token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Wrap as a mountable gRPC service.
    pub fn into_server(self) -> AggregatedDiscoveryServiceServer<Self> {
        AggregatedDiscoveryServiceServer::new(self)
    }
}

/// Failure stopping an ADS server.
#[derive(Debug)]
pub enum ShutdownError {
    /// The server itself failed while serving.
    Serve(tonic::transport::Error),
    /// The serving task panicked or was cancelled.
    Join(tokio::task::JoinError),
}

impl std::fmt::Display for ShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShutdownError::Serve(e) => write!(f, "ads server failed: {e}"),
            ShutdownError::Join(e) => write!(f, "ads server task did not complete: {e}"),
        }
    }
}

impl std::error::Error for ShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShutdownError::Serve(e) => Some(e),
            ShutdownError::Join(e) => Some(e),
        }
    }
}

/// A running ADS server.
///
/// Owns the serving task and the token that stops it. Dropping the handle
/// cancels the token and aborts the task, so a server can never outlive the
/// scope that started it; prefer [`ServerHandle::shutdown`], which cancels and
/// then **awaits** the task so serving errors surface rather than being
/// swallowed.
#[must_use = "dropping the handle stops the server; call shutdown() to stop it gracefully"]
pub struct ServerHandle {
    addr: SocketAddr,
    token: CancellationToken,
    // `Option` because `shutdown` consumes the handle, and a type with a
    // `Drop` impl cannot have fields moved out of it.
    task: Option<tokio::task::JoinHandle<Result<(), tonic::transport::Error>>>,
}

impl ServerHandle {
    /// The address the server is bound to.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// A clone of the token that stops this server.
    ///
    /// Cancelling it is equivalent to calling [`ServerHandle::shutdown`],
    /// except that it does not wait for the task.
    pub fn cancellation_token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Cancel and wait for the server task to finish.
    ///
    /// Returns the serving result: `Err` if the server itself failed, or if the
    /// task panicked or was cancelled.
    pub async fn shutdown(mut self) -> Result<(), ShutdownError> {
        self.token.cancel();
        let Some(task) = self.task.take() else {
            return Ok(());
        };
        match task.await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(ShutdownError::Serve(e)),
            Err(join) => Err(ShutdownError::Join(join)),
        }
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        // Best-effort: a handle dropped without `shutdown()` must not leave a
        // server running for the rest of the process. Cancel first so the task
        // can finish cleanly if it is scheduled before the abort lands;
        // `shutdown` takes the task, so this only fires on the un-awaited path.
        self.token.cancel();
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

impl AdsService {
    /// Serve on an ephemeral loopback port.
    ///
    /// Convenience for tests and single-process hosting: the caller does not
    /// have to plumb a listener or a `tonic` server itself. The returned
    /// [`ServerHandle`] owns the task and must be used to stop it.
    pub async fn serve_on_ephemeral_loopback(self) -> std::io::Result<ServerHandle> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        self.serve_with_listener(listener).await
    }

    /// Serve on a caller-supplied listener.
    pub async fn serve_with_listener(
        self,
        listener: tokio::net::TcpListener,
    ) -> std::io::Result<ServerHandle> {
        let addr = listener.local_addr()?;
        let token = self.cancellation_token();
        let svc = self.into_server();
        let signal = token.clone();
        let task = tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(svc)
                .serve_with_incoming_shutdown(
                    tokio_stream::wrappers::TcpListenerStream::new(listener),
                    // The same cancellation that ends the open ADS streams also
                    // triggers the graceful stop, so connections can drain.
                    signal.cancelled_owned(),
                )
                .await
        });
        Ok(ServerHandle {
            addr,
            token,
            task: Some(task),
        })
    }

    /// Build the resources for one type URL across every registered service.
    ///
    /// `snapshots` is parallel to `registry.entries()`.
    ///
    /// Returns an empty vector when the request names only resources this
    /// server does not publish, which the client reports as resource-not-found.
    /// A `NotFound` entry withholds *its own* Listener and Cluster and leaves
    /// the others untouched.
    ///
    /// The full subscribed set is always returned for a given type. Listeners
    /// and Clusters are "all resources required" in State-of-the-World, so
    /// answering with a subset silently **deletes** the omitted ones on the
    /// client.
    fn resources_for(
        registry: &ServiceRegistry,
        snapshots: &[EndpointSnapshot],
        type_url: &str,
        resource_names: &[String],
    ) -> Vec<Any> {
        // An empty request set is xDS's wildcard: everything of this type.
        // Otherwise resolve each requested name through the registry's
        // `(type_url, name)` index, so name resolution has exactly one
        // implementation. Sorting restores registry order; dedup tolerates a
        // client naming the same resource twice.
        let selected: Vec<usize> = if resource_names.is_empty() {
            (0..registry.len()).collect()
        } else {
            let mut selected: Vec<usize> = resource_names
                .iter()
                .filter_map(|name| registry.index_of(type_url, name))
                .collect();
            selected.sort_unstable();
            selected.dedup();
            selected
        };

        let mut out = Vec::new();
        for i in selected {
            let mapping = registry.entries()[i].mapping();
            let snapshot = &snapshots[i];
            match type_url {
                LISTENER_TYPE_URL => {
                    // A missing service withholds the Listener. Because
                    // Listeners are "all resources required" in SotW, omission
                    // is a deletion.
                    if matches!(snapshot, EndpointSnapshot::NotFound) {
                        continue;
                    }
                    out.push(any(
                        LISTENER_TYPE_URL,
                        build_listener(mapping).encode_to_vec(),
                    ));
                }
                CLUSTER_TYPE_URL => {
                    if matches!(snapshot, EndpointSnapshot::NotFound) {
                        continue;
                    }
                    out.push(any(
                        CLUSTER_TYPE_URL,
                        build_cluster(mapping).encode_to_vec(),
                    ));
                }
                ENDPOINT_TYPE_URL => {
                    out.push(any(
                        ENDPOINT_TYPE_URL,
                        build_endpoints(mapping, snapshot).encode_to_vec(),
                    ));
                }
                _ => return vec![],
            }
        }
        out
    }
}

fn any(type_url: &str, value: Vec<u8>) -> Any {
    Any {
        type_url: type_url.to_string(),
        value,
    }
}

/// Await the next endpoint change across the still-live registered services.
///
/// Returns the index of the entry that changed.
///
/// A source whose sender has been dropped is retired from the fan-in rather
/// than ending the stream. This matters far more with several services than it
/// did with one: a closed receiver reports "changed" immediately and forever,
/// and because every reconnecting stream re-subscribes to the same dead
/// channel, one torn-down source would otherwise break every *other* service
/// on the server, permanently.
///
/// A retired source is deliberately **not** published as `NotFound`. A dropped
/// sender is a host-process lifecycle event, not naming reporting that the
/// service is gone, and `NotFound` is a permanent deletion on the client. The
/// last published state is served instead.
///
/// If every source is gone this never resolves, leaving the `select!`'s other
/// branches -- cancellation above all -- free to run.
///
/// This is a standalone future rather than inline in the `select!` because the
/// fan-in borrows `receivers` mutably for as long as it is alive; resolving
/// here releases that borrow before the caller's handler runs.
/// `watch::Receiver::changed` is cancel-safe, so rebuilding the set on every
/// call is correct.
async fn next_endpoint_change(
    receivers: &mut [watch::Receiver<EndpointSnapshot>],
    alive: &mut [bool],
) -> usize {
    loop {
        let pending: Vec<_> = receivers
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| alive[*i])
            .map(|(i, rx)| Box::pin(async move { (i, rx.changed().await) }))
            .collect();

        if pending.is_empty() {
            std::future::pending::<()>().await;
        }

        let ((which, result), _, _) = select_all(pending).await;
        if result.is_ok() {
            receivers[which].borrow_and_update();
            return which;
        }

        tracing::debug!(
            entry = which,
            "endpoint source dropped; retiring it from the fan-in"
        );
        alive[which] = false;
    }
}

/// Every registered service's current snapshot, in registry order.
fn current_snapshots(readers: &[watch::Receiver<EndpointSnapshot>]) -> Vec<EndpointSnapshot> {
    readers.iter().map(|rx| rx.borrow().clone()).collect()
}

/// Whether a request is a new subscription (needing a response) rather than an
/// ACK of a previous response.
///
/// The discriminator is `response_nonce`, **not** `version_info`. A client that
/// re-establishes its ADS stream retains the last version it accepted and
/// replays it in the bootstrap request for each subscribed type, with an empty
/// nonce. Treating a non-empty version as an ACK would make the server ignore
/// every request on the reconnected stream, so it would send nothing and never
/// learn the stream's subscriptions -- silently and permanently stalling
/// discovery for that client.
fn is_subscription_request(req: &DiscoveryRequest) -> bool {
    req.response_nonce.is_empty()
}

/// Per-stream monotonic version/nonce counter.
struct Versioning(u64);

impl Versioning {
    fn next(&mut self) -> (String, String) {
        self.0 += 1;
        (self.0.to_string(), self.0.to_string())
    }
}

#[tonic::async_trait]
impl AggregatedDiscoveryService for AdsService {
    type StreamAggregatedResourcesStream =
        Pin<Box<dyn Stream<Item = Result<DiscoveryResponse, Status>> + Send>>;

    async fn stream_aggregated_resources(
        &self,
        request: Request<tonic::Streaming<DiscoveryRequest>>,
    ) -> Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
        let mut inbound = request.into_inner();
        let registry = self.registry.clone();
        let mut receivers: Vec<watch::Receiver<EndpointSnapshot>> = registry
            .entries()
            .iter()
            .map(|e| e.source().subscribe())
            .collect();
        // Independent clones used only for reading. `changed()` needs `&mut`,
        // and that borrow lives for as long as the fan-in future does -- which
        // is the whole `select!` -- so no arm can read through the same
        // handles. Reading through clones keeps every response built from the
        // live value rather than a cached copy that could drift.
        let readers: Vec<watch::Receiver<EndpointSnapshot>> = receivers.to_vec();
        let mut alive: Vec<bool> = vec![true; receivers.len()];
        let token = self.token.clone();

        let outbound = async_stream::try_stream! {
            let mut version = Versioning(0);
            // Mark the seeded values seen, so the fan-in does not immediately
            // report a change the client is about to be told about anyway.
            for rx in receivers.iter_mut() {
                rx.borrow_and_update();
            }
            // Resource names this stream has subscribed to, per type URL.
            // Tracked for every type (not just EDS) so a state change can be
            // pushed to whatever the client actually asked for.
            let mut subscriptions: HashMap<String, Vec<String>> = HashMap::new();

            loop {
                // Already cancelled (e.g. a stream opened during shutdown).
                if token.is_cancelled() {
                    break;
                }

                tokio::select! {
                    // Poll in order, cancellation first. The default is a
                    // *random* ready branch, so under steady traffic (client
                    // requests, or endpoint churn) shutdown latency becomes
                    // random and the stream can emit further responses after
                    // cancellation. Biasing makes it deterministic: once
                    // cancelled, the stream ends at the next poll and does no
                    // further work. Shutdown blocks on these streams draining,
                    // so bounded latency here bounds shutdown.
                    biased;

                    // The server is stopping: end the stream so the connection
                    // can drain and graceful shutdown can complete.
                    _ = token.cancelled() => {
                        tracing::debug!("ads stream ending: cancelled");
                        break;
                    }

                    // A request from the client.
                    msg = inbound.message() => {
                        let req = match msg {
                            Ok(Some(req)) => req,
                            // Client closed the stream.
                            Ok(None) => break,
                            // Transport-level failure: the stream is over.
                            Err(status) => {
                                tracing::debug!(%status, "ads client stream ended with an error");
                                break;
                            }
                        };

                        // Record the subscription BEFORE any early return, so
                        // ACKs still keep our view of what the client wants.
                        // Keep the previous set: a client may change its
                        // resource set on an *existing* stream, and that
                        // request carries the last nonce (i.e. looks like an
                        // ACK) once the type has seen a response. Comparing
                        // against the previous set is what distinguishes
                        // "re-subscribe" from "plain ACK".
                        let previous = subscriptions
                            .insert(req.type_url.clone(), req.resource_names.clone());
                        let resources_changed =
                            previous.as_deref() != Some(req.resource_names.as_slice());

                        if let Some(err) = req.error_detail.as_ref() {
                            // NACK: log and do not advance state.
                            tracing::warn!(
                                type_url = %req.type_url,
                                code = err.code,
                                message = %err.message,
                                "xds client NACK'd a resource",
                            );
                            continue;
                        }

                        // ACK/NACK is discriminated by `response_nonce`, NOT by
                        // `version_info` -- see `is_subscription_request`. A
                        // request that also *changes* the resource set is a new
                        // subscription regardless of the nonce, and must be
                        // answered or the client waits forever for a resource
                        // it will otherwise only see on the next state change.
                        if !is_subscription_request(&req) && !resources_changed {
                            continue;
                        }

                        let snapshots = current_snapshots(&readers);
                        let resources = Self::resources_for(
                            &registry, &snapshots, &req.type_url, &req.resource_names,
                        );
                        let (v, nonce) = version.next();
                        yield DiscoveryResponse {
                            version_info: v,
                            type_url: req.type_url,
                            nonce,
                            resources,
                            ..Default::default()
                        };
                    }

                    // Some service's endpoint state changed.
                    which = next_endpoint_change(&mut receivers, &mut alive) => {
                        let snapshots = current_snapshots(&readers);
                        tracing::debug!(
                            entry = which, snapshot = ?snapshots[which], "pushing updated resources",
                        );

                        // Push all subscribed types, not just EDS. A transition
                        // into or out of `NotFound` changes whether the Listener
                        // and Cluster exist at all, and a SotW client never
                        // re-requests LDS on its own -- so an EDS-only push would
                        // leave a client that saw `NotFound` with a permanently
                        // deleted listener even after the service came back.
                        //
                        // LDS and CDS carry the *whole* subscribed set because
                        // they are "all resources required": a response that
                        // omitted a subscribed Listener would delete it on the
                        // client, so pushing only the service that changed
                        // would break every other service on this stream. EDS
                        // is tracked per-resource, so it carries only the
                        // cluster that actually changed.
                        let changed_mapping = registry.entries()[which].mapping();
                        let changed_listener = changed_mapping.xds_name().to_string();
                        let changed_cluster = changed_mapping.cluster_name();

                        for type_url in [LISTENER_TYPE_URL, CLUSTER_TYPE_URL, ENDPOINT_TYPE_URL] {
                            let Some(names) = subscriptions.get(type_url) else { continue };

                            // An empty request set is the wildcard, and always
                            // wants the update. Otherwise a stream that is not
                            // subscribed to the changed service has nothing new
                            // to learn, and re-sending its unchanged set would
                            // make every service's churn cost every client a
                            // response.
                            let changed_name = if type_url == LISTENER_TYPE_URL {
                                &changed_listener
                            } else {
                                &changed_cluster
                            };
                            if !names.is_empty() && !names.contains(changed_name) {
                                continue;
                            }

                            let names: Vec<String> = if type_url == ENDPOINT_TYPE_URL {
                                // Narrow the wildcard explicitly: only the
                                // cluster that moved needs a new assignment.
                                vec![changed_cluster.clone()]
                            } else {
                                names.clone()
                            };
                            let resources = Self::resources_for(
                                &registry, &snapshots, type_url, &names,
                            );
                            let (v, nonce) = version.next();
                            yield DiscoveryResponse {
                                version_info: v,
                                type_url: type_url.to_string(),
                                nonce,
                                resources,
                                ..Default::default()
                            };
                        }
                    }
                }
            }
        };

        Ok(Response::new(Box::pin(outbound)))
    }

    type DeltaAggregatedResourcesStream =
        Pin<Box<dyn Stream<Item = Result<DeltaDiscoveryResponse, Status>> + Send>>;

    async fn delta_aggregated_resources(
        &self,
        _request: Request<tonic::Streaming<DeltaDiscoveryRequest>>,
    ) -> Result<Response<Self::DeltaAggregatedResourcesStream>, Status> {
        Err(Status::unimplemented(
            "delta xDS is not supported; use state-of-the-world",
        ))
    }
}

/// Bootstrap JSON pointing a gRPC xDS client at an ADS server.
///
/// `BootstrapConfig::from_json` is the only public, non-environment-variable
/// way to configure the client, so this helper exists to feed it.
pub fn bootstrap_json(ads_addr: SocketAddr, node_id: &str) -> String {
    // `ads_addr` is a `SocketAddr`, so its Display is always JSON-safe;
    // `node_id` is caller-supplied and is not.
    let node_id = escape_json_string(node_id);
    format!(
        r#"{{
  "xds_servers": [
    {{
      "server_uri": "http://{ads_addr}",
      "channel_creds": [{{"type": "insecure"}}],
      "server_features": ["xds_v3"]
    }}
  ],
  "node": {{"id": "{node_id}"}}
}}"#
    )
}

/// Escape a string for embedding in a JSON string literal (RFC 8259 §7).
fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // All other control characters must be escaped as \u00XX.
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::host_port_interpreter;
    use crate::endpoint::{HostPort, ScriptedEndpointSource};
    use envoy_types::pb::envoy::config::endpoint::v3::ClusterLoadAssignment;
    use envoy_types::pb::envoy::config::listener::v3::Listener;

    fn mapping() -> XdsMapping {
        XdsMapping::new("reflection", "fabric:/App/Svc", host_port_interpreter()).unwrap()
    }

    fn scripted() -> Arc<dyn EndpointSource> {
        ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary).0
    }

    /// `resources_for` over a one-service registry, which is what most of these
    /// cases are about. Multi-service behaviour is covered separately below.
    fn resources_for_one(snap: &EndpointSnapshot, type_url: &str, names: &[String]) -> Vec<Any> {
        let registry = ServiceRegistry::single(mapping(), scripted());
        AdsService::resources_for(&registry, std::slice::from_ref(snap), type_url, names)
    }

    fn decode_cla(resources: &[Any]) -> ClusterLoadAssignment {
        ClusterLoadAssignment::decode(resources[0].value.as_slice()).unwrap()
    }

    #[test]
    fn serves_listener_cluster_and_endpoints_for_wildcard_requests() {
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));

        let l = resources_for_one(&snap, LISTENER_TYPE_URL, &[]);
        assert_eq!(l.len(), 1);
        assert_eq!(
            Listener::decode(l[0].value.as_slice()).unwrap().name,
            "reflection"
        );

        let c = resources_for_one(&snap, CLUSTER_TYPE_URL, &[]);
        assert_eq!(c.len(), 1);

        let e = resources_for_one(&snap, ENDPOINT_TYPE_URL, &[]);
        assert_eq!(decode_cla(&e).endpoints.len(), 1);
    }

    /// An unknown resource name yields an empty response, which the client
    /// reports as resource-not-found rather than hanging.
    #[test]
    fn unknown_resource_name_yields_empty_response() {
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));
        let names = vec!["some-other-service".to_string()];

        assert!(resources_for_one(&snap, LISTENER_TYPE_URL, &names).is_empty());
        assert!(resources_for_one(&snap, CLUSTER_TYPE_URL, &names).is_empty());
        assert!(resources_for_one(&snap, ENDPOINT_TYPE_URL, &names).is_empty());
    }

    /// NotFound withholds the Listener; because Listeners are "all resources
    /// required" in SotW, omission is a deletion on the client.
    #[test]
    fn not_found_withholds_the_listener() {
        let snap = EndpointSnapshot::NotFound;
        assert!(resources_for_one(&snap, LISTENER_TYPE_URL, &[]).is_empty());
    }

    /// NoPrimary keeps a valid Listener and Cluster but empties the endpoints,
    /// so the client reports "no ready endpoints" rather than a missing route.
    #[test]
    fn no_primary_keeps_listener_but_empties_endpoints() {
        let snap = EndpointSnapshot::NoPrimary;
        assert_eq!(resources_for_one(&snap, LISTENER_TYPE_URL, &[]).len(), 1);
        assert_eq!(resources_for_one(&snap, CLUSTER_TYPE_URL, &[]).len(), 1);
        let e = resources_for_one(&snap, ENDPOINT_TYPE_URL, &[]);
        assert_eq!(e.len(), 1, "the EDS resource must still exist");
        assert!(
            decode_cla(&e).endpoints.is_empty(),
            "but carry no endpoints"
        );
    }

    #[test]
    fn unknown_type_url_yields_nothing() {
        let snap = EndpointSnapshot::NoPrimary;
        assert!(resources_for_one(&snap, "type.googleapis.com/nope", &[]).is_empty());
    }

    // ---- multi-service -------------------------------------------------

    fn two_service_registry() -> ServiceRegistry {
        ServiceRegistry::builder()
            .add(
                XdsMapping::new("a", "fabric:/App/A", host_port_interpreter()).unwrap(),
                scripted(),
            )
            .unwrap()
            .add(
                XdsMapping::new("b", "fabric:/App/B", host_port_interpreter()).unwrap(),
                scripted(),
            )
            .unwrap()
            .build()
            .unwrap()
    }

    fn listener_names(resources: &[Any]) -> Vec<String> {
        resources
            .iter()
            .map(|r| Listener::decode(r.value.as_slice()).unwrap().name)
            .collect()
    }

    /// A wildcard request must return *every* registered Listener: Listeners
    /// are "all resources required" in SotW, so returning a subset would delete
    /// the rest on the client.
    #[test]
    fn wildcard_returns_every_registered_service() {
        let registry = two_service_registry();
        let snaps = vec![
            EndpointSnapshot::Primary(HostPort::new("ha", 1)),
            EndpointSnapshot::Primary(HostPort::new("hb", 2)),
        ];

        let l = AdsService::resources_for(&registry, &snaps, LISTENER_TYPE_URL, &[]);
        assert_eq!(listener_names(&l), vec!["a", "b"]);
        assert_eq!(
            AdsService::resources_for(&registry, &snaps, CLUSTER_TYPE_URL, &[]).len(),
            2
        );
        assert_eq!(
            AdsService::resources_for(&registry, &snaps, ENDPOINT_TYPE_URL, &[]).len(),
            2
        );
    }

    #[test]
    fn a_named_request_returns_only_that_service() {
        let registry = two_service_registry();
        let snaps = vec![
            EndpointSnapshot::Primary(HostPort::new("ha", 1)),
            EndpointSnapshot::Primary(HostPort::new("hb", 2)),
        ];

        let l = AdsService::resources_for(&registry, &snaps, LISTENER_TYPE_URL, &["b".to_string()]);
        assert_eq!(listener_names(&l), vec!["b"]);

        let e = AdsService::resources_for(
            &registry,
            &snaps,
            ENDPOINT_TYPE_URL,
            &["a-primary".to_string()],
        );
        assert_eq!(e.len(), 1);
        assert_eq!(decode_cla(&e).cluster_name, "a-primary");
    }

    /// One service disappearing must not take its neighbours' routing with it.
    #[test]
    fn not_found_on_one_service_leaves_the_other_intact() {
        let registry = two_service_registry();
        let snaps = vec![
            EndpointSnapshot::NotFound,
            EndpointSnapshot::Primary(HostPort::new("hb", 2)),
        ];

        let l = AdsService::resources_for(&registry, &snaps, LISTENER_TYPE_URL, &[]);
        assert_eq!(listener_names(&l), vec!["b"]);
        assert_eq!(
            AdsService::resources_for(&registry, &snaps, CLUSTER_TYPE_URL, &[]).len(),
            1
        );
    }

    #[test]
    fn versioning_is_monotonic() {
        let mut v = Versioning(0);
        let (a, na) = v.next();
        let (b, nb) = v.next();
        assert_eq!((a.as_str(), na.as_str()), ("1", "1"));
        assert_eq!((b.as_str(), nb.as_str()), ("2", "2"));
    }

    /// Two concurrently connected streams must both observe an endpoint change.
    #[tokio::test]
    async fn concurrent_subscribers_both_observe_a_change() {
        let (src, handle) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
        let svc = AdsService::new(mapping(), src.clone());

        let mut a = svc.registry().entries()[0].source().subscribe();
        let mut b = svc.registry().entries()[0].source().subscribe();

        handle.set(EndpointSnapshot::Primary(HostPort::new("moved", 9)));

        a.changed().await.unwrap();
        b.changed().await.unwrap();

        let want = EndpointSnapshot::Primary(HostPort::new("moved", 9));
        assert_eq!(*a.borrow(), want);
        assert_eq!(*b.borrow(), want);
    }

    #[test]
    fn bootstrap_json_is_well_formed_and_points_at_the_server() {
        let j = bootstrap_json("127.0.0.1:18000".parse().unwrap(), "test-node");
        assert!(j.contains("http://127.0.0.1:18000"));
        assert!(j.contains("\"id\": \"test-node\""));
        assert!(j.contains("xds_v3"));
    }

    /// A node id containing JSON metacharacters must not corrupt the document.
    #[test]
    fn bootstrap_json_escapes_the_node_id() {
        let j = bootstrap_json("127.0.0.1:1".parse().unwrap(), r#"a"b\c"#);
        assert!(
            j.contains(r#""id": "a\"b\\c""#),
            "quote and backslash must be escaped, got: {j}"
        );

        // Control characters too.
        let j = bootstrap_json("127.0.0.1:1".parse().unwrap(), "a\nb\u{1}c");
        assert!(j.contains(r#""id": "a\nb\u0001c""#), "got: {j}");

        // The result must still parse as the bootstrap the client expects.
        let cfg = tonic_xds::BootstrapConfig::from_json(&bootstrap_json(
            "127.0.0.1:1".parse().unwrap(),
            r#"weird"id"#,
        ));
        assert!(cfg.is_ok(), "escaped bootstrap must still parse: {cfg:?}");
    }

    /// A client may change its resource set on an existing stream; that request
    /// carries the last nonce, so it looks like an ACK. It must still be
    /// answered, or the client waits for a resource it never receives.
    #[test]
    fn resource_set_change_is_answered_even_with_a_nonce() {
        // Plain ACK: same resource set, nonce present -> no response needed.
        let mut subs: HashMap<String, Vec<String>> = HashMap::new();
        subs.insert(LISTENER_TYPE_URL.to_string(), vec!["a".to_string()]);

        let ack = DiscoveryRequest {
            version_info: "1".to_string(),
            response_nonce: "1".to_string(),
            type_url: LISTENER_TYPE_URL.to_string(),
            resource_names: vec!["a".to_string()],
            ..Default::default()
        };
        let previous = subs.insert(ack.type_url.clone(), ack.resource_names.clone());
        let changed = previous.as_deref() != Some(ack.resource_names.as_slice());
        assert!(!changed, "same set is a plain ACK");
        assert!(
            !is_subscription_request(&ack) && !changed,
            "would be skipped"
        );

        // Re-subscribe: different resource set, nonce still present.
        let resub = DiscoveryRequest {
            version_info: "1".to_string(),
            response_nonce: "1".to_string(),
            type_url: LISTENER_TYPE_URL.to_string(),
            resource_names: vec!["a".to_string(), "b".to_string()],
            ..Default::default()
        };
        let previous = subs.insert(resub.type_url.clone(), resub.resource_names.clone());
        let changed = previous.as_deref() != Some(resub.resource_names.as_slice());
        assert!(changed, "an added resource name must be seen as a change");
        assert!(
            !is_subscription_request(&resub) && changed,
            "nonce says ACK, but the changed set must still be answered"
        );
    }

    /// Regression: a reconnecting client replays its retained `version_info`
    /// with an EMPTY `response_nonce`. Discriminating ACKs on `version_info`
    /// would make the server ignore every request on the new stream, stalling
    /// discovery permanently. The nonce is the correct discriminator.
    #[test]
    fn ack_is_discriminated_by_nonce_not_version() {
        let reconnect = DiscoveryRequest {
            version_info: "7".to_string(),
            response_nonce: String::new(),
            type_url: LISTENER_TYPE_URL.to_string(),
            ..Default::default()
        };
        assert!(
            is_subscription_request(&reconnect),
            "a replayed version with no nonce is a subscription, not an ACK"
        );

        let ack = DiscoveryRequest {
            version_info: "7".to_string(),
            response_nonce: "7".to_string(),
            type_url: LISTENER_TYPE_URL.to_string(),
            ..Default::default()
        };
        assert!(!is_subscription_request(&ack), "a nonce present means ACK");

        let initial = DiscoveryRequest {
            version_info: String::new(),
            response_nonce: String::new(),
            type_url: LISTENER_TYPE_URL.to_string(),
            ..Default::default()
        };
        assert!(is_subscription_request(&initial));
    }

    /// Regression: `NotFound` withholds the Listener, which a SotW client
    /// treats as a deletion and never re-requests. So a change must push every
    /// subscribed type, not EDS alone, or a client that saw `NotFound` stays
    /// broken forever once the service comes back.
    #[test]
    fn recovering_from_not_found_requires_listener_and_cluster_again() {
        // While NotFound, LDS and CDS are empty.
        assert!(resources_for_one(&EndpointSnapshot::NotFound, LISTENER_TYPE_URL, &[]).is_empty());
        assert!(resources_for_one(&EndpointSnapshot::NotFound, CLUSTER_TYPE_URL, &[]).is_empty());

        // Once a primary exists they must be populated again -- which only
        // reaches the client if the push covers these type URLs.
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));
        assert_eq!(resources_for_one(&snap, LISTENER_TYPE_URL, &[]).len(), 1);
        assert_eq!(resources_for_one(&snap, CLUSTER_TYPE_URL, &[]).len(), 1);
    }
}
