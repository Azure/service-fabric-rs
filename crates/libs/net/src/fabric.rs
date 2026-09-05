// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Service Fabric-backed [`EndpointSource`].
//!
//! [`FabricNaming`] owns **one** `FabricClient` for the whole process and
//! dispatches its notifications to the per-service [`FabricEndpointSource`]s
//! built from it. Registering N services therefore costs one naming
//! connection and one notification callback, not N of each — which is the
//! point of serving many services from one ADS server in the first place.
//!
//! # COM callback constraints
//!
//! The SF notification callback is **synchronous** and runs on an SF COM
//! thread. It must not await, block, or do heavy work. This module therefore
//! does the absolute minimum there — a read-lock, one hash lookup and a
//! non-blocking `send_replace` of the raw notification — and interprets
//! addresses on a Tokio task instead.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use mssf_core::client::svc_mgmt_client::{
    PartitionKeyType, ResolvedServiceEndpoint, ServiceEndpointRole,
};
use mssf_core::client::{FabricClient, ServiceNotification};
use mssf_core::types::{ServiceNotificationFilterDescription, ServiceNotificationFilterFlags, Uri};
use mssf_core::{ErrorCode, client::svc_mgmt_client::FilterIdHandle};
use mssf_util::resolve::ServicePartitionResolver;
use mssf_util::retry::OperationRetryer;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

use crate::address::AddressInterpreter;
use crate::config::XdsMapping;
use crate::endpoint::{EndpointSnapshot, EndpointSource};
use crate::error::Error;

/// How long the source waits before releasing its `FabricClient` on shutdown.
///
/// Works around <https://github.com/Azure/service-fabric-rs/issues/184>, where
/// dropping a `FabricClient` promptly after use can trigger an invalid memory
/// access. Mirrors the delay used by the reflection sample's live tests.
const DROP_DELAY: Duration = Duration::from_secs(5);

/// Classify a resolve failure into an endpoint state.
///
/// This is deliberately a **pure free function** so it can be unit-tested with
/// no cluster and no `FabricClient`.
///
/// The default direction is safety-critical. [`EndpointSnapshot::NotFound`]
/// causes the Listener to be withheld, and because Listeners are "all
/// resources required" in State-of-the-World xDS, that omission is a
/// **resource deletion** the client treats as permanent. Mis-classifying a
/// transient failure as `NotFound` is therefore severe and hard to recover
/// from, so only explicitly-known "does not exist" codes may produce it —
/// everything else, including unrecognized and non-Fabric errors, falls back
/// to the transient [`EndpointSnapshot::NoPrimary`].
///
/// Note that `FABRIC_E_SERVICE_OFFLINE` rarely arrives here directly:
/// [`ServicePartitionResolver`] retries transient failures and surfaces
/// `FABRIC_E_TIMEOUT` on exhaustion. The outcome is the same either way.
pub fn classify_resolve_error(err: &mssf_core::Error) -> EndpointSnapshot {
    match err.try_as_fabric_error_code() {
        Ok(ErrorCode::FABRIC_E_SERVICE_DOES_NOT_EXIST)
        | Ok(ErrorCode::FABRIC_E_NAME_DOES_NOT_EXIST) => EndpointSnapshot::NotFound,
        // Transient: offline, timeout, communication errors, and anything
        // unrecognized (including non-Fabric HRESULTs).
        _ => EndpointSnapshot::NoPrimary,
    }
}

/// Pick the stateful-primary endpoint address from a resolved endpoint list.
///
/// Role selection is owned by this crate (always the primary); interpreting
/// the resulting opaque address string is the caller's concern.
pub fn primary_address(endpoints: &[ResolvedServiceEndpoint]) -> Option<&ResolvedServiceEndpoint> {
    endpoints
        .iter()
        .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
}

/// Turn a resolved endpoint list into a snapshot using the supplied interpreter.
fn snapshot_from_endpoints(
    endpoints: &[ResolvedServiceEndpoint],
    interpreter: &AddressInterpreter,
) -> EndpointSnapshot {
    let Some(ep) = primary_address(endpoints) else {
        // Resolved, but no primary right now: transient.
        return EndpointSnapshot::NoPrimary;
    };
    match interpreter(&ep.address.to_string()) {
        Ok(hp) => EndpointSnapshot::Primary(hp),
        Err(e) => {
            // A primary exists but its address is unusable. Treat as
            // no-primary rather than not-found: the service does exist.
            tracing::error!(error = %e, "could not interpret primary endpoint address");
            EndpointSnapshot::NoPrimary
        }
    }
}

