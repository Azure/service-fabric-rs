// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    ErrorCode, GUID, WString,
    client::{
        FabricClient,
        svc_mgmt_client::{
            PartitionKeyType, ResolvedServiceEndpoint, ResolvedServicePartition,
            ServiceEndpointRole, ServicePartitionKind,
        },
    },
    types::{
        DeployedServiceReplicaDetailQueryDescription, DeployedServiceReplicaDetailQueryResult,
        DeployedServiceReplicaDetailQueryResultValue, GetPartitionLoadInformationResult,
        NamedPartitionSchemeDescription, NamedRepartitionDescription,
        PartitionLoadInformationQueryDescription, QueryServiceReplicaStatus, ReplicaRole,
        RestartReplicaDescription, ServiceDescription, ServiceNotificationFilterDescription,
        ServiceNotificationFilterFlags, ServicePartitionAccessStatus, ServicePartitionInformation,
        ServicePartitionQueryDescription, ServicePartitionQueryResultItem, ServicePartitionStatus,
        ServiceReplicaQueryDescription, ServiceReplicaQueryResultItem, ServiceUpdateDescription,
        SingletonPartitionInformation, StatefulServiceDescription,
        StatefulServicePartitionQueryResult, StatefulServiceReplicaQueryResult,
        StatefulServiceUpdateDescription, Uri,
    },
};

static SVC_URI: &str = "fabric:/StatefulEchoApp/StatefulEchoAppService";

/// Test client for the stateful service
pub struct TestClient {
    fc: FabricClient,
    service_uri: Uri,
    timeout: Duration,
}

