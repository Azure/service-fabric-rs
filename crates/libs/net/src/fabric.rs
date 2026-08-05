// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Service Fabric-backed [`EndpointSource`].
//!
//! Owns its own `FabricClient`, registers its own notification filter, and
//! publishes the mapped service's current stateful-primary endpoint.
//!
//! # COM callback constraints
//!
//! The SF notification callback is **synchronous** and runs on an SF COM
//! thread. It must not await, block, or do heavy work. This module therefore
//! does the absolute minimum there — a non-blocking `send_replace` of the raw
//! notification — and interprets addresses on a Tokio task instead.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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
pub struct FabricEndpointSource {
    rx: watch::Receiver<EndpointSnapshot>,
    /// Held behind interior mutability because `shutdown` takes `Arc<Self>`
    /// (the ADS server holds its own clone) and so cannot move fields out.
    releasable: Mutex<Option<Releasable>>,
}

struct Releasable {
    client: FabricClient,
    filter: FilterIdHandle,
}

impl FabricEndpointSource {
    /// Build a source for `mapping`, connecting to `connection_strings`.
    ///
    /// Registers the notification callback and the service filter **before**
    /// the initial resolve, so a change occurring during startup is not lost.
    /// The initial resolve then seeds the state, and is applied only if no
    /// notification has already produced one — a later notification must never
    /// be overwritten by an older resolve result.
    pub async fn new(
        mapping: &XdsMapping,
        connection_strings: Vec<mssf_core::WString>,
        timeout: Duration,
    ) -> Result<Arc<Self>, Error> {
        let (tx, rx) = watch::channel(EndpointSnapshot::NoPrimary);
        let tx = Arc::new(tx);

        // Raw notifications are handed off from the COM thread with no
        // interpretation; a Tokio task does the work.
        let (raw_tx, mut raw_rx) = watch::channel::<Option<ServiceNotification>>(None);
        let raw_tx = Arc::new(raw_tx);

        let cb_tx = raw_tx.clone();
        let uri = Uri::from(mapping.service_uri());
        let want = mapping.service_uri().to_string();

        // 1. Install the callback while building the client. This must happen
        //    at construction time; it cannot be added later.
        let client = FabricClient::builder()
            .with_connection_strings(connection_strings)
            .with_on_service_notification(move |n: ServiceNotification| {
                // COM thread: no await, no blocking, no parsing.
                if n.service_name.to_string() == want {
                    cb_tx.send_replace(Some(n));
                }
                Ok(())
            })
            .build()
            .map_err(|e| Error::Source(format!("failed to build FabricClient: {e:?}")))?;

        // 2. Register the filter for exactly this service, primary endpoints
        //    only. Done before the seeding resolve.
        let filter = client
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
            .map_err(|e| Error::Source(format!("failed to register notification filter: {e:?}")))?;

        // 3. Drive interpretation of notifications on a Tokio task.
        let interp = mapping.interpreter().clone();
        let pub_tx = tx.clone();
        // Set *before* the first publish, so the seeding step below can detect
        // "a notification has already won" without a torn check.
        let published = Arc::new(AtomicBool::new(false));
        let task_published = published.clone();
        tokio::spawn(async move {
            while raw_rx.changed().await.is_ok() {
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
        let seeded = Self::initial_resolve(&client, &uri, mapping.interpreter(), timeout).await;
        tx.send_if_modified(|current| {
            if published.load(Ordering::SeqCst) {
                false
            } else {
                *current = seeded;
                true
            }
        });

        Ok(Arc::new(Self {
            rx,
            releasable: Mutex::new(Some(Releasable { client, filter })),
        }))
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

    /// Unregister the filter, release the client, then delay.
    ///
    /// The ordering matters: unregistration is async (so it cannot happen in
    /// `Drop`), and the delay before the client is fully released works around
    /// issue #184.
    async fn shutdown(self: Arc<Self>) {
        let taken = self.releasable.lock().unwrap().take();
        let Some(Releasable { client, filter }) = taken else {
            return;
        };

        if let Err(e) = client
            .get_service_manager()
            .unregister_service_notification_filter(filter, Duration::from_secs(10), None)
            .await
        {
            tracing::warn!(error = ?e, "best-effort filter unregistration failed");
        }

        drop(client);
        tokio::time::sleep(DROP_DELAY).await;
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
