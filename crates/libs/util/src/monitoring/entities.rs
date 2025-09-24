// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::types::{ApplicationQueryResultItem, ClusterHealth, Node, NodeHealthResult};

/// Health entities produced by HealthDataProducer.
#[derive(Debug, Clone)]
pub enum HealthEntity {
    Node(NodeHealthEntity),
    Cluster(ClusterHealthEntity),
    Application(ApplicationHealthEntity),
    Partition(PartitionHealthEntity),
    Service(ServiceHealthEntity),
    Replica(ReplicaHealthEntity),
}

#[derive(Debug, Clone)]
pub struct NodeHealthEntity {
    pub node: Node,
    pub health: NodeHealthResult,
}

#[derive(Debug, Clone)]
pub struct ClusterHealthEntity {
    pub health: ClusterHealth,
}

#[derive(Debug, Clone)]
pub struct ApplicationHealthEntity {
    pub application: ApplicationQueryResultItem,
    pub health: mssf_core::types::ApplicationHealth,
}

#[derive(Debug, Clone)]
pub struct PartitionHealthEntity {
    pub partition: mssf_core::types::ServicePartitionQueryResultItem,
    pub health: mssf_core::types::PartitionHealthResult,
}

#[derive(Debug, Clone)]
pub struct ServiceHealthEntity {
    pub service: mssf_core::types::ServiceQueryResultItem,
    pub health: mssf_core::types::ServiceHealthResult,
}

#[derive(Debug, Clone)]
pub struct ReplicaHealthEntity {
    pub replica: mssf_core::types::ServiceReplicaQueryResultItem,
    pub health: mssf_core::types::ReplicaHealthResult,
}
