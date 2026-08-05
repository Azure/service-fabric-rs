// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! SF-independent endpoint state and the subscription contract.
//!
//! This module is the seam that lets the whole mapping and serving stack be
//! exercised with a scripted endpoint and no `FabricClient` at all. Nothing
//! here refers to Service Fabric types.

use std::sync::Arc;

use tokio::sync::watch;

/// A connectable TCP target.
///
/// Deliberately *not* `mssf_util::tonic::DialTarget`: that type lives in the
/// experimental module this crate is intended to supersede, and depending on
/// it would couple the successor to the incumbent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostPort {
    /// Host name or IP literal.
    pub host: String,
    /// TCP port.
    pub port: u16,
}

impl HostPort {
    /// Construct a target.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }
}

impl std::fmt::Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// The current state of the mapped service's authoritative (primary) endpoint.
///
/// Three states, not two: the transient "there is no primary right now" case
/// must stay distinct from the permanent "this service does not exist" case,
/// because they map to very different xDS outcomes.
///
/// | State | xDS outcome |
/// |---|---|
/// | [`Primary`][Self::Primary] | populated `ClusterLoadAssignment` |
/// | [`NoPrimary`][Self::NoPrimary] | **empty but valid** `ClusterLoadAssignment` |
/// | [`NotFound`][Self::NotFound] | Listener omitted → client sees resource deletion |
///
/// `mssf_util::resolve::ServicePartitionResolver` collapses the empty-endpoint
/// case into `FABRIC_E_SERVICE_OFFLINE`, so it cannot make this distinction on
/// its own; the SF-backed source classifies explicitly instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointSnapshot {
    /// An authoritative endpoint is currently available.
    Primary(HostPort),
    /// The service exists but currently has no reachable primary (transient).
    NoPrimary,
    /// The service does not exist / resolution failed permanently.
    NotFound,
}

/// A source of [`EndpointSnapshot`] values for one mapped service.
///
/// Implementations must deliver a current value immediately on
/// [`subscribe`][EndpointSource::subscribe] and every subsequent change.
#[async_trait::async_trait]
pub trait EndpointSource: Send + Sync + 'static {
    /// Current snapshot plus every subsequent one.
    ///
    /// [`watch`] is used rather than a stream or an mpsc queue because it is
    /// latest-value-wins, which satisfies the "coalesce to the newest state"
    /// requirement *structurally* rather than with bespoke logic; its
    /// `send_replace` is non-blocking and non-async (so it is safe to call
    /// from a synchronous SF COM callback thread); and it carries an initial
    /// value, so a newly connected ADS stream needs no separate "get current"
    /// call.
    fn subscribe(&self) -> watch::Receiver<EndpointSnapshot>;

    /// Explicit asynchronous teardown.
    ///
    /// Exists because SF notification-filter unregistration is `async` (and
    /// `Drop` cannot await), and because an SF-backed implementation must
    /// control *when* its `FabricClient` handles are released.
    ///
    /// Takes `Arc<Self>` because the ADS server holds its own clone of the
    /// source, so `self` cannot be consumed by value. The consequence is that
    /// implementations **cannot move fields out of `self`** and must therefore
    /// hold releasable handles behind interior mutability (e.g.
    /// `Mutex<Option<..>>`) and `take()` them here.
    async fn shutdown(self: Arc<Self>);
}

/// A scripted, in-memory [`EndpointSource`] for tests.
///
/// Lives in the library rather than behind `#[cfg(test)]` because
/// `#[cfg(test)]` items are not visible to this crate's own `tests/`
/// integration binaries, which compile as separate crates.
#[derive(Debug)]
pub struct ScriptedEndpointSource {
    rx: watch::Receiver<EndpointSnapshot>,
}

/// Drives a [`ScriptedEndpointSource`].
#[derive(Debug, Clone)]
pub struct ScriptedEndpointHandle {
    tx: Arc<watch::Sender<EndpointSnapshot>>,
}

impl ScriptedEndpointSource {
    /// Create a source seeded with `initial`, plus the handle that drives it.
    pub fn new(initial: EndpointSnapshot) -> (Arc<Self>, ScriptedEndpointHandle) {
        let (tx, rx) = watch::channel(initial);
        (
            Arc::new(Self { rx }),
            ScriptedEndpointHandle { tx: Arc::new(tx) },
        )
    }
}

impl ScriptedEndpointHandle {
    /// Publish a new snapshot, replacing any value not yet observed.
    pub fn set(&self, snapshot: EndpointSnapshot) {
        self.tx.send_replace(snapshot);
    }
}

#[async_trait::async_trait]
impl EndpointSource for ScriptedEndpointSource {
    fn subscribe(&self) -> watch::Receiver<EndpointSnapshot> {
        self.rx.clone()
    }

    async fn shutdown(self: Arc<Self>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_port_displays_as_host_colon_port() {
        assert_eq!(HostPort::new("localhost", 20001).to_string(), "localhost:20001");
    }

    #[tokio::test]
    async fn scripted_source_delivers_initial_value_immediately() {
        let (src, _h) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
        let rx = src.subscribe();
        assert_eq!(*rx.borrow(), EndpointSnapshot::NoPrimary);
    }

    #[tokio::test]
    async fn scripted_source_publishes_updates() {
        let (src, h) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
        let mut rx = src.subscribe();
        h.set(EndpointSnapshot::Primary(HostPort::new("a", 1)));
        rx.changed().await.unwrap();
        assert_eq!(
            *rx.borrow(),
            EndpointSnapshot::Primary(HostPort::new("a", 1))
        );
    }

    /// A lagging receiver observes only the newest value: this is the
    /// coalescing guarantee FR-004 needs, provided structurally by `watch`.
    #[tokio::test]
    async fn rapid_updates_coalesce_to_latest_for_a_lagging_receiver() {
        let (src, h) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
        let mut rx = src.subscribe();

        h.set(EndpointSnapshot::Primary(HostPort::new("first", 1)));
        h.set(EndpointSnapshot::Primary(HostPort::new("second", 2)));
        h.set(EndpointSnapshot::Primary(HostPort::new("third", 3)));

        rx.changed().await.unwrap();
        assert_eq!(
            *rx.borrow_and_update(),
            EndpointSnapshot::Primary(HostPort::new("third", 3)),
            "a lagging receiver must observe only the latest value"
        );
        // Nothing further is queued.
        assert!(!rx.has_changed().unwrap());
    }

    #[tokio::test]
    async fn multiple_subscribers_each_observe_the_change() {
        let (src, h) = ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary);
        let mut a = src.subscribe();
        let mut b = src.subscribe();
        h.set(EndpointSnapshot::NotFound);
        a.changed().await.unwrap();
        b.changed().await.unwrap();
        assert_eq!(*a.borrow(), EndpointSnapshot::NotFound);
        assert_eq!(*b.borrow(), EndpointSnapshot::NotFound);
    }
}
