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
    GUID, WString,
    client::{FabricClient, svc_mgmt_client::PartitionKeyType},
    types::Uri,
};
use mssf_util::resolve::ServicePartitionResolver;
use mssf_util::tonic::TargetChannel;
use tonic::Request;

use samples_reflection::grpc::{
    MSSF_PARTITION_ID_HEADER, build_primary_channel,
    hello_world::{WriteRequest, greeter_client::GreeterClient},
};
use samples_reflection::test_admin::{
    TestClient, TestCreateUpdateClient, TestPartitionReplicaLayout,
};

const SERVICE_URI: &str = "fabric:/ReflectionApp/TonicFailoverTest";

/// Invalid memory access https://github.com/Azure/service-fabric-rs/issues/184
/// happens when this test finishes. Delaying the process clean
/// up is helping with the issue. Mirrors the helper of the same
/// name in [`failover.rs`](failover.rs).
async fn fabric_client_drop_hack(fc: FabricClient) {
    tokio::time::sleep(Duration::from_secs(5)).await;
    drop(fc);
    tokio::time::sleep(Duration::from_secs(1)).await;
}

/// Resolve the test service until it has the full target
/// replica set, then return the singleton partition id.
async fn wait_for_ready(fc: &FabricClient, uri: &Uri) -> GUID {
    let retryer = mssf_util::retry::OperationRetryer::builder().build();
    let srv = ServicePartitionResolver::new(fc.clone(), retryer);
    let mut prev = None;
    loop {
        let rsp = srv
            .resolve(uri, &PartitionKeyType::None, prev.as_ref(), None, None)
            .await
            .unwrap();
        if rsp.endpoints.len() >= 3 {
            // Find the partition id by querying SF for the
            // service partitions (RSP doesn't carry it
            // directly).
            let tc = TestClient::with_uri(fc.clone(), uri.clone());
            let (_, single) = tc.get_partition().await.unwrap();
            return single.id;
        }
        prev = Some(rsp);
        tracing::debug!("Waiting for service to be ready...");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

/// Build a `Write` request with the required
/// `mssf-partition-id` metadata header. The header value uses
/// the GUID's `Debug` formatting (the canonical hyphenated UUID
/// representation, e.g. `{12345678-1234-1234-1234-1234567890ab}`)
/// so the server's `uuid::Uuid::parse_str` accepts it.
fn write_request(partition_id: GUID, payload: &str) -> Request<WriteRequest> {
    let mut req = Request::new(WriteRequest {
        payload: payload.to_string(),
    });
    // Debug format is `{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`;
    // strip the surrounding braces so `Uuid::parse_str` accepts
    // the plain hyphenated form.
    let raw = format!("{:?}", partition_id);
    let header_value = raw.trim_matches(|c| c == '{' || c == '}').to_string();
    req.metadata_mut().insert(
        MSSF_PARTITION_ID_HEADER,
        header_value.parse().expect("valid ascii UUID"),
    );
    req
}

/// Try `write` up to `max_attempts` times, returning the first
/// `Ok` ack string. Treats `Unavailable` and transport errors
/// as retryable — those are exactly the surfaces the failover
/// channel is supposed to recover from.
async fn write_until_ok(
    client: &mut GreeterClient<TargetChannel>,
    partition_id: GUID,
    payload: &str,
    max_attempts: usize,
) -> String {
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match client.write(write_request(partition_id, payload)).await {
            Ok(resp) => {
                let acked_by = resp.into_inner().acked_by;
                tracing::info!(attempt, %acked_by, "write succeeded");
                return acked_by;
            }
            Err(status) => {
                tracing::info!(attempt, ?status, "write returned error; retrying");
                last_err = Some(status);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
    panic!(
        "write did not succeed within {max_attempts} attempts; last error: {:?}",
        last_err
    );
}

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

    // Setup: create the service.
    let sm = TestCreateUpdateClient::new(fc.clone());
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
    let acked_first = write_until_ok(&mut client, partition_id, "hello", 3).await;
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
    let acked_after_restart = write_until_ok(&mut client, partition_id, "after-restart", 5).await;
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
                5,
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
