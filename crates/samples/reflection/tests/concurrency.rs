// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Concurrent-lifecycle e2e: five single-replica services are
//! brought up, then each is told to delete and parked at the
//! `Close` gate so its `delete_service` task is blocked. While
//! all five are stuck, a sixth service is created and deleted
//! end-to-end (proving SF can interleave create/delete on
//! unrelated partitions independent of in-flight Close approvals
//! elsewhere). Finally the five parked Closes are released and
//! their `delete_service` tasks are joined.
//!
//! Demonstrates two properties of the `ReplicaControl` plane:
//!
//! 1. A replica parked at `Close` does not leak across partitions
//!    — SF can place and drive a brand-new partition's lifecycle
//!    without waiting on the unrelated parked Closes.
//!
//! 2. The test driver can multiplex many partitions through a
//!    single `Cluster` by building a fresh `PartitionDriver` per
//!    partition (drivers borrow the cluster mutably, so they are
//!    used one at a time; concurrent SF work continues in
//!    background tokio tasks regardless).
//!
//! Same prerequisites as `control_e2e.rs`.

use std::time::Duration;

use mssf_core::WString;
use mssf_core::types::{
    PartitionSchemeDescription, ServiceDescription, StatefulServiceDescription, Uri,
};
use mssf_util::tokio::TokioCancelToken;
use prost::Message;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc_control::proto::{ApprovalKind, ReplicaRole as ProtoReplicaRole};
use samples_reflection::test_cluster::{Cluster, TestStep, discover_partition_id, fabric_client};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const APP_NAME: &str = "fabric:/ReflectionApp";
const SERVICE_TYPE: &str = "ReflectionAppService";
const SF_TIMEOUT: Duration = Duration::from_secs(30);

/// Number of services that are parked at `Close` simultaneously.
const STUCK_COUNT: usize = 5;

/// Build a single-replica controlled service description (control=true
/// initdata so the replica uses GrpcController and parks at every
/// lifecycle gate).
fn make_controlled_singleton_desc(service_name: &Uri) -> ServiceDescription {
    let initdata = ReplicaInitData { control: true }.encode_to_vec();
    ServiceDescription::Stateful(
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
    )
}

