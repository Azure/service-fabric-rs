// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::FabricClient,
    types::{
        ServicePartition, ServicePartitionInformation, ServicePartitionQueryDescription,
        ServicePartitionStatus,
    },
    GUID, HSTRING,
};

// Requires app to be deployed on onebox.
// Uses fabric client to to the app parition info.
#[tokio::test]
async fn test_partition_info() {
    let fc = FabricClient::new();
    let qc = fc.get_query_manager();

    let desc = ServicePartitionQueryDescription {
        service_name: HSTRING::from("fabric:/StatefulEchoApp/StatefulEchoAppService"),
        partition_id_filter: None,
    };

    let list = qc
        .get_partition_list(&desc, Duration::from_secs(1))
        .await
        .unwrap();
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
}
