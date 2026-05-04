// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Test helpers for the reflection sample's integration tests.
//!
//! Lives in the library crate (alongside [`test_cluster`])
//! rather than under `tests/common/` so each integration-test
//! file can simply `use samples_reflection::test_admin::*;`
//! without per-file `mod common;` re-declaration.
//!
//! Two top-level facilities:
//!
//! - [`TestClient`] — read-only queries (partition / replica /
//!   resolve / restart-wait) against the deployed
//!   `ReflectionAppService`.
//! - [`TestCreateUpdateClient`] — service create / update /
//!   delete operations plus partition convergence helpers,
//!   parameterized by [`TestPartitionReplicaLayout`].

#![allow(dead_code)] // some helpers are used only by a subset of test files

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
        GetPartitionLoadInformationResult, NamedPartitionInfomation, NamedRepartitionDescription,
        PartitionLoadInformationQueryDescription, QueryServiceReplicaStatus, ReplicaRole,
        RestartReplicaDescription, ServiceDescription, ServicePartitionInformation,
        ServicePartitionQueryDescription, ServicePartitionQueryResultItem, ServicePartitionStatus,
        ServiceReplicaQueryDescription, ServiceReplicaQueryResultItem, ServiceUpdateDescription,
        SingletonPartitionInformation, StatefulServiceDescription,
        StatefulServicePartitionQueryResult, StatefulServiceReplicaQueryResult,
        StatefulServiceUpdateDescription, Uri,
    },
};

/// Default service URI used by the partition-admin tests.
pub const SVC_URI: &str = "fabric:/ReflectionApp/ReflectionAppService";

// ---------------------------------------------------------------
// TestClient — read-only queries against the deployed service.
// ---------------------------------------------------------------

/// Test client for the stateful service. Wraps a [`FabricClient`]
/// with conveniences for partition queries, replica queries,
/// resolve, and restart-wait helpers.
pub struct TestClient {
    fc: FabricClient,
    service_uri: Uri,
    timeout: Duration,
}

impl TestClient {
    pub fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            service_uri: Uri::from(SVC_URI),
            timeout: Duration::from_secs(1),
        }
    }

    pub async fn get_partition(
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

    pub async fn get_partition_loads(
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

    /// Primary replica is returned first.
    pub async fn get_replicas(
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

    pub async fn get_deployed_replica_detail(
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

    /// Resolve the service. The first return param is the primary.
    pub async fn resolve(
        &self,
    ) -> mssf_core::Result<(
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
        ResolvedServiceEndpoint,
    )> {
        let resolved_partition = self.resolve_with_prev(None).await?;
        self.convert_resolve_results(resolved_partition)
    }

    /// Convert the resolved partition to 3 endpoints; the first is the primary.
    pub fn convert_resolve_results(
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

    /// Helper to call resolve for this svc.
    pub async fn resolve_with_prev(
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

    pub async fn restart_primary_wait_for_replica_id_change(&self, partition_id: GUID) {
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
                tracing::info!("replica id updated after {count} retries");
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

// ---------------------------------------------------------------
// TestCreateUpdateClient — service create / update / delete.
// ---------------------------------------------------------------

pub struct TestCreateUpdateClient {
    fc: FabricClient,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum TestPartitionReplicaLayout {
    /// target 1 replica, min 1 replica, no aux replica.
    Target1Min1,
    /// target, min and aux replica count.
    TargetMinAux(i32, i32, i32),
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
    pub fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            timeout: Duration::from_secs(30),
        }
    }

    pub async fn create_service(
        &self,
        service_name: &Uri,
        partition_scheme: &mssf_core::types::PartitionSchemeDescription,
        layout: TestPartitionReplicaLayout,
    ) {
        let (target, min, aux) = layout.tuple();
        // TODO: get service first
        let desc = ServiceDescription::Stateful(
            StatefulServiceDescription::new(
                Uri::from("fabric:/ReflectionApp"),
                service_name.clone(),
                WString::from("ReflectionAppService"),
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
        tracing::info!("creating service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        tokio::spawn(async move { sm.create_service(&desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("create failed");
    }

    pub async fn delete_service(&self, service_name: &Uri) {
        tracing::info!("deleting service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.delete_service(&service_name, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("delete failed");
    }

    pub async fn resolve_service(
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

    /// Returns the ready stateful partitions.
    pub async fn query_service_partition(
        &self,
        service_name: &Uri,
    ) -> Vec<StatefulServicePartitionQueryResult> {
        let qc = self.fc.get_query_manager();
        let desc = ServicePartitionQueryDescription {
            service_name: service_name.clone(),
            partition_id_filter: None,
        };
        let list = match qc.get_partition_list(&desc, self.timeout, None).await {
            Ok(l) => l,
            Err(e) => {
                // There is a known SF issue that this returns FABRIC_E_PARTITION_NOT_FOUND
                // after 2 repartition requests running concurrently.
                tracing::error!("SF BUG: get_partition_list failed: {e}");
                return vec![];
            }
        };

        list.service_partitions
            .iter()
            .filter_map(|p| match p {
                ServicePartitionQueryResultItem::Stateful(s) => {
                    if s.partition_status == ServicePartitionStatus::Ready {
                        Some(s.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    /// Wait until the queried named partitions contain all the expected names.
    /// During convergence, extra names are allowed (e.g. partitions being removed
    /// may still be present). The check succeeds once every expected name is present
    /// and no unexpected names remain.
    /// Panics after `max_retry` attempts.
    pub async fn wait_for_named_partitions(
        &self,
        service_name: &Uri,
        expected_names: &[&str],
        max_retry: u32,
    ) {
        let expected: Vec<WString> = expected_names.iter().map(|n| WString::from(*n)).collect();
        for retry in 0..max_retry {
            let ready = self.query_service_partition(service_name).await;
            let names: Vec<WString> = ready
                .iter()
                .map(|p| match &p.partition_information {
                    ServicePartitionInformation::Named(NamedPartitionInfomation {
                        name, ..
                    }) => name.clone(),
                    other => panic!("expected named partition, got {:?}", other),
                })
                .collect();
            // All expected names must be present.
            let all_expected_present = expected.iter().all(|n| names.contains(n));
            // No extra names beyond what we expect.
            let no_extra = names.iter().all(|n| expected.contains(n));
            if all_expected_present && no_extra {
                tracing::info!("partitions matched {expected:?} after {retry} retries");
                return;
            }
            if all_expected_present {
                // Expected names present but extra partitions still exist; keep waiting.
                tracing::info!(
                    "retry {retry}: expected {expected:?}, got {names:?} (extra partitions still present)"
                );
            } else {
                tracing::info!("retry {retry}: expected {expected:?}, got {names:?}");
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        panic!("partitions did not converge to {expected:?} after {max_retry} retries");
    }

    pub async fn update_service_named_partitions(
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
        tracing::info!("updating service {service_name:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.update_service(&service_name, &desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("update failed");
    }

    pub async fn update_service_replica_layout(
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
        tracing::info!("updating service replica layout {service_name:?} - {layout:?}");
        let sm = self.fc.get_service_manager().clone();
        let timeout = self.timeout;
        let service_name = service_name.clone();
        tokio::spawn(async move { sm.update_service(&service_name, &desc, timeout, None).await })
            .await
            .expect("task panicked")
            .expect("update failed");
    }
}