/// Per-stuck-service bookkeeping.
struct StuckService {
    name: Uri,
    partition_id: String,
    replica_id: i64,
    delete_handle: tokio::task::JoinHandle<mssf_core::Result<()>>,
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn five_stuck_at_close_then_create_delete_sixth() {
    let suffix = Uuid::new_v4().simple().to_string();
    let mut cluster = Cluster::new();

    let fc = fabric_client();
    let sm = fc.get_service_manager().clone();

    // ---- Phase 1: bring up STUCK_COUNT services to Up state ----
    //
    // Drive each service Open + ChangeRole(Primary) sequentially.
    // Each service's `partition_id` is discovered via the SF query
    // manager so the per-partition driver only sees its own gates.
    let mut up_services: Vec<(Uri, String, i64)> = Vec::with_capacity(STUCK_COUNT);
    for i in 0..STUCK_COUNT {
        let name_str = format!("{APP_NAME}/StuckClose_{i}_{suffix}");
        let name = Uri::from(name_str.as_str());
        sm.create_service(&make_controlled_singleton_desc(&name), SF_TIMEOUT, None)
            .await
            .unwrap_or_else(|e| panic!("create_service[{i}] failed: {e}"));
        tracing::info!("svc[{i}] created: {name_str}");

        let pid = discover_partition_id(fc, &name).await;
        let mut driver = cluster.partition_driver(pid.clone());
        let (n, open_ev) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
        let target = open_ev.target.clone().expect("ApprovalEvent.target");
        let rid = target.replica_id;
        tracing::info!("svc[{i}] OPEN node #{n} partition={pid} replica={rid}");

        driver
            .approve_proceed(n, target, open_ev.gate_id.clone())
            .await;
        let cr = driver
            .approve_replica_sequence(rid, &[ApprovalKind::ApprovalChangeRole])
            .await;
        assert_eq!(
            cr[0].new_role,
            ProtoReplicaRole::Primary as i32,
            "svc[{i}]: first ChangeRole should be Primary, got new_role={}",
            cr[0].new_role,
        );
        tracing::info!("svc[{i}] -> Primary; replica is Up");

        up_services.push((name, pid, rid));
    }

    // ---- Phase 2: trigger delete on all five and park each at Close ----
    //
    // For each service: spawn `delete_service`, then approve the
    // ChangeRole(None) that SF emits, then wait for the Close gate to
    // appear *without* approving it. The replica stays parked and the
    // background `delete_service` task stays blocked on that Close.
    let mut stuck: Vec<StuckService> = Vec::with_capacity(STUCK_COUNT);
    for (i, (name, pid, rid)) in up_services.iter().enumerate() {
        let delete_handle = {
            let sm2 = sm.clone();
            let svc = name.clone();
            tokio::spawn(async move { sm2.delete_service(&svc, SF_TIMEOUT, None).await })
        };

        let mut driver = cluster.partition_driver(pid.clone());

        // Approve ChangeRole(None) — SF then issues Close.
        let cr_none = driver
            .approve_replica_sequence(*rid, &[ApprovalKind::ApprovalChangeRole])
            .await;
        assert_eq!(
            cr_none[0].new_role,
            ProtoReplicaRole::None as i32,
            "svc[{i}]: teardown ChangeRole new_role should be None, got {}",
            cr_none[0].new_role,
        );

        // Confirm Close has been emitted (it stays parked because we
        // never approve it). `wait_for_replica` returns as soon as
        // the gate is observed — no approval is sent.
        let (_, close_ev) = driver
            .wait_for_replica(*rid, Some(ApprovalKind::ApprovalClose))
            .await;
        tracing::info!(
            "svc[{i}] parked at Close gate_id={} (delete_service blocked)",
            close_ev.gate_id,
        );

        stuck.push(StuckService {
            name: name.clone(),
            partition_id: pid.clone(),
            replica_id: *rid,
            delete_handle,
        });
    }

    // Sanity: every delete task should still be blocked.
    for (i, s) in stuck.iter().enumerate() {
        assert!(
            !s.delete_handle.is_finished(),
            "delete_service[{i}] for {} should still be blocked on Close approval",
            s.name,
        );
    }

    // ---- Phase 3: while five are stuck, create+delete a sixth ----
    let sixth_name_str = format!("{APP_NAME}/StuckClose_six_{suffix}");
    let sixth_name = Uri::from(sixth_name_str.as_str());
    sm.create_service(
        &make_controlled_singleton_desc(&sixth_name),
        SF_TIMEOUT,
        None,
    )
    .await
    .unwrap_or_else(|e| panic!("create_service[6th] failed: {e}"));
    tracing::info!("6th service created: {sixth_name_str}");

    // Discover the 6th's partition_id by service name and scope the
    // driver to it. This is necessary because the five stuck
    // services still have *Close* gates pending in their own
    // partitions and we don't want to confuse them with the 6th's
    // gates (and protects against parallel test runs too).
    let sixth_pid = discover_partition_id(fc, &sixth_name).await;
    assert!(
        !stuck.iter().any(|s| s.partition_id == sixth_pid),
        "6th partition_id {sixth_pid} unexpectedly matched a stuck service"
    );
    let mut sixth_driver = cluster.partition_driver(sixth_pid.clone());
    let (n_six, sixth_open) = sixth_driver
        .wait_next_kind(ApprovalKind::ApprovalOpen)
        .await;
    let sixth_target = sixth_open.target.clone().expect("ApprovalEvent.target");
    let sixth_rid = sixth_target.replica_id;
    tracing::info!("6th OPEN node #{n_six} partition={sixth_pid} replica={sixth_rid}");

    sixth_driver
        .approve_proceed(n_six, sixth_target, sixth_open.gate_id.clone())
        .await;
    let sixth_cr = sixth_driver
        .approve_replica_sequence(sixth_rid, &[ApprovalKind::ApprovalChangeRole])
        .await;
    assert_eq!(
        sixth_cr[0].new_role,
        ProtoReplicaRole::Primary as i32,
        "6th: first ChangeRole should be Primary, got new_role={}",
        sixth_cr[0].new_role,
    );

    // Delete the 6th end-to-end. Its delete_service must complete
    // while the five stuck Closes are still pending.
    let sixth_delete = {
        let sm2 = sm.clone();
        let svc = sixth_name.clone();
        tokio::spawn(async move { sm2.delete_service(&svc, SF_TIMEOUT, None).await })
    };
    sixth_driver
        .drive_replica_sequence(
            sixth_rid,
            &[
                TestStep::proceed(ApprovalKind::ApprovalChangeRole),
                TestStep::proceed(ApprovalKind::ApprovalClose),
            ],
        )
        .await;
    sixth_delete
        .await
        .expect("6th delete task panicked")
        .expect("6th delete_service failed");
    tracing::info!("6th service deleted while {STUCK_COUNT} are still stuck at Close");

    // Sanity: the five stuck deletes are *still* blocked. The 6th's
    // lifecycle did not accidentally wake them.
    for (i, s) in stuck.iter().enumerate() {
        assert!(
            !s.delete_handle.is_finished(),
            "stuck delete[{i}] for {} unexpectedly finished after 6th delete",
            s.name,
        );
    }

    // ---- Phase 4: release all five parked Close gates ----
    //
    // Approve each Close in turn (driving via `drive_replica_sequence`
    // so the kind is asserted) and join the corresponding delete
    // task. After this phase the cluster is back to its pre-test
    // state for these services.
    for (i, s) in stuck.into_iter().enumerate() {
        let mut driver = cluster.partition_driver(s.partition_id.clone());
        driver
            .drive_replica_sequence(
                s.replica_id,
                &[TestStep::proceed(ApprovalKind::ApprovalClose)],
            )
            .await;
        s.delete_handle
            .await
            .unwrap_or_else(|e| panic!("stuck delete[{i}] task panicked: {e}"))
            .unwrap_or_else(|e| panic!("stuck delete[{i}] failed: {e}"));
        tracing::info!("svc[{i}] {} fully deleted", s.name);
    }

    tracing::info!(
        "concurrent-close e2e complete: {STUCK_COUNT} parked-then-released, plus 1 in-between"
    );
}

/// Three-delete e2e: a single controlled service is brought up
/// and driven to its parked `Close` gate (same setup as the
/// multi-service test above), then three back-to-back
/// `delete_service` calls exercise three different
/// client-cancellation paths against the same parked Close:
///
/// 1. **Delete with a short timeout.** First call uses a 3 s
///    timeout (much shorter than the test-wide `SF_TIMEOUT`). Since
///    Close is never approved, SF times out the operation and the
///    call returns `FABRIC_E_TIMEOUT`. The cluster-side delete
///    remains in flight.
///
/// 2. **Delete with explicit token cancel.** Second call uses
///    `SF_TIMEOUT` and a SF cancellation token. Once the call is
///    in flight, the token is cancelled; per the
///    `fabric_begin_end_proxy` contract the receiver is awaited to
///    completion so SF flushes the cancel. The call returns
///    `E_ABORT` / `OperationCanceled`.
///
/// 3. **Plain delete + approve Close.** Third call uses
///    `SF_TIMEOUT` and no token. The parked `Close` gate is then
///    approved; SF tears the replica down and the call completes.
///    It may return `Ok` or `FABRIC_E_SERVICE_DOES_NOT_EXIST`
///    depending on whether the cluster-side delete races ahead of
///    the retried client call. Either way the service is gone.
///
/// Verifies that neither a SF-side timeout nor a client-side
/// token cancel disturb the in-progress cluster-side delete: the
/// Close gate stays parked across both, and a final retried
/// delete drains correctly once Close is approved.
///
/// Same prerequisites as the other e2e tests in this file.
#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn cancel_delete_then_retry_succeeds_after_close() {
    let suffix = Uuid::new_v4().simple().to_string();
    let mut cluster = Cluster::new();

