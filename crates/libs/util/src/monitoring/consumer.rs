use mssf_core::runtime::executor::BoxedCancelToken;
use tokio::sync::mpsc;

use crate::monitoring::HealthEntity;

pub struct HealthDataConsumer {
    receiver: mpsc::UnboundedReceiver<HealthEntity>,
}

impl HealthDataConsumer {
    pub fn new(receiver: mpsc::UnboundedReceiver<HealthEntity>) -> Self {
        HealthDataConsumer { receiver }
    }

    pub async fn run(&mut self, token: BoxedCancelToken) {
        loop {
            ::tokio::select! {
                _ = token.wait() => {
                    break;
                }
                maybe_entity = self.receiver.recv() => {
                    match maybe_entity {
                        Some(entity) => {
                            self.process_entity(entity).await;
                        }
                        None => {
                            // Channel closed, exit the loop.
                            break;
                        }
                    }
                }
            }
        }
    }

    async fn process_entity(&self, entity: HealthEntity) {
        match entity {
            HealthEntity::Node(node) => {
                tracing::info!("Received node health entity: {:?}", node.node_name);
                // TODO: Process node health entity here.
            }
        }
    }
}
