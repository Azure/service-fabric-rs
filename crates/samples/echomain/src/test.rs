// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::FabricClient,
    types::{
        QueryServiceReplicaStatus, ServicePartition, ServicePartitionInformation,
        ServicePartitionQueryDescription, ServicePartitionStatus, ServiceReplicaQueryDescription,
        ServiceReplicaQueryResult,
    },
    GUID, HSTRING,
};

// Requires app to be deployed on onebox.
// Uses fabric client to get the app parition and replica info.
#[tokio::test]
async fn test_partition_and_replica_info() {
    let fc = FabricClient::new();
    let qc = fc.get_query_manager();

    let desc = ServicePartitionQueryDescription {
        service_name: HSTRING::from("fabric:/EchoApp/EchoAppService"),
        partition_id_filter: None,
    };
    let timeout = Duration::from_secs(1);

    let list = qc.get_partition_list(&desc, timeout).await.unwrap();
    // there is only one partition
    let p = list.iter().next().unwrap();
    let stateless = match p {
        ServicePartition::Stateless(s) => s,
        _ => panic!("not stateless"),
    };

    assert_eq!(stateless.instance_count, 1);
    assert_eq!(stateless.partition_status, ServicePartitionStatus::Ready);
    // For some reason the state is unknown
    // assert_eq!(stateless.health_state, HealthState::Ok);
    let info = stateless.partition_information;
    let single = match info {
        ServicePartitionInformation::Singleton(s) => s,
        _ => panic!("not singleton"),
    };
    assert_ne!(single.id, GUID::zeroed());

    // test get replica info
    let desc = ServiceReplicaQueryDescription {
        partition_id: single.id,
        replica_id_or_instance_id_filter: None,
    };
    let replicas = qc.get_replica_list(&desc, timeout).await.unwrap();
    {
        let replica = replicas.iter().next().unwrap(); // only one replica
        let stateless = match replica {
            ServiceReplicaQueryResult::Stateless(s) => s,
            _ => panic!("not stateless"),
        };
        // TODO: health is unknown
        // assert_eq!(stateless.aggregated_health_state, HealthState::Ok);
        assert_eq!(stateless.replica_status, QueryServiceReplicaStatus::Ready);
        assert_ne!(stateless.node_name, HSTRING::new());
    }
}
