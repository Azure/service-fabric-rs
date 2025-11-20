// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::types::{
    ApplicationHealth, ApplicationQueryResultItem, ClusterHealth, NodeHealthResult,
    NodeQueryResultItem, PartitionHealthResult, ReplicaHealthResult, ServiceHealthResult,
    ServicePartitionQueryResultItem, ServiceQueryResultItem, ServiceReplicaQueryResultItem, Uri,
};

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

/// There is no info for cluster name in FabricClient.
/// User is supposed to inject the cluster name in consumer side.
#[derive(Debug, Clone)]
pub struct ClusterHealthEntity {
    pub health: ClusterHealth,
}

#[derive(Debug, Clone)]
pub struct NodeHealthEntity {
    pub node: NodeQueryResultItem,
    pub health: NodeHealthResult,
}

#[derive(Debug, Clone)]
pub struct ApplicationHealthEntity {
    pub application: ApplicationQueryResultItem,
    pub health: ApplicationHealth,
}

#[derive(Debug, Clone)]
pub struct ServiceHealthEntity {
    pub service: ServiceQueryResultItem,
    pub health: ServiceHealthResult,
}

#[derive(Debug, Clone)]
pub struct PartitionHealthEntity {
    pub partition: ServicePartitionQueryResultItem,
    pub health: PartitionHealthResult,
    pub service_name: Uri,
    pub application_name: Uri,
}

#[derive(Debug, Clone)]
pub struct ReplicaHealthEntity {
    pub replica: ServiceReplicaQueryResultItem,
    pub health: ReplicaHealthResult,
    pub service_name: Uri,
    pub application_name: Uri,
}
