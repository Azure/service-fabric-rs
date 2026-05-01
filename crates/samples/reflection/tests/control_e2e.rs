// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! End-to-end approval-gate test against a deployed onebox cluster.
//!
//! Walks one replica through its entire lifecycle by approving each gate
//! over the `ReplicaControl` gRPC service:
//!
//! `Open` -> `ChangeRole(Primary)` -> `ChangeRole(None)` -> `Close`
//!
//! Requirements (same as the other integration tests in this crate):
//! - `fabric:/ReflectionApp` is provisioned (via `scripts/prepare_test_apps.sh`).
//! - The reflection sample binary is the current build.
//! - `localhost:19000` resolves to the cluster's client gateway.
//! - The cluster's gRPC `ReplicaControl` ports `28000..=28004` are reachable;
//!   set `REFLECTION_CLUSTER_HOST` to override the default hostname `onebox`.

use std::time::Duration;

use mssf_core::WString;
use mssf_core::client::FabricClient;
use mssf_core::types::{
    PartitionSchemeDescription, ServiceDescription, StatefulServiceDescription, Uri,
};
use prost::Message;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc_control::proto::ApprovalKind;
use samples_reflection::test_cluster::Cluster;
use uuid::Uuid;

const APP_NAME: &str = "fabric:/ReflectionApp";
const SERVICE_TYPE: &str = "ReflectionAppService";
const SF_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn approve_open_change_role_close_singleton_replica() {
    // Unique service name per run so the test is repeatable.
    let svc_suffix = Uuid::new_v4().simple().to_string();
    let service_name_str = format!("{APP_NAME}/ApprovalE2e_{svc_suffix}");
    let service_name = Uri::from(service_name_str.as_str());

    // The reflection sample's process only binds the ReplicaControl
    // port once SF activates the code package on a node. Connecting
    // before service creation would just race against placement.
    // `Cluster::poll_for_pending` calls `ensure()` on every poll
    // iteration, so cold-then-warm nodes are picked up automatically
    // within `POLL_INTERVAL` once SF starts the process.
    let mut cluster = Cluster::new();

    // Construct service description with control=true initdata so the
    // replica uses GrpcController and parks at every lifecycle gate.
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
    tracing::info!("service created; waiting for OPEN gate to appear");

    // Wait for OPEN to learn (node, target). Then bind a ReplicaClient
    // that owns those for the rest of the test.
    let (node_idx, open_ev) = cluster
        .poll_for_pending(Some(ApprovalKind::ApprovalOpen))
        .await;
    let target = open_ev.target.clone().expect("ApprovalEvent.target");
    tracing::info!(
        "OPEN gate observed on node #{node_idx}: partition={}, replica={}, gate_id={}",
        target.partition_id,
        target.replica_id,
        open_ev.gate_id,
    );

    let mut replica = cluster.replica_client(node_idx, target.clone());

    // ---- Approve OPEN ----
    replica.approve_proceed(open_ev.gate_id.clone()).await;

    // ---- ChangeRole(Primary) ----
    let cr_ev = replica
        .observe_and_approve(ApprovalKind::ApprovalChangeRole)
        .await;
    tracing::info!(
        "CHANGE_ROLE gate observed: new_role={}, gate_id={}",
        cr_ev.new_role,
        cr_ev.gate_id,
    );

    // Service should now be Up. Trigger close by deleting the service.
    // delete_service blocks until the replica's close completes, which
    // itself blocks on our CLOSE-gate Approve, so we must spawn the
    // delete in the background and drain teardown gates concurrently.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc_name = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc_name, SF_TIMEOUT, None).await })
    };

    // ---- Teardown: drain change_role(None) (if any) then Close ----
    // SF may issue any number of change_role gates before close (e.g.
    // demoting Primary -> None). Approve them all; stop after Close.
    loop {
        let ev = replica.drain_next_gate().await;
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        tracing::info!(
            "teardown gate observed: kind={kind:?}, new_role={}, gate_id={}",
            ev.new_role,
            ev.gate_id,
        );
        if matches!(kind, ApprovalKind::ApprovalClose) {
            break;
        }
    }

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    // After close completes the registry entry is gone. A follow-up
    // ListPending should not show the replica.
    tracing::info!("verifying replica is removed from registry");
    let probe = replica.list_pending_in_partition().await;
    assert!(
        probe
            .iter()
            .all(|ev| ev.target.as_ref().map(|t| t.replica_id) != Some(target.replica_id)),
        "replica still appears in ListPending after Close approval: {probe:?}"
    );

    tracing::info!("e2e flow complete");
}
