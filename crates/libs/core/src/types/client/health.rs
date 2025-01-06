// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::ffi::c_void;

use mssf_com::FabricTypes::{
    FABRIC_APPLICATION_HEALTH_REPORT, FABRIC_CLUSTER_HEALTH_REPORT,
    FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT,
    FABRIC_HEALTH_INFORMATION, FABRIC_HEALTH_REPORT, FABRIC_HEALTH_REPORT_KIND_APPLICATION,
    FABRIC_HEALTH_REPORT_KIND_CLUSTER, FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
    FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE, FABRIC_HEALTH_REPORT_KIND_INVALID,
    FABRIC_HEALTH_REPORT_KIND_NODE, FABRIC_HEALTH_REPORT_KIND_PARTITION,
    FABRIC_HEALTH_REPORT_KIND_SERVICE, FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
    FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE, FABRIC_NODE_HEALTH_REPORT,
    FABRIC_PARTITION_HEALTH_REPORT, FABRIC_SERVICE_HEALTH_REPORT,
    FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT, FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT,
    FABRIC_URI,
};
use windows_core::{WString, GUID, PCWSTR};

use crate::{strings::WStringWrap, types::HealthInformation};

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

impl From<&FABRIC_HEALTH_REPORT> for HealthReport {
    fn from(value: &FABRIC_HEALTH_REPORT) -> Self {
        match value.Kind {
            FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA => {
                HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport::from(
                    unsafe {
                        &*(value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT)
                    },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE => {
                HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport::from(
                    unsafe {
                        &*(value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT)
                    },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_PARTITION => {
                HealthReport::Partition(PartitionHealthReport::from(unsafe {
                    &*(value.Value as *const FABRIC_PARTITION_HEALTH_REPORT)
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_NODE => HealthReport::Node(NodeHealthReport::from(unsafe {
                &*(value.Value as *const FABRIC_NODE_HEALTH_REPORT)
            })),
            FABRIC_HEALTH_REPORT_KIND_SERVICE => {
                HealthReport::Service(ServiceHealthReport::from(unsafe {
                    &*(value.Value as *const FABRIC_SERVICE_HEALTH_REPORT)
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_APPLICATION => {
                HealthReport::Application(ApplicationHealthReport::from(unsafe {
                    &*(value.Value as *const FABRIC_APPLICATION_HEALTH_REPORT)
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION => {
                HealthReport::DeployedApplication(DeployedApplicationHealthReport::from(unsafe {
                    &*(value.Value as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT)
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE => {
                HealthReport::DeployedServicePackage(DeployedServicePackageHealthReport::from(
                    unsafe {
                        &*(value.Value as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT)
                    },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_CLUSTER => {
                HealthReport::Cluster(ClusterHealthReport::from(unsafe {
                    &*(value.Value as *const FABRIC_CLUSTER_HEALTH_REPORT)
                }))
            }
            _ => HealthReport::Invalid,
        }
    }
}

impl From<&HealthReport> for FABRIC_HEALTH_REPORT {
    fn from(value: &HealthReport) -> Self {
        match value {
            HealthReport::Invalid => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_INVALID,
                Value: std::ptr::null_mut(),
            },
            HealthReport::StatefulServiceReplica(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
                Value: &FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT::from(report)
                    as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::StatelessServiceInstance(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE,
                Value: &FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT::from(report)
                    as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::Partition(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_PARTITION,
                Value: &FABRIC_PARTITION_HEALTH_REPORT::from(report)
                    as *const FABRIC_PARTITION_HEALTH_REPORT as *mut c_void,
            },
            HealthReport::Node(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_NODE,
                Value: &FABRIC_NODE_HEALTH_REPORT::from(report) as *const FABRIC_NODE_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::Service(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_SERVICE,
                Value: &FABRIC_SERVICE_HEALTH_REPORT::from(report)
                    as *const FABRIC_SERVICE_HEALTH_REPORT as *mut c_void,
            },
            HealthReport::Application(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_APPLICATION,
                Value: &FABRIC_APPLICATION_HEALTH_REPORT::from(report)
                    as *const FABRIC_APPLICATION_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::DeployedApplication(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
                Value: &FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT::from(report)
                    as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::DeployedServicePackage(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE,
                Value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT::from(report)
                    as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
                    as *mut c_void,
            },
            HealthReport::Cluster(report) => Self {
                Kind: FABRIC_HEALTH_REPORT_KIND_CLUSTER,
                Value: &FABRIC_CLUSTER_HEALTH_REPORT::from(report)
                    as *const FABRIC_CLUSTER_HEALTH_REPORT as *mut c_void,
            },
        }
    }
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

impl From<&StatefulServiceReplicaHealthReport> for FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT {
    fn from(value: &StatefulServiceReplicaHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            PartitionId: value.partition_id,
            ReplicaId: value.replica_id,
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT> for StatefulServiceReplicaHealthReport {
    fn from(value: &FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            partition_id: value.PartitionId,
            replica_id: value.ReplicaId,
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
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

impl From<&StatelessServiceInstanceHealthReport>
    for FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
{
    fn from(value: &StatelessServiceInstanceHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            PartitionId: value.partition_id,
            InstanceId: value.instance_id,
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT>
    for StatelessServiceInstanceHealthReport
{
    fn from(value: &FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            partition_id: value.PartitionId,
            instance_id: value.InstanceId,
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

/// FABRIC_PARTITION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct PartitionHealthReport {
    pub partition_id: GUID,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

impl From<&PartitionHealthReport> for FABRIC_PARTITION_HEALTH_REPORT {
    fn from(value: &PartitionHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            PartitionId: value.partition_id,
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_PARTITION_HEALTH_REPORT> for PartitionHealthReport {
    fn from(value: &FABRIC_PARTITION_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            partition_id: value.PartitionId,
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

/// FABRIC_NODE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct NodeHealthReport {
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

impl From<&NodeHealthReport> for FABRIC_NODE_HEALTH_REPORT {
    fn from(value: &NodeHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            NodeName: value.node_name.as_pcwstr(),
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_NODE_HEALTH_REPORT> for NodeHealthReport {
    fn from(value: &FABRIC_NODE_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            node_name: WStringWrap::from(PCWSTR::from_raw(value.NodeName.0)).into(),
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

/// FABRIC_SERVICE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ServiceHealthReport {
    pub service_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

impl From<&ServiceHealthReport> for FABRIC_SERVICE_HEALTH_REPORT {
    fn from(value: &ServiceHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            ServiceName: FABRIC_URI(value.service_name.as_ptr() as *mut u16),
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_SERVICE_HEALTH_REPORT> for ServiceHealthReport {
    fn from(value: &FABRIC_SERVICE_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            service_name: WStringWrap::from(PCWSTR::from_raw(value.ServiceName.0)).into(),
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

/// FABRIC_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ApplicationHealthReport {
    pub application_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

impl From<&ApplicationHealthReport> for FABRIC_APPLICATION_HEALTH_REPORT {
    fn from(value: &ApplicationHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_APPLICATION_HEALTH_REPORT> for ApplicationHealthReport {
    fn from(value: &FABRIC_APPLICATION_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
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

impl From<&DeployedApplicationHealthReport> for FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT {
    fn from(value: &DeployedApplicationHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
            NodeName: value.node_name.as_pcwstr(),
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT> for DeployedApplicationHealthReport {
    fn from(value: &FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
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

impl From<&DeployedServicePackageHealthReport> for FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT {
    fn from(value: &DeployedServicePackageHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
            ServiceManifestName: value.service_manifest_name.as_pcwstr(),
            NodeName: value.node_name.as_pcwstr(),
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT> for DeployedServicePackageHealthReport {
    fn from(value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            service_manifest_name: WStringWrap::from(value.ServiceManifestName).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

/// FABRIC_CLUSTER_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ClusterHealthReport {
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

impl From<&ClusterHealthReport> for FABRIC_CLUSTER_HEALTH_REPORT {
    fn from(value: &ClusterHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            HealthInformation: Box::into_raw(boxed_health_info),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl From<&FABRIC_CLUSTER_HEALTH_REPORT> for ClusterHealthReport {
    fn from(value: &FABRIC_CLUSTER_HEALTH_REPORT) -> Self {
        let boxed_health_info = unsafe {
            Box::<FABRIC_HEALTH_INFORMATION>::from_raw(value.HealthInformation as *mut _)
        };
        Self {
            health_information: HealthInformation::from(boxed_health_info.as_ref()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cluster_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let cluster_health_report = HealthReport::Cluster(ClusterHealthReport {
            health_information: health_info.clone(),
        });

        let com_cluster_health_report: FABRIC_HEALTH_REPORT = (&cluster_health_report).into();
        assert_eq!(
            com_cluster_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_CLUSTER
        );

        let cluster_health_report2: HealthReport = (&com_cluster_health_report).into();

        // Check the inner values are matching
        if let HealthReport::Cluster(report) = cluster_health_report2 {
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_invalid_health_report_conversion() {
        let invalid_health_report = HealthReport::Invalid;

        let com_invalid_health_report: FABRIC_HEALTH_REPORT = (&invalid_health_report).into();
        assert_eq!(
            com_invalid_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_INVALID
        );

        let invalid_health_report2: HealthReport = (&com_invalid_health_report).into();
        matches!(invalid_health_report2, HealthReport::Invalid);
    }

    #[test]
    fn test_deployed_service_package_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let deployed_service_package_health_report =
            HealthReport::DeployedServicePackage(DeployedServicePackageHealthReport {
                application_name: "fabric:/MyApp".into(),
                service_manifest_name: "manifest_name".into(),
                node_name: "node_name".into(),
                health_information: health_info.clone(),
            });
        let com_deployed_service_package_health_report: FABRIC_HEALTH_REPORT =
            (&deployed_service_package_health_report).into();

        assert_eq!(
            com_deployed_service_package_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE
        );

        let deployed_service_package_health_report2: HealthReport =
            (&com_deployed_service_package_health_report).into();

        if let HealthReport::DeployedServicePackage(report) =
            deployed_service_package_health_report2
        {
            assert_eq!(report.application_name, "fabric:/MyApp".into());
            assert_eq!(report.service_manifest_name, "manifest_name".into());
            assert_eq!(report.node_name, "node_name".into());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_deployed_application_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let deployed_application_health_report =
            HealthReport::DeployedApplication(DeployedApplicationHealthReport {
                application_name: "fabric:/MyApp".into(),
                node_name: "node_name".into(),
                health_information: health_info.clone(),
            });
        let com_deployed_application_health_report: FABRIC_HEALTH_REPORT =
            (&deployed_application_health_report).into();

        assert_eq!(
            com_deployed_application_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION
        );

        let deployed_application_health_report2: HealthReport =
            (&com_deployed_application_health_report).into();

        if let HealthReport::DeployedApplication(report) = deployed_application_health_report2 {
            assert_eq!(report.application_name, "fabric:/MyApp".into());
            assert_eq!(report.node_name, "node_name".into());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_application_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let application_health_report = HealthReport::Application(ApplicationHealthReport {
            application_name: "fabric:/MyApp".into(),
            health_information: health_info.clone(),
        });
        let com_application_health_report: FABRIC_HEALTH_REPORT =
            (&application_health_report).into();

        assert_eq!(
            com_application_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_APPLICATION
        );

        let application_health_report2: HealthReport = (&com_application_health_report).into();

        if let HealthReport::Application(report) = application_health_report2 {
            assert_eq!(report.application_name, "fabric:/MyApp".into());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_service_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let service_health_report = HealthReport::Service(ServiceHealthReport {
            service_name: "fabric:/MyService".into(),
            health_information: health_info.clone(),
        });
        let com_service_health_report: FABRIC_HEALTH_REPORT = (&service_health_report).into();

        assert_eq!(
            com_service_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_SERVICE
        );

        let service_health_report2: HealthReport = (&com_service_health_report).into();

        if let HealthReport::Service(report) = service_health_report2 {
            assert_eq!(report.service_name, "fabric:/MyService".into());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_node_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let node_health_report = HealthReport::Node(NodeHealthReport {
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        });
        let com_node_health_report: FABRIC_HEALTH_REPORT = (&node_health_report).into();

        assert_eq!(com_node_health_report.Kind, FABRIC_HEALTH_REPORT_KIND_NODE);

        let node_health_report2: HealthReport = (&com_node_health_report).into();

        if let HealthReport::Node(report) = node_health_report2 {
            assert_eq!(report.node_name, "node_name".into());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_partition_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let partition_health_report = HealthReport::Partition(PartitionHealthReport {
            partition_id: GUID::zeroed(),
            health_information: health_info.clone(),
        });
        let com_partition_health_report: FABRIC_HEALTH_REPORT = (&partition_health_report).into();

        assert_eq!(
            com_partition_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_PARTITION
        );

        let partition_health_report2: HealthReport = (&com_partition_health_report).into();

        if let HealthReport::Partition(report) = partition_health_report2 {
            assert_eq!(report.partition_id, GUID::zeroed());
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_stateless_service_instance_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let stateless_service_instance_health_report =
            HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport {
                partition_id: GUID::zeroed(),
                instance_id: 1,
                health_information: health_info.clone(),
            });
        let com_stateless_service_instance_health_report: FABRIC_HEALTH_REPORT =
            (&stateless_service_instance_health_report).into();
        assert_eq!(
            com_stateless_service_instance_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE
        );

        let stateless_service_instance_health_report2: HealthReport =
            (&com_stateless_service_instance_health_report).into();

        if let HealthReport::StatelessServiceInstance(report) =
            stateless_service_instance_health_report2
        {
            assert_eq!(report.partition_id, GUID::zeroed());
            assert_eq!(report.instance_id, 1);
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }

    #[test]
    fn test_stateful_service_replica_health_report_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let stateful_service_replica_health_report =
            HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport {
                partition_id: GUID::zeroed(),
                replica_id: 1,
                health_information: health_info.clone(),
            });
        let com_stateful_service_replica_health_report: FABRIC_HEALTH_REPORT =
            (&stateful_service_replica_health_report).into();
        assert_eq!(
            com_stateful_service_replica_health_report.Kind,
            FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA
        );

        let stateful_service_replica_health_report2: HealthReport =
            (&com_stateful_service_replica_health_report).into();

        if let HealthReport::StatefulServiceReplica(report) =
            stateful_service_replica_health_report2
        {
            assert_eq!(report.partition_id, GUID::zeroed());
            assert_eq!(report.replica_id, 1);
            assert_eq!(report.health_information.source_id, health_info.source_id);
            assert_eq!(report.health_information.property, health_info.property);
            assert_eq!(
                report.health_information.time_to_live_seconds,
                health_info.time_to_live_seconds
            );
            assert_eq!(report.health_information.state, health_info.state);
            assert_eq!(
                report.health_information.description,
                health_info.description
            );
            assert_eq!(
                report.health_information.sequence_number,
                health_info.sequence_number
            );
            assert_eq!(
                report.health_information.remove_when_expired,
                health_info.remove_when_expired
            );
        } else {
            panic!("Test Failed - Unexpected HealthReport type.");
        }
    }
}
