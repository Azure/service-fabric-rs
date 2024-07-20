// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::FabricClient,
    types::{
        QueryServiceReplicaStatus, RestartReplicaDescription, ServicePartition,
        ServicePartitionInformation, ServicePartitionQueryDescription, ServicePartitionStatus,
        ServiceReplicaQueryDescription, ServiceReplicaQueryResult,
    },
    GUID, HSTRING,
};

// Requires app to be deployed on onebox.
// Uses fabric client to perform various actions for this service.
#[tokio::test]
async fn test_partition_info() {
    let fc = FabricClient::new();
    let qc = fc.get_query_manager();

    let timeout = Duration::from_secs(1);

    let desc = ServicePartitionQueryDescription {
        service_name: HSTRING::from("fabric:/StatefulEchoApp/StatefulEchoAppService"),
        partition_id_filter: None,
    };

    let list = qc.get_partition_list(&desc, timeout).await.unwrap();
    // there is only one partition
    let p = list.iter().next().unwrap();
    let stateful = match p {
        ServicePartition::Stateful(s) => s,
        _ => panic!("not stateless"),
    };

    // TODO: not sure why state is unknown.
    // assert_eq!(stateful.health_state, HealthState::Ok);
    assert_eq!(stateful.partition_status, ServicePartitionStatus::Ready);
    assert_eq!(stateful.target_replica_set_size, 3);
    assert_eq!(stateful.min_replica_set_size, 1);
    let info = stateful.partition_information;
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
    let replicas = qc
        .get_replica_list(&desc, timeout)
        .await
        .unwrap()
        .iter()
        .collect::<Vec<_>>();
    assert_eq!(replicas.len(), 3);
    let replica = &replicas[0];
    let stateful_replica = match replica {
        ServiceReplicaQueryResult::Stateful(s) => s,
        _ => panic!("not stateful"),
    };
    assert_eq!(
        stateful_replica.replica_status,
        QueryServiceReplicaStatus::Ready
    );
    assert_ne!(stateful_replica.node_name, HSTRING::new());

    let desc = RestartReplicaDescription {
        node_name: stateful_replica.node_name.clone(),
        partition_id: single.id,
        replica_or_instance_id: stateful_replica.replica_id,
    };
    let mgmt = fc.get_service_manager();
    mgmt.restart_replica(&desc, timeout).await.unwrap();
}