/// An [`EndpointSource`] backed by Service Fabric naming.
///
/// Build these through [`FabricNaming::source_for`] so several services share
/// one `FabricClient`. [`FabricEndpointSource::new`] remains for the
/// single-service case, where sharing has nothing to share with.
pub struct FabricEndpointSource {
    rx: watch::Receiver<EndpointSnapshot>,
    /// Held behind interior mutability because `shutdown` takes `Arc<Self>`
    /// (the ADS server holds its own clone) and so cannot move fields out.
    releasable: Mutex<Option<Releasable>>,
}

struct Releasable {
    /// The shared naming host. Released here only when this source created it.
    naming: Arc<FabricNaming>,
    /// Whether `shutdown` must also release `naming`.
    owns_naming: bool,
    /// The SF service URI, used to drop this source's notification route.
    service_name: String,
    filter: FilterIdHandle,
    /// Stops the notification-interpreting task.
    task_token: CancellationToken,
    task: tokio::task::JoinHandle<()>,
}

/// Where a raw notification is handed off to, per service.
///
/// Generic over the payload purely so the routing rules can be unit-tested
/// without constructing a `ServiceNotification`, which wraps a COM object.
struct Routes<T> {
    by_service: HashMap<String, Arc<watch::Sender<Option<T>>>>,
}

impl<T> Default for Routes<T> {
    fn default() -> Self {
        Self {
            by_service: HashMap::new(),
        }
    }
}

impl<T> Routes<T> {
    /// Claim `service_name`, rejecting a second claim on the same service.
    ///
    /// Two sources for one SF service would each register a filter and then
    /// race for the same notifications, with only one of them winning the
    /// route. Refusing is better than silently starving one of them.
    fn insert(
        &mut self,
        service_name: String,
        tx: Arc<watch::Sender<Option<T>>>,
    ) -> Result<(), Error> {
        match self.by_service.entry(service_name) {
            Entry::Occupied(e) => Err(Error::Config(format!(
                "service {:?} already has an endpoint source on this naming client",
                e.key()
            ))),
            Entry::Vacant(e) => {
                e.insert(tx);
                Ok(())
            }
        }
    }

    fn remove(&mut self, service_name: &str) {
        self.by_service.remove(service_name);
    }

    fn get(&self, service_name: &str) -> Option<Arc<watch::Sender<Option<T>>>> {
        self.by_service.get(service_name).cloned()
    }
}

/// One `FabricClient` shared by every service this process maps.
///
/// SF installs the notification callback when the client is *built*, so a
/// shared client needs a callback that dispatches by service name rather than
/// one closure per service. That dispatch table is the whole of this type.
///
/// Teardown order is: every [`FabricEndpointSource`] first, then
/// [`FabricNaming::shutdown`], which releases the client.
pub struct FabricNaming {
    /// `None` once released by [`FabricNaming::shutdown`].
    client: Mutex<Option<FabricClient>>,
    routes: Arc<RwLock<Routes<ServiceNotification>>>,
}

impl FabricNaming {
    /// Connect to naming, installing the dispatching notification callback.
    pub fn new(connection_strings: Vec<mssf_core::WString>) -> Result<Arc<Self>, Error> {
        let routes: Arc<RwLock<Routes<ServiceNotification>>> =
            Arc::new(RwLock::new(Routes::<ServiceNotification>::default()));
        let cb_routes = routes.clone();

        let client = FabricClient::builder()
            .with_connection_strings(connection_strings)
            .with_on_service_notification(move |n: ServiceNotification| {
                // COM thread: no await, no blocking, no parsing. A poisoned
                // lock must not panic here either -- unwinding through a COM
                // callback is undefined behaviour -- so the guard is recovered
                // instead. The map is only ever fully-formed under the lock.
                let name = n.service_name.to_string();
                let route = cb_routes
                    .read()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .get(&name);
                if let Some(tx) = route {
                    tx.send_replace(Some(n));
                }
                Ok(())
            })
            .build()
            .map_err(|e| Error::Source(format!("failed to build FabricClient: {e:?}")))?;

        Ok(Arc::new(Self {
            client: Mutex::new(Some(client)),
            routes,
        }))
    }

