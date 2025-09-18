// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::types::{ClusterHealth, Node, NodeHealthResult};

/// Health entities produced by HealthDataProducer.
#[derive(Debug, Clone)]
pub enum HealthEntity {
    Node(NodeHealthEntity),
    Cluster(ClusterHealthEntity),
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
