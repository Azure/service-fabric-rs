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
use samples_reflection::grpc_control::proto::{ApprovalKind, ReplicaRole as ProtoReplicaRole};
use samples_reflection::test_cluster::{Cluster, TestStep, discover_partition_id};
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

    // Discover the partition_id for this specific service via the SF
    // query manager rather than relying on cluster-wide polling. That
    // way we don't accidentally pick up an OPEN gate belonging to a
    // sibling test running on the same cluster.
    let partition_id = discover_partition_id(&fc, &service_name).await;
    tracing::info!("partition_id={partition_id}");

    let mut driver = cluster.partition_driver(partition_id);
    let (node_idx, open_ev) = driver.wait_next_kind(ApprovalKind::ApprovalOpen).await;
    let target = open_ev.target.clone().expect("ApprovalEvent.target");
    let replica_id = target.replica_id;
    tracing::info!(
        "OPEN gate observed on node #{node_idx}: replica={replica_id}, gate_id={}",
        open_ev.gate_id,
    );

    driver
        .approve_proceed(node_idx, target.clone(), open_ev.gate_id.clone())
        .await;

    // Drive the rest of the singleton replica's lifecycle as one
    // per-replica sequence. SF emits ChangeRole(Primary), and once
    // the service is Up we delete it (in a background task) so SF
    // emits ChangeRole(None) -> Close. The Approve for Close is
    // what unblocks the delete task.
    let observed = driver
        .approve_replica_sequence(replica_id, &[ApprovalKind::ApprovalChangeRole])
        .await;
    let cr_ev = &observed[0];
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

    // Teardown sequence for the same replica: ChangeRole(None) then Close.
    let teardown = driver
        .drive_replica_sequence(
            replica_id,
            &[
                TestStep::proceed(ApprovalKind::ApprovalChangeRole),
                TestStep::proceed(ApprovalKind::ApprovalClose),
            ],
        )
        .await;
    for ev in &teardown {
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        tracing::info!(
            "teardown gate observed: kind={kind:?}, new_role={}, gate_id={}",
            ev.new_role,
            ev.gate_id,
        );
    }

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    // After close completes the registry entry is gone. A follow-up
    // ListPending in the partition should not show the replica.
    tracing::info!("verifying replica is removed from registry");
    let probe = driver.list_pending().await;
    assert!(
        probe
            .iter()
            .all(|ev| ev.target.as_ref().map(|t| t.replica_id) != Some(replica_id)),
        "replica still appears in ListPending after Close approval: {probe:?}"
    );

    tracing::info!("e2e flow complete");
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn approve_open_change_role_close_two_replicas() {
    // Two-replica happy path: target=2, min=2.
    //
    // SF activation order is **not** uniform across platforms:
    // - On Linux onebox SF tends to be strictly serial (primary
    //   reaches stable role before the secondary's Open is even
    //   created), so the first OPEN observed is always the primary.
    // - On Windows onebox SF can interleave: it places both
    //   replicas first, then issues ChangeRole — which means the
    //   *first* OPEN may belong to the replica that ends up as
    //   IdleSecondary.
    //
    // Rather than encode that platform difference, the test:
    //   1. Discovers the partition via the SF query manager.
    //   2. Drains every gate within the partition, approving each.
    //   3. Identifies which replica is primary / secondary by the
    //      `new_role` on its ChangeRole event, not by ordering.
    //   4. Stops once a Primary and an ActiveSecondary have been
    //      observed.
    // Then teardown drives the deterministic `ChangeRole(None) ->
    // Close` per replica via the server-side `replica_filter`.

    let svc_suffix = Uuid::new_v4().simple().to_string();
    let service_name_str = format!("{APP_NAME}/ApprovalE2eTwo_{svc_suffix}");
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
        .with_min_replica_set_size(2)
        .with_target_replica_set_size(2)
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
    tracing::info!("service {service_name_str} created (target=2 min=2)");

    let partition_id = discover_partition_id(&fc, &service_name).await;
    tracing::info!("partition_id={partition_id}");
    let mut driver = cluster.partition_driver(partition_id);

    // ---- Activation drain ----
    //
    // Approve every gate the partition emits, classifying each
    // ChangeRole's `new_role`. Stop once one replica has reached
    // Primary and another has reached ActiveSecondary.
    let mut primary_id: Option<i64> = None;
    let mut secondary_id: Option<i64> = None;
    let mut seen_replicas: Vec<i64> = Vec::new();

    while primary_id.is_none() || secondary_id.is_none() {
        let (n, ev) = driver.wait_next().await;
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        let target = ev.target.clone().expect("ApprovalEvent.target");
        let rid = target.replica_id;

        if !seen_replicas.contains(&rid) {
            seen_replicas.push(rid);
            tracing::info!("discovered replica={rid} on node #{n} via {kind:?}");
        }

        if kind == ApprovalKind::ApprovalChangeRole {
            if ev.new_role == ProtoReplicaRole::Primary as i32 {
                primary_id = Some(rid);
                tracing::info!("replica {rid} -> Primary (gate_id={})", ev.gate_id);
            } else if ev.new_role == ProtoReplicaRole::ActiveSecondary as i32 {
                secondary_id = Some(rid);
                tracing::info!("replica {rid} -> ActiveSecondary (gate_id={})", ev.gate_id);
            } else {
                tracing::info!(
                    "replica {rid} ChangeRole intermediate new_role={} gate_id={}",
                    ev.new_role,
                    ev.gate_id,
                );
            }
        }

        driver.approve_proceed(n, target, ev.gate_id.clone()).await;
    }

    let primary = primary_id.expect("Primary replica observed");
    let secondary = secondary_id.expect("ActiveSecondary replica observed");
    assert_ne!(primary, secondary, "primary and secondary must differ");
    assert_eq!(
        seen_replicas.len(),
        2,
        "expected exactly 2 replica IDs across activation, got {seen_replicas:?}",
    );
    tracing::info!("activation done; primary={primary} secondary={secondary}");

    // ---- Teardown ----
    //
    // Delete the service in the background; SF will drive both
    // replicas through ChangeRole(None) -> Close. Each per-replica
    // sequence uses server-side `replica_filter`, so call order is
    // independent of SF's actual close ordering.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    let teardown_steps = [
        TestStep::proceed(ApprovalKind::ApprovalChangeRole),
        TestStep::proceed(ApprovalKind::ApprovalClose),
    ];

    let primary_teardown = driver
        .drive_replica_sequence(primary, &teardown_steps)
        .await;
    assert_eq!(
        primary_teardown[0].new_role,
        ProtoReplicaRole::None as i32,
        "primary teardown ChangeRole new_role should be None, got {}",
        primary_teardown[0].new_role,
    );
    tracing::info!("primary {primary} teardown done");

    let secondary_teardown = driver
        .drive_replica_sequence(secondary, &teardown_steps)
        .await;
    assert_eq!(
        secondary_teardown[0].new_role,
        ProtoReplicaRole::None as i32,
        "secondary teardown ChangeRole new_role should be None, got {}",
        secondary_teardown[0].new_role,
    );
    tracing::info!("secondary {secondary} teardown done");

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    // After both Closes the partition's registry is empty.
    let probe = driver.list_pending().await;
    assert!(
        probe.iter().all(|ev| {
            let rid = ev.target.as_ref().map(|t| t.replica_id);
            rid != Some(primary) && rid != Some(secondary)
        }),
        "replica still pending after Close: {probe:?}"
    );

    tracing::info!("two-replica e2e flow complete");
}