    /// A handle to the shared client, or an error once it has been released.
    fn client(&self) -> Result<FabricClient, Error> {
        self.client
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| Error::Source("naming client has been shut down".into()))
    }

    /// Build an [`EndpointSource`] for `mapping` on this shared client.
    ///
    /// Registers the notification route and the SF filter **before** the
    /// initial resolve, so a change occurring during startup is not lost. The
    /// initial resolve then seeds the state, and is applied only if no
    /// notification has already produced one — a later notification must never
    /// be overwritten by an older resolve result.
    pub async fn source_for(
        self: &Arc<Self>,
        mapping: &XdsMapping,
        timeout: Duration,
    ) -> Result<Arc<FabricEndpointSource>, Error> {
        self.build_source(mapping, timeout, false).await
    }

    async fn build_source(
        self: &Arc<Self>,
        mapping: &XdsMapping,
        timeout: Duration,
        owns_naming: bool,
    ) -> Result<Arc<FabricEndpointSource>, Error> {
        let client = self.client()?;
        let (tx, rx) = watch::channel(EndpointSnapshot::NoPrimary);
        let tx = Arc::new(tx);

        // Raw notifications are handed off from the COM thread with no
        // interpretation; a Tokio task does the work.
        let (raw_tx, mut raw_rx) = watch::channel::<Option<ServiceNotification>>(None);
        let raw_tx = Arc::new(raw_tx);

        let uri = Uri::from(mapping.service_uri());
        let service_name = mapping.service_uri().to_string();

        // 1. Claim the route before anything can deliver to it.
        self.routes
            .write()
            .unwrap()
            .insert(service_name.clone(), raw_tx)?;

        // 2. Register the filter for exactly this service, primary endpoints
        //    only. Done before the seeding resolve. On failure the route must
        //    be released again, or the name stays claimed by a source that
        //    was never returned.
        let filter = match client
            .get_service_manager()
            .register_service_notification_filter(
                &ServiceNotificationFilterDescription {
                    name: uri.clone(),
                    flags: ServiceNotificationFilterFlags::PrimaryOnly,
                },
                timeout,
                None,
            )
            .await
        {
            Ok(filter) => filter,
            Err(e) => {
                self.routes.write().unwrap().remove(&service_name);
                return Err(Error::Source(format!(
                    "failed to register notification filter: {e:?}"
                )));
            }
        };

        // 3. Drive interpretation of notifications on a Tokio task.
        let interp = mapping.interpreter().clone();
        let pub_tx = tx.clone();
        // Set *before* the first publish, so the seeding step below can detect
        // "a notification has already won" without a torn check.
        let published = Arc::new(AtomicBool::new(false));
        let task_published = published.clone();
        // Stopping the task is explicit rather than inferred. It would in fact
        // exit on its own once every sender drops, but that couples teardown to
        // a non-obvious ownership chain; an owned token plus the JoinHandle lets
        // `shutdown` stop and *await* it directly.
        let task_token = CancellationToken::new();
        let loop_token = task_token.clone();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Cancellation first, so teardown is not delayed by a burst
                    // of notifications.
                    biased;

                    _ = loop_token.cancelled() => break,

                    changed = raw_rx.changed() => {
                        if changed.is_err() {
                            // All senders dropped; nothing more can arrive.
                            break;
                        }
                        let latest = raw_rx.borrow_and_update().clone();
                        let Some(n) = latest else { continue };
                        // An empty endpoint list means the service was removed.
                        let snapshot = if n.endpoints.is_empty() {
                            EndpointSnapshot::NotFound
                        } else {
                            snapshot_from_endpoints(&n.endpoints, &interp)
                        };
                        tracing::debug!(?snapshot, "notification produced a new endpoint state");
                        task_published.store(true, Ordering::SeqCst);
                        pub_tx.send_replace(snapshot);
                    }
                }
            }
        });

        // 4. Seed with an initial resolve, but never clobber a state a
        //    notification has already published.
        //
        //    The check and the write must be one atomic step against the
        //    publishing channel. Checking the *raw* channel and then writing the
        //    published one is two operations on two channels with writers on
        //    other threads, so a notification could land in between and be
        //    overwritten by this older resolve result -- and since SF only sends
        //    further notifications on subsequent changes, that stale state would
        //    be sticky. `send_if_modified` holds the channel's write lock across
        //    the closure, which closes the window: either the flag is already set
        //    (the notification won, we skip), or it is not, in which case the
        //    task has not published yet and its later send correctly supersedes
        //    ours.
        let seeded =
            FabricEndpointSource::initial_resolve(&client, &uri, mapping.interpreter(), timeout)
                .await;
        tx.send_if_modified(|current| {
            if published.load(Ordering::SeqCst) {
                false
            } else {
                *current = seeded;
                true
            }
        });

        Ok(Arc::new(FabricEndpointSource {
            rx,
            releasable: Mutex::new(Some(Releasable {
                naming: self.clone(),
                owns_naming,
                service_name,
                filter,
                task_token,
                task,
            })),
        }))
    }

    /// Release the shared `FabricClient`.
    ///
    /// Call **after** every [`FabricEndpointSource`] built from this naming has
    /// been shut down. The delay before returning works around issue #184.
    pub async fn shutdown(self: Arc<Self>) {
        let client = self.client.lock().unwrap().take();
        if client.is_none() {
            return;
        }
        drop(client);
        tokio::time::sleep(DROP_DELAY).await;
    }
}

