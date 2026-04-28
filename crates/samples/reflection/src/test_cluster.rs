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
//! The cluster's hostname comes from `$REFLECTION_CLUSTER_HOST`,
//! defaulting to `"onebox"` (the sibling-container DNS name in the
//! Linux devcontainer). Override the env var for non-devcontainer
//! topologies.

use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::grpc_control::REFLECTION_CONTROL_BASE_PORT;
use crate::grpc_control::proto::{
    ApprovalEvent, ApprovalKind, ApproveRequest, Empty, ListPendingRequest, ReplicaRef,
    WaitForApprovalRequest, approve_request::Decision as ApproveDecisionOneof,
    replica_control_client::ReplicaControlClient,
};

/// Number of candidate node ports the harness will dial.
pub const NODE_COUNT: u16 = 5;

/// Default poll backoff between `ListPending` iterations.
pub const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// Default total time `poll_for_pending` will wait before panicking.
pub const POLL_BUDGET: Duration = Duration::from_secs(30);

/// gRPC `WaitForApproval` deadline sent to the server.
pub const WAIT_FOR_APPROVAL_TIMEOUT_MS: u32 = 30_000;

/// Hostname-or-IP that resolves to the cluster. Defaults to `"onebox"`
/// (the docker-compose / devcontainer hostname). Override with
/// `REFLECTION_CLUSTER_HOST` for non-devcontainer setups.
pub fn cluster_host() -> String {
    std::env::var("REFLECTION_CLUSTER_HOST").unwrap_or_else(|_| "onebox".to_string())
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
    /// Panics if no matching gate appears within [`POLL_BUDGET`].
    pub async fn poll_for_pending(
        &mut self,
        expected_kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        self.poll_for_pending_inner(None, expected_kind).await
    }

    /// Like [`Cluster::poll_for_pending`] but only matches gates whose
    /// target's `partition_id` equals `partition_id`. Useful when SF
    /// rebuilds a replica after a failed `change_role` (the
    /// `replica_id` changes but the partition does not), or when the
    /// test must coexist with parallel test runs in the same cluster
    /// (each test scopes its polling to its own partition).
    pub async fn poll_for_pending_in_partition(
        &mut self,
        partition_id: &str,
        expected_kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        self.poll_for_pending_inner(Some(partition_id), expected_kind)
            .await
    }

    async fn poll_for_pending_inner(
        &mut self,
        partition_id: Option<&str>,
        expected_kind: Option<ApprovalKind>,
    ) -> (usize, ApprovalEvent) {
        let deadline = std::time::Instant::now() + POLL_BUDGET;
        loop {
            self.ensure().await;
            let mut to_invalidate = Vec::new();
            for (i, client) in self.iter_connected_mut() {
                let resp = match client
                    .list_pending(ListPendingRequest {
                        partition_id: partition_id.unwrap_or("").to_string(),
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
                     (partition_filter={partition_id:?}, connected_nodes={})",
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
    /// Build a [`ReplicaClient`] handle pointing at one specific
    /// replica. The returned handle borrows the cluster mutably; it
    /// owns the `(node_idx, target)` pair and exposes every per-replica
    /// operation (`wait_for_gate`, `approve_proceed`, etc.) as methods.
    ///
    /// Typical flow after [`Cluster::poll_for_pending`]:
    ///
    /// ```ignore
    /// let (node_idx, ev) = cluster.poll_for_pending(Some(ApprovalKind::ApprovalOpen)).await;
    /// let target = ev.target.clone().unwrap();
    /// let mut replica = cluster.replica_client(node_idx, target);
    /// replica.approve_proceed(ev.gate_id).await;
    /// let cr = replica.observe_and_approve(ApprovalKind::ApprovalChangeRole).await;
    /// ```
    pub fn replica_client(&mut self, node_idx: usize, target: ReplicaRef) -> ReplicaClient<'_> {
        ReplicaClient {
            cluster: self,
            node_idx,
            target,
        }
    }
}

// ----------------------------------------------------------------------
// ReplicaClient — per-replica handle borrowing the cluster.
// ----------------------------------------------------------------------

/// Borrowing handle that pins a single `(node_idx, replica_target)` and
/// exposes every per-replica operation as a method, so tests don't
/// have to thread `node_idx` and `target` through every call.
///
/// One `ReplicaClient` may exist at a time per `Cluster` (it holds an
/// exclusive borrow). Tests that need to drive multiple replicas
/// concurrently should keep `(node_idx, target)` pairs and re-acquire
/// the handle via [`Cluster::replica_client`] for each operation.
pub struct ReplicaClient<'a> {
    cluster: &'a mut Cluster,
    node_idx: usize,
    target: ReplicaRef,
}

impl<'a> ReplicaClient<'a> {
    pub fn node_idx(&self) -> usize {
        self.node_idx
    }

    pub fn target(&self) -> &ReplicaRef {
        &self.target
    }

    fn client(&mut self) -> &mut ReplicaControlClient<Channel> {
        self.cluster.client_mut(self.node_idx)
    }

    /// Wait for a *specific* gate kind on this replica.
    pub async fn wait_for_gate(&mut self, expected_kind: ApprovalKind) -> ApprovalEvent {
        let target = self.target.clone();
        let resp = self
            .client()
            .wait_for_approval(WaitForApprovalRequest {
                target: Some(target),
                timeout_ms: WAIT_FOR_APPROVAL_TIMEOUT_MS,
                expected: expected_kind as i32,
            })
            .await
            .unwrap_or_else(|s| panic!("WaitForApproval({expected_kind:?}) failed: {s}"));
        resp.into_inner()
    }

    /// Wait for the *next* gate of any kind on this replica. Useful
    /// during teardown where SF may issue `change_role(None)` before
    /// `close` and the test wants to drain whatever comes next.
    pub async fn wait_for_any_gate(&mut self) -> ApprovalEvent {
        let target = self.target.clone();
        let resp = self
            .client()
            .wait_for_approval(WaitForApprovalRequest {
                target: Some(target),
                timeout_ms: WAIT_FOR_APPROVAL_TIMEOUT_MS,
                expected: ApprovalKind::ApprovalUnspecified as i32,
            })
            .await
            .unwrap_or_else(|s| panic!("WaitForApproval(any) failed: {s}"));
        resp.into_inner()
    }

    /// Approve a specific gate with `Decision::Proceed`.
    pub async fn approve_proceed(&mut self, gate_id: String) {
        let target = self.target.clone();
        self.client()
            .approve(ApproveRequest {
                target: Some(target),
                gate_id: gate_id.clone(),
                decision: Some(ApproveDecisionOneof::Proceed(Empty {})),
            })
            .await
            .unwrap_or_else(|s| panic!("Approve(gate_id={gate_id}) failed: {s}"));
    }

    /// Approve a specific gate with `Decision::Fail(message)`. Note:
    /// `fail_message` is rejected with `InvalidArgument` for an
    /// `Approval::Abort` gate (SF's `abort` cannot fail).
    pub async fn approve_fail(&mut self, gate_id: String, message: String) {
        let target = self.target.clone();
        self.client()
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

    /// Wait for a gate of `expected_kind`, then approve it with
    /// `Decision::Proceed`. Returns the observed event so the test
    /// can inspect `new_role` etc.
    pub async fn observe_and_approve(&mut self, expected_kind: ApprovalKind) -> ApprovalEvent {
        let ev = self.wait_for_gate(expected_kind).await;
        let gate_id = ev.gate_id.clone();
        self.approve_proceed(gate_id).await;
        ev
    }

    /// Wait for the next gate of any kind, then approve with
    /// `Decision::Proceed`. Used during teardown loops where the
    /// caller doesn't care which gate fires next.
    pub async fn drain_next_gate(&mut self) -> ApprovalEvent {
        let ev = self.wait_for_any_gate().await;
        let gate_id = ev.gate_id.clone();
        self.approve_proceed(gate_id).await;
        ev
    }

    /// `ListPending` filtered to this replica's partition. Useful for
    /// "is the replica gone?" assertions after `Close` is approved.
    pub async fn list_pending_in_partition(&mut self) -> Vec<ApprovalEvent> {
        let partition_id = self.target.partition_id.clone();
        self.client()
            .list_pending(ListPendingRequest {
                partition_id,
                replica_filter: None,
            })
            .await
            .expect("ListPending failed")
            .into_inner()
            .events
    }
}
