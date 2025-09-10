mod producer;
pub use producer::HealthDataProducer;
mod consumer;
pub use consumer::HealthDataConsumer;
mod entities;
pub use entities::{HealthEntity, NodeHealthEntity};

use mssf_core::client::FabricClient;
use tokio::sync::mpsc;
use tokio::time::Duration;

pub fn new_health_data(fc: FabricClient) -> (HealthDataProducer, HealthDataConsumer) {
    let (sender, receiver) = mpsc::unbounded_channel();
    let producer = HealthDataProducer::new(fc, Duration::from_secs(30), sender);
    let consumer = HealthDataConsumer::new(receiver);
    (producer, consumer)
}
