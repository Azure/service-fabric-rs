// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Per-instance lifecycle state machine for an
//! [`IStatefulServiceReplica`](mssf_core::runtime::IStatefulServiceReplica)
//! implementation.
//!
//! See [`Lifecycle`] for the design and rationale. The state machine
//! is exercised by `crate::statefulstore::Replica`'s lifecycle
//! methods (`open`, `change_role`, `close`, `abort`); contract
//! violations are converted to `E_UNEXPECTED` HRESULTs that propagate
//! back to SF, with a structured `tracing::error!` line capturing the
//! observed state for diagnosis.

use std::sync::Mutex;

/// Per-instance lifecycle state for an `IStatefulServiceReplica`.
///
/// Encodes the SF contract: a `Replica` is single-shot. The state
/// machine is:
///
/// ```text
///                       enter_opening              complete_open
///   Created ─────────────────────────► Opening ─────────────────► Active
///                                          │                         │
///                                          │ abort                   │ enter_closing
///                                          ▼                         ▼
///                                       Terminal ◄──────────────── Closing
///                                          ▲                         │
///                                          │ complete_close ─────────┘
///                                          │
///                                          │ abort (from Created/Active too)
///                                          │
///                                       Terminal
/// ```
///
/// Two intermediate states (`Opening`, `Closing`) record that the
/// corresponding lifecycle method is *in flight*. They matter for two
/// reasons:
///
/// 1. **Honest state on rejection.** If `open` or `close` returns
///    `Err` (e.g. controller decision is `Fail`), the state stays in
///    `Opening` / `Closing` rather than incorrectly advancing.
/// 2. **`abort` after a failed close still publishes its gate.** SF
///    may call `abort()` after `close()` returns `Err`. In that case
///    `Lifecycle::abort` sees state `Closing` (not `Terminal`),
///    returns `false`, and `Replica::abort` runs the abort gate so
///    the test can observe whether SF actually called abort.
///
/// After `Terminal`, SF must construct a new `Replica` via
/// `Factory::create_replica` for any further activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Just constructed; only `open` is valid.
    Created,
    /// `open` invoked, gate not yet returned `Ok`.
    Opening,
    /// `open` returned `Ok`; `change_role` and `close`/`abort` are valid.
    Active,
    /// `close` invoked, gate not yet returned `Ok`.
    Closing,
    /// `close` returned `Ok`, or `abort` ran. No further calls valid.
    Terminal,
}

/// Owns the [`LifecycleState`] for one `Replica` and exposes the
/// legal transitions as named methods. Each method takes the lock
/// for the duration of the state check so the transition is atomic.
///
/// On rejection the methods log a precise `tracing::error!` line
/// (with the partition / replica context captured at construction)
/// and return [`mssf_core::ErrorCode::E_UNEXPECTED`] for the caller
/// to propagate to SF directly via `?`. Lifecycle errors are treated
/// as a contract violation by SF, not a recoverable application
/// failure.
pub struct Lifecycle {
    partition_id: mssf_core::GUID,
    replica_id: i64,
    state: Mutex<LifecycleState>,
}

impl Lifecycle {
    pub fn new(partition_id: mssf_core::GUID, replica_id: i64) -> Self {
        Self {
            partition_id,
            replica_id,
            state: Mutex::new(LifecycleState::Created),
        }
    }

    /// Snapshot the current state. Test/diagnostic helper.
    pub fn current(&self) -> LifecycleState {
        *self.state.lock().unwrap()
    }

    /// `Created -> Opening` (entering the `open` path).
    ///
    /// Logs and returns `E_UNEXPECTED` if the instance has already
    /// been opened or is past it.
    pub fn enter_opening(&self) -> mssf_core::Result<()> {
        let mut s = self.state.lock().unwrap();
        match *s {
            LifecycleState::Created => {
                *s = LifecycleState::Opening;
                Ok(())
            }
            other => {
                drop(s);
                self.reject("open", other)
            }
        }
    }

    /// `Opening -> Active` (the open gate succeeded).
    ///
    /// Returns `E_UNEXPECTED` if not in `Opening`. In practice the
    /// caller is `Replica::open` which only reaches this after
    /// `enter_opening` succeeded, so a failure here would mean a
    /// concurrent abort raced ahead.
    pub fn complete_open(&self) -> mssf_core::Result<()> {
        let mut s = self.state.lock().unwrap();
        match *s {
            LifecycleState::Opening => {
                *s = LifecycleState::Active;
                Ok(())
            }
            other => {
                drop(s);
                self.reject("complete_open", other)
            }
        }
    }

