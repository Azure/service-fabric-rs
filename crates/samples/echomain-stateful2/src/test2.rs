use crate::test::TestCreateUpdateClient;
use mssf_core::{
    WString,
    client::{
        FabricClient, ServicePartitionResolver,
        svc_mgmt_client::{PartitionKeyType, ServiceEndpointRole},
    },
    types::{ReplicaRole, ServicePartitionInformation, ServicePartitionQueryResult, Uri},
};

async fn restart_primary(uri: &Uri, fc: &FabricClient) {
    let sm = TestCreateUpdateClient::new(fc.clone());
    // find the primary node
    let q = fc.get_query_manager();

    let desc = mssf_core::types::ServicePartitionQueryDescription {
        service_name: uri.clone().0,
        partition_id_filter: None,
    };

    let ptt = q.get_partition_list(&desc, sm.timeout, None).await.unwrap();
    let partitions = ptt
        .iter()
        .filter_map(|p| match p {
            ServicePartitionQueryResult::Stateful(s) => Some(s),
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
            mssf_core::types::ServiceReplicaQueryResult::Stateful(s) => Some(s),
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

/// For manual clean up:
/// Remove-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/ResolveNotificationTest
#[tokio::test]
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
        Some(3), // replica count
    )
    .await;

    // Resolve the service until all replicas are ready.
    let srv = ServicePartitionResolver::builder(fc.clone())
        .with_notification(true)
        .build();
    let mut prev = None;
    let rsp = loop {
        let rsp = srv
            .resolve(&uri.0, &PartitionKeyType::None, prev.as_ref(), None, None)
            .await
            .unwrap();
        if rsp.get_endpoint_list().iter().count() >= 3 {
            break rsp;
        } else {
            prev = Some(rsp);
        }
        println!("Waiting for service to be ready...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    let p1 = rsp
        .get_endpoint_list()
        .iter()
        .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
        .collect::<Vec<_>>();
    assert_eq!(p1.len(), 1);
    let p1 = p1.first().unwrap();

    // Trigger a failover
    restart_primary(&uri, &fc).await;

    // retry for 30 seconds without complaints
    // notification should eventually arrive.
    let mut notified = false;
    for i in 0..30 {
        let rsp2 = srv
            .resolve(&uri.0, &PartitionKeyType::None, None, None, None)
            .await
            .unwrap();
        let p2 = rsp2
            .get_endpoint_list()
            .iter()
            .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
            .collect::<Vec<_>>();
        assert_eq!(p2.len(), 1);
        if i < 1 {
            // Not changed because we do not complain. And notification has not triggered yet.
            let p2 = p2.first().unwrap();
            assert_eq!(rsp.compare_version(&rsp2).unwrap(), 0);
            assert_eq!(p1, p2);
            println!("Primary replica is still the same: {}", p2.address);
        } else {
            // After 2 seconds, we should have a new primary.
            if rsp.compare_version(&rsp2).unwrap() == 0 {
                // Not changed yet.
            } else {
                notified = true;
                assert_ne!(p1, p2.first().unwrap());
                println!(
                    "Primary replica has changed after {i} seconds: {}",
                    p2.first().unwrap().address
                );
                break;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await
    }
    assert!(notified, "Notification did not arrive after 30 seconds.");

    // {
    //     // resolve with complaint
    //     let rsp3 = srv
    //         .resolve(&uri.0, &PartitionKeyType::None, Some(&rsp), None, None)
    //         .await
    //         .unwrap();
    //     let p3 = rsp3
    //         .get_endpoint_list()
    //         .iter()
    //         .filter(|ep| ep.role == ServiceEndpointRole::StatefulPrimary)
    //         .collect::<Vec<_>>();
    //     assert_eq!(p3.len(), 1);
    //     let p3 = p3.first().unwrap();
    //     // Should be different because we complain.
    //     assert!(rsp.compare_version(&rsp3).unwrap() < 0);
    //     assert_ne!(p1, p3);
    //     println!("Primary replica has changed: {}", p3.address);
    // }

    sm.delete_service(&uri).await;
}
