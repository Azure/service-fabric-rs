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
//!     SF -> open()           Approve(Proceed)   // SAME replica_id, fresh activation
//!     SF -> change_role(Primary)  Approve(Proceed)   // service is now Up
//! ```
//!
//! Two non-obvious points:
//!
//! 1. **`replica_id` is preserved across the failure.** SF reuses the
//!    same replica id for the recovery activation rather than minting
//!    a new one. We discovered this by logging `target.replica_id`
//!    on every gate — the first OPEN and the second OPEN see the
//!    *same* id. Polling by `partition_id` (what this test does)
//!    works either way and is more robust against future SF
//!    behaviour changes.
//!
//! 2. **There is a noticeable back-off (~15 s in onebox)** between
//!    the abort approval and the second open. This is SF's internal
//!    retry delay after a transient replica failure. Tests that
//!    exercise this path should expect to be in the
//!    10-second-and-up runtime category, not sub-second.
//!
//! On the gRPC side, each gate gets a fresh `gate_id` UUID even
//! though the `(partition_id, replica_id)` tuple is unchanged — the
//! controller mints a new id every time `await_approval` populates
//! `pending`. So a test holding a stale `gate_id` from before the
//! failure cannot misroute an `Approve` to the recovery gate.
//!
//! ## Discovery strategy
//!
//! Polls by `partition_id` rather than `replica_id`. Even though SF
//! happens to reuse the replica_id in our case, polling by partition
//! also works for any future variation where SF rebuilds with a fresh
//! id, and it lets parallel tests in the same cluster operate with
//! clean isolation by scoping each test's polling to its own
//! partition.
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
use samples_reflection::test_cluster::Cluster;
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

    // Wait for the first OPEN to learn our partition_id. From this
    // point on we poll only within this partition so parallel tests
    // can't see each other's gates.
    let (mut n, first_ev) = cluster
        .poll_for_pending(Some(ApprovalKind::ApprovalOpen))
        .await;
    let partition_id = first_ev
        .target
        .as_ref()
        .expect("ApprovalEvent.target")
        .partition_id
        .clone();
    tracing::info!("OPEN observed; partition_id={partition_id}");

    // Drive the first OPEN through the same loop used for everything
    // else by feeding it as if poll_for_pending_in_partition had just
    // returned it. After this initial seed, every subsequent gate is
    // discovered by polling within the partition.
    let mut next: Option<(
        usize,
        samples_reflection::grpc_control::proto::ApprovalEvent,
    )> = Some((n, first_ev));

    let mut open_count = 0usize;
    let mut change_role_count = 0usize;
    let mut abort_count = 0usize;

    loop {
        let (node_idx, ev) = match next.take() {
            Some(pair) => pair,
            None => {
                cluster
                    .poll_for_pending_in_partition(&partition_id, None)
                    .await
            }
        };
        n = node_idx;
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        let target = ev.target.clone().expect("ApprovalEvent.target");

        match kind {
            ApprovalKind::ApprovalOpen => {
                // Two cases observed in practice:
                //   open #1: the initial replica activation
                //   open #2: SF re-activating the SAME replica_id
                //            after the failed change_role + abort
                open_count += 1;
                tracing::info!(
                    "OPEN #{open_count} replica={} gate_id={} -> approving",
                    target.replica_id,
                    ev.gate_id,
                );
                cluster
                    .replica_client(n, target)
                    .approve_proceed(ev.gate_id)
                    .await;
            }
            ApprovalKind::ApprovalChangeRole => {
                // First change_role: inject a Fail to trigger SF's
                // recovery path. Second change_role (after the abort
                // + reopen): approve so the service comes up.
                change_role_count += 1;
                if change_role_count == 1 {
                    tracing::info!(
                        "CHANGE_ROLE #1 replica={} gate_id={} -> failing",
                        target.replica_id,
                        ev.gate_id,
                    );
                    cluster
                        .replica_client(n, target)
                        .approve_fail(ev.gate_id, "test-induced ChangeRole failure".to_string())
                        .await;
                } else {
                    tracing::info!(
                        "CHANGE_ROLE #{change_role_count} replica={} gate_id={} -> approving",
                        target.replica_id,
                        ev.gate_id,
                    );
                    cluster
                        .replica_client(n, target)
                        .approve_proceed(ev.gate_id)
                        .await;
                    break;
                }
            }
            ApprovalKind::ApprovalAbort => {
                // SF aborts the failed replica before re-opening it.
                // Approve immediately so SF can move on; the
                // ~15s back-off before the second OPEN happens
                // entirely on SF's side after this approval returns.
                abort_count += 1;
                tracing::info!(
                    "ABORT #{abort_count} replica={} gate_id={} -> approving (recovery)",
                    target.replica_id,
                    ev.gate_id,
                );
                cluster
                    .replica_client(n, target)
                    .approve_proceed(ev.gate_id)
                    .await;
            }
            ApprovalKind::ApprovalClose => {
                // Not expected on the recovery path — SF prefers
                // abort over close after a lifecycle failure — but
                // approve defensively if it ever shows up so the
                // test doesn't deadlock.
                tracing::info!(
                    "CLOSE replica={} gate_id={} -> approving (unexpected before retry but ok)",
                    target.replica_id,
                    ev.gate_id,
                );
                cluster
                    .replica_client(n, target)
                    .approve_proceed(ev.gate_id)
                    .await;
            }
            ApprovalKind::ApprovalUnspecified => {
                panic!("ApprovalKind::ApprovalUnspecified observed: {ev:?}");
            }
        }
    }

    tracing::info!(
        "recovery summary: open={open_count}, change_role={change_role_count}, abort={abort_count}"
    );
    assert!(
        change_role_count >= 2,
        "expected SF to issue at least 2 CHANGE_ROLE gates; saw {change_role_count}"
    );
    assert!(
        abort_count >= 1,
        "expected SF to abort the failed replica at least once; saw {abort_count}"
    );

    // Teardown: delete the service and drain remaining gates in this
    // partition until Close is approved.
    //
    // For a normal happy-path teardown SF issues:
    //   change_role(None)   // demote primary
    //   close()
    // The test approves both with Proceed and exits when Close fires.
    tracing::info!("deleting service to trigger teardown gates");
    let delete_handle = {
        let sm = fc.get_service_manager().clone();
        let svc = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&svc, SF_TIMEOUT, None).await })
    };

    loop {
        let (n, ev) = cluster
            .poll_for_pending_in_partition(&partition_id, None)
            .await;
        let kind = ApprovalKind::try_from(ev.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);
        let target = ev.target.clone().expect("ApprovalEvent.target");
        tracing::info!(
            "teardown gate kind={kind:?} replica={} gate_id={}",
            target.replica_id,
            ev.gate_id,
        );
        cluster
            .replica_client(n, target)
            .approve_proceed(ev.gate_id)
            .await;
        if matches!(kind, ApprovalKind::ApprovalClose) {
            break;
        }
    }

    delete_handle
        .await
        .expect("delete_service task panicked")
        .expect("delete_service failed");

    tracing::info!("fail-change-role e2e flow complete");
}