impl TestClient {
    fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            service_uri: Uri::from(SVC_URI),
            timeout: Duration::from_secs(1),
        }
    }

    async fn get_partition(
        &self,
    ) -> mssf_core::Result<(
        StatefulServicePartitionQueryResult,
        SingletonPartitionInformation,
    )> {
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
        let p = list.service_partitions.first().unwrap().clone();
        let stateful = match p {
            ServicePartitionQueryResultItem::Stateful(s) => s,
            _ => panic!("not stateless"),
        };
        let info = stateful.clone().partition_information;
        let single = match info {
            ServicePartitionInformation::Singleton(s) => s,
            _ => panic!("not singleton"),
        };
        Ok((stateful, single))
    }

    async fn get_partition_loads(
        &self,
        partition_id: GUID,
    ) -> mssf_core::Result<GetPartitionLoadInformationResult> {
        let qc = self.fc.get_query_manager();
        let desc = PartitionLoadInformationQueryDescription { partition_id };
        let partition_load_info = qc
            .get_partition_load_information(&desc, self.timeout, None)
            .await?;

        Ok(partition_load_info)
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
            .service_replicas;
        if replicas.len() < 3 {
            // replica are not ready.
            return Err(ErrorCode::E_FAIL.into());
        }
        let stateful = replicas
            .iter()
            .map(|replica| match replica.clone() {
                ServiceReplicaQueryResultItem::Stateful(s) => s,
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

    async fn get_deployed_replica_detail(
        &self,
        node_name: &WString,
        partition_id: GUID,
        replica_id: i64,
    ) -> mssf_core::Result<DeployedServiceReplicaDetailQueryResult> {
        let qc = self.fc.get_query_manager();
        let desc = DeployedServiceReplicaDetailQueryDescription {
            node_name: node_name.clone(),
            partition_id,
            replica_id,
        };

        qc.get_deployed_replica_detail(&desc, self.timeout, None)
            .await
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
        assert_eq!(partition.partition_key_type, PartitionKeyType::None);
        assert_eq!(partition.service_name, self.service_uri);
        assert_eq!(
            partition.service_partition_kind,
            ServicePartitionKind::Singleton
        );
        let endpoints = partition.endpoints;
        if endpoints.len() < 3 {
            // not available yet.
            return Err(ErrorCode::E_FAIL.into());
        }
        let primary = endpoints
            .iter()
            .find(|r| r.role == ServiceEndpointRole::StatefulPrimary);
        if primary.is_none() {
            // primary not available yet.
            return Err(ErrorCode::E_FAIL.into());
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
    ) -> mssf_core::Result<ResolvedServicePartition> {
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
        assert_ne!(p.node_name, WString::new());

        // restart primary
        let desc = RestartReplicaDescription {
            node_name: p.node_name.clone(),
            partition_id,
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
                println!("replica id updated after {count} retries");
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
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let tc = TestClient::new(fc.clone());
    let timeout = Duration::from_secs(1);

    let (stateful, single) = tc.get_partition().await.unwrap();

    // TODO: not sure why state is unknown.
    // assert_eq!(stateful.health_state, HealthState::Ok);
    assert_eq!(stateful.partition_status, ServicePartitionStatus::Ready);
    assert_eq!(stateful.target_replica_set_size, 3);
    assert_eq!(stateful.min_replica_set_size, 2);
    assert_eq!(stateful.auxiliary_replica_count, 1);
    assert_ne!(single.id, GUID::zeroed());

    // test get replica info
    let (p, _, _) = tc.get_replicas(single.id).await.unwrap();
    assert_eq!(p.replica_status, QueryServiceReplicaStatus::Ready);
    assert_ne!(p.node_name, WString::new());

    let mgmt = fc.get_service_manager();
    // register service notification filter
    let filter_handle = {
        let desc = ServiceNotificationFilterDescription {
            name: Uri::from(SVC_URI),
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
                println!("addr updated after {count} retries");
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
                        println!("addr updated after {count} retries");
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

    // Test get partition load
    let partition_load_info = tc.get_partition_loads(single.id).await.unwrap();
    let primary_loads = partition_load_info
        .primary_load_metric_reports
        .iter()
        .collect::<Vec<_>>();
    println!("Primary metric loads: {primary_loads:?}");
    let secondary_loads = partition_load_info
        .secondary_load_metric_reports
        .iter()
        .collect::<Vec<_>>();
    println!("Secondary metric loads: {secondary_loads:?}");

    // test get deployed service info after failover
    let (p, s1, _) = tc.get_replicas(single.id).await.unwrap();
    let deployed_replica_detail = tc
        .get_deployed_replica_detail(&p.node_name, single.id, p.replica_id)
        .await
        .expect("get deployed replica detail failed");
    let result = match deployed_replica_detail.value {
        DeployedServiceReplicaDetailQueryResultValue::Stateful(s) => s,
        _ => panic!("not stateful"),
    };
    // check write ad read status of primary is GRANTED
    assert_eq!(result.write_status, ServicePartitionAccessStatus::Granted);
    assert_eq!(result.read_status, ServicePartitionAccessStatus::Granted);

    // check write and read status of secondary is NOT_PRIMARY
    let deployed_replica_detail = tc
        .get_deployed_replica_detail(&s1.node_name, single.id, s1.replica_id)
        .await
        .expect("get deployed replica detail failed");
    let result = match deployed_replica_detail.value {
        DeployedServiceReplicaDetailQueryResultValue::Stateful(s) => s,
        _ => panic!("not stateful"),
    };
    assert_eq!(
        result.write_status,
        ServicePartitionAccessStatus::NotPrimary
    );
    assert_eq!(result.read_status, ServicePartitionAccessStatus::NotPrimary);
}

pub struct TestCreateUpdateClient {
    fc: FabricClient,
    pub(crate) timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum TestPartitionReplicaLayout {
    Target1Min1,                 // target 1 replica, min 1 replica, no aux replica
    TargetMinAux(i32, i32, i32), // target, min and aux replica count
}

impl TestPartitionReplicaLayout {
    pub fn tuple(&self) -> (i32, i32, i32) {
        match self {
            TestPartitionReplicaLayout::Target1Min1 => (1, 1, 0),
            TestPartitionReplicaLayout::TargetMinAux(target, min, aux) => (*target, *min, *aux),
        }
    }
}

impl TestCreateUpdateClient {
    pub(crate) fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            timeout: Duration::from_secs(30),
        }
    }

    pub(crate) async fn create_service(
        &self,
        service_name: &Uri,
        partition_scheme: &mssf_core::types::PartitionSchemeDescription,
        layout: TestPartitionReplicaLayout,
    ) {
        let (target, min, aux) = layout.tuple();
        // TODO: get service first
        let desc = ServiceDescription::Stateful(
            StatefulServiceDescription::new(
                Uri::from("fabric:/StatefulEchoApp"),
                service_name.clone(),
                WString::from("StatefulEchoAppService"),
                partition_scheme.clone(),
            )
            .with_has_persistent_state(true)
            .with_service_activation_mode(
                mssf_core::types::ServicePackageActivationMode::SharedProcess,
            )
            .with_min_replica_set_size(min)
            .with_target_replica_set_size(target)
            .with_auxiliary_replica_count(aux),
        );
        // Run client operation on separate task to ensure that the api is task safe.
        println!("creating service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        tokio::spawn(async move { sm.create_service(&desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("create failed");
    }

    pub(crate) async fn delete_service(&self, service_name: &Uri) {
        println!("deleting service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&service_name, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("delete failed");
    }

    async fn resolve_service(
        &self,
        service_name: &Uri,
        key_type: PartitionKeyType,
    ) -> Vec<ResolvedServiceEndpoint> {
        let smgr = self.fc.get_service_manager();
        // resolve until the service is ready
        let mut count = 0;
        loop {
            let res = smgr
                .resolve_service_partition(service_name, &key_type, None, self.timeout, None)
                .await;
            match res {
                Ok(info) => {
                    return info.endpoints;
                }
                Err(e) => {
                    if count > 30 {
                        panic!("service not ready, {e}");
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    }

    pub(crate) async fn update_service_named_partitions(
        &self,
        service_name: &Uri,
        names_to_add: Vec<WString>,
        names_to_remove: Vec<WString>,
    ) {
        let desc = ServiceUpdateDescription::Stateful(
            StatefulServiceUpdateDescription::new().with_repartition_description(
                mssf_core::types::ServiceRepartitionDescription::Named(
                    NamedRepartitionDescription {
                        names_to_add,
                        names_to_remove,
                    },
                ),
            ),
        );
        println!("updating service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.update_service(&service_name, &desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("update failed");
    }

    pub(crate) async fn update_service_replica_layout(
        &self,
        service_name: &Uri,
        layout: TestPartitionReplicaLayout,
    ) {
        let (target, min, aux) = layout.tuple();
        let desc = ServiceUpdateDescription::Stateful(
            StatefulServiceUpdateDescription::new()
                .with_target_replica_set_size(target)
                .with_min_replica_set_size(min)
                .with_auxiliary_replica_count(aux),
        );
        println!("updating service replica layout {service_name:?} - {layout:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.update_service(&service_name, &desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("update failed");
    }
}

async fn test_service_create_delete(
    fc: &FabricClient,
    partition_scheme: &mssf_core::types::PartitionSchemeDescription,
    service_name: &Uri,
) {
    // TODO: get service first
    let tc = TestCreateUpdateClient::new(fc.clone());
    // create service
    tc.create_service(
        service_name,
        partition_scheme,
        TestPartitionReplicaLayout::Target1Min1,
    )
    .await;

    let key_type = match partition_scheme {
        mssf_core::types::PartitionSchemeDescription::Singleton => PartitionKeyType::None,
        mssf_core::types::PartitionSchemeDescription::Named(names) => {
            PartitionKeyType::String(names.get_ref().first().unwrap().clone())
        }
        mssf_core::types::PartitionSchemeDescription::Int64Range(data) => {
            PartitionKeyType::Int64(data.as_raw().LowKey)
        }
        mssf_core::types::PartitionSchemeDescription::Invalid => panic!("invalid partition scheme"),
    };

    // resolve until the service is ready
    let addrs = tc.resolve_service(service_name, key_type).await;
    println!("resolved service {addrs:?}");

    // delete service
    tc.delete_service(service_name).await;
}

#[tokio::test]
async fn test_service_curd_singleton() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Singleton;
    let service_name = Uri::from("fabric:/StatefulEchoApp/CurdTestServiceSingleton");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
async fn test_service_curd_named() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Named(
        NamedPartitionSchemeDescription::new(vec![WString::from("test")]),
    );
    let service_name = Uri::from("fabric:/StatefulEchoApp/CurdTestServiceNamed");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
async fn test_service_curd_range() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Int64Range(
        mssf_core::types::UniformIn64PartitionSchemeDescription::new(1, 10, 100),
    );
    let service_name = Uri::from("fabric:/StatefulEchoApp/CurdTestServiceRange");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
async fn test_service_reparition() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let tc = TestCreateUpdateClient::new(fc.clone());
    // Note: SF does not support 0 named partitions.
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Named(
        NamedPartitionSchemeDescription::new(vec![WString::from("1")]),
    );
    let service_name = Uri::from("fabric:/StatefulEchoApp/RepartitionTest");

    // create service
    tc.create_service(
        &service_name,
        &partition_scheme,
        TestPartitionReplicaLayout::Target1Min1,
    )
    .await;

    // resolve until the service is ready
    {
        let key_type = PartitionKeyType::String(WString::from("1"));
        let addrs = tc.resolve_service(&service_name, key_type).await;
        println!("resolved service {addrs:?}");
    }

    // Add a partition 2
    {
        println!("adding partition 2: {service_name:?}");
        let names_to_add = vec![WString::from("2")];
        let names_to_remove = vec![];
        tc.update_service_named_partitions(&service_name, names_to_add, names_to_remove)
            .await;
    }

    {
        let key_type = PartitionKeyType::String(WString::from("2"));
        let addrs = tc.resolve_service(&service_name, key_type).await;
        println!("resolved service partition 2 {addrs:?}");
    }

    // remove parition 1
    {
        println!("removing partition 1: {service_name:?}");
        let names_to_add = vec![];
        let names_to_remove = vec![WString::from("1")];
        tc.update_service_named_partitions(&service_name, names_to_add, names_to_remove)
            .await;
    }

    // delete service
    tc.delete_service(&service_name).await;
}