    let fc = fabric_client();
    let sm = fc.get_service_manager().clone();

    // ---- Phase 1: bring the service up to Primary ----
    let name_str = format!("{APP_NAME}/CancelRetryDelete_{suffix}");
    let name = Uri::from(name_str.as_str());
    sm.create_service(&make_controlled_singleton_desc(&name), SF_TIMEOUT, None)
        .await
        .unwrap_or_else(|e| panic!("create_service failed: {e}"));
    tracing::info!("svc created: {name_str}");

    let pid = discover_partition_id(fc, &name).await;
    let mut driver = cluster.partition_driver(pid.clone());
    let (n, open_ev) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let target = open_ev.target.clone().expect("ApprovalEvent.target");
    let rid = target.replica_id;
    tracing::info!("OPEN node #{n} partition={pid} replica={rid}");

    driver
        .approve_proceed(n, target, open_ev.gate_id.clone())
        .await;
    let cr = driver
        .approve_replica_sequence(rid, &[ApprovalKind::ApprovalChangeRole])
        .await;
    assert_eq!(
        cr[0].new_role,
        ProtoReplicaRole::Primary as i32,
        "first ChangeRole should be Primary, got new_role={}",
        cr[0].new_role,
    );
    tracing::info!("svc -> Primary; replica is Up");

    // ---- Phase 2: first delete with short timeout, expect FABRIC_E_TIMEOUT ----
    //
    // Spawn delete with a 3 s SF timeout. Drive ChangeRole(None) so
    // SF emits the Close gate and confirm the Close is parked
    // (without approving). The first delete will time out on the
    // SF side because Close is never approved within 3 s.
    const SHORT_TIMEOUT: Duration = Duration::from_secs(3);
    let first_delete = {
        let sm2 = sm.clone();
        let svc = name.clone();
        tokio::spawn(async move { sm2.delete_service(&svc, SHORT_TIMEOUT, None).await })
    };

