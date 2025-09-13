// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::monitoring::{HealthEntity, NodeHealthEntity};
use ::tokio::sync::mpsc;
use mssf_core::{
    WString,
    client::FabricClient,
    runtime::executor::BoxedCancelToken,
    types::{HealthEventsFilter, HealthStateFilterFlags, Node, NodeHealthQueryDescription},
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
    Continue,
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

    /// Run once to produce health data.
    pub(crate) async fn run_once(&self, token: BoxedCancelToken) -> Action {
        let mut health_entities = Vec::new();
        // Get node information.
        if let Ok(nodes) = self.get_all_nodes(token.clone()).await {
            for node in nodes {
                if let Some(entity) = self
                    .produce_node_health_entity(token.clone(), node.name)
                    .await
                {
                    health_entities.push(entity);
                }
            }
        }

        // Send the health entities to the consumer.
        for entity in health_entities {
            if self.sender.send(entity).is_err() {
                tracing::warn!("Receiver dropped, exit the loop.");
                return Action::Stop;
            }
        }
        self.iteration
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Action::Continue
    }

    /// Run a loop to produce health data.
    pub async fn run_loop(&self, token: BoxedCancelToken) {
        loop {
            let start_time = ::tokio::time::Instant::now();
            match self.run_once(token.clone()).await {
                Action::Stop => {
                    tracing::info!("Health data producer stopped.");
                    break;
                }
                Action::Continue => {
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

    /// Produce the health entity for a node.
    async fn produce_node_health_entity(
        &self,
        token: BoxedCancelToken,
        node_name: WString,
    ) -> Option<HealthEntity> {
        // Logic to get node health goes here.

        let desc = NodeHealthQueryDescription {
            node_name,
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
            node_name: node_healths.node_name.to_string(),
            aggregated_health_state: node_healths.aggregated_health_state,
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
}
