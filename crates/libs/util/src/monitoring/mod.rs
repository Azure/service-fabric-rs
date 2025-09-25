// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

mod producer;
pub use producer::HealthDataProducer;
mod entities;
pub use entities::{HealthEntity, NodeHealthEntity};

#[cfg(test)]
mod tests {

    use std::{sync::Arc, time::Duration};

    use mssf_core::{WString, client::FabricClient};
    use tokio::sync::mpsc;

    use crate::monitoring::{
        HealthDataProducer, HealthEntity, NodeHealthEntity, entities::ClusterHealthEntity,
    };

    pub struct MockHealthDataConsumer {
        receiver: mpsc::UnboundedReceiver<HealthEntity>,
    }

    pub struct HealthDataCollection {
        pub cluster_health_entity: Vec<ClusterHealthEntity>,
        pub node_health_entities: Vec<NodeHealthEntity>,
        pub application_health_entities: Vec<crate::monitoring::entities::ApplicationHealthEntity>,
        pub partition_health_entities: Vec<crate::monitoring::entities::PartitionHealthEntity>,
        pub service_health_entities: Vec<crate::monitoring::entities::ServiceHealthEntity>,
        pub replica_health_entities: Vec<crate::monitoring::entities::ReplicaHealthEntity>,
    }

    impl MockHealthDataConsumer {
        pub fn new(receiver: mpsc::UnboundedReceiver<HealthEntity>) -> Self {
            MockHealthDataConsumer { receiver }
        }

        /// Producer must close.
        pub async fn get_all_data(&mut self) -> HealthDataCollection {
            let mut data = HealthDataCollection {
                node_health_entities: Vec::new(),
                cluster_health_entity: Vec::new(),
                application_health_entities: Vec::new(),
                partition_health_entities: Vec::new(),
                service_health_entities: Vec::new(),
                replica_health_entities: Vec::new(),
            };
            while let Some(entity) = self.receiver.recv().await {
                match entity {
                    HealthEntity::Node(node_entity) => {
                        data.node_health_entities.push(node_entity);
                    }
                    HealthEntity::Cluster(cluster_entity) => {
                        data.cluster_health_entity.push(cluster_entity);
                    }
                    HealthEntity::Application(application_entity) => {
                        data.application_health_entities.push(application_entity);
                    }
                    HealthEntity::Partition(partition_entity) => {
                        data.partition_health_entities.push(partition_entity);
                    }
                    HealthEntity::Service(service_entity) => {
                        data.service_health_entities.push(service_entity);
                    }
                    HealthEntity::Replica(replica_entity) => {
                        data.replica_health_entities.push(replica_entity);
                    }
                }
            }
            data
        }
    }

    pub fn new_health_data_producer_consumer(
        fc: FabricClient,
    ) -> (HealthDataProducer, MockHealthDataConsumer) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let producer = HealthDataProducer::new(fc, Duration::from_secs(30), sender);
        let consumer = MockHealthDataConsumer::new(receiver);
        (producer, consumer)
    }

    #[tokio::test]
    async fn test_health_data() {
        // set up tracing
        let _ = tracing_subscriber::fmt().try_init();

        let fc = FabricClient::builder()
            .with_connection_strings(vec![WString::from("localhost:19000")])
            .build()
            .unwrap();
        let (producer, mut consumer) = new_health_data_producer_consumer(fc);

        let token = mssf_core::sync::SimpleCancelToken::new_boxed();
        // Simulate producing health data
        let token_clone = token.clone();
        let producer = Arc::new(producer);
        let producer_clone = producer.clone();
        let ph = tokio::spawn(async move {
            producer_clone.run_loop(token_clone).await;
        });

        // Wait at least 1 iteration bit and then stop the producer
        let max_iteration = 10;
        for _ in 0..max_iteration {
            if producer.get_iteration() > 0 {
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        assert_ne!(
            producer.get_iteration(),
            0,
            "Producer did not run any iteration"
        );
        drop(producer); // this is required for consumer to finish.
        token.cancel();
        ph.await.unwrap();

        // Consume the health data
        let data = consumer.get_all_data().await;
        // check cluster health entity
        assert_eq!(
            data.cluster_health_entity.len(),
            1,
            "Should have one cluster health entity"
        );
        let cluster_health = &data.cluster_health_entity[0];
        assert!(
            cluster_health.health.aggregated_health_state == mssf_core::types::HealthState::Ok
                || cluster_health.health.aggregated_health_state
                    == mssf_core::types::HealthState::Warning
        );
        assert!(
            cluster_health.health.node_health_states.is_empty(),
            "Cluster health should not have nodes, we retrieve them separately."
        );
        assert!(
            cluster_health.health.application_health_states.is_empty(),
            "Cluster health should not have application health states, we retrieve them separately."
        );

        // We have 5 nodes in local SF windows cluster
        // and 3 nodes for linux cluster.
        assert!(
            data.node_health_entities.len() >= 3,
            "Not enough nodes {:?}",
            data.node_health_entities
        );
        let node1 = &data.node_health_entities[0];
        assert!(!node1.node.name.is_empty());
        assert!(
            node1.health.aggregated_health_state == mssf_core::types::HealthState::Ok
                || node1.health.aggregated_health_state == mssf_core::types::HealthState::Warning
        );

        // Get applications
        // For empty cluster applications is 0
        if data.application_health_entities.is_empty() {
            tracing::warn!("No applications found in the cluster");
        } else {
            let app1 = &data.application_health_entities[0];
            assert_eq!(
                app1.application.health_state,
                app1.health.aggregated_health_state
            );
            assert!(
                app1.health.aggregated_health_state == mssf_core::types::HealthState::Ok
                    || app1.health.aggregated_health_state
                        == mssf_core::types::HealthState::Warning
            );
        }
        if data.partition_health_entities.is_empty() {
            tracing::warn!("No partitions found in the cluster");
        } else {
            let partition1 = &data.partition_health_entities[0];
            assert_eq!(
                partition1.partition.get_health_state(),
                partition1.health.aggregated_health_state
            );
            assert!(
                partition1.health.aggregated_health_state == mssf_core::types::HealthState::Ok
                    || partition1.health.aggregated_health_state
                        == mssf_core::types::HealthState::Warning
            );
        }
        if data.service_health_entities.is_empty() {
            tracing::warn!("No services found in the cluster");
        } else {
            let service1 = &data.service_health_entities[0];
            assert_eq!(
                service1.service.get_health_state(),
                service1.health.aggregated_health_state
            );
            assert!(
                service1.health.aggregated_health_state == mssf_core::types::HealthState::Ok
                    || service1.health.aggregated_health_state
                        == mssf_core::types::HealthState::Warning
            );
        }
        if data.replica_health_entities.is_empty() {
            tracing::warn!("No replicas found in the cluster");
        } else {
            let replica1 = &data.replica_health_entities[0];
            assert_eq!(
                replica1.replica.get_aggregated_health_state(),
                replica1.health.replica_health.get_aggregated_health_state()
            );
            assert!(
                replica1.health.replica_health.get_aggregated_health_state()
                    == mssf_core::types::HealthState::Ok
                    || replica1.health.replica_health.get_aggregated_health_state()
                        == mssf_core::types::HealthState::Warning
            );
        }
    }
}
