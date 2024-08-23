// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::{
        svc_mgmt_client::{
            PartitionKeyType, ResolvedServiceEndpoint, ResolvedServicePartitionInfo,
            ServiceEndpointRole, ServicePartitionKind,
        },
        FabricClient,
    },
    error::FabricErrorCode,
    types::{
        QueryServiceReplicaStatus, RemoveReplicaDescription, ServiceNotificationFilterDescription,
        ServiceNotificationFilterFlags, ServicePartition, ServicePartitionInformation,
        ServicePartitionQueryDescription, ServicePartitionStatus, ServiceReplicaQueryDescription,
        ServiceReplicaQueryResult, SingletonPartitionInfomation,
        StatelessServiceInstanceQueryResult, StatelessServicePartition,
    },
    GUID, HSTRING,
};

static ECHO_SVC_URI: &str = "fabric:/EchoApp/EchoAppService";

// Test client for echo server.
pub struct EchoTestClient {
    fc: FabricClient,
    service_uri: HSTRING,
    timeout: Duration,
}

impl EchoTestClient {
    pub fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            service_uri: HSTRING::from(ECHO_SVC_URI),
            timeout: Duration::from_secs(1),
        }
    }

    pub async fn get_partition(&self) -> (StatelessServicePartition, SingletonPartitionInfomation) {
        let qc = self.fc.get_query_manager();
        let desc = ServicePartitionQueryDescription {
            service_name: self.service_uri.clone(),
            partition_id_filter: None,
        };
        let list = qc
            .get_partition_list(&desc, self.timeout, None)
            .await
            .unwrap();
        // there is only one partition
        let p = list.iter().next().unwrap();
        let stateless = match p {
            ServicePartition::Stateless(s) => s,
            _ => panic!("not stateless"),
        };
        let info = stateless.clone().partition_information;
        let single = match info {
            ServicePartitionInformation::Singleton(s) => s,
            _ => panic!("not singleton"),
        };
        (stateless, single)
    }

    pub async fn get_replica(
        &self,
        partition_id: GUID,
    ) -> mssf_core::Result<StatelessServiceInstanceQueryResult> {
        let qc = self.fc.get_query_manager();
        let desc = ServiceReplicaQueryDescription {
            partition_id,
            replica_id_or_instance_id_filter: None,
        };
        let replicas = qc.get_replica_list(&desc, self.timeout, None).await?;
        let replica_op = replicas.iter().next(); // only one replica
        match replica_op {
            Some(replica) => Ok(match replica {
                ServiceReplicaQueryResult::Stateless(s) => s,
                _ => panic!("not stateless"),
            }),
            // replica might be restarting
            None => Err(FabricErrorCode::OperationFailed.into()),
        }
    }

    pub async fn resolve(&self) -> (ResolvedServicePartitionInfo, ResolvedServiceEndpoint) {
        let mgmt = self.fc.get_service_manager();
        let resolved_partition = mgmt
            .resolve_service_partition(
                &self.service_uri,
                &PartitionKeyType::None,
                None,
                self.timeout,
                None,
            )
            .await
            .expect("resolve failed");
        let info = resolved_partition.get_info();
        let endpoints = resolved_partition
            .get_endpoint_list()
            .iter()
            .collect::<Vec<_>>();
        // only has 1 instance
        assert_eq!(endpoints.len(), 1);
        (info, endpoints.first().unwrap().clone())
    }
}

// Requires app to be deployed on onebox.
// Uses fabric client to perform various actions to the app.
#[tokio::test]
async fn test_fabric_client() {
    let fc = FabricClient::new();
    let ec = EchoTestClient::new(fc.clone());

    let timeout = Duration::from_secs(1);
    let service_uri = HSTRING::from(ECHO_SVC_URI);

    // Get partition info
    let (stateless, single) = ec.get_partition().await;
    assert_eq!(stateless.instance_count, 1);
    assert_eq!(stateless.partition_status, ServicePartitionStatus::Ready);
    // For some reason the state is unknown
    // assert_eq!(stateless.health_state, HealthState::Ok);
    assert_ne!(single.id, GUID::zeroed());

    // Get replica info
    let stateless_replica = ec.get_replica(single.id).await.unwrap();

    // TODO: health is unknown
    // assert_eq!(stateless.aggregated_health_state, HealthState::Ok);
    assert_eq!(
        stateless_replica.replica_status,
        QueryServiceReplicaStatus::Ready
    );
    assert_ne!(stateless_replica.node_name, HSTRING::new());

    let mgmt = fc.get_service_manager();
    // register service notification filter
    let filter_handle = {
        let desc = ServiceNotificationFilterDescription {
            name: service_uri.clone(),
            flags: ServiceNotificationFilterFlags::NamePrefix,
        };
        // register takes more than 1 sec.
        mgmt.register_service_notification_filter(&desc, Duration::from_secs(10), None)
            .await
            .unwrap()
    };

    // try resolve the app
    let (info, endpoint) = ec.resolve().await;
    assert_eq!(info.partition_key_type, PartitionKeyType::None);
    assert_eq!(info.service_name, service_uri);
    assert_eq!(info.service_partition_kind, ServicePartitionKind::Singleton);
    assert_eq!(endpoint.role, ServiceEndpointRole::Stateless);

    // Restart the stateless instance by removing it.
    {
        let desc = RemoveReplicaDescription {
            node_name: stateless_replica.node_name,
            partition_id: single.id,
            replica_or_instance_id: stateless_replica.instance_id,
        };
        mgmt.remove_replica(&desc, timeout, None)
            .await
            .expect("Failed to remove replica");
    }

    // replica id should be changed eventually.
    let mut count = 0;
    loop {
        let res = ec.get_replica(single.id).await;
        if res.is_err() {
            continue; // replica might be down.
        }
        let replica2 = res.unwrap();
        if replica2.instance_id != stateless_replica.instance_id {
            break;
        } else {
            if count > 5 {
                panic!(
                    "replica id not changed after retry. original {}, new {}",
                    stateless_replica.instance_id, replica2.instance_id
                );
            }
            // replica has not changed yet.
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        count += 1;
    }

    // unregisters the notification
    mgmt.unregister_service_notification_filter(filter_handle, timeout, None)
        .await
        .unwrap();
}