impl FabricEndpointSource {
    /// Build a source for a single `mapping` on its own `FabricClient`.
    ///
    /// Convenience for the one-service case. When mapping several services,
    /// build one [`FabricNaming`] and call [`FabricNaming::source_for`] per
    /// service instead: SF installs the notification callback when the client
    /// is built, so a client per service also means a naming connection and a
    /// callback per service.
    ///
    /// The returned source owns its naming host and releases it on
    /// [`EndpointSource::shutdown`].
    pub async fn new(
        mapping: &XdsMapping,
        connection_strings: Vec<mssf_core::WString>,
        timeout: Duration,
    ) -> Result<Arc<Self>, Error> {
        let naming = FabricNaming::new(connection_strings)?;
        naming.build_source(mapping, timeout, true).await
    }

    async fn initial_resolve(
        client: &FabricClient,
        uri: &Uri,
        interpreter: &AddressInterpreter,
        timeout: Duration,
    ) -> EndpointSnapshot {
        let resolver =
            ServicePartitionResolver::new(client.clone(), OperationRetryer::builder().build());
        match resolver
            .resolve(uri, &PartitionKeyType::None, None, Some(timeout), None)
            .await
        {
            Ok(rsp) => snapshot_from_endpoints(&rsp.endpoints, interpreter),
            Err(e) => {
                let s = classify_resolve_error(&e);
                tracing::warn!(error = ?e, snapshot = ?s, "initial resolve failed");
                s
            }
        }
    }
}

#[async_trait::async_trait]
impl EndpointSource for FabricEndpointSource {
    fn subscribe(&self) -> watch::Receiver<EndpointSnapshot> {
        self.rx.clone()
    }

