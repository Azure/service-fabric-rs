// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricTypes::{
    FABRIC_APPLICATION_HEALTH_REPORT, FABRIC_CLUSTER_HEALTH_REPORT,
    FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT,
    FABRIC_HEALTH_REPORT, FABRIC_HEALTH_REPORT_KIND_APPLICATION, FABRIC_HEALTH_REPORT_KIND_CLUSTER,
    FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
    FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE, FABRIC_HEALTH_REPORT_KIND_INVALID,
    FABRIC_HEALTH_REPORT_KIND_NODE, FABRIC_HEALTH_REPORT_KIND_PARTITION,
    FABRIC_HEALTH_REPORT_KIND_SERVICE, FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
    FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE, FABRIC_NODE_HEALTH_REPORT,
    FABRIC_PARTITION_HEALTH_REPORT, FABRIC_SERVICE_HEALTH_REPORT,
    FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT, FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT,
};
use windows_core::{WString, GUID, PCWSTR};

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

impl From<&FABRIC_HEALTH_REPORT> for HealthReport {
    fn from(value: &FABRIC_HEALTH_REPORT) -> Self {
        match value.Kind {
            FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA => {
                HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport {
                    partition_id: unsafe {
                        (*(value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT))
                            .PartitionId
                    },
                    replica_id: unsafe {
                        (*(value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT))
                            .ReplicaId
                    },
                    health_information: HealthInformation::from(unsafe {
                        &*(*(value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT))
                            .HealthInformation
                    }),
                })
            }
            FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE => {
                HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport {
                    partition_id: unsafe {
                        (*(value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT))
                            .PartitionId
                    },
                    instance_id: unsafe {
                        (*(value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT))
                            .InstanceId
                    },
                    health_information: HealthInformation::from(unsafe {
                        &*(*(value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT))
                            .HealthInformation
                    }),
                })
            }
            FABRIC_HEALTH_REPORT_KIND_PARTITION => HealthReport::Partition(PartitionHealthReport {
                partition_id: unsafe {
                    (*(value.Value as *const FABRIC_PARTITION_HEALTH_REPORT)).PartitionId
                },
                health_information: HealthInformation::from(unsafe {
                    &*(*(value.Value as *const FABRIC_PARTITION_HEALTH_REPORT)).HealthInformation
                }),
            }),
            FABRIC_HEALTH_REPORT_KIND_NODE => HealthReport::Node(NodeHealthReport {
                node_name: WString::from_wide(unsafe {
                    (*(value.Value as *const FABRIC_NODE_HEALTH_REPORT))
                        .NodeName
                        .as_wide()
                }),
                health_information: HealthInformation::from(unsafe {
                    &*(*(value.Value as *const FABRIC_NODE_HEALTH_REPORT)).HealthInformation
                }),
            }),
            FABRIC_HEALTH_REPORT_KIND_SERVICE => HealthReport::Service(ServiceHealthReport {
                service_name: WString::from_wide(unsafe {
                    PCWSTR::from_raw(
                        (*(value.Value as *const FABRIC_SERVICE_HEALTH_REPORT))
                            .ServiceName
                            .0,
                    )
                    .as_wide()
                }),
                health_information: HealthInformation::from(unsafe {
                    &*(*(value.Value as *const FABRIC_SERVICE_HEALTH_REPORT)).HealthInformation
                }),
            }),
            FABRIC_HEALTH_REPORT_KIND_APPLICATION => {
                HealthReport::Application(ApplicationHealthReport {
                    application_name: WString::from_wide(unsafe {
                        PCWSTR::from_raw(
                            (*(value.Value as *const FABRIC_APPLICATION_HEALTH_REPORT))
                                .ApplicationName
                                .0,
                        )
                        .as_wide()
                    }),
                    health_information: HealthInformation::from(unsafe {
                        &*(*(value.Value as *const FABRIC_APPLICATION_HEALTH_REPORT))
                            .HealthInformation
                    }),
                })
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
#[derive(Debug, Clone)]
pub struct StatefulServiceReplicaHealthReport {
    pub partition_id: GUID,
    pub replica_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct StatelessServiceInstanceHealthReport {
    pub partition_id: GUID,
    pub instance_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_PARTITION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct PartitionHealthReport {
    pub partition_id: GUID,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_NODE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct NodeHealthReport {
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_SERVICE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ServiceHealthReport {
    pub service_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ApplicationHealthReport {
    pub application_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealthReport {
    pub application_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealthReport {
    pub application_name: WString,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// Wrapper of FABRIC_CLUSTER_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ClusterHealthReport {
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}
