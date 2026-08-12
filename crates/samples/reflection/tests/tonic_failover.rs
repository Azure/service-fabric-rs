// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Live-cluster integration test for the failover-aware tonic
//! channel built by
//! [`samples_reflection::grpc::build_primary_channel`].
//!
//! Drives the `Write` RPC against `fabric:/ReflectionApp/TonicFailoverTest`
//! through a [`TargetChannel`](mssf_util::tonic::TargetChannel)
//! and asserts that the channel rebuilds (and the next request
//! succeeds against the new primary) after a `restart_replica`
//! triggered failover. Companion to
//! [`failover.rs`](failover.rs) — that test exercises the
//! bare-resolve path; this one exercises the channel.
//!
//! Requires the `ReflectionApp` application package to be
//! deployed on the local onebox (see top-level test
//! instructions). The new partitioned service is created and
//! deleted by the test itself.

use std::time::Duration;

use mssf_core::{
    WString,
    client::FabricClient,
    types::{DeleteServiceDescription, Uri},
};

use samples_reflection::grpc::{build_primary_channel, hello_world::greeter_client::GreeterClient};
use samples_reflection::test_admin::{
    TestClient, TestCreateUpdateClient, TestPartitionReplicaLayout, fabric_client_drop_hack,
    wait_for_ready, write_until_ok,
};

const SERVICE_URI: &str = "fabric:/ReflectionApp/TonicFailoverTest";

/// End-to-end failover via `restart_replica`. Exercises the
/// design's [`Case 1 — connection lost`](../../docs/design/TonicConnectorDesign.md#case-1--connection-lost-transport-level).
#[tokio::test]
#[test_log::test]
async fn tonic_channel_recovers_after_primary_restart() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let uri = Uri::from(SERVICE_URI);

    // Setup: best-effort cleanup of any leftover from a
    // previous failed run, so re-runs are idempotent.
    let sm = TestCreateUpdateClient::new(fc.clone());
    if let Err(e) = fc
        .get_service_manager()
        .delete_service2(
            &DeleteServiceDescription::new(uri.clone()),
            Duration::from_secs(10),
            None,
        )
        .await
    {
        tracing::debug!(?e, "pre-cleanup delete (expected if service doesn't exist)");
    }

    // Create the service.
    sm.create_service(
        &uri,
        &mssf_core::types::PartitionSchemeDescription::Singleton,
        TestPartitionReplicaLayout::TargetMinAux(3, 3, 0),
    )
    .await;

    let partition_id = wait_for_ready(&fc, &uri).await;

    // Build the failover-aware channel.
    let channel = build_primary_channel(fc.clone(), SERVICE_URI);
    let mut client = GreeterClient::new(channel);

    // Steady-state write succeeds on the current primary.
    let acked_first =
        write_until_ok(&mut client, partition_id, "hello", Duration::from_secs(30)).await;
    tracing::info!(%acked_first, "steady-state ack");

    // Trigger failover (restart the current primary). Reuses
    // the helper from `test_admin.rs`, which waits until the
    // primary's node changes.
    let tc = TestClient::with_uri(fc.clone(), uri.clone());
    tc.restart_primary_wait_for_replica_id_change(partition_id)
        .await;

    // Next write should eventually succeed against the new
    // primary. May surface one `Unavailable` (with
    // `mssf-status: not-primary`) or one transport error
    // first, depending on whether hyper's connection pool
    // still holds the old TCP/HTTP2 connection.
    let acked_after_restart = write_until_ok(
        &mut client,
        partition_id,
        "after-restart",
        Duration::from_secs(30),
    )
    .await;
    tracing::info!(%acked_after_restart, "post-failover ack");
    assert_ne!(
        acked_first, acked_after_restart,
        "after restart_replica the ack should come from a different replica"
    );

    // Concurrency / dedup. Restart again, then fire N writes
    // concurrently. All must eventually succeed; we don't
    // attempt to count rebuilds from outside (that would
    // require test-only instrumentation on the resolver) —
    // the deterministic dedup behaviour is covered by the
    // unit-test suite at
    // `crates/libs/util/tests/tonic_middleware.rs`.
    tc.restart_primary_wait_for_replica_id_change(partition_id)
        .await;
    let mut handles = Vec::new();
    for i in 0..5 {
        // `GreeterClient<TargetChannel>` is `Clone` because
        // `TargetChannel` (= `ResolveStatusMiddleware<SwapChannel>`)
        // is `Clone`. Each clone shares the same dedup state via
        // the inner `Arc<Mutex<...>>` in the middleware.
        let mut client_clone = client.clone();
        handles.push(tokio::spawn(async move {
            write_until_ok(
                &mut client_clone,
                partition_id,
                &format!("concurrent-{i}"),
                Duration::from_secs(30),
            )
            .await
        }));
    }
    for h in handles {
        let ack = h.await.unwrap();
        tracing::info!(%ack, "concurrent ack");
    }

    // Teardown.
    sm.delete_service(&uri).await;
    fabric_client_drop_hack(fc).await;
}
