// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    client::{
        svc_mgmt_client::{
            PartitionKeyType, ResolvedServiceEndpoint, ResolvedServicePartition,
            ServiceEndpointRole, ServicePartitionKind,
        },
        FabricClient,
    },
    error::FabricErrorCode,
    types::{
        QueryServiceReplicaStatus, ReplicaRole, RestartReplicaDescription,
        ServiceNotificationFilterDescription, ServiceNotificationFilterFlags, ServicePartition,
        ServicePartitionInformation, ServicePartitionQueryDescription, ServicePartitionStatus,
        ServiceReplicaQueryDescription, ServiceReplicaQueryResult, SingletonPartitionInfomation,
        StatefulServicePartition, StatefulServiceReplicaQueryResult,
    },
    GUID, HSTRING,
};

static SVC_URI: &str = "fabric:/StatefulEchoApp/StatefulEchoAppService";

/// Test client for the stateful service
pub struct TestClient {
    fc: FabricClient,
    service_uri: HSTRING,
    timeout: Duration,
}

impl TestClient {
    fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            service_uri: HSTRING::from(SVC_URI),
            timeout: Duration::from_secs(1),
        }
    }

    async fn get_partition(
        &self,
    ) -> mssf_core::Result<(StatefulServicePartition, SingletonPartitionInfomation)> {
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
        let stateful = match p {
            ServicePartition::Stateful(s) => s,
            _ => panic!("not stateless"),
        };
        let info = stateful.clone().partition_information;
        let single = match info {
            ServicePartitionInformation::Singleton(s) => s,
            _ => panic!("not singleton"),
        };
        Ok((stateful, single))
    }

    // primary replica is returned first.
    async fn get_replicas(
        &self,
        partition_id: GUID,
    ) -> mssf_core::Result<(
        StatefulServiceReplicaQueryResult,
        StatefulServiceReplicaQueryResult,
        StatefulServiceReplicaQueryResult,
    )> {
        let qc = self.fc.get_query_manager();
        // test get replica info
        let desc = ServiceReplicaQueryDescription {
            partition_id,
            replica_id_or_instance_id_filter: None,
        };
        let replicas = qc
            .get_replica_list(&desc, self.timeout, None)
            .await?
            .iter()
            .collect::<Vec<_>>();
        if replicas.len() < 3 {
            // replica are not ready.
            return Err(FabricErrorCode::OperationFailed.into());
        }
        let stateful = replicas
            .iter()
            .map(|replica| match replica.clone() {
                ServiceReplicaQueryResult::Stateful(s) => s,
                _ => panic!("not stateful"),
            })
            .collect::<Vec<_>>();

        let primary = stateful
            .iter()
            .find(|x| x.replica_role == ReplicaRole::Primary)
            .expect("no primary found")
            .clone();

        let secondary = stateful
            .iter()
            .filter(|x| x.replica_role != ReplicaRole::Primary)
            .collect::<Vec<_>>();
        assert_eq!(secondary.len(), 2);
        Ok((primary, secondary[0].clone(), secondary[1].clone()))
    }

    // Resolve the service. The first return param is the primary.
    async fn resolve(
        &self,
    ) -> mssf_core::Result<(
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
    )> {
        let resolved_partition = self.resolve_with_prev(None).await?;
        self.convert_resolve_results(resolved_partition)
    }

    // converts the resolved partition to 3 endpoints and the first one is primary
    fn convert_resolve_results(
        &self,
        partition: ResolvedServicePartition,
    ) -> mssf_core::Result<(
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
    )> {
        let info = partition.get_info();
        assert_eq!(info.partition_key_type, PartitionKeyType::None);
        assert_eq!(info.service_name, self.service_uri);
        assert_eq!(info.service_partition_kind, ServicePartitionKind::Singleton);
        let endpoints = partition.get_endpoint_list().iter().collect::<Vec<_>>();
        if endpoints.len() < 3 {
            // not available yet.
            return Err(FabricErrorCode::OperationFailed.into());
        }
        let primary = endpoints
            .iter()
            .find(|r| r.role == ServiceEndpointRole::StatefulPrimary);
        if primary.is_none() {
            // primary not available yet.
            return Err(FabricErrorCode::OperationFailed.into());
        }
        let secondary = endpoints
            .iter()
            .filter(|r| r.role != ServiceEndpointRole::StatefulPrimary)
            .collect::<Vec<_>>();
        assert_eq!(secondary.len(), 2);
        Ok((
            primary.unwrap().clone(),
            secondary[0].clone(),
            secondary[1].clone(),
        ))
    }

    // helper to call resolve for this svc
    async fn resolve_with_prev(
        &self,
        prev: Option<&ResolvedServicePartition>,
    ) -> windows_core::Result<ResolvedServicePartition> {
        let mgmt = self.fc.get_service_manager();
        mgmt.resolve_service_partition(
            &self.service_uri,
            &PartitionKeyType::None,
            prev,
            self.timeout,
            None,
        )
        .await
    }

    async fn restart_primary_wait_for_replica_id_change(&self, partition_id: GUID) {
        // test get replica info
        let (p, _, _) = self.get_replicas(partition_id).await.unwrap();
        assert_eq!(p.replica_status, QueryServiceReplicaStatus::Ready);
        assert_ne!(p.node_name, HSTRING::new());

        // restart primary
        let desc = RestartReplicaDescription {
            node_name: p.node_name.clone(),
            partition_id: partition_id,
            replica_or_instance_id: p.replica_id,
        };
        let mgmt = self.fc.get_service_manager();
        mgmt.restart_replica(&desc, self.timeout, None)
            .await
            .unwrap();

        // get replica info to see primary has changed
        let mut count = 0;
        loop {
            let res = self.get_replicas(partition_id).await;
            let p2 = match res {
                Ok((p2, _, _)) => p2,
                Err(_) => {
                    // replica not yet ready
                    count += 1;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };
            if p2.node_name != p.node_name {
                assert_ne!(p.replica_id, p2.replica_id);
                println!("replica id updated after {} retries", count);
                break;
            } else {
                // failover is not yet finished.
                if count > 5 {
                    panic!(
                        "replica id not changed after retry. original {}, new {}",
                        p.replica_id, p2.replica_id
                    );
                }
                // replica has not changed yet.
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            count += 1;
        }
    }
}

// Requires app to be deployed on onebox.
// Uses fabric client to perform various actions for this service.
#[tokio::test]
async fn test_partition_info() {
    let fc = FabricClient::new();
    let tc = TestClient::new(fc.clone());
    let timeout = Duration::from_secs(1);

    let (stateful, single) = tc.get_partition().await.unwrap();

    // TODO: not sure why state is unknown.
    // assert_eq!(stateful.health_state, HealthState::Ok);
    assert_eq!(stateful.partition_status, ServicePartitionStatus::Ready);
    assert_eq!(stateful.target_replica_set_size, 3);
    assert_eq!(stateful.min_replica_set_size, 1);
    assert_ne!(single.id, GUID::zeroed());

    // test get replica info
    let (p, _, _) = tc.get_replicas(single.id).await.unwrap();
    assert_eq!(p.replica_status, QueryServiceReplicaStatus::Ready);
    assert_ne!(p.node_name, HSTRING::new());

    let mgmt = fc.get_service_manager();
    // register service notification filter
    let filter_handle = {
        let desc = ServiceNotificationFilterDescription {
            name: HSTRING::from(SVC_URI),
            flags: ServiceNotificationFilterFlags::NamePrefix,
        };
        // register takes more than 1 sec.
        mgmt.register_service_notification_filter(&desc, Duration::from_secs(10), None)
            .await
            .unwrap()
    };

    // resolve the service
    let (p_endpoint, _, _) = tc.resolve().await.unwrap();

    // restart primary
    tc.restart_primary_wait_for_replica_id_change(single.id)
        .await;

    // resolve again the primary addr should change
    {
        let mut count = 0;
        loop {
            let res = tc.resolve().await;
            let p2_endpoint = match res {
                Ok((p2, _, _)) => p2,
                Err(_) => {
                    // not yet ready
                    count += 1;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            if p2_endpoint.address != p_endpoint.address {
                println!("addr updated after {} retries", count);
                break;
            } else {
                // addr update might be slow.
                // This typically takes 8 seconds which includes service boot time.
                if count > 30 {
                    panic!("addr for primary is not changed {}", p2_endpoint.address);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            count += 1;
        }
    }
    // unregisters the notification
    mgmt.unregister_service_notification_filter(filter_handle, timeout, None)
        .await
        .unwrap();

    // resolve new addr
    let (p_endpoint, _, _) = tc.resolve().await.unwrap();
    // restart primary again and use brute force resolve.
    // This gives a comparison of perf of the notification based resolve and the brute force way.
    tc.restart_primary_wait_for_replica_id_change(single.id)
        .await;
    {
        let mut count = 0;
        let mut prev: Option<ResolvedServicePartition> = None;
        loop {
            let res = tc.resolve_with_prev(prev.as_ref()).await;
            let p2_endpoint_res = match res {
                Ok(p_res) => {
                    // save the prev result
                    prev = Some(p_res.clone());
                    tc.convert_resolve_results(p_res).map(|(p, _, _)| p)
                }
                Err(e) => Err(e),
            };

            match p2_endpoint_res {
                Ok(p2_endpoint) => {
                    if p2_endpoint.address != p_endpoint.address {
                        println!("addr updated after {} retries", count);
                        break;
                    } else {
                        // addr update might be slow.
                        // This typically takes 8 seconds which includes service boot time.
                        if count > 30 {
                            panic!("addr for primary is not changed {}", p2_endpoint.address);
                        }
                    }
                }
                Err(_) => {
                    // retry in next loop
                    if count > 30 {
                        panic!("retry max limit reached");
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    }
}
