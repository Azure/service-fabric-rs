pub enum HealthEntity {
    Node(NodeHealthEntity),
}

pub struct NodeHealthEntity {
    pub node_name: String,
}