    /// Verify the instance is `Active` (the `change_role` path).
    ///
    /// Returns `E_UNEXPECTED` if the instance is `Created`,
    /// `Opening`, `Closing`, or `Terminal`.
    pub fn require_active(&self) -> mssf_core::Result<()> {
        let s = *self.state.lock().unwrap();
        if s == LifecycleState::Active {
            Ok(())
        } else {
            self.reject("change_role", s)
        }
    }

    /// `Active -> Closing` (entering the `close` path).
    ///
    /// Returns `E_UNEXPECTED` if the instance is not `Active`.
    pub fn enter_closing(&self) -> mssf_core::Result<()> {
        let mut s = self.state.lock().unwrap();
        match *s {
            LifecycleState::Active => {
                *s = LifecycleState::Closing;
                Ok(())
            }
            other => {
                drop(s);
                self.reject("close", other)
            }
        }
    }

    /// `Closing -> Terminal` (the close gate succeeded).
    ///
    /// Returns `E_UNEXPECTED` if not in `Closing` (e.g. a concurrent
    /// abort raced ahead).
    pub fn complete_close(&self) -> mssf_core::Result<()> {
        let mut s = self.state.lock().unwrap();
        match *s {
            LifecycleState::Closing => {
                *s = LifecycleState::Terminal;
                Ok(())
            }
            other => {
                drop(s);
                self.reject("complete_close", other)
            }
        }
    }

    /// Force-transition to `Terminal` (the `abort` path). Returns
    /// `true` if the swap was a no-op (already `Terminal`), so the
    /// caller knows to skip the abort gate and cleanup. Logs a
    /// `tracing::warn!` in the no-op case; valid swaps from
    /// `Created`, `Opening`, `Active`, or `Closing` are silent (the
    /// caller logs its own info-level entry).
    ///
    /// Note that aborting from `Closing` is the path the test
    /// `fail_close_during_delete_*` exercises: a failed `close`
    /// leaves the state in `Closing`, so SF's subsequent `abort()`
    /// finds a non-terminal state and the abort gate fires.
    pub fn abort(&self) -> bool {
        let mut s = self.state.lock().unwrap();
        let prev = *s;
        *s = LifecycleState::Terminal;
        if prev == LifecycleState::Terminal {
            tracing::warn!(
                partition = ?self.partition_id,
                replica = self.replica_id,
                "abort called on already-terminal Replica; ignoring"
            );
            true
        } else {
            false
        }
    }