    /// Stop the interpretation task, unregister the filter, drop the
    /// notification route, and release the naming host if this source owns it.
    ///
    /// The ordering matters: the task is stopped and awaited first so nothing
    /// publishes during teardown, unregistration is async (so it cannot happen
    /// in `Drop`), and the route is dropped last so a notification arriving
    /// mid-teardown has nowhere to go rather than reaching a half-torn source.
    ///
    /// When several sources share a [`FabricNaming`], the client outlives them
    /// all and the caller releases it with [`FabricNaming::shutdown`].
    async fn shutdown(self: Arc<Self>) {
        let taken = self.releasable.lock().unwrap().take();
        let Some(Releasable {
            naming,
            owns_naming,
            service_name,
            filter,
            task_token,
            task,
        }) = taken
        else {
            return;
        };

        // Stop interpreting notifications before tearing down the source of
        // them, and await so no publish races the rest of shutdown.
        task_token.cancel();
        if let Err(e) = task.await {
            tracing::warn!(error = ?e, "notification task did not complete cleanly");
        }

        match naming.client() {
            Ok(client) => {
                if let Err(e) = client
                    .get_service_manager()
                    .unregister_service_notification_filter(filter, Duration::from_secs(10), None)
                    .await
                {
                    tracing::warn!(error = ?e, "best-effort filter unregistration failed");
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "naming already released; skipping unregistration")
            }
        }

        naming.routes.write().unwrap().remove(&service_name);

        if owns_naming {
            naming.shutdown().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::host_port_interpreter;
    use crate::endpoint::HostPort;
    use mssf_core::WString;

    fn err(code: ErrorCode) -> mssf_core::Error {
        code.into()
    }

    /// The routing table is generic over its payload so the rules can be
    /// checked with no `FabricClient` and no COM object in sight.
    mod routes {
        use super::*;

        fn tx() -> Arc<watch::Sender<Option<String>>> {
            Arc::new(watch::channel(None).0)
        }

        #[test]
        fn dispatches_by_service_name() {
            let mut routes = Routes::<String>::default();
            let a = tx();
            let b = tx();
            routes.insert("fabric:/App/A".into(), a.clone()).unwrap();
            routes.insert("fabric:/App/B".into(), b.clone()).unwrap();

            let mut rx_a = a.subscribe();
            let mut rx_b = b.subscribe();

            routes
                .get("fabric:/App/A")
                .expect("route for A")
                .send_replace(Some("for-a".into()));

            assert_eq!(rx_a.borrow_and_update().clone(), Some("for-a".to_string()));
            assert_eq!(
                rx_b.borrow_and_update().clone(),
                None,
                "a notification for A must not reach B"
            );
        }

        /// Two sources for one service would each register a filter and then
        /// race for the same notifications, with only one winning the route.
        #[test]
        fn rejects_a_second_source_for_the_same_service() {
            let mut routes = Routes::<String>::default();
            routes.insert("fabric:/App/A".into(), tx()).unwrap();
            let err = routes.insert("fabric:/App/A".into(), tx()).unwrap_err();
            assert!(err.to_string().contains("already has an endpoint source"));
        }

        #[test]
        fn an_unknown_service_has_no_route() {
            let routes = Routes::<String>::default();
            assert!(routes.get("fabric:/App/Nope").is_none());
        }

        /// Removal frees the name, so a service can be re-registered after its
        /// source is shut down.
        #[test]
        fn removal_frees_the_name() {
            let mut routes = Routes::<String>::default();
            routes.insert("fabric:/App/A".into(), tx()).unwrap();
            routes.remove("fabric:/App/A");
            assert!(routes.get("fabric:/App/A").is_none());
            routes
                .insert("fabric:/App/A".into(), tx())
                .expect("the name must be reusable once removed");
        }
    }

    #[test]
    fn does_not_exist_codes_are_the_only_permanent_ones() {
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_SERVICE_DOES_NOT_EXIST)),
            EndpointSnapshot::NotFound
        );
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_NAME_DOES_NOT_EXIST)),
            EndpointSnapshot::NotFound
        );
    }

    #[test]
    fn service_offline_is_transient() {
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_SERVICE_OFFLINE)),
            EndpointSnapshot::NoPrimary
        );
    }

    /// The retryer surfaces a timeout on exhaustion, so this is the code that
    /// actually arrives in practice. It must stay transient.
    #[test]
    fn timeout_is_transient() {
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_TIMEOUT)),
            EndpointSnapshot::NoPrimary
        );
    }

    #[test]
    fn unrecognized_errors_default_to_transient() {
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_COMMUNICATION_ERROR)),
            EndpointSnapshot::NoPrimary
        );
        assert_eq!(
            classify_resolve_error(&err(ErrorCode::FABRIC_E_GATEWAY_NOT_REACHABLE)),
            EndpointSnapshot::NoPrimary
        );
    }

    fn ep(role: ServiceEndpointRole, address: &str) -> ResolvedServiceEndpoint {
        ResolvedServiceEndpoint {
            address: WString::from(address),
            role,
        }
    }

    #[test]
    fn selects_only_the_stateful_primary() {
        let eps = vec![
            ep(ServiceEndpointRole::StatefulSecondary, "sec:1"),
            ep(ServiceEndpointRole::StatefulPrimary, "prim:2"),
        ];
        assert_eq!(primary_address(&eps).unwrap().address.to_string(), "prim:2");
    }

    #[test]
    fn no_primary_when_only_secondaries_present() {
        let eps = vec![ep(ServiceEndpointRole::StatefulSecondary, "sec:1")];
        assert!(primary_address(&eps).is_none());
        assert_eq!(
            snapshot_from_endpoints(&eps, &host_port_interpreter()),
            EndpointSnapshot::NoPrimary
        );
    }

    #[test]
    fn interprets_the_primary_address() {
        let eps = vec![ep(ServiceEndpointRole::StatefulPrimary, "host:20001")];
        assert_eq!(
            snapshot_from_endpoints(&eps, &host_port_interpreter()),
            EndpointSnapshot::Primary(HostPort::new("host", 20001))
        );
    }

    /// An uninterpretable address is *not* "service does not exist".
    #[test]
    fn uninterpretable_primary_address_is_transient_not_permanent() {
        let eps = vec![ep(ServiceEndpointRole::StatefulPrimary, "garbage")];
        assert_eq!(
            snapshot_from_endpoints(&eps, &host_port_interpreter()),
            EndpointSnapshot::NoPrimary
        );
    }
}
