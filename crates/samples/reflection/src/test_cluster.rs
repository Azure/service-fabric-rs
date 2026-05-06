// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Reusable test harness for `ReplicaControl`-based integration tests.
//!
//! Every test against the reflection sample's gRPC control plane needs
//! the same plumbing:
//!
//! - Dial a candidate set of `28000..=28004` ports, with re-dial on
//!   each poll iteration so newly-activated nodes are picked up.
//! - Poll `ListPending` across all reachable nodes until a gate of the
//!   expected kind appears (the test driver doesn't know up-front
//!   which node SF placed the replica on).
//! - Wait for / approve specific gates by `(partition_id, replica_id,
//!   gate_id)`.
//! - Best-effort release of leftover gates from prior failed runs.
//!
//! This module factors that out so individual test files can focus on
//! the lifecycle scenario being verified. Add a new test by copying
//! [`tests/control_e2e.rs`](../../tests/control_e2e.rs) and changing
//! only the gate sequence at the bottom.
//!
//! The cluster's hostname comes from `$REFLECTION_CLUSTER_HOST`. The
//! compile-time default is platform-specific:
//! - Windows → `"localhost"` (SF onebox runs on the same host).
//! - Unix → `"onebox"` (the sibling-container DNS name in the
//!   `.devcontainer/*` docker-compose setup).
//!
//! Override the env var for any other topology.

use std::sync::OnceLock;
use std::time::Duration;

use mssf_core::WString;
use mssf_core::client::FabricClient;
use mssf_core::types::{
    ServicePartitionInformation, ServicePartitionQueryDescription, ServicePartitionQueryResultItem,
    Uri,
};
use tonic::transport::{Channel, Endpoint};

use crate::grpc_control::REFLECTION_CONTROL_BASE_PORT;
use crate::grpc_control::proto::{
    ApprovalEvent, ApprovalKind, ApproveRequest, Empty, ListPendingRequest, ReplicaRef,
    approve_request::Decision as ApproveDecisionOneof, list_pending_request::ReplicaFilter,
    replica_control_client::ReplicaControlClient,
};

/// Number of candidate node ports the harness will dial.
pub const NODE_COUNT: u16 = 5;

/// Default poll backoff between `ListPending` iterations.
pub const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// Default total time `poll_for_pending` will wait before panicking.
pub const POLL_BUDGET: Duration = Duration::from_secs(30);

