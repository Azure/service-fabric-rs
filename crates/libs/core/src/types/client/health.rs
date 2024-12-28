// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricTypes::{
    FABRIC_CLUSTER_HEALTH_REPORT, FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT,
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT, FABRIC_HEALTH_REPORT,
    FABRIC_HEALTH_REPORT_KIND_APPLICATION, FABRIC_HEALTH_REPORT_KIND_CLUSTER,
    FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
    FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE, FABRIC_HEALTH_REPORT_KIND_INVALID,
    FABRIC_HEALTH_REPORT_KIND_NODE, FABRIC_HEALTH_REPORT_KIND_PARTITION,
    FABRIC_HEALTH_REPORT_KIND_SERVICE, FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
    FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE,
};
use windows_core::{WString, PCWSTR};

use crate::types::HealthInformation;

/// Wrapper of FABRIC_HEALTH_REPORT
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

// TODO: Implement the conversion functions for the HealthReport enum properly.
impl From<&FABRIC_HEALTH_REPORT> for HealthReport {
    fn from(value: &FABRIC_HEALTH_REPORT) -> Self {
        match value.Kind {
            FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA => {
                HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport)
            }
            FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE => {
                HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport)
            }
            FABRIC_HEALTH_REPORT_KIND_PARTITION => HealthReport::Partition(PartitionHealthReport),
            FABRIC_HEALTH_REPORT_KIND_NODE => HealthReport::Node(NodeHealthReport),
            FABRIC_HEALTH_REPORT_KIND_SERVICE => HealthReport::Service(ServiceHealthReport),
            FABRIC_HEALTH_REPORT_KIND_APPLICATION => {
                HealthReport::Application(ApplicationHealthReport)
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION => {
                HealthReport::DeployedApplication(DeployedApplicationHealthReport {
                    application_name: WString::from_wide(unsafe {
                        PCWSTR::from_raw(
                            (*(value.Value as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT))
                                .ApplicationName
                                .0,
                        )
                        .as_wide()
                    }),
                    node_name: WString::from_wide(unsafe {
                        (*(value.Value as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT))
                            .NodeName
                            .as_wide()
                    }),
                    health_information: HealthInformation::from(unsafe {
                        &*(*(value.Value as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT))
                            .HealthInformation
                    }),
                })
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE => {
                HealthReport::DeployedServicePackage(DeployedServicePackageHealthReport {
                    application_name: WString::from_wide(unsafe {
                        PCWSTR::from_raw(
                            (*(value.Value
                                as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT))
                                .ApplicationName
                                .0,
                        )
                        .as_wide()
                    }),
                    service_manifest_name: WString::from_wide(unsafe {
                        PCWSTR::from_raw(
                            (*(value.Value
                                as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT))
                                .ServiceManifestName
                                .0,
                        )
                        .as_wide()
                    }),
                    node_name: WString::from_wide(unsafe {
                        (*(value.Value as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT))
                            .NodeName
                            .as_wide()
                    }),
                    health_information: HealthInformation::from(unsafe {
                        &*(*(value.Value as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT))
                            .HealthInformation
                    }),
                })
            }
            FABRIC_HEALTH_REPORT_KIND_CLUSTER => HealthReport::Cluster(ClusterHealthReport {
                health_information: HealthInformation::from(unsafe {
                    &*(*(value.Value as *const FABRIC_CLUSTER_HEALTH_REPORT)).HealthInformation
                }),
            }),
            _ => HealthReport::Invalid,
        }
    }
}

impl From<&HealthReport> for FABRIC_HEALTH_REPORT {
    fn from(value: &HealthReport) -> Self {
        match value {
            HealthReport::StatefulServiceReplica(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
                Value: v as *const StatefulServiceReplicaHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::StatelessServiceInstance(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE,
                Value: v as *const StatelessServiceInstanceHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Partition(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_PARTITION,
                Value: v as *const PartitionHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Node(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_NODE,
                Value: v as *const NodeHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Service(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_SERVICE,
                Value: v as *const ServiceHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Application(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_APPLICATION,
                Value: v as *const ApplicationHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::DeployedApplication(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
                Value: v as *const DeployedApplicationHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::DeployedServicePackage(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE,
                Value: v as *const DeployedServicePackageHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Cluster(v) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_CLUSTER,
                Value: v as *const ClusterHealthReport as *mut std::ffi::c_void,
            },
            HealthReport::Invalid => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_INVALID,
                Value: std::ptr::null_mut(),
            },
        }
    }
}

/// Wrapper of FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct StatefulServiceReplicaHealthReport;

/// Wrapper of FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct StatelessServiceInstanceHealthReport;

/// Wrapper of FABRIC_PARTITION_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct PartitionHealthReport;

/// Wrapper of FABRIC_NODE_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct NodeHealthReport;

/// Wrapper of FABRIC_SERVICE_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct ServiceHealthReport;

/// Wrapper of FABRIC_APPLICATION_HEALTH_REPORT
/// TODO: Implement this struct.
pub struct ApplicationHealthReport;

/// Wrapper of FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
pub struct DeployedApplicationHealthReport {
    pub application_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
pub struct DeployedServicePackageHealthReport {
    pub application_name: WString,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_CLUSTER_HEALTH_REPORT
pub struct ClusterHealthReport {
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}
