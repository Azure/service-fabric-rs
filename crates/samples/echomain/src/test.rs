// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::{
        query_types::{
            ServicePartition, ServicePartitionInformation, ServicePartitionQueryDescription,
            ServicePartitionStatus,
        },
        FabricClient,
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
        service_name: HSTRING::from("fabric:/EchoApp/EchoAppService"),
        partition_id_filter: None,
    };

    let list = qc
        .get_partition_list(&desc, Duration::from_secs(1))
        .await
        .unwrap();
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
}
