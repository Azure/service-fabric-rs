// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! State-of-the-World Aggregated Discovery Service (ADS).
//!
//! Serves the resource graph built by [`crate::resources`] and *pushes* a fresh
//! `ClusterLoadAssignment` to every connected stream whenever the mapped
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
use prost::Message;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};

use crate::config::XdsMapping;
use crate::endpoint::{EndpointSnapshot, EndpointSource};
use crate::resources::{
    CLUSTER_TYPE_URL, ENDPOINT_TYPE_URL, LISTENER_TYPE_URL, build_cluster, build_endpoints,
    build_listener,
};

/// Serves one [`XdsMapping`] over ADS, backed by an [`EndpointSource`].
///
/// **One mapping per service instance.** Serving several SF services requires
/// several `AdsService`s on separate ports. This is a limitation of this crate,
/// not of xDS — see the "Known limitations" section of
/// `docs/design/XdsNamingDesign.md` for what lifting it would involve.
pub struct AdsService {
    mapping: Arc<XdsMapping>,
    source: Arc<dyn EndpointSource>,
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
    /// Create the service with its own cancellation token.
    pub fn new(mapping: XdsMapping, source: Arc<dyn EndpointSource>) -> Self {
        Self::with_cancellation(mapping, source, CancellationToken::new())
    }

