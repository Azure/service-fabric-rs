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
use mssf_core::client::FabricClient;
use mssf_core::types::{
    PartitionSchemeDescription, ServiceDescription, StatefulServiceDescription, Uri,
};
use prost::Message;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc_control::proto::{ApprovalKind, ReplicaRole as ProtoReplicaRole};
use samples_reflection::test_cluster::{Cluster, TestStep, discover_partition_id};
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

    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
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

        let pid = discover_partition_id(&fc, &name).await;
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
    let sixth_pid = discover_partition_id(&fc, &sixth_name).await;
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
