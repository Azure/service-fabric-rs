// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Failure-injection e2e tests for `IStatefulServiceReplica`
//! lifecycle methods. Each test rejects one specific lifecycle gate
//! and asserts the SF recovery shape that follows.
//!
//! Currently covered:
//! - `open` returning `Err`: SF re-opens with a fresh `Replica`
//!   without an intervening `abort`. A failed open never acquired
//!   any state, so SF treats the returned `Err` as both "open
//!   failed" and "the replica is already torn down".
//! - `change_role` returning `Err`: SF aborts and re-opens with a
//!   fresh `Replica` (typically reusing `replica_id` on Linux onebox).
//!   The recovery `Open + ChangeRole(Primary)` brings the service
//!   back Up.
//! - `close` returning `Err` during service deletion: SF aborts the
//!   failed replica, then the deletion completes — no second open
//!   because the user wants the service gone.
//!
//! ## Observed Service Fabric recovery behaviour
//!
//! When `IStatefulServiceReplica::change_role` returns an `Err`, SF
//! does NOT simply re-issue `change_role` on the same in-memory
//! replica object. Instead it tears the replica down and re-opens it.
//! The exact sequence we observe in onebox (SF 11.4.x) is:
//!
//! ```text
//!     SF -> open()           Approve(Proceed)
//!     SF -> change_role(Primary)  Approve(Fail("..."))
//!     SF -> abort()          Approve(Proceed)   // tear down the failed replica
//!     [~15 s back-off]
//!     SF -> open()           Approve(Proceed)   // fresh activation, possibly new replica_id / node
//!     SF -> change_role(Primary)  Approve(Proceed)   // service is now Up
//! ```
//!
//! Two non-obvious points:
//!
//! 1. **`replica_id` is reused across the failure on Linux onebox.**
//!    Across runs we have consistently seen SF reuse the original
//!    `replica_id` (and node) for the recovery activation. The test
//!    asserts this so a future SF behaviour change is caught
//!    immediately. Either way the second activation is a *new*
//!    `Replica` object — SF aborts the failed one and starts over
//!    rather than retrying `change_role` on the existing instance.
//!    This is enforced by lifecycle guards in
//!    [`crate::statefulstore::Replica`]: a second `open()` on an
//!    instance whose state has advanced past `Created` returns
//!    `E_UNEXPECTED`, so if SF ever did call `open` on the same
//!    instance the recovery `Open` gate below would never appear and
//!    this test would time out.
//!
//! 2. **There is a noticeable back-off (~15 s in onebox)** between
//!    the abort approval and the second open. This is SF's internal
//!    retry delay after a transient replica failure. Tests that
//!    exercise this path should expect to be in the
//!    10-second-and-up runtime category, not sub-second.
//!
//! On the gRPC side, each gate gets a fresh `gate_id` UUID even
//! when the `(partition_id, replica_id)` tuple is unchanged — the
//! controller mints a new id every time `await_approval` populates
//! `pending`. A test holding a stale `gate_id` from before the
//! failure cannot misroute an `Approve` to the recovery gate.
//!
//! ## Discovery strategy
//!
//! Polls by `partition_id` rather than `replica_id`. The partition
//! is stable for the service's lifetime; the replica may be rebuilt
//! one or more times during the recovery sequence. Each test's
//! `partition_id` also keeps it isolated from any parallel test runs
//! in the same cluster.
//!
//! Same prerequisites as `control_e2e.rs`.

use std::time::Duration;

use mssf_core::WString;
use mssf_core::types::{
    PartitionSchemeDescription, ServiceDescription, StatefulServiceDescription, Uri,
};
use prost::Message;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc_control::proto::{ApprovalKind, ReplicaRole as ProtoReplicaRole};
use samples_reflection::test_cluster::{Cluster, TestStep, discover_partition_id, fabric_client};
use uuid::Uuid;

