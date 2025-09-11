// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::types::HealthState;

/// Health entities produced by HealthDataProducer.
#[derive(Debug, Clone)]
pub enum HealthEntity {
    Node(NodeHealthEntity),
}

#[derive(Debug, Clone)]
pub struct NodeHealthEntity {
    pub node_name: String,
    pub aggregated_health_state: HealthState,
}
