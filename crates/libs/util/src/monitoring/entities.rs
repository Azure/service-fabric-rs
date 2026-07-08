// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::types::{
    ApplicationHealth, ApplicationQueryResultItem, ApplicationUpgradeProgress, ClusterHealth,
    NodeHealthResult, NodeQueryResultItem, PartitionHealthResult, ReplicaHealthResult,
    ServiceHealthResult, ServicePartitionQueryResultItem, ServiceQueryResultItem,
    ServiceReplicaQueryResultItem, Uri,
};

/// Events produced by HealthDataProducer.
///
/// Most variants carry a health entity; `IterationComplete` is a control
/// marker signalling that a producer loop finished an iteration.
#[derive(Debug, Clone)]
pub enum ProducerEvent {
    Node(NodeHealthEntity),
    Cluster(ClusterHealthEntity),
    Application(ApplicationHealthEntity),
    Partition(PartitionHealthEntity),
    Service(ServiceHealthEntity),
    Replica(ReplicaHealthEntity),
    /// An application that is actively going through an upgrade. Only emitted
    /// when upgrade reporting is enabled on the producer (
    /// [`crate::monitoring::HealthDataProducer::with_upgrade_reporting`]).
    Upgrade(ApplicationUpgradeProgress),
    /// Marker emitted at the end of a producer loop iteration. Allows a
    /// consumer to detect that a loop has produced a full set of data for the
    /// current iteration.
    IterationComplete(LoopKind),
}

/// Identifies which producer loop an [`ProducerEvent::IterationComplete`] marker
/// belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopKind {
    /// The cluster and node health loop.
    ClusterNode,
    /// The application (and services, partitions, replicas) health loop.
    Application,
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
