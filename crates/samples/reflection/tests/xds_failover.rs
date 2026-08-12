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
use mssf_net::ads::ServerHandle;
use mssf_net::endpoint::{EndpointSnapshot, EndpointSource};
use mssf_net::{
    AddressError, AddressInterpreter, AdsService, FabricEndpointSource, FabricNaming, HostPort,
    ServiceRegistry, XdsMapping, bootstrap_json,
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
async fn start_ads(mapping: XdsMapping, source: Arc<FabricEndpointSource>) -> ServerHandle {
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
    let bootstrap =
        BootstrapConfig::from_json(&bootstrap_json(ads.addr(), "mssf-net-live-test")).unwrap();
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

    // Teardown, in dependency order: drop the client so its connections close,
    // stop and AWAIT the ADS server, then release the endpoint source. Both
    // FabricClients are subject to issue #184 -- the source owns one (released
    // by its own `shutdown`), and this test owns the admin one.
    drop(client);
    ads.shutdown().await.expect("ads server failed");
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

// ---- two services, one ADS server -----------------------------------------

const SERVICE_URI_A: &str = "fabric:/ReflectionApp/XdsMultiA";
const SERVICE_URI_B: &str = "fabric:/ReflectionApp/XdsMultiB";

/// Delete `uri` if it exists, then create it with a 3-replica set.
///
/// Returns the partition id once the service is ready.
async fn recreate_service(fc: &FabricClient, uri: &Uri) -> mssf_core::GUID {
    if let Err(e) = fc
        .get_service_manager()
        .delete_service2(
            &DeleteServiceDescription::new(uri.clone()),
            Duration::from_secs(10),
            None,
        )
        .await
    {
        tracing::debug!(?e, %uri, "pre-cleanup delete (expected if service doesn't exist)");
    }

    TestCreateUpdateClient::new(fc.clone())
        .create_service(
            uri,
            &mssf_core::types::PartitionSchemeDescription::Singleton,
            TestPartitionReplicaLayout::TargetMinAux(3, 3, 0),
        )
        .await;

    wait_for_ready(fc, uri).await
}

async fn delete_service(fc: &FabricClient, uri: &Uri) {
    fc.get_service_manager()
        .delete_service2(
            &DeleteServiceDescription::new(uri.clone()),
            Duration::from_secs(10),
            None,
        )
        .await
        .unwrap_or_else(|e| panic!("failed to delete {uri}: {e:?}"));
}

/// Build a stock xDS client for `target` against `ads_addr`.
fn xds_greeter(
    ads_addr: std::net::SocketAddr,
    target: &str,
) -> GreeterClient<tonic_xds::XdsChannelGrpc> {
    let bootstrap =
        BootstrapConfig::from_json(&bootstrap_json(ads_addr, "mssf-net-live-multi")).unwrap();
    let uri = XdsUri::parse(target).unwrap();
    let channel = XdsChannelBuilder::new(XdsChannelConfig::new(uri).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .unwrap();
    GreeterClient::new(channel)
}

fn snapshot_of(source: &Arc<FabricEndpointSource>) -> EndpointSnapshot {
    source.subscribe().borrow().clone()
}

/// Two SF services published by **one** ADS server, sharing **one**
/// `FabricClient`.
///
/// What this adds over `tests/scripted_ads.rs`, which already covers the
/// registry, the ALL_RESOURCES_REQUIRED rule and routing isolation with
/// scripted sources: this is the only test that exercises [`FabricNaming`]
/// against a real cluster — one client, one notification callback, two
/// filters, dispatching to two sources by service name. It also covers the
/// teardown ordering (sources, then the shared client) under issue #184.
///
/// The load-bearing assertion is the failover one: relocating A's primary must
/// leave B's published endpoint and serving replica **unchanged**, which is
/// what proves the two notification paths do not cross-talk.
///
/// Note what is deliberately *not* asserted. The reflection sample runs one
/// gRPC server per activated process with a process-wide replica registry, so
/// if SF happens to co-locate A's and B's primaries, both clients reach the
/// same endpoint and no response content can tell them apart. Likewise A's
/// endpoint only moves if its new primary lands in a different process. The
/// test detects and logs both cases rather than pretending to discriminate;
/// the assertions it does make hold either way. Endpoint-level isolation is
/// proven exhaustively and deterministically in `mssf-net`'s
/// `tests/scripted_ads.rs`, which is where that belongs.
#[tokio::test]
#[test_log::test]
async fn two_services_share_one_ads_server() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let uri_a = Uri::from(SERVICE_URI_A);
    let uri_b = Uri::from(SERVICE_URI_B);

    let partition_a = recreate_service(&fc, &uri_a).await;
    let partition_b = recreate_service(&fc, &uri_b).await;
    assert_ne!(partition_a, partition_b);

    // The SF URI doubles as the xDS resource name, so clients target
    // `xds:///fabric:/ReflectionApp/XdsMultiA` with no alias to maintain.
    let mapping_a = XdsMapping::for_service_uri(SERVICE_URI_A, reflection_interpreter()).unwrap();
    let mapping_b = XdsMapping::for_service_uri(SERVICE_URI_B, reflection_interpreter()).unwrap();
    // A distinct xDS name for the same SF service, to prove the duplicate
    // check keys on the service, not on the xDS name.
    let mapping_a_again =
        XdsMapping::new("alias-for-a", SERVICE_URI_A, reflection_interpreter()).unwrap();

    // ONE FabricClient for both services. SF installs the notification
    // callback when the client is built, so a client per service would also
    // mean a naming connection and a callback per service -- exactly the cost
    // that serving many services from one ADS server is meant to avoid.
    let naming = FabricNaming::new(vec![WString::from("localhost:19000")])
        .expect("failed to build the shared naming client");

    let source_a = naming
        .source_for(&mapping_a, Duration::from_secs(10))
        .await
        .expect("failed to build svc-a's endpoint source");
    let source_b = naming
        .source_for(&mapping_b, Duration::from_secs(10))
        .await
        .expect("failed to build svc-b's endpoint source");

    // A second source for a service already mapped on this client would race
    // for the same notifications, so it is refused.
    assert!(
        naming
            .source_for(&mapping_a_again, Duration::from_secs(10))
            .await
            .is_err(),
        "a duplicate source for the same service must be rejected"
    );

    let registry = ServiceRegistry::builder()
        .add(mapping_a, source_a.clone())
        .unwrap()
        .add(mapping_b, source_b.clone())
        .unwrap()
        .build()
        .unwrap();

    let ads = AdsService::from_registry(registry)
        .serve_on_ephemeral_loopback()
        .await
        .expect("failed to start the ADS server");

    // Two stock xDS clients, one bootstrap, one ADS server.
    let mut client_a = xds_greeter(ads.addr(), &format!("xds:///{SERVICE_URI_A}"));
    let mut client_b = xds_greeter(ads.addr(), &format!("xds:///{SERVICE_URI_B}"));

    let replica_a_before =
        serving_primary_until(&mut client_a, partition_a, Duration::from_secs(60)).await;
    let replica_b_before =
        serving_primary_until(&mut client_b, partition_b, Duration::from_secs(60)).await;

    let endpoint_a_before = snapshot_of(&source_a);
    let endpoint_b_before = snapshot_of(&source_b);
    tracing::info!(
        ?endpoint_a_before,
        ?endpoint_b_before,
        replica_a_before,
        replica_b_before,
        "steady state",
    );
    if endpoint_a_before == endpoint_b_before {
        tracing::warn!(
            "svc-a and svc-b are co-located in one activated process; this run \
             cannot discriminate routing by response content"
        );
    }

    // Relocate only A's primary.
    TestClient::with_uri(fc.clone(), uri_a.clone())
        .restart_primary_wait_for_replica_id_change(partition_a)
        .await;

    // A recovers on a different replica, without the client or server
    // restarting.
    let replica_a_after =
        serving_primary_until(&mut client_a, partition_a, Duration::from_secs(90)).await;
    assert_ne!(
        replica_a_before, replica_a_after,
        "after restart_replica svc-a must be served by a different replica"
    );

    // B is untouched: still routing, and its published endpoint never moved.
    // A stale or cross-wired notification filter would show up here.
    let replica_b_after =
        serving_primary_until(&mut client_b, partition_b, Duration::from_secs(60)).await;
    assert_eq!(
        replica_b_before, replica_b_after,
        "svc-b's primary must not have changed"
    );
    assert_eq!(
        endpoint_b_before,
        snapshot_of(&source_b),
        "svc-a's failover must not perturb svc-b's endpoint"
    );

    // Log rather than assert: whether A's *endpoint* moves depends on whether
    // SF placed the new primary in a different activated process. The replica
    // id changing is the guaranteed signal; the endpoint changing is the
    // stronger one when it happens, and is what makes "B unchanged" a contrast
    // rather than a tautology.
    let endpoint_a_after = snapshot_of(&source_a);
    if endpoint_a_after == endpoint_a_before {
        tracing::warn!(
            ?endpoint_a_after,
            "svc-a's new primary landed in the same process, so its endpoint is \
             unchanged; this run proves replica movement but not endpoint movement"
        );
    }

    tracing::info!(
        replica_a_before,
        replica_a_after,
        replica_b_after,
        ?endpoint_a_before,
        ?endpoint_a_after,
        "svc-a failed over; svc-b undisturbed",
    );

    // Teardown in dependency order: clients, then the server, then both
    // sources (each drops its notification filter and route), and only then
    // the shared naming client, which is what actually releases the single
    // FabricClient under issue #184.
    drop(client_a);
    drop(client_b);
    ads.shutdown().await.expect("ads server failed");
    source_a.shutdown().await;
    source_b.shutdown().await;
    naming.shutdown().await;

    delete_service(&fc, &uri_a).await;
    delete_service(&fc, &uri_b).await;
    fabric_client_drop_hack(fc).await;
}
