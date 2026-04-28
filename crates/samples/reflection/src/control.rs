// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Test-driven control plane for replica lifecycle methods.
//!
//! See `docs/design/ReflectionReplicaTestControl.md` for the full design.
//!
//! - [`ReplicaController`] is the lifecycle hook trait. Every `Replica`
//!   holds an `Arc<dyn ReplicaController>` and calls
//!   `await_approval(...)` from inside `open` / `change_role` / `close`
//!   / `abort`.
//! - [`NoopController`] is the production path: every `await_approval`
//!   returns [`Decision::Proceed`] inline; never registered with the
//!   gRPC `ReplicaControl` server.
//! - [`GrpcController`] is the test path: lifecycle methods park on a
//!   oneshot until a gRPC `Approve` arrives. A `tokio::sync::Mutex`
//!   (`gate_lock`) held across the wait makes single-occupancy of the
//!   `pending` slot an enforced invariant.
//! - [`decode_init_data`] / [`ControlMode::from_init_data`] split
//!   wire-format decoding from policy mapping so each can be
//!   unit-tested in isolation.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};

use mssf_core::async_trait;
use mssf_core::types::ReplicaRole;
use prost::Message;
use tokio::sync::{Mutex as TokioMutex, Notify, oneshot};
use uuid::Uuid;

pub mod initdata_proto {
    tonic::include_proto!("reflection.initdata.v1");
}

/// Identifies which lifecycle gate the replica is currently waiting at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    Open,
    ChangeRole(ReplicaRole),
    Close,
    Abort,
}

/// What `await_approval` should return to its caller.
///
/// `Fail` is ignored for `Approval::Abort` because
/// `IStatefulServiceReplica::abort` returns `()` and cannot propagate
/// an error.
#[derive(Debug)]
pub enum Decision {
    Proceed,
    Fail(mssf_core::Error),
}

/// Lifecycle hook implemented by both the production
/// (`NoopController`) and test-driven (`GrpcController`) controllers.
#[async_trait]
pub trait ReplicaController: Send + Sync + std::fmt::Debug {
    /// Called by every lifecycle gate (`Open`, `ChangeRole`, `Close`,
    /// `Abort`). `NoopController` returns `Decision::Proceed`
    /// immediately; `GrpcController` blocks until a test sends
    /// `Approve` over gRPC. The sync `abort` call site bridges to
    /// this with `TokioExecutor::block_on_any`.
    async fn await_approval(&self, gate: Approval) -> Decision;

    /// Whether this controller should be registered with the
    /// `ReplicaControl` gRPC server. `NoopController` returns `false`
    /// (and therefore is invisible to test traffic); `GrpcController`
    /// returns `true`.
    fn is_controllable(&self) -> bool {
        false
    }

