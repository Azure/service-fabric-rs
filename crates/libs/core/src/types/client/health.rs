// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use windows_core::{WString, GUID};

use crate::types::HealthInformation;

/// FABRIC_HEALTH_REPORT
#[derive(Debug, Clone)]
pub enum HealthReport {
    Invalid,
    StatefulServiceReplica(StatefulServiceReplicaHealthReport),
    StatelessServiceInstance(StatelessServiceInstanceHealthReport),
    Partition(PartitionHealthReport),
    Node(NodeHealthReport),
    Service(ServiceHealthReport),
    Application(ApplicationHealthReport),
    DeployedApplication(DeployedApplicationHealthReport),
    DeployedServicePackage(DeployedServicePackageHealthReport),
    Cluster(ClusterHealthReport),
}

/// FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct StatefulServiceReplicaHealthReport {
    pub partition_id: GUID,
    pub replica_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct StatelessServiceInstanceHealthReport {
    pub partition_id: GUID,
    pub instance_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_PARTITION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct PartitionHealthReport {
    pub partition_id: GUID,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_NODE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct NodeHealthReport {
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_SERVICE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ServiceHealthReport {
    pub service_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ApplicationHealthReport {
    pub application_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealthReport {
    pub application_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealthReport {
    pub application_name: WString,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_CLUSTER_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ClusterHealthReport {
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}