    let cr_none = driver
        .approve_replica_sequence(rid, &[ApprovalKind::ApprovalChangeRole])
        .await;
    assert_eq!(
        cr_none[0].new_role,
        ProtoReplicaRole::None as i32,
        "teardown ChangeRole new_role should be None, got {}",
        cr_none[0].new_role,
    );

    let (_, close_ev) = driver
        .wait_for_replica(rid, Some(ApprovalKind::ApprovalClose))
        .await;
    let close_gate_id = close_ev.gate_id.clone();
    tracing::info!(
        "parked at Close gate_id={} (first delete blocked, will time out)",
        close_gate_id,
    );

    let first_outcome = first_delete
        .await
        .expect("first delete_service task panicked");
    let first_err =
        first_outcome.expect_err("first delete_service should error out (timeout), not return Ok");
    let first_code = first_err
        .try_as_fabric_error_code()
        .expect("first delete error should be a fabric error code");
    assert_eq!(
        first_code,
        mssf_core::ErrorCode::FABRIC_E_TIMEOUT,
        "first delete_service should fail with FABRIC_E_TIMEOUT, got: {first_err:?}",
    );
    tracing::info!("first delete_service timed out: {first_err:?}");

    // The cluster-side delete is still in progress; Close gate must
    // still be parked with the same gate_id.
    let (_, close_ev_after_timeout) = driver
        .wait_for_replica(rid, Some(ApprovalKind::ApprovalClose))
        .await;
    assert_eq!(
        close_ev_after_timeout.gate_id, close_gate_id,
        "Close gate_id should be unchanged after first delete timed out",
    );

    // ---- Phase 3: second delete with cancellation token, cancelled mid-flight ----
    //
    // Issue a second `delete_service` with a SF cancellation token
    // (`BoxedCancelToken`). Cancel the token and await the task to
    // completion. The proxy contract requires polling the receiver
    // to completion to flush the cancel through to SF. Expected
    // result: `E_ABORT`.
    let second_ct = CancellationToken::new();
    let second_delete = {
        let sm2 = sm.clone();
        let svc = name.clone();
        let token = TokioCancelToken::boxed_from(second_ct.clone());
        tokio::spawn(async move { sm2.delete_service(&svc, SF_TIMEOUT, Some(token)).await })
    };

    // Confirm the Close gate is still parked and the second delete
    // is in flight before we cancel.
    let (_, close_ev2) = driver
        .wait_for_replica(rid, Some(ApprovalKind::ApprovalClose))
        .await;
    assert_eq!(
        close_ev2.gate_id, close_gate_id,
        "Close gate_id should be unchanged before token cancel",
    );
    assert!(
        !second_delete.is_finished(),
        "second delete_service should still be blocked on Close approval"
    );

    second_ct.cancel();
    let second_outcome = second_delete
        .await
        .expect("second delete_service task panicked");
    let second_err = second_outcome
        .expect_err("second delete_service should error after token cancel, not return Ok");
    let second_code = second_err
        .try_as_fabric_error_code()
        .expect("second delete error should be a fabric error code");
    assert_eq!(
        second_code,
        mssf_core::ErrorCode::E_ABORT,
        "second delete_service should fail with E_ABORT after token cancel, got: {second_err:?}",
    );
    tracing::info!("second delete_service cancelled: {second_err:?}");

    // ---- Phase 4: third delete, approve Close, expect success ----
    //
    // Re-issue `delete_service` with no token and approve the
    // parked Close. SF tears down the replica and the call
    // completes; either `Ok` (the retried op sees its own
    // completion) or `FABRIC_E_SERVICE_DOES_NOT_EXIST` (the
    // cluster-side delete from earlier requests already removed
    // the service by the time this op reaches SF). Both are
    // correct idempotent endings.
    let third_delete = {
        let sm2 = sm.clone();
        let svc = name.clone();
        tokio::spawn(async move { sm2.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    driver
        .drive_replica_sequence(rid, &[TestStep::proceed(ApprovalKind::ApprovalClose)])
        .await;
    let third_outcome = third_delete
        .await
        .expect("third delete_service task panicked");
    match third_outcome {
        Ok(()) => {
            tracing::info!("third delete_service returned Ok");
        }
        Err(e) => {
            let code = e
                .try_as_fabric_error_code()
                .expect("third delete error should be a fabric error code");
            assert_eq!(
                code,
                mssf_core::ErrorCode::FABRIC_E_SERVICE_DOES_NOT_EXIST,
                "third delete_service should be Ok or FABRIC_E_SERVICE_DOES_NOT_EXIST, got: {e:?}",
            );
            tracing::info!(
                "third delete_service returned FABRIC_E_SERVICE_DOES_NOT_EXIST (idempotent: service already deleted)"
            );
        }
    }
    tracing::info!("svc {name_str} fully deleted via re-issued delete_service");
}
