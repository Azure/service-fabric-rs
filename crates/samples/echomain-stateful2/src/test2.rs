// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::test::TestCreateUpdateClient;
use mssf_core::{
    WString,
    client::{
        FabricClient,
        svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition, ServiceEndpointRole},
    },
    types::{ReplicaRole, ServicePartitionInformation, ServicePartitionQueryResultItem, Uri},
};
use mssf_util::resolve::ServicePartitionResolver;

async fn restart_primary(uri: &Uri, fc: &FabricClient) {
    let sm = TestCreateUpdateClient::new(fc.clone());
    // find the primary node
    let q = fc.get_query_manager();

    let desc = mssf_core::types::ServicePartitionQueryDescription {
        service_name: uri.clone(),
        partition_id_filter: None,
    };

    let ptt = q.get_partition_list(&desc, sm.timeout, None).await.unwrap();
    let partitions = ptt
        .iter()
        .filter_map(|p| match p {
            ServicePartitionQueryResultItem::Stateful(s) => Some(s),
            _ => None,
        })
        .filter_map(|p| match p.partition_information {
            ServicePartitionInformation::Singleton(s) => Some(s),
            _ => None,
        })
        .collect::<Vec<_>>();
    let partition = partitions.first().unwrap();
    let desc = mssf_core::types::ServiceReplicaQueryDescription {
        partition_id: partition.id,
        replica_id_or_instance_id_filter: None,
    };
    let replicas = q
        .get_replica_list(&desc, sm.timeout, None)
        .await
        .unwrap()
        .iter()
        .filter_map(|r| match r {
            mssf_core::types::ServiceReplicaQueryResultItem::Stateful(s) => Some(s),
            _ => None,
        })
        .filter_map(|r| match r.replica_role {
            ReplicaRole::Primary => Some(r),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(replicas.len(), 1);
    let primary_replica = replicas.first().unwrap();
    // restart the primary replica
    let desc = mssf_core::types::RestartReplicaDescription {
        partition_id: partition.id,
        node_name: primary_replica.node_name.clone(),
        replica_or_instance_id: primary_replica.replica_id,
    };
    fc.get_service_manager()
        .restart_replica(&desc, sm.timeout, None)
        .await
        .unwrap();
}

// returns the new rsp.
async fn resolve_until_change(
    srv: &ServicePartitionResolver,
    uri: &Uri,
    prev: ResolvedServicePartition,
    complain: bool,
) -> ResolvedServicePartition {
    let start_time = std::time::Instant::now();
    // retry for 30 seconds without complaints
    // notification should eventually arrive.
    let prev_original = prev.clone();
    let p1 = prev_original
        .get_endpoint_list()
        .iter()
        .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
        .collect::<Vec<_>>();
    assert_eq!(p1.len(), 1);
    let p1 = p1.first().unwrap();
    let old_addr = p1.address.clone();
    // Use prev if we complain to force client to refresh cache.
    let mut rsp_opt = if complain { Some(prev) } else { None };
    let mut rsp_final = None;
    for _ in 0..30 {
        let new_rsp = srv
            .resolve(uri, &PartitionKeyType::None, rsp_opt.as_ref(), None, None)
            .await
            .unwrap();
        let p2 = new_rsp
            .get_endpoint_list()
            .iter()
            .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
            .collect::<Vec<_>>();
        assert_eq!(p2.len(), 1);

        // We should eventually have a new primary.
        if prev_original.compare_version(&new_rsp).unwrap() < 0 && p1 != p2.first().unwrap() {
            rsp_final = Some(new_rsp);
            break;
        } else {
            // Not changed yet, retry.
        }

        if complain {
            rsp_opt = Some(new_rsp);
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await
    }
    assert!(
        rsp_final.is_some(),
        "Notification did not arrive after 30 seconds."
    );
    let elapsed = start_time.elapsed();
    assert!(elapsed.as_secs() < 30, "Test took too long.");
    let new_addr = rsp_final
        .as_ref()
        .unwrap()
        .get_endpoint_list()
        .iter()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .address
        .clone();
    tracing::info!(
        "Addr changed: {old_addr} -> {new_addr}, after {elapsed:?}, complain: {complain}"
    );
    rsp_final.unwrap()
}

/// For manual clean up:
/// Remove-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/ResolveNotificationTest
/// Resolve-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/ResolveNotificationTest -PartitionKindSingleton
#[tokio::test]
async fn test_resolve_notification() {
    // set up tracing
    let _ = tracing_subscriber::fmt().try_init();
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let uri = Uri::from("fabric:/StatefulEchoApp/ResolveNotificationTest");
    // Create the service
    let sm = TestCreateUpdateClient::new(fc.clone());
    sm.create_service(
        &uri,
        &mssf_core::types::PartitionSchemeDescription::Singleton,
        Some(3), // replica count
    )
    .await;

    // Register notification of the service.
    let filter_id = {
        let desc = mssf_core::types::ServiceNotificationFilterDescription {
            name: uri.clone(),
            flags: mssf_core::types::ServiceNotificationFilterFlags::NamePrefix,
        };
        fc.get_service_manager()
            .register_service_notification_filter(&desc, sm.timeout, None)
            .await
            .unwrap()
    };

    // Resolve the service until all replicas are ready.
    let srv = ServicePartitionResolver::builder(fc.clone()).build();
    let mut prev = None;
    let rsp = loop {
        let rsp = srv
            .resolve(&uri, &PartitionKeyType::None, prev.as_ref(), None, None)
            .await
            .unwrap();
        if rsp.get_endpoint_list().iter().count() >= 3 {
            break rsp;
        } else {
            prev = Some(rsp);
        }
        tracing::debug!("Waiting for service to be ready...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    // Trigger a failover
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    restart_primary(&uri, &fc).await;
    // (it happended once that primary not changed after restart, need to stress this in future
    // tests to find if the notification is missing or restart failed.)
    let rsp2 = resolve_until_change(&srv, &uri, rsp.clone(), false).await;
    // restart primary again
    restart_primary(&uri, &fc).await;
    let rsp3 = resolve_until_change(&srv, &uri, rsp2, true).await;
    restart_primary(&uri, &fc).await;
    let rsp4 = resolve_until_change(&srv, &uri, rsp3, true).await;
    restart_primary(&uri, &fc).await;
    let _ = resolve_until_change(&srv, &uri, rsp4, false).await;

    // Unregister the notification filter.
    fc.get_service_manager()
        .unregister_service_notification_filter(filter_id, sm.timeout, None)
        .await
        .unwrap();
    sm.delete_service(&uri).await;

    // Invalid memory access Issue 184 happens when this test finishes.
    // Delay the process clean up is helping with the issue.
    // It seems like FabricClient cleanup has some background tasks that need to finish.
    // This is a bug in FabricClient.
    drop(fc);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
