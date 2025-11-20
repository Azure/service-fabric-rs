// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::monitoring::{HealthEntity, NodeHealthEntity, entities::ClusterHealthEntity};
use ::tokio::sync::mpsc;
use mssf_core::{
    client::FabricClient,
    runtime::executor::BoxedCancelToken,
    types::{
        ApplicationHealthStatesFilter, ApplicationQueryDescription, ClusterHealthQueryDescription,
        HealthEventsFilter, HealthStateFilterFlags, NodeHealthQueryDescription,
        NodeHealthStatesFilter, NodeQueryResultItem, Uri,
    },
};
use std::time::Duration;

/// Queries SF and produces health data.
/// User is responsible to implement a consumer to receive the data.
pub struct HealthDataProducer {
    fc: FabricClient,
    interval: Duration,
    sender: mpsc::UnboundedSender<HealthEntity>,
    iteration: std::sync::atomic::AtomicU64,
}

/// Default timeout for FabricClient operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub enum Action {
    Stop,
}

impl HealthDataProducer {
    pub fn new(
        fc: FabricClient,
        interval: Duration,
        sender: mpsc::UnboundedSender<HealthEntity>,
    ) -> Self {
        HealthDataProducer {
            fc,
            interval,
            sender,
            iteration: std::sync::atomic::AtomicU64::new(0),
        }
    }

    fn send_entity(&self, entity: HealthEntity) -> Result<(), Action> {
        self.sender.send(entity).map_err(|_| {
            tracing::error!("Receiver dropped, cannot send more data.");
            Action::Stop
        })
    }