const APP_NAME: &str = "fabric:/ReflectionApp";
const SERVICE_TYPE: &str = "ReflectionAppService";
const SF_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn fail_change_role_then_approve_retry() {
    // Unique service name per run so tests can coexist.
    let svc_suffix = Uuid::new_v4().simple().to_string();
    let service_name_str = format!("{APP_NAME}/FailCrE2e_{svc_suffix}");
    let service_name = Uri::from(service_name_str.as_str());

    let mut cluster = Cluster::new();

    // Create the controlled service.
    let initdata = ReplicaInitData { control: true }.encode_to_vec();
    let desc = ServiceDescription::Stateful(
        StatefulServiceDescription::new(
            Uri::from(APP_NAME),
            service_name.clone(),
            WString::from(SERVICE_TYPE),
            PartitionSchemeDescription::Singleton,
        )
        .with_has_persistent_state(true)
        .with_service_activation_mode(mssf_core::types::ServicePackageActivationMode::SharedProcess)
        .with_min_replica_set_size(1)
        .with_target_replica_set_size(1)
        .with_initialization_data(initdata),
    );

    let fc = fabric_client();

    fc.get_service_manager()
        .create_service(&desc, SF_TIMEOUT, None)
        .await
        .expect("create_service failed");
    tracing::info!("service {service_name_str} created");

    // Discover this service's partition_id via the SF query manager
    // so the test only sees gates from its own partition (avoids
    // cross-test pollution when integration tests run in parallel).
    let partition_id = discover_partition_id(fc, &service_name).await;
    tracing::info!("partition_id={partition_id}");
    let mut driver = cluster.partition_driver(partition_id);

    // First OPEN within this partition: scoped poll, no risk of
    // hijacking another test's gate.
    let (node_idx, first_open) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let first_target = first_open.target.clone().expect("ApprovalEvent.target");
    let r1_id = first_target.replica_id;
    tracing::info!(
        "OPEN #1 observed on node #{node_idx} replica={r1_id} gate_id={}",
        first_open.gate_id,
    );

    driver
        .approve_proceed(node_idx, first_target, first_open.gate_id.clone())
        .await;

    // First-replica failure sequence: ChangeRole(fail) -> Abort.
    // Pinned to r1_id so SF's recovery activation cannot be confused
    // with the original replica's gates.
    let r1_failure = driver
        .drive_replica_sequence(
            r1_id,
            &[
                TestStep::fail(
                    ApprovalKind::ApprovalChangeRole,
                    "test-induced ChangeRole failure",
                ),
                TestStep::proceed(ApprovalKind::ApprovalAbort),
            ],
        )
        .await;
    tracing::info!(
        "replica {r1_id} failure sequence done: {} gates",
        r1_failure.len(),
    );

    // Recovery: SF re-activates after a back-off (~15 s in onebox).
    // Across runs on Linux onebox the recovery activation has
    // consistently reused the original `replica_id` (and node).
    // Assert that here so a future SF behaviour change — where
    // recovery uses a fresh `replica_id` — surfaces as an obvious
    // test failure rather than silently passing on a different
    // code path. The `Lifecycle` guard in `statefulstore.rs` proves
    // that even with a reused id, this is a brand-new `Replica`
    // object: a second `open` on the same instance would have been
    // rejected with `E_UNEXPECTED` and `OPEN #2` would never appear.
    let (n2, second_open) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let second_target = second_open.target.clone().expect("ApprovalEvent.target");
    let r2_id = second_target.replica_id;
    tracing::info!(
        "OPEN #2 observed on node #{n2} replica={r2_id} gate_id={} (previously r1={r1_id})",
        second_open.gate_id,
    );
    assert_eq!(
        r2_id, r1_id,
        "SF used to reuse the replica_id across change_role failure recovery; \
         a different id here means SF behaviour changed — review the test \
         (and Lifecycle invariant) before relaxing this assertion"
    );
    assert_ne!(
        second_open.gate_id, first_open.gate_id,
        "each gate must have a fresh gate_id even when replica_id is reused"
    );
    driver
        .approve_proceed(n2, second_target, second_open.gate_id.clone())
        .await;

    // Recovery replica: ChangeRole(Primary) -> service is Up.
    let r2_up = driver
        .approve_replica_sequence(r2_id, &[ApprovalKind::ApprovalChangeRole])
        .await;
    tracing::info!(
        "replica {r2_id} ChangeRole done: new_role={}, gate_id={}",
        r2_up[0].new_role,
        r2_up[0].gate_id,
    );

    // Teardown: delete the service and drive r2's shutdown sequence
    // (ChangeRole(None) -> Close). The Approve for Close unblocks
    // the background delete task.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    let teardown = driver
        .drive_replica_sequence(
            r2_id,
            &[
                TestStep::proceed(ApprovalKind::ApprovalChangeRole),
                TestStep::proceed(ApprovalKind::ApprovalClose),
            ],
        )
        .await;
    for ev in &teardown {
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        tracing::info!(
            "teardown gate kind={kind:?} replica={r2_id} gate_id={}",
            ev.gate_id,
        );
    }

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    tracing::info!("fail-change-role e2e flow complete");
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn fail_close_during_delete_then_abort() {
    // Observed SF behaviour on Linux onebox:
    //
    // ```text
    //     SF -> open()                Approve(Proceed)
    //     SF -> change_role(Primary)  Approve(Proceed)        // service Up
    //     [delete_service in background]
    //     SF -> change_role(None)     Approve(Proceed)
    //     SF -> close()               Approve(Fail("..."))    // close returns Err
    //     SF -> abort()               Approve(Proceed)        // SF tears the replica down
    //     [delete_service returns Ok(())]
    // ```
    //
    // SF treats fail-close on a service-deletion path by calling
    // `abort` to force-tear-down, then completing the deletion.
    // **No second open** — the user has asked for the service to
    // go away, so SF does not bother re-opening to retry close.
    // This differs from the failed-change-role path where SF does
    // re-open after the back-off.
    //
    // ## SF does not retry close — and why that's mostly fine
    //
    // SF's recovery for `change_role` failure is "abort + reopen +
    // retry"; the recovery for `close` failure on the deletion path
    // is "abort + give up". The replica is never given a second
    // chance to perform a clean close from the `IStatefulServiceReplica`
    // surface — only `abort()` runs after a failed close, and
    // `abort()` returns `()` so there is no way for the application
    // to retry or signal a second error.
    //
    // **In our reflection sample (and in most well-structured SF
    // services) this is not a real correctness issue.** By the time
    // SF calls `IStatefulServiceReplica::close` on the user code,
    // the underlying replicator (state-replicator + log + persistent
    // store) has already been closed cleanly by SF's own runtime —
    // the sample's `close` body therefore only stops the in-process
    // gRPC echo server and removes a registry entry. A failure here
    // doesn't leave replicated state in a bad shape; SF's own
    // teardown of the replicator already happened.
    //
    // Where to be careful:
    //
    // - **Out-of-process resources** owned by the user code (file
    //   handles outside SF's managed store, sockets, external
    //   service registrations, distributed locks, etc.) are *not*
    //   covered by SF's replicator teardown. Anything in this
    //   category should be released from a path that runs on both
    //   `close` and `abort` (or only on `abort`, since `abort`
    //   *always* runs even when close fails).
    // - **Idempotence.** `close` may be called once and then never
    //   retried, so it should never assume "I'll get another shot".
    //
    // The conservative shape is to treat `abort` as the
    // unconditional cleanup path: invoke external-resource release
    // from `abort`, and let `close` be a best-effort prelude.
    //
    // Note: the abort gate is only observable here because of the
    // `Closing` intermediate lifecycle state. With the previous
    // two-state machine (`Active -> Terminal` on `enter_close`),
    // a fail-close left the lifecycle in `Terminal`, and any
    // subsequent SF abort short-circuited inside
    // `Lifecycle::abort()` without publishing the gate.
    //
    // The test asserts:
    //
    // 1. r1's first ChangeRole during teardown was `ChangeRole(None)`.
    // 2. r1's Close was issued and we failed it.
    // 3. SF then emits **exactly one Abort gate** for r1.
    // 4. After we approve the abort, no further gates appear.
    // 5. `delete_service` completes successfully.

    let svc_suffix = Uuid::new_v4().simple().to_string();
    let service_name_str = format!("{APP_NAME}/FailCloseE2e_{svc_suffix}");
    let service_name = Uri::from(service_name_str.as_str());

    let mut cluster = Cluster::new();

    let initdata = ReplicaInitData { control: true }.encode_to_vec();
    let desc = ServiceDescription::Stateful(
        StatefulServiceDescription::new(
            Uri::from(APP_NAME),
            service_name.clone(),
            WString::from(SERVICE_TYPE),
            PartitionSchemeDescription::Singleton,
        )
        .with_has_persistent_state(true)
        .with_service_activation_mode(mssf_core::types::ServicePackageActivationMode::SharedProcess)
        .with_min_replica_set_size(1)
        .with_target_replica_set_size(1)
        .with_initialization_data(initdata),
    );

    let fc = fabric_client();

    fc.get_service_manager()
        .create_service(&desc, SF_TIMEOUT, None)
        .await
        .expect("create_service failed");
    tracing::info!("service {service_name_str} created");

    let partition_id = discover_partition_id(fc, &service_name).await;
    tracing::info!("partition_id={partition_id}");
    let mut driver = cluster.partition_driver(partition_id);

    // Activation: Open -> ChangeRole(Primary).
    let (n1, first_open) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let r1_target = first_open.target.clone().expect("ApprovalEvent.target");
    let r1_id = r1_target.replica_id;
    tracing::info!(
        "OPEN #1 on node #{n1} replica={r1_id} gate_id={}",
        first_open.gate_id,
    );
    driver
        .approve_proceed(n1, r1_target, first_open.gate_id.clone())
        .await;
    driver
        .approve_replica_sequence(r1_id, &[ApprovalKind::ApprovalChangeRole])
        .await;
    tracing::info!("replica {r1_id} -> Primary");

    // Trigger deletion in the background. SF then drives r1 through
    // ChangeRole(None) -> Close. Approve the ChangeRole; fail Close.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    let r1_failure = driver
        .drive_replica_sequence(
            r1_id,
            &[
                TestStep::proceed(ApprovalKind::ApprovalChangeRole),
                TestStep::fail(ApprovalKind::ApprovalClose, "test-induced Close failure"),
                TestStep::proceed(ApprovalKind::ApprovalAbort),
            ],
        )
        .await;
    assert_eq!(
        r1_failure[0].new_role,
        ProtoReplicaRole::None as i32,
        "first ChangeRole during teardown should be ChangeRole(None), got new_role={}",
        r1_failure[0].new_role,
    );
    tracing::info!("r1 {r1_id} ChangeRole(None) approved + Close failed + Abort approved");

    // delete_service should complete shortly after the abort. Allow
    // a generous budget; observed latency on Linux onebox is < 1 s.
    let delete_result = tokio::time::timeout(Duration::from_secs(30), delete_handle)
        .await
        .expect("delete_service did not complete within 30s of fail-close + abort");
    delete_result
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    // After delete_service returns there should be no further gates
    // (no second open, no retry close). If SF ever does retry on a
    // fresh replica, this would surface as a non-empty list_pending.
    let probe = driver.list_pending().await;
    assert!(
        probe.is_empty(),
        "expected no further gates in partition after delete; got: {probe:?}"
    );

    tracing::info!("fail-close-during-delete e2e flow complete");
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn fail_open_then_approve_retry() {
    // Reject the very first `Open` gate and verify SF recreates a
    // fresh `Replica` and the service comes Up cleanly on the
    // recovery activation.
    //
    // Observed SF sequence (single-replica, control=true) on Linux
    // onebox:
    //
    // ```text
    //     SF -> open()                Approve(Fail("..."))     // r1 fails open
    //     [no abort gate]
    //     [~15 s back-off]
    //     SF -> open()                Approve(Proceed)         // r2 (same replica_id)
    //     SF -> change_role(Primary)  Approve(Proceed)         // service Up
    //     [delete to clean up]
    //     SF -> change_role(None)     Approve(Proceed)
    //     SF -> close()               Approve(Proceed)
    // ```
    //
    // **SF does not call `abort` after a failed `open`.** The
    // explanatory model: an `open` that returns `Err` never finished
    // acquiring the replica's state machine in the first place, so
    // there's nothing for `abort` to roll back. SF treats the
    // returned `Err` as both "open failed" and "the replica is
    // already torn down", drops its in-memory replica object, and
    // schedules a fresh activation after the standard back-off.
    //
    // Compare:
    // - `fail_change_role_then_approve_retry`: SF *does* call abort,
    //   because the replica reached a partly-active state during
    //   change_role.
    // - `fail_close_during_delete_then_abort`: SF calls abort to
    //   force-tear-down a replica whose close failed.
    //
    // The lifecycle guard in `crate::lifecycle::Lifecycle` is what
    // makes this distinction observable on the wire: a failed open
    // leaves the replica's state in `Opening`, so any subsequent
    // `abort()` would publish its gate (rather than being
    // short-circuited as it would have been with a two-state
    // machine that advanced straight to `Active` on `enter_open`).
    // The fact that we *don't* see an Abort gate is therefore
    // strong evidence that SF didn't call `abort` at all, not that
    // it did and we missed the wire signal.

    let svc_suffix = Uuid::new_v4().simple().to_string();
    let service_name_str = format!("{APP_NAME}/FailOpenE2e_{svc_suffix}");
    let service_name = Uri::from(service_name_str.as_str());

    let mut cluster = Cluster::new();

    let initdata = ReplicaInitData { control: true }.encode_to_vec();
    let desc = ServiceDescription::Stateful(
        StatefulServiceDescription::new(
            Uri::from(APP_NAME),
            service_name.clone(),
            WString::from(SERVICE_TYPE),
            PartitionSchemeDescription::Singleton,
        )
        .with_has_persistent_state(true)
        .with_service_activation_mode(mssf_core::types::ServicePackageActivationMode::SharedProcess)
        .with_min_replica_set_size(1)
        .with_target_replica_set_size(1)
        .with_initialization_data(initdata),
    );

    let fc = fabric_client();

    fc.get_service_manager()
        .create_service(&desc, SF_TIMEOUT, None)
        .await
        .expect("create_service failed");
    tracing::info!("service {service_name_str} created");

    let partition_id = discover_partition_id(fc, &service_name).await;
    tracing::info!("partition_id={partition_id}");
    let mut driver = cluster.partition_driver(partition_id);

    // Discover r1's first OPEN and reject it with Decision::Fail.
    let (n1, first_open) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let r1_target = first_open.target.clone().expect("ApprovalEvent.target");
    let r1_id = r1_target.replica_id;
    tracing::info!(
        "OPEN #1 on node #{n1} replica={r1_id} gate_id={}",
        first_open.gate_id,
    );
    driver
        .approve_fail(
            n1,
            r1_target,
            first_open.gate_id.clone(),
            "test-induced Open failure".to_string(),
        )
        .await;
    tracing::info!("r1 {r1_id} Open failed");

    // Wait for the next gate in the partition. By the contract
    // documented above we expect this to be a fresh OPEN (no
    // intermediate Abort). The 60 s budget covers SF's ~15 s
    // back-off plus margin. Anything else (Abort, Close,
    // ChangeRole) is a contract change and panics with the
    // observed kind.
    let (n2, next_ev) = tokio::time::timeout(Duration::from_secs(60), driver.wait_next())
        .await
        .expect("expected a recovery gate within 60s of failed open");
    let next_kind =
        ApprovalKind::try_from(next_ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
    assert_eq!(
        next_kind,
        ApprovalKind::ApprovalOpen,
        "after fail-open SF should re-open without an intervening abort \
         (failed open never acquired state, so there's nothing to abort); \
         observed kind={next_kind:?}, replica={}, gate_id={} — SF behaviour \
         may have changed",
        next_ev.target.as_ref().map(|t| t.replica_id).unwrap_or(0),
        next_ev.gate_id,
    );

    let r2_target = next_ev.target.clone().expect("ApprovalEvent.target");
    let r2_id = r2_target.replica_id;
    let second_open = next_ev;
    tracing::info!(
        "OPEN #2 on node #{n2} replica={r2_id} gate_id={} (previously r1={r1_id})",
        second_open.gate_id,
    );
    assert_eq!(
        r2_id, r1_id,
        "SF used to reuse the replica_id across open failure recovery; \
         a different id here means SF behaviour changed — review the test \
         before relaxing this assertion"
    );
    assert_ne!(
        second_open.gate_id, first_open.gate_id,
        "each gate must have a fresh gate_id even when replica_id is reused"
    );
    driver
        .approve_proceed(n2, r2_target, second_open.gate_id.clone())
        .await;

    // Recovery replica's ChangeRole(Primary): service should now be Up.
    let r2_up = driver
        .approve_replica_sequence(r2_id, &[ApprovalKind::ApprovalChangeRole])
        .await;
    tracing::info!(
        "replica {r2_id} ChangeRole done: new_role={}, gate_id={}",
        r2_up[0].new_role,
        r2_up[0].gate_id,
    );

    // Teardown: delete the service and drive r2's shutdown
    // sequence (ChangeRole(None) -> Close). The Approve for Close
    // unblocks the background delete task.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    let teardown = driver
        .drive_replica_sequence(
            r2_id,
            &[
                TestStep::proceed(ApprovalKind::ApprovalChangeRole),
                TestStep::proceed(ApprovalKind::ApprovalClose),
            ],
        )
        .await;
    for ev in &teardown {
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        tracing::info!(
            "teardown gate kind={kind:?} replica={r2_id} gate_id={}",
            ev.gate_id,
        );
    }

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    tracing::info!("fail-open e2e flow complete");
}
