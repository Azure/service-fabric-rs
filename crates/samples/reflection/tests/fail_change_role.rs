// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Failure-injection e2e: reject the first `ChangeRole` and verify SF
//! recovers, then approve the second `ChangeRole` so the service comes
//! up cleanly.
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
//! 1. **`replica_id` and node placement may change across the
//!    failure.** SF sometimes reuses the same `replica_id` and node
//!    for the recovery activation, but not always — across runs we
//!    have seen both same-id-same-node and new-id-different-node
//!    outcomes. Polling by `partition_id` is therefore mandatory for
//!    this test (the [`PartitionDriver`] used below re-discovers the
//!    current pending replica per gate).
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
use mssf_core::client::FabricClient;
use mssf_core::types::{
    PartitionSchemeDescription, ServiceDescription, StatefulServiceDescription, Uri,
};
use prost::Message;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc_control::proto::ApprovalKind;
use samples_reflection::test_cluster::{Cluster, TestStep, discover_partition_id};
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

    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();

    fc.get_service_manager()
        .create_service(&desc, SF_TIMEOUT, None)
        .await
        .expect("create_service failed");
    tracing::info!("service {service_name_str} created");

    // Discover this service's partition_id via the SF query manager
    // so the test only sees gates from its own partition (avoids
    // cross-test pollution when integration tests run in parallel).
    let partition_id = discover_partition_id(&fc, &service_name).await;
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

    // Recovery: SF re-activates with a fresh `replica_id` (and
    // possibly a different node) after a back-off (~15 s in onebox).
    // Discover that new replica via a partition-wide wait, then drive
    // its happy-path lifecycle.
    let (n2, second_open) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let second_target = second_open.target.clone().expect("ApprovalEvent.target");
    let r2_id = second_target.replica_id;
    tracing::info!(
        "OPEN #2 observed on node #{n2} replica={r2_id} gate_id={} (previously r1={r1_id})",
        second_open.gate_id,
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