    /// Run once to produce health data.
    pub(crate) async fn run_once(&self, token: BoxedCancelToken) -> Result<(), Action> {
        // Get cluster health information.
        if let Some(entity) = self.produce_cluster_health_entity(token.clone()).await {
            self.send_entity(entity)?;
        }
        // Get node information.
        if let Ok(nodes) = self.get_all_nodes(token.clone()).await {
            for node in nodes {
                if let Some(entity) = self.produce_node_health_entity(token.clone(), node).await {
                    self.send_entity(entity)?;
                }
            }
        }
        // Get application information.
        if let Ok(apps) = self.get_all_applications(token.clone()).await {
            for app in apps {
                let app_name = app.application_name.clone();
                if let Some(entity) = self
                    .produce_application_health_entity(token.clone(), app)
                    .await
                {
                    self.send_entity(entity)?;
                }

                // Get service information for the application.
                if let Ok(services) = self.get_all_services_for_app(token.clone(), app_name.clone()).await {
                    for svc in services {
                        let svc_name = svc.get_service_name().clone();
                        // produce service health entity
                        if let Some(entity) =
                            self.produce_service_health_entity(token.clone(), svc).await
                        {
                            self.send_entity(entity)?;
                        }

                        // Get partition information for the service.
                        if let Ok(partitions) = self
                            .get_all_partitions_for_svc(token.clone(), svc_name.clone())
                            .await
                        {
                            for partition in partitions {
                                let partition_id = partition.get_partition_id();
                                // produce partition health entity
                                if let Some(entity) = self
                                    .produce_partition_health_entity(
                                        token.clone(),
                                        partition,
                                        svc_name.clone(),
                                        app_name.clone(),
                                    )
                                    .await
                                {
                                    self.send_entity(entity)?;
                                }
                                // Get replica information for the partition.
                                if let Ok(replicas) = self
                                    .get_all_replicas_for_partition(token.clone(), partition_id)
                                    .await
                                {
                                    for replica in replicas {
                                        // produce replica health entity
                                        if let Some(entity) = self
                                            .produce_replica_health_entity(
                                                token.clone(),
                                                partition_id,
                                                replica,
                                                svc_name.clone(),
                                                app_name.clone(),
                                            )
                                            .await
                                        {
                                            self.send_entity(entity)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.iteration
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    /// Run a loop to produce health data.
    pub async fn run_loop(&self, token: BoxedCancelToken) {
        loop {
            let start_time = ::tokio::time::Instant::now();
            match self.run_once(token.clone()).await {
                Err(Action::Stop) => {
                    tracing::info!("Health data producer stopped.");
                    break;
                }
                _ => {
                    // continue the loop
                }
            }

            // remaining time
            let elapsed = start_time.elapsed();
            // wait for more time if necessary.
            if elapsed < self.interval {
                let wait_duration = self.interval - elapsed;

                tokio::select! {
                    _ = token.wait() => {
                        tracing::info!("Cancellation requested, exiting health data producer loop.");
                        break;
                    }
                    _ = tokio::time::sleep(wait_duration) => {}
                }
            }

            if token.is_cancelled() {
                tracing::info!("Cancellation requested, exiting health data producer loop.");
                break;
            }
        }
        tracing::info!("Health data producer loop exited.");
    }

    pub fn get_iteration(&self) -> u64 {
        self.iteration.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl HealthDataProducer {
    async fn produce_cluster_health_entity(&self, token: BoxedCancelToken) -> Option<HealthEntity> {
        // Ignore nodes and app health because we retrieve them separately.
        // Technically we can get everything in one call, but the payload might be too large,
        // and we want to get other entities not present in the cluster health.
        // For example, each system service health is not present in this result.
        let desc = ClusterHealthQueryDescription {
            nodes_filter: Some(NodeHealthStatesFilter {
                health_state_filter: HealthStateFilterFlags::NONE,
            }),
            applications_filter: Some(ApplicationHealthStatesFilter {
                health_state_filter: HealthStateFilterFlags::NONE,
            }),
            ..Default::default()
        };
        let cluster_healths = self
            .fc
            .get_health_manager()
            .get_cluster_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get cluster health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Cluster(ClusterHealthEntity {
            health: cluster_healths,
        }))
    }

    /// Produce the health entity for a node.
    async fn produce_node_health_entity(
        &self,
        token: BoxedCancelToken,
        node: NodeQueryResultItem,
    ) -> Option<HealthEntity> {
        // Logic to get node health goes here.

        let desc = NodeHealthQueryDescription {
            node_name: node.name.clone(),
            // We only care about the aggregated health state.
            events_filter: Some(HealthEventsFilter {
                health_state_filter: HealthStateFilterFlags::NONE,
            }),
            ..Default::default()
        };
        let node_healths = self
            .fc
            .get_health_manager()
            .get_node_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get node health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Node(NodeHealthEntity {
            node,
            health: node_healths,
        }))
    }

    async fn produce_application_health_entity(
        &self,
        token: BoxedCancelToken,
        app: mssf_core::types::ApplicationQueryResultItem,
    ) -> Option<HealthEntity> {
        let desc = mssf_core::types::ApplicationHealthQueryDescription {
            application_name: app.application_name.clone(),
            ..Default::default()
        };
        let app_health = self
            .fc
            .get_health_manager()
            .get_application_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get application health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Application(
            crate::monitoring::entities::ApplicationHealthEntity {
                application: app,
                health: app_health,
            },
        ))
    }

    async fn produce_service_health_entity(
        &self,
        token: BoxedCancelToken,
        svc: mssf_core::types::ServiceQueryResultItem,
    ) -> Option<HealthEntity> {
        let svc_name = svc.get_service_name().clone();
        let desc = mssf_core::types::ServiceHealthQueryDescription {
            service_name: svc_name,
            ..Default::default()
        };
        let svc_health = self
            .fc
            .get_health_manager()
            .get_service_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get service health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Service(
            crate::monitoring::entities::ServiceHealthEntity {
                health: svc_health,
                service: svc,
            },
        ))
    }
    
    async fn produce_partition_health_entity(
        &self,
        token: BoxedCancelToken,
        part: mssf_core::types::ServicePartitionQueryResultItem,
        service_name: Uri,
        application_name: Uri,
    ) -> Option<HealthEntity> {
        let partition_id = part.get_partition_id();
        let desc = mssf_core::types::PartitionHealthQueryDescription {
            partition_id,
            ..Default::default()
        };
        let part_health = self
            .fc
            .get_health_manager()
            .get_partition_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get partition health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Partition(
            crate::monitoring::entities::PartitionHealthEntity {
                health: part_health,
                partition: part,
                service_name: service_name.to_string(),
                application_name: application_name.to_string(),
            },
        ))
    }
    
    async fn produce_replica_health_entity(
        &self,
        token: BoxedCancelToken,
        partition_id: mssf_core::GUID,
        replica: mssf_core::types::ServiceReplicaQueryResultItem,
        service_name: Uri,
        application_name: Uri,
    ) -> Option<HealthEntity> {
        let desc = mssf_core::types::ReplicaHealthQueryDescription {
            partition_id,
            replica_id_or_instance_id: replica.get_replica_or_instance_id(),
            ..Default::default()
        };
        let replica_health = self
            .fc
            .get_health_manager()
            .get_replica_health(&desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get replica health: {}", err);
            })
            .ok()?;
        Some(HealthEntity::Replica(
            crate::monitoring::entities::ReplicaHealthEntity {
                health: replica_health,
                replica,
                service_name: service_name.to_string(),
                application_name: application_name.to_string(),
            },
        ))
    }
}

// Get lists of entities
impl HealthDataProducer {
    async fn get_all_nodes(
        &self,
        token: BoxedCancelToken,
    ) -> mssf_core::Result<Vec<NodeQueryResultItem>> {
        // Logic to get node information goes here.
        let desc = &Default::default();
        let nodes = self
            .fc
            .get_query_manager()
            .get_node_list(desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get node list: {}", err);
            })?
            .nodes;
        Ok(nodes)
    }

    /// This does not include system application.
    /// We will report system service health separately.
    async fn get_all_applications(
        &self,
        token: BoxedCancelToken,
    ) -> mssf_core::Result<Vec<mssf_core::types::ApplicationQueryResultItem>> {
        let desc = ApplicationQueryDescription::default();
        let apps = self
            .fc
            .get_query_manager()
            .get_application_list(&desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get application list: {}", err);
            })?
            .items;
        Ok(apps)
    }
    async fn get_all_services_for_app(
        &self,
        token: BoxedCancelToken,
        app_name: Uri,
    ) -> mssf_core::Result<Vec<mssf_core::types::ServiceQueryResultItem>> {
        let app_name_cp = app_name.clone();
        // Logic to get service information goes here.
        let desc = mssf_core::types::ServiceQueryDescription {
            application_name: app_name_cp,
            ..Default::default()
        };
        let services = self
            .fc
            .get_query_manager()
            .get_service_list(&desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get service list for app {app_name}: {err}");
            })?;
        Ok(services.items)
    }

    async fn get_all_partitions_for_svc(
        &self,
        token: BoxedCancelToken,
        service_name: Uri,
    ) -> mssf_core::Result<Vec<mssf_core::types::ServicePartitionQueryResultItem>> {
        // Logic to get partition information goes here.
        let desc = mssf_core::types::ServicePartitionQueryDescription {
            service_name,
            partition_id_filter: None,
        };
        let partitions = self
            .fc
            .get_query_manager()
            .get_partition_list(&desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get partition list: {}", err);
            })?
            .service_partitions;
        Ok(partitions)
    }

    async fn get_all_replicas_for_partition(
        &self,
        token: BoxedCancelToken,
        partition_id: mssf_core::GUID,
    ) -> mssf_core::Result<Vec<mssf_core::types::ServiceReplicaQueryResultItem>> {
        // Logic to get replica information goes here.
        let desc = mssf_core::types::ServiceReplicaQueryDescription {
            partition_id,
            ..Default::default()
        };
        let replicas = self
            .fc
            .get_query_manager()
            .get_replica_list(&desc, DEFAULT_TIMEOUT, Some(token.clone()))
            .await
            .inspect_err(|err| {
                tracing::error!("Failed to get replica list: {}", err);
            })?
            .service_replicas;
        Ok(replicas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mssf_core::WString;

    /// Test fixture containing common test objects
    struct TestFixture {
        producer: HealthDataProducer,
        _receiver: mpsc::UnboundedReceiver<HealthEntity>,
        token: BoxedCancelToken,
    }

    impl TestFixture {
        fn new() -> Self {
            let fc = FabricClient::builder()
                .with_connection_strings(vec![WString::from("localhost:19000")])
                .build()
                .expect("Failed to create FabricClient");
            
            let (sender, receiver) = mpsc::unbounded_channel();
            let producer = HealthDataProducer::new(fc, Duration::from_secs(1), sender);
            let token = mssf_core::sync::SimpleCancelToken::new_boxed();
            
            Self {
                producer,
                _receiver: receiver,
                token,
            }
        }

        /// Get first available app, service, partition, and replica from cluster
        async fn get_test_entities(&self) -> Option<TestEntities> {
            let apps = self.producer.get_all_applications(self.token.clone()).await.ok()?;
            if apps.is_empty() {
                tracing::warn!("No applications in cluster");
                return None;
            }

            let app = &apps[0];
            let app_name = app.application_name.clone();
            
            let services = self.producer.get_all_services_for_app(self.token.clone(), app_name.clone()).await.ok()?;
            if services.is_empty() {
                tracing::warn!("No services in application");
                return None;
            }

            let service = &services[0];
            let service_name = service.get_service_name().clone();

            let partitions = self.producer.get_all_partitions_for_svc(self.token.clone(), service_name.clone()).await.ok()?;
            if partitions.is_empty() {
                tracing::warn!("No partitions in service");
                return None;
            }

            let partition = partitions[0].clone();
            let partition_id = partition.get_partition_id();
            
            let replicas = self.producer.get_all_replicas_for_partition(self.token.clone(), partition_id).await.ok()?;
            let replica = replicas.first().cloned();

            Some(TestEntities {
                app_name,
                service_name,
                partition,
                partition_id,
                replica,
            })
        }
    }

    /// Container for test entities from the cluster
    struct TestEntities {
        app_name: Uri,
        service_name: Uri,
        partition: mssf_core::types::ServicePartitionQueryResultItem,
        partition_id: mssf_core::GUID,
        replica: Option<mssf_core::types::ServiceReplicaQueryResultItem>,
    }

    /// Test produce_partition_health_entity properly populates service_name and application_name
    #[tokio::test]
    async fn test_produce_partition_health_entity_with_context() {
        let _ = tracing_subscriber::fmt().try_init();
        let fixture = TestFixture::new();

        let Some(entities) = fixture.get_test_entities().await else {
            tracing::warn!("Skipping test - no test entities available");
            return;
        };
        
        // Call the method under test
        let entity = fixture.producer.produce_partition_health_entity(
            fixture.token.clone(),
            entities.partition,
            entities.service_name.clone(),
            entities.app_name.clone(),
        ).await;

        assert!(entity.is_some(), "Should produce a partition health entity");
        
        if let Some(HealthEntity::Partition(partition_entity)) = entity {
            // Verify the service_name and application_name are properly set
            assert_eq!(partition_entity.service_name, entities.service_name.to_string());
            assert_eq!(partition_entity.application_name, entities.app_name.to_string());
            assert!(!partition_entity.service_name.is_empty());
            assert!(!partition_entity.application_name.is_empty());
        } else {
            panic!("Expected HealthEntity::Partition");
        }
    }

    /// Test produce_replica_health_entity properly populates service_name and application_name
    #[tokio::test]
    async fn test_produce_replica_health_entity_with_context() {
        let _ = tracing_subscriber::fmt().try_init();
        let fixture = TestFixture::new();

        let Some(entities) = fixture.get_test_entities().await else {
            tracing::warn!("Skipping test - no test entities available");
            return;
        };

        let Some(replica) = entities.replica else {
            tracing::warn!("Skipping test - no replicas in partition");
            return;
        };
        
        // Call the method under test
        let entity = fixture.producer.produce_replica_health_entity(
            fixture.token.clone(),
            entities.partition_id,
            replica,
            entities.service_name.clone(),
            entities.app_name.clone(),
        ).await;

        assert!(entity.is_some(), "Should produce a replica health entity");
        
        if let Some(HealthEntity::Replica(replica_entity)) = entity {
            // Verify the service_name and application_name are properly set
            assert_eq!(replica_entity.service_name, entities.service_name.to_string());
            assert_eq!(replica_entity.application_name, entities.app_name.to_string());
            assert!(!replica_entity.service_name.is_empty());
            assert!(!replica_entity.application_name.is_empty());
        } else {
            panic!("Expected HealthEntity::Replica");
        }
    }

    /// Test that produce methods return None on error (e.g., invalid partition ID)
    #[tokio::test]
    async fn test_produce_partition_health_entity_returns_none_on_error() {
        let _ = tracing_subscriber::fmt().try_init();
        let fixture = TestFixture::new();

        // Create a fake partition with invalid data
        let fake_partition = mssf_core::types::ServicePartitionQueryResultItem::Stateful(
            mssf_core::types::StatefulServicePartitionQueryResult {
                partition_information: mssf_core::types::ServicePartitionInformation::Int64Range(
                    mssf_core::types::Int64PartitionInfomation {
                        id: mssf_core::GUID::zeroed(), // Invalid GUID likely doesn't exist
                        low_key: 0,
                        high_key: 0,
                    }
                ),
                target_replica_set_size: 1,
                min_replica_set_size: 1,
                health_state: mssf_core::types::HealthState::Ok,
                partition_status: mssf_core::types::ServicePartitionStatus::Ready,
                last_quorum_loss_duration_in_seconds: 0,
            }
        );

        let entity = fixture.producer.produce_partition_health_entity(
            fixture.token,
            fake_partition,
            Uri::from("fabric:/FakeApp/FakeService"),
            Uri::from("fabric:/FakeApp"),
        ).await;

        // Should return None because health query will fail
        assert!(entity.is_none(), "Should return None for invalid partition");
    }

    /// Test that produce methods return None on error (e.g., invalid replica ID)
    #[tokio::test]
    async fn test_produce_replica_health_entity_returns_none_on_error() {
        let _ = tracing_subscriber::fmt().try_init();
        let fixture = TestFixture::new();

        // Create a fake replica with invalid data
        let fake_replica = mssf_core::types::ServiceReplicaQueryResultItem::Stateful(
            mssf_core::types::StatefulServiceReplicaQueryResult {
                replica_id: 999999999, // Likely doesn't exist
                replica_role: mssf_core::types::ReplicaRole::Primary,
                replica_status: mssf_core::types::QueryServiceReplicaStatus::Ready,
                aggregated_health_state: mssf_core::types::HealthState::Ok,
                replica_address: WString::from(""),
                node_name: WString::from("FakeNode"),
                last_in_build_duration_in_seconds: 0,
            }
        );

        let entity = fixture.producer.produce_replica_health_entity(
            fixture.token,
            mssf_core::GUID::zeroed(),
            fake_replica,
            Uri::from("fabric:/FakeApp/FakeService"),
            Uri::from("fabric:/FakeApp"),
        ).await;

        // Should return None because health query will fail
        assert!(entity.is_none(), "Should return None for invalid replica");
    }

    /// Test iteration counter functionality
    #[tokio::test]
    async fn test_iteration_counter() {
        let fixture = TestFixture::new();
        
        assert_eq!(fixture.producer.get_iteration(), 0);
        
        // Manually increment to test the atomic counter
        fixture.producer.iteration.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(fixture.producer.get_iteration(), 1);
        
        fixture.producer.iteration.fetch_add(5, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(fixture.producer.get_iteration(), 6);
    }
}