    /// Erased self-cast for the gRPC handler to downcast to a
    /// concrete controller type when it needs the inspection
    /// methods (`peek_pending`, `wait_for_approval`, `approve`).
    /// `NoopController` returns its own `&dyn Any`; downcasts that
    /// don't match yield `None` and the caller maps that to a clear
    /// gRPC status.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Production path. Stateless, constant-time, never registered with
/// the gRPC service.
#[derive(Debug, Default)]
pub struct NoopController;

#[async_trait]
impl ReplicaController for NoopController {
    async fn await_approval(&self, _gate: Approval) -> Decision {
        Decision::Proceed
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Decision sent on the controller's oneshot channel; produced by the
/// gRPC `Approve` handler or by `Drop`. Internal to this module.
type GateDecision = Decision;

/// One occupant of the controller's `pending` slot. Cleared either by
/// the gRPC `Approve` handler (which takes the `sender` and sends the
/// caller's decision) or by `Drop` (which sends `Decision::Proceed`).
#[derive(Debug)]
pub(crate) struct Pending {
    pub gate_id: Uuid,
    pub gate: Approval,
    sender: oneshot::Sender<GateDecision>,
}

/// Test-driven controller. See module docs for the state-machine
/// invariants.
///
/// State layout:
/// - `gate_lock` (tokio mutex) is held across the *entire* body of
///   `await_approval`, including the receiver `await`. This serializes
///   lifecycle gates: a second `await_approval` from a different
///   lifecycle method blocks at `gate_lock` until the first is
///   approved or the controller is dropped.
/// - `pending` (std mutex) is the observation slot read by the gRPC
///   handler. Held only briefly to publish or clear `Pending`; never
///   across an await.
/// - `notify` wakes any `WaitForApproval` handler when `pending`
///   transitions from `None` to `Some`.
///
/// Lock order is fixed: `gate_lock` outer, `pending` mutex inner.
#[derive(Debug)]
pub struct GrpcController {
    gate_lock: TokioMutex<()>,
    pending: StdMutex<Option<Pending>>,
    notify: Notify,
    /// Once set to `true`, all future `await_approval` calls return
    /// `Decision::Proceed` immediately and any currently-pending
    /// gate is released with `Decision::Proceed`. Irreversible by
    /// design: a test that wants "control phase, then cluster-driven
    /// teardown" calls `detach()` once and SF takes over.
    detached: AtomicBool,
}

impl GrpcController {
    pub fn new() -> Self {
        Self {
            gate_lock: TokioMutex::new(()),
            pending: StdMutex::new(None),
            notify: Notify::new(),
            detached: AtomicBool::new(false),
        }
    }

    /// Switch this controller into proceed-forever mode. After this
    /// call:
    /// - Any currently-pending gate is released with
    ///   `Decision::Proceed` so the parked lifecycle method unblocks
    ///   immediately.
    /// - All future `await_approval` calls short-circuit to
    ///   `Decision::Proceed` without ever touching `pending` or
    ///   `gate_lock`.
    ///
    /// Idempotent and safe to call concurrently.
    pub fn detach(&self) {
        self.detached.store(true, Ordering::SeqCst);
        if let Ok(mut slot) = self.pending.lock()
            && let Some(p) = slot.take()
        {
            let _ = p.sender.send(Decision::Proceed);
        }
        // Wake any handler currently parked in `wait_for_approval`
        // so it observes the now-empty slot and (typically) the
        // caller will then issue a fresh request that returns
        // NotFound once the registry entry is dropped.
        self.notify.notify_waiters();
    }

    pub fn is_detached(&self) -> bool {
        self.detached.load(Ordering::Relaxed)
    }

    /// Snapshot the currently pending gate for read-only use by the
    /// gRPC `WaitForApproval` / `ListPending` handlers. Returns
    /// `(gate_id, gate)`; does NOT consume the slot.
    pub fn peek_pending(&self) -> Option<(Uuid, Approval)> {
        let guard = self.pending.lock().unwrap();
        guard.as_ref().map(|p| (p.gate_id, p.gate))
    }

    /// Block until `pending` is populated and matches `expected` (or
    /// `expected` is `None`). Returns the `(gate_id, gate)` snapshot.
    /// Cancellation-safe: when the future is dropped, the slot is
    /// untouched (only `approve()` consumes the sender).
    pub async fn wait_for_approval(
        &self,
        expected: Option<ApprovalKindFilter>,
    ) -> (Uuid, Approval) {
        loop {
            // Register interest BEFORE inspecting state. tokio's Notify
            // delivers wake-ups to listeners that exist at the time of
            // `notify_waiters()`; the first poll of `notified` registers
            // the listener, so the structure here (construct, pin, peek,
            // await) ensures that any `await_approval` populate that
            // happens after the construction of `notified` will wake us.
            // Constructing `notified` AFTER the peek would reintroduce a
            // missed-wakeup window.
            let notified = self.notify.notified();
            tokio::pin!(notified);

            if let Some((gate_id, gate)) = self.peek_pending()
                && expected.is_none_or(|f| f.matches(gate))
            {
                return (gate_id, gate);
            }
            notified.as_mut().await;
        }
    }

    /// Consume the pending slot if `gate_id` matches; send `decision`
    /// on the stored oneshot. Returns the result of the id check so
    /// the gRPC handler can map to the right tonic Status.
    pub fn approve(&self, gate_id: Uuid, decision: Decision) -> ApproveResult {
        let mut guard = self.pending.lock().unwrap();
        let pending = match guard.take() {
            None => return ApproveResult::SlotEmpty,
            Some(p) => p,
        };
        if pending.gate_id != gate_id {
            // Put it back and tell the caller their id is stale.
            let returned_id = pending.gate_id;
            *guard = Some(pending);
            return ApproveResult::IdMismatch {
                pending_id: returned_id,
                requested_id: gate_id,
            };
        }
        // Drop the lock before sending so the await_approval body can
        // immediately reacquire it on the std mutex side. The receiver
        // is awaiting a oneshot; we don't need to hold the std mutex.
        drop(guard);
        // Receiver is `await`ing inside `await_approval`. If the future
        // has been dropped (e.g., the replica is being torn down right
        // now) the send returns Err; we discard it because there is
        // nobody to report to.
        let _ = pending.sender.send(decision);
        ApproveResult::Released
    }
}

impl Default for GrpcController {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReplicaController for GrpcController {
    async fn await_approval(&self, gate: Approval) -> Decision {
        // Fast path: if detach() has been called, every future gate
        // proceeds immediately without touching gate_lock or pending.
        if self.is_detached() {
            return Decision::Proceed;
        }

        // Acquire the serialization lock for the entire body of this
        // call. A concurrent await_approval (e.g., abort arriving while
        // close is parked) blocks here until we release.
        let _guard = self.gate_lock.lock().await;

        // Double-check after acquiring: detach() may have fired while
        // we were queued. Without this, a queued lifecycle method
        // would still publish a fresh gate after detach().
        if self.is_detached() {
            return Decision::Proceed;
        }

        let gate_id = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();
        {
            let mut slot = self.pending.lock().unwrap();
            // gate_lock guarantees the slot is empty here; debug-assert
            // is informational rather than a safety net.
            debug_assert!(
                slot.is_none(),
                "pending slot must be empty under gate_lock; got {slot:?}"
            );
            *slot = Some(Pending {
                gate_id,
                gate,
                sender: tx,
            });
        }
        self.notify.notify_waiters();

        // Wait for either an Approve (sender.send) or a Drop (sender
        // dropped without sending -> Err). Both unblock us; on Drop
        // we treat it as Proceed so the lifecycle method can complete
        // cleanup without making SF think the operation failed.
        let decision = rx.await.unwrap_or(Decision::Proceed);

        // Clear the slot. In the normal Approve path, `approve()`
        // already cleared it (took on send). In the Drop path,
        // nobody cleared it. Either way, leaving the slot None on
        // exit is the invariant.
        {
            let mut slot = self.pending.lock().unwrap();
            *slot = None;
        }
        decision
    }

    fn is_controllable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drop for GrpcController {
    fn drop(&mut self) {
        // Release any pending oneshot with Decision::Proceed so a
        // parked await_approval future can complete instead of hanging
        // when the controller goes away (Replica being dropped, etc.).
        // The receiver may itself already be dropped; ignore the send
        // error in that case.
        if let Ok(mut slot) = self.pending.lock()
            && let Some(p) = slot.take()
        {
            let _ = p.sender.send(Decision::Proceed);
        }
    }
}

/// Outcome of `GrpcController::approve` for the gRPC handler.
#[derive(Debug)]
pub enum ApproveResult {
    Released,
    SlotEmpty,
    IdMismatch {
        pending_id: Uuid,
        requested_id: Uuid,
    },
}

/// Filter predicate matching one or more gate kinds. Used by
/// `WaitForApproval`'s `expected` field.
#[derive(Debug, Clone, Copy)]
pub enum ApprovalKindFilter {
    Open,
    ChangeRole,
    Close,
    Abort,
}

impl ApprovalKindFilter {
    fn matches(self, gate: Approval) -> bool {
        matches!(
            (self, gate),
            (ApprovalKindFilter::Open, Approval::Open)
                | (ApprovalKindFilter::ChangeRole, Approval::ChangeRole(_))
                | (ApprovalKindFilter::Close, Approval::Close)
                | (ApprovalKindFilter::Abort, Approval::Abort)
        )
    }
}

// ----------------------------------------------------------------------
// Initdata: decode + policy mapping (split for independent testing)
// ----------------------------------------------------------------------

pub use initdata_proto::ReplicaInitData;

/// Decode the bytes SF passes to `create_replica`. Empty bytes or a
/// decode failure map to the default-valued `ReplicaInitData`, which
/// keeps the rest of the pipeline a pure function of the message.
pub fn decode_init_data(initdata: &[u8]) -> ReplicaInitData {
    if initdata.is_empty() {
        return ReplicaInitData::default();
    }
    match ReplicaInitData::decode(initdata) {
        Ok(msg) => msg,
        Err(e) => {
            tracing::warn!("failed to decode ReplicaInitData ({e}); using default");
            ReplicaInitData::default()
        }
    }
}

/// Policy mapping from a decoded `ReplicaInitData` to the runtime
/// controller mode. No bytes, no I/O, no logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    NoControl,
    Control,
}

impl ControlMode {
    pub fn from_init_data(msg: &ReplicaInitData) -> Self {
        if msg.control {
            ControlMode::Control
        } else {
            ControlMode::NoControl
        }
    }
}

/// Build a controller from the chosen mode. Used by `Factory::create_replica`.
pub fn make_controller(mode: ControlMode) -> Arc<dyn ReplicaController> {
    match mode {
        ControlMode::NoControl => Arc::new(NoopController),
        ControlMode::Control => Arc::new(GrpcController::new()),
    }
}

// ----------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // ---- decode_init_data / ControlMode -----------------------------

    #[test]
    fn decode_empty_bytes_yields_default_message() {
        let msg = decode_init_data(&[]);
        assert!(!msg.control);
        assert_eq!(ControlMode::from_init_data(&msg), ControlMode::NoControl);
    }

    #[test]
    fn decode_control_true_round_trips() {
        let bytes = ReplicaInitData { control: true }.encode_to_vec();
        let msg = decode_init_data(&bytes);
        assert!(msg.control);
        assert_eq!(ControlMode::from_init_data(&msg), ControlMode::Control);
    }

    #[test]
    fn decode_control_false_round_trips() {
        let bytes = ReplicaInitData { control: false }.encode_to_vec();
        let msg = decode_init_data(&bytes);
        assert!(!msg.control);
        assert_eq!(ControlMode::from_init_data(&msg), ControlMode::NoControl);
    }

    #[test]
    fn decode_garbage_bytes_falls_back_to_default() {
        let msg = decode_init_data(&[0xff, 0xff, 0xff, 0xff, 0xff]);
        assert!(!msg.control);
        assert_eq!(ControlMode::from_init_data(&msg), ControlMode::NoControl);
    }

    // ---- NoopController --------------------------------------------

    #[tokio::test]
    async fn noop_controller_proceeds_immediately() {
        let c = NoopController;
        for gate in [
            Approval::Open,
            Approval::ChangeRole(ReplicaRole::Primary),
            Approval::Close,
            Approval::Abort,
        ] {
            match c.await_approval(gate).await {
                Decision::Proceed => {}
                Decision::Fail(_) => panic!("noop should always proceed"),
            }
        }
        assert!(!c.is_controllable());
    }

    // ---- GrpcController happy path ---------------------------------

    #[tokio::test(flavor = "multi_thread")]
    async fn grpc_controller_approve_releases_pending() {
        let c = Arc::new(GrpcController::new());
        let c2 = c.clone();
        let parked = tokio::spawn(async move { c2.await_approval(Approval::Open).await });

        // Wait until the gate is observable.
        let (gate_id, gate) = c.wait_for_approval(Some(ApprovalKindFilter::Open)).await;
        assert_eq!(gate, Approval::Open);

        match c.approve(gate_id, Decision::Proceed) {
            ApproveResult::Released => {}
            other => panic!("expected Released, got {other:?}"),
        }

        let decision = parked.await.unwrap();
        assert!(matches!(decision, Decision::Proceed));
        assert!(c.peek_pending().is_none(), "slot should be cleared on exit");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn grpc_controller_approve_with_fail_propagates() {
        let c = Arc::new(GrpcController::new());
        let c2 = c.clone();
        let parked = tokio::spawn(async move {
            c2.await_approval(Approval::ChangeRole(ReplicaRole::Primary))
                .await
        });

        let (gate_id, _) = c
            .wait_for_approval(Some(ApprovalKindFilter::ChangeRole))
            .await;
        let err = mssf_core::Error::from(mssf_core::HRESULT(-1));
        c.approve(gate_id, Decision::Fail(err));

        let decision = parked.await.unwrap();
        assert!(matches!(decision, Decision::Fail(_)));
    }

    // ---- gate_id stale-approve protection --------------------------

    #[tokio::test(flavor = "multi_thread")]
    async fn approve_with_wrong_gate_id_returns_id_mismatch() {
        let c = Arc::new(GrpcController::new());
        let c2 = c.clone();
        let parked = tokio::spawn(async move { c2.await_approval(Approval::Open).await });

        let (real_gate_id, _) = c.wait_for_approval(None).await;
        let stale_gate_id = Uuid::new_v4();
        match c.approve(stale_gate_id, Decision::Proceed) {
            ApproveResult::IdMismatch {
                pending_id,
                requested_id,
            } => {
                assert_eq!(pending_id, real_gate_id);
                assert_eq!(requested_id, stale_gate_id);
            }
            other => panic!("expected IdMismatch, got {other:?}"),
        }
        // The replica is still parked because we didn't consume the slot.
        assert!(!parked.is_finished());
        assert!(c.peek_pending().is_some());

        // Real id still works.
        c.approve(real_gate_id, Decision::Proceed);
        let _ = parked.await.unwrap();
    }

    #[tokio::test]
    async fn approve_when_slot_empty_returns_slot_empty() {
        let c = GrpcController::new();
        match c.approve(Uuid::new_v4(), Decision::Proceed) {
            ApproveResult::SlotEmpty => {}
            other => panic!("expected SlotEmpty, got {other:?}"),
        }
    }

    // ---- gate_lock serialization (close -> abort queueing) ---------

    #[tokio::test(flavor = "multi_thread")]
    async fn second_lifecycle_call_blocks_on_gate_lock() {
        let c = Arc::new(GrpcController::new());
        let c1 = c.clone();
        let close = tokio::spawn(async move { c1.await_approval(Approval::Close).await });

        // Let close park.
        let (close_gate_id, _) = c.wait_for_approval(Some(ApprovalKindFilter::Close)).await;

        // Now spawn abort; it should queue on gate_lock, not race onto pending.
        let c2 = c.clone();
        let abort = tokio::spawn(async move { c2.await_approval(Approval::Abort).await });

        // Give the runtime a beat to actually let abort try to acquire.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // pending should still hold the close gate; abort hasn't published.
        let (still_pending_id, still_pending_gate) = c.peek_pending().unwrap();
        assert_eq!(still_pending_id, close_gate_id);
        assert_eq!(still_pending_gate, Approval::Close);
        assert!(!abort.is_finished());

        // Approve close. abort should then publish its own gate.
        c.approve(close_gate_id, Decision::Proceed);
        let _ = close.await.unwrap();

        // Abort gate eventually shows up.
        let (abort_gate_id, abort_gate) =
            c.wait_for_approval(Some(ApprovalKindFilter::Abort)).await;
        assert_eq!(abort_gate, Approval::Abort);
        assert_ne!(abort_gate_id, close_gate_id, "fresh UUID per gate");

        c.approve(abort_gate_id, Decision::Proceed);
        let _ = abort.await.unwrap();
    }

    // ---- WaitForApproval ordering races ----------------------------

    #[tokio::test(flavor = "multi_thread")]
    async fn wait_first_then_await_approval() {
        let c = Arc::new(GrpcController::new());
        let c1 = c.clone();
        let waiter = tokio::spawn(async move { c1.wait_for_approval(None).await });

        // Give the waiter a moment to park.
        tokio::time::sleep(Duration::from_millis(20)).await;

        let c2 = c.clone();
        let parked = tokio::spawn(async move { c2.await_approval(Approval::Close).await });

        let (gate_id, gate) = waiter.await.unwrap();
        assert_eq!(gate, Approval::Close);
        c.approve(gate_id, Decision::Proceed);
        let _ = parked.await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn await_approval_first_then_wait() {
        let c = Arc::new(GrpcController::new());
        let c1 = c.clone();
        let parked = tokio::spawn(async move { c1.await_approval(Approval::Close).await });

        // Give the parker time to publish.
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Now ask: WaitForApproval should see the populated pending on
        // the very first peek.
        let (gate_id, gate) = c.wait_for_approval(None).await;
        assert_eq!(gate, Approval::Close);
        c.approve(gate_id, Decision::Proceed);
        let _ = parked.await.unwrap();
    }

    // ---- Drop releases pending with Proceed ------------------------

    #[tokio::test(flavor = "multi_thread")]
    async fn dropping_controller_unblocks_parked_lifecycle() {
        let c = Arc::new(GrpcController::new());
        let c1 = c.clone();
        let parked = tokio::spawn(async move { c1.await_approval(Approval::Open).await });

        // Wait until the gate is published.
        let _ = c.wait_for_approval(None).await;

        // Drop the only Arcs we hold; the spawned task still has one
        // via its captured c1, so the controller stays alive long
        // enough for the await_approval future to be polled, but Drop
        // will fire when the spawned task finishes its await.
        // To force Drop synchronously, we instead approve via a direct
        // path: simulate Drop by taking the slot and explicitly
        // dropping its sender.
        {
            let mut slot = c.pending.lock().unwrap();
            // mimic Drop's behavior: take and send Proceed
            if let Some(p) = slot.take() {
                let _ = p.sender.send(Decision::Proceed);
            }
        }

        let decision = parked.await.unwrap();
        assert!(matches!(decision, Decision::Proceed));
    }

    // ---- ApprovalKindFilter ----------------------------------------

    #[test]
    fn approval_kind_filter_matches_correctly() {
        assert!(ApprovalKindFilter::Open.matches(Approval::Open));
        assert!(!ApprovalKindFilter::Open.matches(Approval::Close));
        assert!(ApprovalKindFilter::ChangeRole.matches(Approval::ChangeRole(ReplicaRole::Primary)));
        assert!(ApprovalKindFilter::ChangeRole.matches(Approval::ChangeRole(ReplicaRole::None)));
        assert!(ApprovalKindFilter::Close.matches(Approval::Close));
        assert!(ApprovalKindFilter::Abort.matches(Approval::Abort));
    }

    // ---- Detach -----------------------------------------------------

    #[tokio::test(flavor = "multi_thread")]
    async fn detach_releases_pending_and_skips_future_gates() {
        let c = Arc::new(GrpcController::new());

        // Park a gate.
        let c1 = c.clone();
        let parked = tokio::spawn(async move { c1.await_approval(Approval::Open).await });
        let _ = c.wait_for_approval(None).await;
        assert!(c.peek_pending().is_some());
        assert!(!c.is_detached());

        // Detach: the parked gate releases with Proceed and the slot
        // clears; controller flips into proceed-forever mode.
        c.detach();
        assert!(c.is_detached());
        let decision = parked.await.unwrap();
        assert!(matches!(decision, Decision::Proceed));

        // Future gates short-circuit without parking.
        let t0 = std::time::Instant::now();
        for gate in [
            Approval::ChangeRole(ReplicaRole::Primary),
            Approval::Close,
            Approval::Abort,
        ] {
            let d = c.await_approval(gate).await;
            assert!(matches!(d, Decision::Proceed));
        }
        // Sanity: no parking means each call is sub-millisecond.
        assert!(
            t0.elapsed() < Duration::from_millis(50),
            "post-detach await_approval should not park (took {:?})",
            t0.elapsed(),
        );
        // pending stays empty.
        assert!(c.peek_pending().is_none());
    }

    #[tokio::test]
    async fn detach_before_any_gate_is_idempotent() {
        let c = GrpcController::new();
        c.detach();
        c.detach(); // second call no-ops
        let d = c.await_approval(Approval::Open).await;
        assert!(matches!(d, Decision::Proceed));
        assert!(c.is_detached());
    }
}