/// Hostname-or-IP that resolves to the cluster.
///
/// Compile-time default:
/// - Windows → `"localhost"` (SF onebox runs on the same host).
/// - Unix → `"onebox"` (the docker-compose / devcontainer hostname).
///
/// Override at runtime with `REFLECTION_CLUSTER_HOST`.
pub fn cluster_host() -> String {
    #[cfg(windows)]
    const DEFAULT_HOST: &str = "localhost";
    #[cfg(not(windows))]
    const DEFAULT_HOST: &str = "onebox";
    std::env::var("REFLECTION_CLUSTER_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

/// Process-wide [`FabricClient`] connection string.
///
/// Tests connect to the SF gateway via the `localhost:19000` client
/// endpoint. In the devcontainer this is forwarded to the `onebox`
/// sibling container's gateway. Override with `SF_CLIENT_ENDPOINT`
/// for non-default topologies.
pub fn fabric_client_endpoint() -> WString {
    WString::from(
        std::env::var("SF_CLIENT_ENDPOINT")
            .unwrap_or_else(|_| "localhost:19000".to_string())
            .as_str(),
    )
}

/// Process-wide [`FabricClient`] cached on first use.
///
/// Constructing a `FabricClient` does a non-trivial COM dance and
/// blocks until the gateway is reachable; constructing one per test
/// adds noticeable overhead when many tests run in parallel against
/// the same onebox cluster. Tests should call [`fabric_client`] (or
/// [`fabric_client_clone`]) instead of building their own.
///
/// `FabricClient` is `Clone` and the COM handles inside are reference
/// counted, so cloning the cached instance is cheap.
static FABRIC_CLIENT: OnceLock<FabricClient> = OnceLock::new();

/// Borrow the process-wide [`FabricClient`], building it on first call.
///
/// Builds the client with the connection string from
/// [`fabric_client_endpoint`]. Panics if the build fails — the SF
/// runtime is a hard prerequisite for any test that uses this
/// helper, so a failure to build is not a recoverable test
/// condition.
pub fn fabric_client() -> &'static FabricClient {
    FABRIC_CLIENT.get_or_init(|| {
        FabricClient::builder()
            .with_connection_strings(vec![fabric_client_endpoint()])
            .build()
            .expect("failed to build FabricClient")
    })
}

/// Clone of the process-wide [`FabricClient`]. Equivalent to
/// `fabric_client().clone()` but reads more naturally at call sites
/// that want owned values (e.g. moving into a tokio task).
pub fn fabric_client_clone() -> FabricClient {
    fabric_client().clone()
}

/// Look up the (Singleton) `partition_id` for `service_name` via the
/// SF query manager, retrying until SF surfaces the partition (it is
/// not necessarily visible the instant `create_service` returns).
///
/// Returns the partition_id formatted exactly as the
/// `ReplicaControl` proto expects (`{:?}` of `mssf_core::GUID`,
/// uppercase dashed, no braces) so the result can be passed straight
/// into [`Cluster::partition_driver`].
///
/// Tests must use this instead of [`Cluster::poll_for_pending`] when
/// they may run in parallel with other tests on the same cluster:
/// cluster-wide polling can otherwise grab another test's `Open`
/// gate by mistake.
pub async fn discover_partition_id(fc: &FabricClient, service_name: &Uri) -> String {
    const QUERY_TIMEOUT: Duration = Duration::from_secs(10);
    const POLL_BACKOFF: Duration = Duration::from_millis(250);
    const POLL_BUDGET: Duration = Duration::from_secs(30);

    let q = fc.get_query_manager();
    let desc = ServicePartitionQueryDescription {
        service_name: service_name.clone(),
        partition_id_filter: None,
    };

    let deadline = std::time::Instant::now() + POLL_BUDGET;
    loop {
        match q.get_partition_list(&desc, QUERY_TIMEOUT, None).await {
            Ok(list) => {
                let pid = list
                    .service_partitions
                    .into_iter()
                    .filter_map(|p| match p {
                        ServicePartitionQueryResultItem::Stateful(s) => {
                            match s.partition_information {
                                ServicePartitionInformation::Singleton(info) => Some(info.id),
                                _ => None,
                            }
                        }
                        _ => None,
                    })
                    .next();
                if let Some(guid) = pid {
                    return format!("{guid:?}");
                }
            }
            Err(e) => tracing::debug!("get_partition_list({service_name}): {e}; retrying"),
        }
        if std::time::Instant::now() >= deadline {
            panic!(
                "no Singleton partition for {service_name} within {POLL_BUDGET:?}: \
                 service may not have been created or is not stateful Singleton"
            );
        }
        tokio::time::sleep(POLL_BACKOFF).await;
    }
}

/// Holds one connection slot per candidate `ReplicaControl` port.
/// Reconnects lazily so a node that activates the reflection sample
/// *after* the test starts (e.g., when SF places a fresh primary on a
/// previously idle node) becomes reachable on the next [`Cluster::ensure`]
/// call without the test having to retry the whole startup loop.
pub struct Cluster {
    host: String,
    clients: Vec<Option<ReplicaControlClient<Channel>>>,
    connect_timeout: Duration,
}

impl Cluster {
    /// Build an empty cluster handle pointing at `cluster_host()`. No
    /// connections happen until [`Cluster::ensure`] is called.
    pub fn new() -> Self {
        Self::with_host(cluster_host())
    }

    pub fn with_host(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            clients: (0..NODE_COUNT).map(|_| None).collect(),
            connect_timeout: Duration::from_millis(500),
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    /// Try to connect any slot that is currently `None`. Errors are
    /// suppressed because a node may not host the reflection sample
    /// yet (or ever).
    pub async fn ensure(&mut self) {
        for (i, slot) in self.clients.iter_mut().enumerate() {
            if slot.is_some() {
                continue;
            }
            let port = REFLECTION_CONTROL_BASE_PORT + i as u16;
            let url = format!("http://{}:{}", self.host, port);
            let endpoint = match Endpoint::from_shared(url.clone()) {
                Ok(e) => e
                    .connect_timeout(self.connect_timeout)
                    .timeout(Duration::from_secs(60)),
                Err(_) => continue,
            };
            match endpoint.connect().await {
                Ok(ch) => {
                    tracing::info!("connected to {url}");
                    *slot = Some(ReplicaControlClient::new(ch));
                }
                Err(e) => {
                    tracing::debug!("skip {url}: {e}");
                }
            }
        }
    }

    pub fn connected_count(&self) -> usize {
        self.clients.iter().filter(|c| c.is_some()).count()
    }

    /// Iterate over all currently-connected `(node_index, client)`.
    pub fn iter_connected_mut(
        &mut self,
    ) -> impl Iterator<Item = (usize, &mut ReplicaControlClient<Channel>)> {
        self.clients
            .iter_mut()
            .enumerate()
            .filter_map(|(i, c)| c.as_mut().map(|client| (i, client)))
    }

    /// Get a mutable reference to the client for `idx`. Panics if the
    /// slot is not connected — callers that already discovered `idx`
    /// via [`Cluster::poll_for_pending`] are guaranteed to find it
    /// connected.
    pub fn client_mut(&mut self, idx: usize) -> &mut ReplicaControlClient<Channel> {
        self.clients[idx]
            .as_mut()
            .unwrap_or_else(|| panic!("cluster slot {idx} is not connected"))
    }

    /// Drop a stale connection so the next [`Cluster::ensure`] reconnects.
    pub fn invalidate(&mut self, idx: usize) {
        if let Some(slot) = self.clients.get_mut(idx) {
            *slot = None;
        }
    }

    /// Find a pending gate of `expected_kind` on any reachable node,
    /// or any pending gate if `expected_kind` is `None`. Polls
    /// `ListPending` with backoff up to [`POLL_BUDGET`]. Re-dials
    /// any newly-activated nodes on every iteration so a replica
    /// placed on a previously-idle node is discovered without extra
    /// retry logic in the caller.
    ///
    /// Cluster-wide (no partition filter); use this for the very
    /// first OPEN before the partition_id is known. After that, build
    /// a [`PartitionDriver`] and use its `wait_*` methods.
    ///
    /// Panics if no matching gate appears within [`POLL_BUDGET`].
    pub async fn poll_for_pending(
        &mut self,
        expected_kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        let deadline = std::time::Instant::now() + POLL_BUDGET;
        loop {
            self.ensure().await;
            let mut to_invalidate = Vec::new();
            for (i, client) in self.iter_connected_mut() {
                let resp = match client
                    .list_pending(ListPendingRequest {
                        partition_id: String::new(),
                        replica_filter: None,
                    })
                    .await
                {
                    Ok(r) => r,
                    Err(_) => {
                        to_invalidate.push(i);
                        continue;
                    }
                };
                for ev in resp.into_inner().events {
                    let kind_matches = match expected_kind {
                        None => true,
                        Some(k) => ev.kind == k as i32,
                    };
                    if kind_matches {
                        return (i, ev);
                    }
                }
            }
            for i in to_invalidate {
                self.invalidate(i);
            }
            if std::time::Instant::now() >= deadline {
                panic!(
                    "no pending gate of kind {expected_kind:?} found within {POLL_BUDGET:?} \
                     (connected_nodes={})",
                    self.connected_count(),
                );
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    /// Approve every pending gate on every reachable node with
    /// `Decision::Proceed`. Returns the count of gates released.
    ///
    /// **Operator-style cleanup, not for routine test setup.** Tests
    /// running in parallel could each see the others' gates and
    /// approve them prematurely. Prefer `remove_test_apps.sh` (which
    /// calls `reflection_ctl approve-all`) between cluster lifecycles,
    /// and unique service names per run for isolation.
    pub async fn approve_all_pending(&mut self) -> usize {
        let mut released = 0;
        let mut to_invalidate = Vec::new();
        for (i, client) in self.iter_connected_mut() {
            let resp = match client
                .list_pending(ListPendingRequest {
                    partition_id: String::new(),
                    replica_filter: None,
                })
                .await
            {
                Ok(r) => r,
                Err(_) => {
                    to_invalidate.push(i);
                    continue;
                }
            };
            for ev in resp.into_inner().events {
                let target = match ev.target.clone() {
                    Some(t) => t,
                    None => continue,
                };
                tracing::warn!(
                    "releasing leftover gate kind={} partition={} replica={} gate_id={}",
                    ev.kind,
                    target.partition_id,
                    target.replica_id,
                    ev.gate_id,
                );
                let _ = client
                    .approve(ApproveRequest {
                        target: Some(target),
                        gate_id: ev.gate_id,
                        decision: Some(ApproveDecisionOneof::Proceed(Empty {})),
                    })
                    .await;
                released += 1;
            }
        }
        for i in to_invalidate {
            self.invalidate(i);
        }
        released
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Self::new()
    }
}

impl Cluster {
    /// Build a [`PartitionDriver`] for the given partition. The driver
    /// exposes manual primitives (`wait_next`, `wait_next_kind`,
    /// `wait_new_replica`, `wait_for_replica`, `approve_proceed`,
    /// `approve_fail`) that the test composes explicitly.
    ///
    /// The driver does **not** assume a single linear gate stream per
    /// partition. SF brings replicas up serially, but later gates from
    /// different replicas in the same partition interleave. The test
    /// is responsible for routing each gate to the appropriate
    /// per-replica logic.
    pub fn partition_driver(&mut self, partition_id: impl Into<String>) -> PartitionDriver<'_> {
        PartitionDriver {
            cluster: self,
            partition_id: partition_id.into(),
        }
    }
}

// ----------------------------------------------------------------------
// PartitionDriver — partition-scoped manual primitives
// ----------------------------------------------------------------------

/// Partition-scoped handle for waiting on and approving gates within
/// one partition. Provides both manual primitives and a per-replica
/// sequence helper.
///
/// ## Single replica
///
/// After discovering the replica's first gate via `wait_*` (or via
/// the cluster-wide [`Cluster::poll_for_pending`]), drive the rest of
/// its lifecycle with [`PartitionDriver::drive_replica_sequence`] or
/// [`PartitionDriver::approve_replica_sequence`].
///
/// ## Multiple replicas
///
/// SF brings replicas up serially (replica 1's Open parks before SF
/// starts replica 2), but **post-Open lifecycle gates can interleave
/// across replicas** with no SF-guaranteed order. Sequences are
/// per-replica precisely so the test does not bake a cross-replica
/// race into its expectations.
///
/// ```ignore
/// // 1. Discover replica 1.
/// let (n, ev1) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
/// let r1 = ev1.target.as_ref().unwrap().replica_id;
/// driver.approve_proceed(n, ev1.target.clone().unwrap(), ev1.gate_id.clone()).await;
///
/// // 2. Discover replica 2 (any open from a different replica).
/// let (n, ev2) = driver.wait_new_replica(&[r1], Some(ApprovalKind::ApprovalOpen)).await;
/// let r2 = ev2.target.as_ref().unwrap().replica_id;
/// driver.approve_proceed(n, ev2.target.clone().unwrap(), ev2.gate_id.clone()).await;
///
/// // 3. Drive the rest concurrently. Two PartitionDrivers built from
/// //    the same cluster cannot coexist (mutable borrow); use two
/// //    cluster handles, or drive sequentially when ordering is OK.
/// driver.drive_replica_sequence(r1, &[
///     TestStep::proceed(ApprovalKind::ApprovalChangeRole), // -> Primary
///     TestStep::proceed(ApprovalKind::ApprovalClose),
/// ]).await;
/// driver.drive_replica_sequence(r2, &[
///     TestStep::proceed(ApprovalKind::ApprovalChangeRole), // -> Secondary
///     TestStep::proceed(ApprovalKind::ApprovalClose),
/// ]).await;
/// ```
///
/// ## Rebuild handling
///
/// SF may rebuild a replica with a different `replica_id` (e.g. after
/// `change_role` returns Err — see `tests/fail_change_role.rs`).
/// `drive_replica_sequence` is pinned to a single `replica_id`, so a
/// rebuild ends the sequence; the test re-discovers the new
/// `replica_id` via `wait_*` and starts a fresh sequence against it.
pub struct PartitionDriver<'a> {
    cluster: &'a mut Cluster,
    partition_id: String,
}

impl<'a> PartitionDriver<'a> {
    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }

    /// Wait for any pending gate in this partition (any replica, any kind).
    pub async fn wait_next(&mut self) -> (usize, ApprovalEvent) {
        self.poll_partition(None, &[], None).await
    }

    /// Wait for the next pending gate of `kind` in this partition (any replica).
    pub async fn wait_next_kind(&mut self, kind: ApprovalKind) -> (usize, ApprovalEvent) {
        self.poll_partition(Some(kind), &[], None).await
    }

    /// Wait for a pending gate from a replica whose `replica_id` is
    /// *not* in `seen`. Use this to discover the next replica that SF
    /// brings up after the previously-known ones; pass
    /// `Some(ApprovalKind::ApprovalOpen)` when waiting for a fresh
    /// activation.
    pub async fn wait_new_replica(
        &mut self,
        seen: &[i64],
        kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        self.poll_partition(kind, seen, None).await
    }

    /// Wait for a pending gate from a specific `replica_id`. `kind`
    /// filters the gate kind; `None` accepts any kind.
    pub async fn wait_for_replica(
        &mut self,
        replica_id: i64,
        kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        self.poll_partition(kind, &[], Some(replica_id)).await
    }

    /// Approve a previously-discovered gate with `Decision::Proceed`.
    /// `node_idx`, `target` and `gate_id` come from the matching
    /// `wait_*` return value.
    pub async fn approve_proceed(&mut self, node_idx: usize, target: ReplicaRef, gate_id: String) {
        self.cluster
            .client_mut(node_idx)
            .approve(ApproveRequest {
                target: Some(target),
                gate_id: gate_id.clone(),
                decision: Some(ApproveDecisionOneof::Proceed(Empty {})),
            })
            .await
            .unwrap_or_else(|s| panic!("Approve(gate_id={gate_id}) failed: {s}"));
    }

    /// Approve a previously-discovered gate with `Decision::Fail(message)`.
    /// Note: `Approval::Abort` rejects `fail_message` with
    /// `InvalidArgument` (SF's `abort` cannot fail).
    pub async fn approve_fail(
        &mut self,
        node_idx: usize,
        target: ReplicaRef,
        gate_id: String,
        message: String,
    ) {
        self.cluster
            .client_mut(node_idx)
            .approve(ApproveRequest {
                target: Some(target),
                gate_id: gate_id.clone(),
                decision: Some(ApproveDecisionOneof::FailMessage(message.clone())),
            })
            .await
            .unwrap_or_else(|s| {
                panic!("Approve(gate_id={gate_id}, fail_message={message:?}) failed: {s}")
            });
    }

    /// Snapshot of every pending gate in this partition across all
    /// reachable nodes. Useful for assertions like "no replica is
    /// parked after teardown".
    pub async fn list_pending(&mut self) -> Vec<ApprovalEvent> {
        let mut out = Vec::new();
        let mut to_invalidate = Vec::new();
        let pid = self.partition_id.clone();
        for (i, client) in self.cluster.iter_connected_mut() {
            match client
                .list_pending(ListPendingRequest {
                    partition_id: pid.clone(),
                    replica_filter: None,
                })
                .await
            {
                Ok(r) => out.extend(r.into_inner().events),
                Err(_) => to_invalidate.push(i),
            }
        }
        for i in to_invalidate {
            self.cluster.invalidate(i);
        }
        out
    }

    /// Drive a fixed sequence of gates against one specific replica.
    /// Each step polls `ListPending` with the server-side
    /// `replica_filter` so it only matches gates from `replica_id`,
    /// then approves with the step's decision.
    ///
    /// Per-replica (not partition-wide) by design: SF makes no
    /// guarantees about cross-replica ordering once both replicas in
    /// a partition are open, so a single tagged sequence covering
    /// multiple replicas would bake a race into the test. To drive
    /// two replicas concurrently, run two `drive_replica_sequence`
    /// futures from separate `PartitionDriver`s and `tokio::join!`
    /// them.
    ///
    /// Strict on kind matching — panics with actual vs. expected if
    /// SF's lifecycle changes shape.
    pub async fn drive_replica_sequence(
        &mut self,
        replica_id: i64,
        steps: &[TestStep],
    ) -> Vec<ApprovalEvent> {
        let mut observed = Vec::with_capacity(steps.len());
        for (i, step) in steps.iter().enumerate() {
            let (node_idx, ev) = self
                .poll_partition(Some(step.expected), &[], Some(replica_id))
                .await;
            let actual =
                ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
            assert_eq!(
                actual, step.expected,
                "drive_replica_sequence step {i} (replica={replica_id}): \
                 expected {:?}, got {actual:?} (gate_id={})",
                step.expected, ev.gate_id,
            );
            let target = ev
                .target
                .clone()
                .expect("ApprovalEvent.target on partition-scoped poll");
            let gate_id = ev.gate_id.clone();
            match &step.decision {
                TestDecision::Proceed => self.approve_proceed(node_idx, target, gate_id).await,
                TestDecision::Fail(msg) => {
                    self.approve_fail(node_idx, target, gate_id, msg.clone())
                        .await
                }
            }
            observed.push(ev);
        }
        observed
    }

    /// Convenience for the all-`Proceed` case of
    /// [`PartitionDriver::drive_replica_sequence`].
    pub async fn approve_replica_sequence(
        &mut self,
        replica_id: i64,
        kinds: &[ApprovalKind],
    ) -> Vec<ApprovalEvent> {
        let steps: Vec<TestStep> = kinds
            .iter()
            .map(|k| TestStep::new(*k, TestDecision::Proceed))
            .collect();
        self.drive_replica_sequence(replica_id, &steps).await
    }

    /// Internal polling loop with combined filters. Polls
    /// `ListPending` across every reachable node with backoff up to
    /// [`POLL_BUDGET`].
    async fn poll_partition(
        &mut self,
        kind_filter: Option<ApprovalKind>,
        exclude_replicas: &[i64],
        require_replica: Option<i64>,
    ) -> (usize, ApprovalEvent) {
        let pid = self.partition_id.clone();
        let deadline = std::time::Instant::now() + POLL_BUDGET;
        // Server-side replica_filter when we know the exact id.
        let replica_filter = require_replica.map(ReplicaFilter::SpecificReplicaId);
        loop {
            self.cluster.ensure().await;
            let mut to_invalidate = Vec::new();
            for (i, client) in self.cluster.iter_connected_mut() {
                let resp = match client
                    .list_pending(ListPendingRequest {
                        partition_id: pid.clone(),
                        replica_filter,
                    })
                    .await
                {
                    Ok(r) => r,
                    Err(_) => {
                        to_invalidate.push(i);
                        continue;
                    }
                };
                for ev in resp.into_inner().events {
                    if let Some(k) = kind_filter
                        && ev.kind != k as i32
                    {
                        continue;
                    }
                    let rid = ev.target.as_ref().map(|t| t.replica_id).unwrap_or(0);
                    if exclude_replicas.contains(&rid) {
                        continue;
                    }
                    return (i, ev);
                }
            }
            for i in to_invalidate {
                self.cluster.invalidate(i);
            }
            if std::time::Instant::now() >= deadline {
                panic!(
                    "no pending gate (partition={pid}, kind={kind_filter:?}, \
                     exclude={exclude_replicas:?}, require={require_replica:?}) \
                     within {POLL_BUDGET:?} (connected_nodes={})",
                    self.cluster.connected_count(),
                );
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }
}

/// Decision passed to [`PartitionDriver::drive_replica_sequence`]. A
/// test-side mirror of the proto `Decision` oneof so callers don't
/// have to import generated types.
#[derive(Debug, Clone)]
pub enum TestDecision {
    Proceed,
    Fail(String),
}

/// One entry in a [`PartitionDriver::drive_replica_sequence`] step list.
#[derive(Debug, Clone)]
pub struct TestStep {
    pub expected: ApprovalKind,
    pub decision: TestDecision,
}

impl TestStep {
    pub fn new(expected: ApprovalKind, decision: TestDecision) -> Self {
        Self { expected, decision }
    }

    pub fn proceed(expected: ApprovalKind) -> Self {
        Self::new(expected, TestDecision::Proceed)
    }

    pub fn fail(expected: ApprovalKind, message: impl Into<String>) -> Self {
        Self::new(expected, TestDecision::Fail(message.into()))
    }
}
