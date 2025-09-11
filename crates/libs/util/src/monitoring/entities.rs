use mssf_core::types::HealthState;

#[derive(Debug, Clone)]
pub enum HealthEntity {
    Node(NodeHealthEntity),
}

#[derive(Debug, Clone)]
pub struct NodeHealthEntity {
    pub node_name: String,
    pub aggregated_health_state: HealthState,
}
