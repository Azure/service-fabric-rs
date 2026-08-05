// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Live-cluster proof for the xDS successor path.
//!
//! Companion to [`tonic_failover.rs`](tonic_failover.rs): that test drives the
//! incumbent `mssf_util::tonic` channel, this one drives a **stock
//! `tonic-xds` client** through `mssf-net`'s ADS server against the same
//! scenario, reusing the same scaffolding.
//!
//! What it proves that the cluster-free test cannot:
//! - a real `ReflectionUrl` endpoint address is interpreted correctly (SC-008);
//! - the source registers its own SF notification filter and reacts to a
//!   genuine primary relocation (SC-009);
//! - reachability survives a real `restart_replica` failover (SC-003).
//!
//! Requires the `ReflectionApp` application package deployed on the local
//! onebox. The test service itself is created and deleted by the test.

use std::sync::Arc;
use std::time::Duration;

use mssf_core::{
    WString,
    client::FabricClient,
    types::{DeleteServiceDescription, Uri},
};
use mssf_net::endpoint::EndpointSource;
use mssf_net::{
    AddressError, AddressInterpreter, AdsService, FabricEndpointSource, HostPort, XdsMapping,
    bootstrap_json,
};
use samples_reflection::grpc::ReflectionUrl;
use samples_reflection::grpc::hello_world::{ReplicaRole, greeter_client::GreeterClient};
use samples_reflection::test_admin::{
    TestClient, TestCreateUpdateClient, TestPartitionReplicaLayout, fabric_client_drop_hack,
    get_replicas_until_ok, wait_for_ready,
};
use tonic_xds::{BootstrapConfig, XdsChannelBuilder, XdsChannelConfig, XdsUri};

const SERVICE_URI: &str = "fabric:/ReflectionApp/XdsFailoverTest";
const XDS_NAME: &str = "reflection";

/// Interpret the reflection sample's `ReflectionUrl`-encoded endpoint address.
///
/// This is exactly the pluggable-address concern the design calls out: the
/// address is a URL carrying query parameters, **not** a bare `host:port`, so
/// a generic interpreter would not work here.
fn reflection_interpreter() -> AddressInterpreter {
    Arc::new(|raw: &str| {
        let url = ReflectionUrl::parse(raw).map_err(AddressError::Unparseable)?;
        let host = url
            .base_url
            .host_str()
            .ok_or_else(|| AddressError::MissingHost(raw.to_string()))?
            .to_string();
        let port = url
            .base_url
            .port()
            .ok_or_else(|| AddressError::MissingPort(raw.to_string()))?;
        Ok(HostPort::new(host, port))
    })
}

/// Start the ADS server on an ephemeral loopback port.
async fn start_ads(mapping: XdsMapping, source: Arc<FabricEndpointSource>) -> std::net::SocketAddr {
    AdsService::new(mapping, source)
        .serve_on_ephemeral_loopback()
        .await
        .expect("failed to start the ADS server")
}

/// The replica that served the call, isolated to `partition_id`.
///
/// `GetReplicas` returns a list built from a **process-wide** registry keyed by
/// partition id, covering every partition hosted in the answering process.
/// Several reflection test services can share one activated process, so
/// filtering by this test's partition is required — a naive "find any primary"
/// could match another service's primary and pass even if xDS had routed us to
/// a secondary.
fn find_serving_replica(
    resp: &samples_reflection::grpc::hello_world::GetReplicasResponse,
    partition_id: mssf_core::GUID,
) -> Option<(i64, i32)> {
    let want = format!("{partition_id:?}");
    let mine: Vec<_> = resp
        .replicas
        .iter()
        .filter(|r| r.partition_id.eq_ignore_ascii_case(&want))
        .collect();
    match mine.len() {
        0 => None,
        1 => Some((mine[0].replica_id, mine[0].role)),
        _ => panic!("expected at most one replica entry for partition {want}, got {mine:?}"),
    }
}

