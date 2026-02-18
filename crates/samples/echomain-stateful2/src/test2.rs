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
use mssf_util::{resolve::ServicePartitionResolver, tokio::TokioExecutor};

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
        .service_partitions
        .into_iter()
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
        .service_replicas
        .into_iter()
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
        .endpoints
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
            .endpoints
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
        .endpoints
        .first()
        .unwrap()
        .address
        .clone();
    tracing::info!(
        "Addr changed: {old_addr} -> {new_addr}, after {elapsed:?}, complain: {complain}"
    );
    rsp_final.unwrap()
}

/// Invalid memory access https://github.com/Azure/service-fabric-rs/issues/184 happens when this test finishes.
/// Delay the process clean up is helping with the issue.
/// It seems like FabricClient cleanup has some background tasks that need to finish.
/// This is a bug in FabricClient.
async fn fabric_client_drop_hack(fc: FabricClient) {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    drop(fc);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

/// For manual clean up:
/// Remove-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/ResolveNotificationTest
/// Resolve-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/ResolveNotificationTest -PartitionKindSingleton
#[tokio::test]
#[test_log::test]
async fn test_resolve_notification() {
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
        // target 3, min 3 and aux 0.
        crate::test::TestPartitionReplicaLayout::TargetMinAux(3, 3, 0),
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
    let retryer = mssf_util::retry::OperationRetryer::builder().build();
    let srv = ServicePartitionResolver::new(fc.clone(), retryer);
    let mut prev = None;
    let rsp = loop {
        let rsp = srv
            .resolve(&uri, &PartitionKeyType::None, prev.as_ref(), None, None)
            .await
            .unwrap();
        if rsp.endpoints.len() >= 3 {
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

    fabric_client_drop_hack(fc).await;
}

#[tokio::test]
#[test_log::test]
async fn test_aux_replicas() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let uri = Uri::from("fabric:/StatefulEchoApp/AuxiliaryReplicaTest");
    // Create the service
    let sm = TestCreateUpdateClient::new(fc.clone());
    sm.create_service(
        &uri,
        &mssf_core::types::PartitionSchemeDescription::Singleton,
        // target 3, min 2 and aux 1.
        crate::test::TestPartitionReplicaLayout::TargetMinAux(3, 2, 1),
    )
    .await;

    let retryer = mssf_util::retry::OperationRetryer::builder().build();
    let srv = ServicePartitionResolver::new(fc.clone(), retryer);

    async fn resolve_until_condition(
        srv: &ServicePartitionResolver,
        uri: &Uri,
        prev: Option<ResolvedServicePartition>,
        condition: impl Fn(&ResolvedServicePartition) -> bool,
    ) -> ResolvedServicePartition {
        let mut prev = prev;
        let max_retry = 60; // retry for 1 minutes
        let mut retry_count = 0;
        loop {
            let rsp = srv
                .resolve(uri, &PartitionKeyType::None, prev.as_ref(), None, None)
                .await
                .unwrap();
            if condition(&rsp) {
                break rsp;
            } else {
                prev = Some(rsp);
            }
            tracing::debug!("Waiting for condition to be met...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            retry_count += 1;
            if retry_count >= max_retry {
                panic!("Condition not met within the maximum retry limit");
            }
        }
    }

    fn get_replica_counts(rsp: &ResolvedServicePartition) -> (usize, usize, usize) {
        // Check there is 1 primary, 1 secondary and 1 aux replica.
        let primary_count = rsp
            .endpoints
            .iter()
            .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
            .count();
        let secondary_count = rsp
            .endpoints
            .iter()
            .filter(|ep| ep.role == ServiceEndpointRole::StatefulSecondary)
            .count();
        let aux_count = rsp
            .endpoints
            .iter()
            .filter(|ep| ep.role == ServiceEndpointRole::StatefulAuxiliary)
            .count();
        (primary_count, secondary_count, aux_count)
    }

    // Resolve the service until all replicas are ready.
    let rsp = resolve_until_condition(&srv, &uri, None, |rsp| rsp.endpoints.len() >= 3).await;

    let (primary_count, secondary_count, aux_count) = get_replica_counts(&rsp);
    assert_eq!(primary_count, 1);
    assert_eq!(secondary_count, 1);
    assert_eq!(aux_count, 1);

    // Update the service to have 0 aux replica.
    sm.update_service_replica_layout(
        &uri,
        crate::test::TestPartitionReplicaLayout::TargetMinAux(3, 3, 0),
    )
    .await;

    let rsp = resolve_until_condition(&srv, &uri, Some(rsp), |rsp| {
        let (primary_count, secondary_count, aux_count) = get_replica_counts(rsp);
        primary_count == 1 && secondary_count == 2 && aux_count == 0
    })
    .await;

    // Update the service to have 1 aux replica again.
    sm.update_service_replica_layout(
        &uri,
        crate::test::TestPartitionReplicaLayout::TargetMinAux(3, 2, 1),
    )
    .await;

    let _rsp = resolve_until_condition(&srv, &uri, Some(rsp), |rsp| {
        let (primary_count, secondary_count, aux_count) = get_replica_counts(rsp);
        primary_count == 1 && secondary_count == 1 && aux_count == 1
    })
    .await;

    sm.delete_service(&uri).await;

    fabric_client_drop_hack(fc).await;
}

async fn test_replica_mock(replica_count: usize) {
    use crate::Factory;
    let rt = TokioExecutor::new(tokio::runtime::Handle::current());
    let factory = Box::new(Factory::create(12312, "localhost".into(), rt));

    let mut driver = mssf_util::mock::StatefulServicePartitionDriver::new();
    driver.register_service_factory(factory);
    let args = mssf_util::mock::CreateStatefulServicePartitionArg {
        partition_id: 1.into(),
        replica_count,
        service_type_name: crate::SERVICE_TYPE_NAME.into(),
        service_name: "fabric:/StatefulEchoApp/DummyTest".into(),
        init_data: vec![],
    };
    driver.create_service_partition(&args).await.unwrap();

    let primary_id = driver.get_primary_replica_id();
    let replica_count = driver.list_replica_ids().len();
    assert_eq!(replica_count, args.replica_count);

    // restart secondaries one by one
    let secondary_ids = driver
        .list_replica_ids()
        .into_iter()
        .filter(|id| *id != primary_id)
        .collect::<Vec<_>>();
    for sid in &secondary_ids {
        tracing::info!("Restarting secondary with id {}", sid);
        driver.restart_secondary_graceful(*sid).await.unwrap();
    }
    // primary should not change
    assert_eq!(driver.get_primary_replica_id(), primary_id);
    assert_eq!(driver.list_replica_ids().len(), replica_count);

    tracing::info!("Deleting the service partition");
    driver.delete_service_partition().await.unwrap();
}

// Test logs are too verbose. Uncomment to enable logs.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// #[test_log::test]
async fn test_replica_mock_1_replica() {
    test_replica_mock(1).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// #[test_log::test]
async fn test_replica_mock_2_replicas() {
    test_replica_mock(2).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[test_log::test]
async fn test_replica_mock_3_replicas() {
    test_replica_mock(3).await;
}