    /// Common rejection path: log the precise diagnosis, return
    /// `E_UNEXPECTED`.
    fn reject(&self, method: &'static str, observed: LifecycleState) -> mssf_core::Result<()> {
        tracing::error!(
            partition = ?self.partition_id,
            replica = self.replica_id,
            state = ?observed,
            method,
            "lifecycle method called in wrong state; SF must use a fresh Replica instance"
        );
        Err(mssf_core::ErrorCode::E_UNEXPECTED.into())
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::{Lifecycle, LifecycleState};

    fn lc() -> Lifecycle {
        // partition / replica context only feeds tracing fields,
        // which are not exercised by these tests.
        Lifecycle::new(mssf_core::GUID::zeroed(), 0)
    }

    /// Drive the full happy path Created -> Opening -> Active ->
    /// Closing -> Terminal in one go.
    fn drive_happy_path(lc: &Lifecycle) {
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        lc.enter_closing().unwrap();
        lc.complete_close().unwrap();
    }

    // ---- happy path -------------------------------------------------

    #[test]
    fn happy_path_full_lifecycle() {
        let lc = lc();
        lc.enter_opening().expect("Created -> Opening");
        assert_eq!(lc.current(), LifecycleState::Opening);
        lc.complete_open().expect("Opening -> Active");
        assert_eq!(lc.current(), LifecycleState::Active);
        lc.require_active().expect("Active accepts change_role");
        lc.enter_closing().expect("Active -> Closing");
        assert_eq!(lc.current(), LifecycleState::Closing);
        lc.complete_close().expect("Closing -> Terminal");
        assert_eq!(lc.current(), LifecycleState::Terminal);
    }

    // ---- open rejections --------------------------------------------

    #[test]
    fn open_rejected_when_already_opening() {
        let lc = lc();
        lc.enter_opening().unwrap();
        assert!(lc.enter_opening().is_err());
        assert_eq!(lc.current(), LifecycleState::Opening);
    }

    #[test]
    fn open_rejected_after_active() {
        let lc = lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        assert!(lc.enter_opening().is_err());
        assert_eq!(lc.current(), LifecycleState::Active);
    }

    #[test]
    fn open_rejected_after_terminal() {
        let lc = lc();
        drive_happy_path(&lc);
        assert!(lc.enter_opening().is_err());
        assert_eq!(lc.current(), LifecycleState::Terminal);
    }

    #[test]
    fn complete_open_rejected_outside_opening() {
        let lc = lc();
        // From Created.
        assert!(lc.complete_open().is_err());
        // From Active.
        let lc = self::lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        assert!(lc.complete_open().is_err());
    }

    // ---- change_role rejections -------------------------------------

    #[test]
    fn change_role_rejected_in_created() {
        let lc = lc();
        assert!(lc.require_active().is_err());
    }

    #[test]
    fn change_role_rejected_in_opening() {
        let lc = lc();
        lc.enter_opening().unwrap();
        assert!(lc.require_active().is_err());
    }

    #[test]
    fn change_role_rejected_in_closing() {
        let lc = lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        lc.enter_closing().unwrap();
        assert!(lc.require_active().is_err());
    }

    #[test]
    fn change_role_rejected_in_terminal() {
        let lc = lc();
        drive_happy_path(&lc);
        assert!(lc.require_active().is_err());
    }

    // ---- close rejections -------------------------------------------

    #[test]
    fn close_rejected_in_created() {
        let lc = lc();
        assert!(lc.enter_closing().is_err());
        assert_eq!(lc.current(), LifecycleState::Created);
    }

    #[test]
    fn close_rejected_in_opening() {
        let lc = lc();
        lc.enter_opening().unwrap();
        assert!(lc.enter_closing().is_err());
        assert_eq!(lc.current(), LifecycleState::Opening);
    }

    #[test]
    fn close_rejected_when_already_closing() {
        let lc = lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        lc.enter_closing().unwrap();
        assert!(lc.enter_closing().is_err());
    }

    #[test]
    fn close_rejected_when_already_terminal() {
        let lc = lc();
        drive_happy_path(&lc);
        assert!(lc.enter_closing().is_err());
    }

    #[test]
    fn complete_close_rejected_outside_closing() {
        let lc = lc();
        assert!(lc.complete_close().is_err());
        let lc = self::lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        assert!(lc.complete_close().is_err());
    }

    // ---- abort behaviour --------------------------------------------

    #[test]
    fn abort_from_created_runs_gate() {
        let lc = lc();
        assert!(
            !lc.abort(),
            "abort from Created is a real abort, not a no-op"
        );
        assert_eq!(lc.current(), LifecycleState::Terminal);
        assert!(lc.enter_opening().is_err());
    }

    #[test]
    fn abort_from_opening_runs_gate() {
        let lc = lc();
        lc.enter_opening().unwrap();
        assert!(
            !lc.abort(),
            "abort from Opening (failed open) is a real abort"
        );
        assert_eq!(lc.current(), LifecycleState::Terminal);
    }

    #[test]
    fn abort_from_active_runs_gate() {
        let lc = lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        assert!(!lc.abort(), "abort from Active is a real abort");
        assert_eq!(lc.current(), LifecycleState::Terminal);
    }

    #[test]
    fn abort_from_closing_runs_gate() {
        // The new state machine's payoff: a failed close leaves the
        // replica in Closing, and a follow-up SF abort still
        // publishes the abort gate. This was previously masked by
        // close advancing straight to Terminal.
        let lc = lc();
        lc.enter_opening().unwrap();
        lc.complete_open().unwrap();
        lc.enter_closing().unwrap();
        assert!(
            !lc.abort(),
            "abort after a failed close (state Closing) must run the gate"
        );
        assert_eq!(lc.current(), LifecycleState::Terminal);
    }

    #[test]
    fn abort_from_terminal_is_idempotent_no_op() {
        let lc = lc();
        drive_happy_path(&lc);
        assert!(lc.abort(), "abort after Terminal is a no-op");
        assert!(lc.abort(), "second abort is also a no-op");
    }
}
