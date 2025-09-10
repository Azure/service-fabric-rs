use std::time::Duration;

use ::tokio::sync::mpsc;
use mssf_core::{client::FabricClient, runtime::executor::BoxedCancelToken};

use crate::monitoring::{HealthEntity, NodeHealthEntity};

/// Queries SF and produces health data.
pub struct HealthDataProducer {
    fc: FabricClient,
    interval: Duration,
    sender: mpsc::UnboundedSender<HealthEntity>,
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

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
        }
    }

    pub async fn run(&self, token: BoxedCancelToken) {
        // Monitoring logic goes here.
        let start_time = ::tokio::time::Instant::now();
        loop {
            // Get node information.
            if let Some(nodes) = self.get_node_info(token.clone()).await {
                for node in nodes {
                    let _ = self.sender.send(node);
                }
            }
            // TODO: get more health entities here.

            // remaining time
            let elapsed = start_time.elapsed();
            // wait for more time if necessary.
            if elapsed < self.interval {
                let wait_duration = self.interval - elapsed;

                ::tokio::select! {
                    _ = token.wait() => {
                        break;
                    }
                    _ = ::tokio::time::sleep(wait_duration) => {}
                }
            }

            if token.is_cancelled() {
                break;
            }
        }
    }

    async fn get_node_info(&self, token: BoxedCancelToken) -> Option<Vec<HealthEntity>> {
        // Logic to get node information goes here.
        let desc = &Default::default();
        let nodes = self
            .fc
            .get_query_manager()
            .get_node_list(desc, DEFAULT_TIMEOUT, Some(token))
            .await
            .ok()?;
        let entities = nodes
            .iter()
            .map(|node| {
                HealthEntity::Node(NodeHealthEntity {
                    node_name: node.name.to_string_lossy(),
                })
            })
            .collect();
        // TODO: get node health.
        Some(entities)
    }
}
