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
        HealthEventsFilter, HealthStateFilterFlags, Node, NodeHealthQueryDescription,
        NodeHealthStatesFilter,
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
                if let Some(entity) = self
                    .produce_application_health_entity(token.clone(), app)
                    .await
                {
                    self.send_entity(entity)?;
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
        node: Node,
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

    async fn get_all_nodes(&self, token: BoxedCancelToken) -> mssf_core::Result<Vec<Node>> {
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
            .iter()
            .collect::<Vec<_>>();
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
}