/// Poll until the serving process reports **our** partition's primary.
///
/// A bare `get_replicas` call is not sufficient on its own: it succeeds as soon
/// as *some* process answers, and immediately after a failover that process may
/// legitimately report an empty registry (the replica has not re-registered, or
/// xDS has not yet pushed the new endpoint). Retrying only on transport failure
/// would therefore accept a response that says nothing about our partition.
async fn serving_primary_until(
    client: &mut GreeterClient<tonic_xds::XdsChannelGrpc>,
    partition_id: mssf_core::GUID,
    timeout: Duration,
) -> i64 {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let resp = get_replicas_until_ok(client, Duration::from_secs(30)).await;
        let found = find_serving_replica(&resp, partition_id);
        if let Some((replica_id, role)) = found
            && role == ReplicaRole::Primary as i32
        {
            return replica_id;
        }
        if tokio::time::Instant::now() >= deadline {
            panic!(
                "no primary for partition {partition_id:?} within {timeout:?}; \
                 last match: {found:?}, full response: {:?}",
                resp.replicas
            );
        }
        tracing::debug!(?found, "not our primary yet; retrying");
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// SC-001/002/003/008/009: a stock xDS client reaches the reflection service's
/// primary with no SF-specific request metadata, and keeps reaching it (at a
/// *different* replica) after a real primary relocation.
#[tokio::test]
#[test_log::test]
async fn xds_client_recovers_after_primary_restart() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let uri = Uri::from(SERVICE_URI);

    // Best-effort cleanup so re-runs are idempotent.
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

    // A replica set larger than one makes "primary only" a meaningful claim.
    sm.create_service(
        &uri,
        &mssf_core::types::PartitionSchemeDescription::Singleton,
        TestPartitionReplicaLayout::TargetMinAux(3, 3, 0),
    )
    .await;

    let partition_id = wait_for_ready(&fc, &uri).await;

    // The successor path: SF naming -> xDS, with its own FabricClient and its
    // own notification filter.
    let mapping = XdsMapping::new(XDS_NAME, SERVICE_URI, reflection_interpreter()).unwrap();
    let source = FabricEndpointSource::new(
        &mapping,
        vec![WString::from("localhost:19000")],
        Duration::from_secs(10),
    )
    .await
    .expect("failed to build the fabric endpoint source");

    let ads = start_ads(mapping, source.clone()).await;

    // A completely stock xDS client. No SF types, no SF metadata.
    let bootstrap = BootstrapConfig::from_json(&bootstrap_json(ads, "mssf-net-live-test")).unwrap();
    let target = XdsUri::parse(&format!("xds:///{XDS_NAME}")).unwrap();
    let channel = XdsChannelBuilder::new(XdsChannelConfig::new(target).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .unwrap();
    let mut client = GreeterClient::new(channel);

    // Steady state: we reach the primary. The helper asserts both the role and
    // the partition isolation.
    let replica_before =
        serving_primary_until(&mut client, partition_id, Duration::from_secs(60)).await;
    tracing::info!(replica_before, "steady-state primary");

    // Trigger a genuine relocation.
    let tc = TestClient::with_uri(fc.clone(), uri.clone());
    tc.restart_primary_wait_for_replica_id_change(partition_id)
        .await;

    // The same client, against the same ADS server, must reach the new primary
    // without either being restarted.
    let replica_after =
        serving_primary_until(&mut client, partition_id, Duration::from_secs(90)).await;
    assert_ne!(
        replica_before, replica_after,
        "after restart_replica the serving replica must have changed"
    );
    tracing::info!(
        replica_before,
        replica_after,
        "recovered on the new primary"
    );

    // Teardown. Both FabricClients are subject to issue #184: the source owns
    // one (released by `shutdown`), and this test owns the admin one.
    source.shutdown().await;

    fc.get_service_manager()
        .delete_service2(
            &DeleteServiceDescription::new(uri.clone()),
            Duration::from_secs(10),
            None,
        )
        .await
        .expect("failed to delete the test service");

    fabric_client_drop_hack(fc).await;
}