    /// Create the service driven by a caller-supplied cancellation token.
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
        Self {
            mapping: Arc::new(mapping),
            source,
            token,
        }
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
    pub async fn shutdown(mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.token.cancel();
        let Some(task) = self.task.take() else {
            return Ok(());
        };
        match task.await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(Box::new(e)),
            Err(join) => Err(Box::new(join)),
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

    /// Build the resources for one type URL at a given endpoint state.
    ///
    /// Returns an empty vector when the request names a resource this mapping
    /// does not serve, which the client reports as resource-not-found. The
    /// Listener is likewise withheld when the service does not exist.
    fn resources_for(
        mapping: &XdsMapping,
        snapshot: &EndpointSnapshot,
        type_url: &str,
        resource_names: &[String],
    ) -> Vec<Any> {
        let wants =
            |name: &str| resource_names.is_empty() || resource_names.iter().any(|n| n == name);

        match type_url {
            LISTENER_TYPE_URL => {
                // A missing service withholds the Listener. Because Listeners
                // are "all resources required" in SotW, omission is a deletion.
                if matches!(snapshot, EndpointSnapshot::NotFound) || !wants(mapping.xds_name()) {
                    return vec![];
                }
                vec![any(
                    LISTENER_TYPE_URL,
                    build_listener(mapping).encode_to_vec(),
                )]
            }
            CLUSTER_TYPE_URL => {
                if matches!(snapshot, EndpointSnapshot::NotFound) || !wants(&mapping.cluster_name())
                {
                    return vec![];
                }
                vec![any(
                    CLUSTER_TYPE_URL,
                    build_cluster(mapping).encode_to_vec(),
                )]
            }
            ENDPOINT_TYPE_URL => {
                if !wants(&mapping.cluster_name()) {
                    return vec![];
                }
                vec![any(
                    ENDPOINT_TYPE_URL,
                    build_endpoints(mapping, snapshot).encode_to_vec(),
                )]
            }
            _ => vec![],
        }
    }
}

fn any(type_url: &str, value: Vec<u8>) -> Any {
    Any {
        type_url: type_url.to_string(),
        value,
    }
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
        let mapping = self.mapping.clone();
        let mut rx: watch::Receiver<EndpointSnapshot> = self.source.subscribe();
        let token = self.token.clone();

        let outbound = async_stream::try_stream! {
            let mut version = Versioning(0);
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
                        subscriptions
                            .insert(req.type_url.clone(), req.resource_names.clone());

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
                        // `version_info` -- see `is_subscription_request`.
                        if !is_subscription_request(&req) {
                            continue;
                        }

                        let snapshot = rx.borrow().clone();
                        let resources = Self::resources_for(
                            &mapping, &snapshot, &req.type_url, &req.resource_names,
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

                    // The endpoint state changed: push to every subscribed type.
                    changed = rx.changed() => {
                        if changed.is_err() {
                            // Source dropped; nothing more will ever be pushed.
                            break;
                        }
                        let snapshot = rx.borrow_and_update().clone();
                        tracing::debug!(?snapshot, "pushing updated resources");

                        // Push all subscribed types, not just EDS. A transition
                        // into or out of `NotFound` changes whether the Listener
                        // and Cluster exist at all, and a SotW client never
                        // re-requests LDS on its own -- so an EDS-only push would
                        // leave a client that saw `NotFound` with a permanently
                        // deleted listener even after the service came back.
                        for type_url in [LISTENER_TYPE_URL, CLUSTER_TYPE_URL, ENDPOINT_TYPE_URL] {
                            let Some(names) = subscriptions.get(type_url) else { continue };
                            let resources = Self::resources_for(
                                &mapping, &snapshot, type_url, names,
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

    fn decode_cla(resources: &[Any]) -> ClusterLoadAssignment {
        ClusterLoadAssignment::decode(resources[0].value.as_slice()).unwrap()
    }

    #[test]
    fn serves_listener_cluster_and_endpoints_for_wildcard_requests() {
        let m = mapping();
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));

        let l = AdsService::resources_for(&m, &snap, LISTENER_TYPE_URL, &[]);
        assert_eq!(l.len(), 1);
        assert_eq!(
            Listener::decode(l[0].value.as_slice()).unwrap().name,
            "reflection"
        );

        let c = AdsService::resources_for(&m, &snap, CLUSTER_TYPE_URL, &[]);
        assert_eq!(c.len(), 1);

        let e = AdsService::resources_for(&m, &snap, ENDPOINT_TYPE_URL, &[]);
        assert_eq!(decode_cla(&e).endpoints.len(), 1);
    }

    /// An unknown resource name yields an empty response, which the client
    /// reports as resource-not-found rather than hanging.
    #[test]
    fn unknown_resource_name_yields_empty_response() {
        let m = mapping();
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));
        let names = vec!["some-other-service".to_string()];

        assert!(AdsService::resources_for(&m, &snap, LISTENER_TYPE_URL, &names).is_empty());
        assert!(AdsService::resources_for(&m, &snap, CLUSTER_TYPE_URL, &names).is_empty());
        assert!(AdsService::resources_for(&m, &snap, ENDPOINT_TYPE_URL, &names).is_empty());
    }

    /// NotFound withholds the Listener; because Listeners are "all resources
    /// required" in SotW, omission is a deletion on the client.
    #[test]
    fn not_found_withholds_the_listener() {
        let m = mapping();
        let snap = EndpointSnapshot::NotFound;
        assert!(AdsService::resources_for(&m, &snap, LISTENER_TYPE_URL, &[]).is_empty());
    }

    /// NoPrimary keeps a valid Listener and Cluster but empties the endpoints,
    /// so the client reports "no ready endpoints" rather than a missing route.
    #[test]
    fn no_primary_keeps_listener_but_empties_endpoints() {
        let m = mapping();
        let snap = EndpointSnapshot::NoPrimary;
        assert_eq!(
            AdsService::resources_for(&m, &snap, LISTENER_TYPE_URL, &[]).len(),
            1
        );
        assert_eq!(
            AdsService::resources_for(&m, &snap, CLUSTER_TYPE_URL, &[]).len(),
            1
        );
        let e = AdsService::resources_for(&m, &snap, ENDPOINT_TYPE_URL, &[]);
        assert_eq!(e.len(), 1, "the EDS resource must still exist");
        assert!(
            decode_cla(&e).endpoints.is_empty(),
            "but carry no endpoints"
        );
    }

    #[test]
    fn unknown_type_url_yields_nothing() {
        let m = mapping();
        let snap = EndpointSnapshot::NoPrimary;
        assert!(AdsService::resources_for(&m, &snap, "type.googleapis.com/nope", &[]).is_empty());
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

        let mut a = svc.source.subscribe();
        let mut b = svc.source.subscribe();

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
        let m = mapping();

        // While NotFound, LDS and CDS are empty.
        assert!(
            AdsService::resources_for(&m, &EndpointSnapshot::NotFound, LISTENER_TYPE_URL, &[])
                .is_empty()
        );
        assert!(
            AdsService::resources_for(&m, &EndpointSnapshot::NotFound, CLUSTER_TYPE_URL, &[])
                .is_empty()
        );

        // Once a primary exists they must be populated again -- which only
        // reaches the client if the push covers these type URLs.
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 1));
        assert_eq!(
            AdsService::resources_for(&m, &snap, LISTENER_TYPE_URL, &[]).len(),
            1
        );
        assert_eq!(
            AdsService::resources_for(&m, &snap, CLUSTER_TYPE_URL, &[]).len(),
            1
        );
    }
}
