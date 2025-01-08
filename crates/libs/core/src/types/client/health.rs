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

/// An intermediate representation of [FABRIC_HEALTH_REPORT] to workaround the unmanaged raw pointer of [c_void]
/// of the Value field in [FABRIC_HEALTH_REPORT].
pub(crate) struct FabricHealthReportWrapper {
    /// The inner [FABRIC_HEALTH_REPORT] wrapped by FabricHealthReportWrapper.
    /// The instance of [FABRIC_HEALTH_REPORT] is created by converting the HealthReport to FabricHealthReportWrapper.
    /// `Kind` is a primitive type and does not require any memory management.
    /// `Value` field is a raw pointer to the actual health report and its ownership is managed by the owner field.
    pub inner: Option<FABRIC_HEALTH_REPORT>,
    /// A pointer to the owner of the memory allocated for the health report.
    /// This is used to free the memory when the wrapper is dropped.
    owner: *mut c_void,
}

impl Drop for FabricHealthReportWrapper {
    /// Free the memory allocated by FabricHealthReportWrapper to initialize the raw pointer of [FABRIC_HEALTH_REPORT].
    fn drop(&mut self) {
        // We only need to free the memory pointed by the owner, while using the Kind value to determine which type of health report it is.
        if let Some(inner) = self.inner.take() {
            match inner.Kind {
                FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA => {
                    let _ = unsafe {
                        Box::<FabricStatefulServiceReplicaHealthReportWrapper>::from_raw(
                            self.owner as *mut _,
                        )
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE => {
                    let _ = unsafe {
                        Box::<FabricStatelessServiceInstanceHealthReportWrapper>::from_raw(
                            self.owner as *mut _,
                        )
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_PARTITION => {
                    let _ = unsafe {
                        Box::<FabricPartitionHealthReportWrapper>::from_raw(self.owner as *mut _)
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_NODE => {
                    let _ = unsafe {
                        Box::<FabricNodeHealthReportWrapper>::from_raw(self.owner as *mut _)
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_SERVICE => {
                    let _ = unsafe {
                        Box::<FabricServiceHealthReportWrapper>::from_raw(self.owner as *mut _)
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_APPLICATION => {
                    let _ = unsafe {
                        Box::<FabricApplicationHealthReportWrapper>::from_raw(self.owner as *mut _)
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION => {
                    let _ = unsafe {
                        Box::<FabricDeployedApplicationHealthReportWrapper>::from_raw(
                            self.owner as *mut _,
                        )
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE => {
                    let _ = unsafe {
                        Box::<FabricDeployedServicePackageHealthReportWrapper>::from_raw(
                            self.owner as *mut _,
                        )
                    };
                }
                FABRIC_HEALTH_REPORT_KIND_CLUSTER => {
                    let _ = unsafe {
                        Box::<FabricClusterHealthReportWrapper>::from_raw(self.owner as *mut _)
                    };
                }
                _ => {
                    // Do nothing for INVALID or other kind as they did not allocate memory
                }
            }
        }
    }
}

impl From<&HealthReport> for FabricHealthReportWrapper {
    fn from(value: &HealthReport) -> Self {
        match value {
            HealthReport::Invalid => Self {
                inner: Some(FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_INVALID,
                    Value: std::ptr::null_mut(),
                }),
                owner: std::ptr::null_mut(),
            },
            HealthReport::StatefulServiceReplica(report) => {
                let boxed_report = Box::new(FabricStatefulServiceReplicaHealthReportWrapper::from(
                    report,
                ));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void, // Save the owner pointer to free the memory later
                }
            }
            HealthReport::StatelessServiceInstance(report) => {
                let boxed_report = Box::new(
                    FabricStatelessServiceInstanceHealthReportWrapper::from(report),
                );
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::Partition(report) => {
                let boxed_report = Box::new(FabricPartitionHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_PARTITION,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_PARTITION_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::Node(report) => {
                let boxed_report = Box::new(FabricNodeHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_NODE,
                        Value: &(boxed_report.inner.unwrap()) as *const FABRIC_NODE_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::Service(report) => {
                let boxed_report = Box::new(FabricServiceHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_SERVICE,
                        Value: &(boxed_report.inner.unwrap()) as *const FABRIC_SERVICE_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::Application(report) => {
                let boxed_report = Box::new(FabricApplicationHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_APPLICATION,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_APPLICATION_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::DeployedApplication(report) => {
                let boxed_report =
                    Box::new(FabricDeployedApplicationHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::DeployedServicePackage(report) => {
                let boxed_report = Box::new(FabricDeployedServicePackageHealthReportWrapper::from(
                    report,
                ));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE,
                        Value: &(boxed_report.inner.unwrap())
                            as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
            HealthReport::Cluster(report) => {
                let boxed_report = Box::new(FabricClusterHealthReportWrapper::from(report));
                Self {
                    inner: Some(FABRIC_HEALTH_REPORT {
                        Kind: FABRIC_HEALTH_REPORT_KIND_CLUSTER,
                        Value: &(boxed_report.inner.unwrap()) as *const FABRIC_CLUSTER_HEALTH_REPORT
                            as *mut c_void,
                    }),
                    owner: Box::into_raw(boxed_report) as *mut c_void,
                }
            }
        }
    }
}

impl From<&FABRIC_HEALTH_REPORT> for HealthReport {
    fn from(value: &FABRIC_HEALTH_REPORT) -> Self {
        // The following code:
        //     let report = match {...};
        //     report
        // is a workaround prevent access violation on Windows.
        // TODO: Investigate the root cause and remove the workaround.
        let report = match value.Kind {
            FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA => {
                let report = value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT;
                HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport::from(
                    unsafe { report.as_ref().unwrap() },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE => {
                let report = value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT;
                HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport::from(
                    unsafe { report.as_ref().unwrap() },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_PARTITION => {
                let report = value.Value as *const FABRIC_PARTITION_HEALTH_REPORT;
                HealthReport::Partition(PartitionHealthReport::from(unsafe {
                    report.as_ref().unwrap()
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_NODE => {
                let report = value.Value as *const FABRIC_NODE_HEALTH_REPORT;
                HealthReport::Node(NodeHealthReport::from(unsafe { report.as_ref().unwrap() }))
            }
            FABRIC_HEALTH_REPORT_KIND_SERVICE => {
                let report = value.Value as *const FABRIC_SERVICE_HEALTH_REPORT;
                HealthReport::Service(ServiceHealthReport::from(unsafe {
                    report.as_ref().unwrap()
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_APPLICATION => {
                let report = value.Value as *const FABRIC_APPLICATION_HEALTH_REPORT;
                HealthReport::Application(ApplicationHealthReport::from(unsafe {
                    report.as_ref().unwrap()
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION => {
                let report = value.Value as *const FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT;
                HealthReport::DeployedApplication(DeployedApplicationHealthReport::from(unsafe {
                    report.as_ref().unwrap()
                }))
            }
            FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE => {
                let report = value.Value as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT;
                HealthReport::DeployedServicePackage(DeployedServicePackageHealthReport::from(
                    unsafe { report.as_ref().unwrap() },
                ))
            }
            FABRIC_HEALTH_REPORT_KIND_CLUSTER => {
                let report = value.Value as *const FABRIC_CLUSTER_HEALTH_REPORT;
                HealthReport::Cluster(ClusterHealthReport::from(unsafe {
                    report.as_ref().unwrap()
                }))
            }
            _ => HealthReport::Invalid,
        };
        report
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

/// An intermediate representation of [FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT].
pub(crate) struct FabricStatefulServiceReplicaHealthReportWrapper {
    pub inner: Option<FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT>,
}

impl Drop for FabricStatefulServiceReplicaHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&StatefulServiceReplicaHealthReport> for FabricStatefulServiceReplicaHealthReportWrapper {
    fn from(value: &StatefulServiceReplicaHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT {
                PartitionId: value.partition_id,
                ReplicaId: value.replica_id,
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT> for StatefulServiceReplicaHealthReport {
    fn from(value: &FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        StatefulServiceReplicaHealthReport {
            partition_id: value.PartitionId,
            replica_id: value.ReplicaId,
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT].
pub(crate) struct FabricStatelessServiceInstanceHealthReportWrapper {
    pub inner: Option<FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT>,
}

impl Drop for FabricStatelessServiceInstanceHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&StatelessServiceInstanceHealthReport>
    for FabricStatelessServiceInstanceHealthReportWrapper
{
    fn from(value: &StatelessServiceInstanceHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT {
                PartitionId: value.partition_id,
                InstanceId: value.instance_id,
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT>
    for StatelessServiceInstanceHealthReport
{
    fn from(value: &FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        StatelessServiceInstanceHealthReport {
            partition_id: value.PartitionId,
            instance_id: value.InstanceId,
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_PARTITION_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_PARTITION_HEALTH_REPORT].
pub(crate) struct FabricPartitionHealthReportWrapper {
    pub inner: Option<FABRIC_PARTITION_HEALTH_REPORT>,
}

impl Drop for FabricPartitionHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&PartitionHealthReport> for FabricPartitionHealthReportWrapper {
    fn from(value: &PartitionHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_PARTITION_HEALTH_REPORT {
                PartitionId: value.partition_id,
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_PARTITION_HEALTH_REPORT> for PartitionHealthReport {
    fn from(value: &FABRIC_PARTITION_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        PartitionHealthReport {
            partition_id: value.PartitionId,
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_NODE_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_NODE_HEALTH_REPORT].
pub(crate) struct FabricNodeHealthReportWrapper {
    pub inner: Option<FABRIC_NODE_HEALTH_REPORT>,
}

impl Drop for FabricNodeHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&NodeHealthReport> for FabricNodeHealthReportWrapper {
    fn from(value: &NodeHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_NODE_HEALTH_REPORT {
                NodeName: value.node_name.as_pcwstr(),
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_NODE_HEALTH_REPORT> for NodeHealthReport {
    fn from(value: &FABRIC_NODE_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        NodeHealthReport {
            node_name: WStringWrap::from(value.NodeName).into(),
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_SERVICE_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_SERVICE_HEALTH_REPORT].
pub(crate) struct FabricServiceHealthReportWrapper {
    pub inner: Option<FABRIC_SERVICE_HEALTH_REPORT>,
}

impl Drop for FabricServiceHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&ServiceHealthReport> for FabricServiceHealthReportWrapper {
    fn from(value: &ServiceHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_SERVICE_HEALTH_REPORT {
                ServiceName: FABRIC_URI(value.service_name.as_ptr() as *mut u16),
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_SERVICE_HEALTH_REPORT> for ServiceHealthReport {
    fn from(value: &FABRIC_SERVICE_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        ServiceHealthReport {
            service_name: WStringWrap::from(PCWSTR::from_raw(value.ServiceName.0)).into(),
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_APPLICATION_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_APPLICATION_HEALTH_REPORT].
pub(crate) struct FabricApplicationHealthReportWrapper {
    pub inner: Option<FABRIC_APPLICATION_HEALTH_REPORT>,
}

impl Drop for FabricApplicationHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&ApplicationHealthReport> for FabricApplicationHealthReportWrapper {
    fn from(value: &ApplicationHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_APPLICATION_HEALTH_REPORT {
                ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_APPLICATION_HEALTH_REPORT> for ApplicationHealthReport {
    fn from(value: &FABRIC_APPLICATION_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        ApplicationHealthReport {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT].
pub(crate) struct FabricDeployedApplicationHealthReportWrapper {
    pub inner: Option<FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT>,
}

impl Drop for FabricDeployedApplicationHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&DeployedApplicationHealthReport> for FabricDeployedApplicationHealthReportWrapper {
    fn from(value: &DeployedApplicationHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT {
                ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
                NodeName: value.node_name.as_pcwstr(),
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT> for DeployedApplicationHealthReport {
    fn from(value: &FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        DeployedApplicationHealthReport {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT].
pub(crate) struct FabricDeployedServicePackageHealthReportWrapper {
    pub inner: Option<FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT>,
}

impl Drop for FabricDeployedServicePackageHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&DeployedServicePackageHealthReport> for FabricDeployedServicePackageHealthReportWrapper {
    fn from(value: &DeployedServicePackageHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT {
                ApplicationName: FABRIC_URI(value.application_name.as_ptr() as *mut u16),
                ServiceManifestName: value.service_manifest_name.as_pcwstr(),
                NodeName: value.node_name.as_pcwstr(),
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT> for DeployedServicePackageHealthReport {
    fn from(value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        DeployedServicePackageHealthReport {
            application_name: WStringWrap::from(PCWSTR::from_raw(value.ApplicationName.0)).into(),
            service_manifest_name: WStringWrap::from(value.ServiceManifestName).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            health_information: HealthInformation::from(health_info),
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

/// An intermediate representation of [FABRIC_CLUSTER_HEALTH_REPORT] to workaround the
/// unmanaged raw pointer to [FABRIC_HEALTH_INFORMATION] in [FABRIC_CLUSTER_HEALTH_REPORT].
pub(crate) struct FabricClusterHealthReportWrapper {
    pub inner: Option<FABRIC_CLUSTER_HEALTH_REPORT>,
}

impl Drop for FabricClusterHealthReportWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            unsafe {
                let _boxed_val =
                    Box::<FABRIC_HEALTH_INFORMATION>::from_raw(inner.HealthInformation as *mut _);
            }
        }
    }
}

impl From<&ClusterHealthReport> for FabricClusterHealthReportWrapper {
    fn from(value: &ClusterHealthReport) -> Self {
        let boxed_health_info =
            Box::new(FABRIC_HEALTH_INFORMATION::from(&value.health_information));
        Self {
            inner: Some(FABRIC_CLUSTER_HEALTH_REPORT {
                HealthInformation: Box::into_raw(boxed_health_info),
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
}

impl From<&FABRIC_CLUSTER_HEALTH_REPORT> for ClusterHealthReport {
    fn from(value: &FABRIC_CLUSTER_HEALTH_REPORT) -> Self {
        let health_info = unsafe { value.HealthInformation.as_ref().unwrap() };
        ClusterHealthReport {
            health_information: HealthInformation::from(health_info),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_health_report_conversion() {
        let health_report = HealthReport::Invalid;
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        matches!(health_report2, HealthReport::Invalid);
    }

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
        let health_report = HealthReport::Cluster(ClusterHealthReport {
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::Cluster(cluster_health_report) = health_report2 else {
            panic!("Expected ClusterHealthReport, but got {:?}", health_report2);
        };

        assert_eq!(
            cluster_health_report.health_information.source_id,
            health_info.source_id
        );
        assert_eq!(
            cluster_health_report.health_information.property,
            health_info.property
        );
        assert_eq!(
            cluster_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            cluster_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            cluster_health_report.health_information.description,
            health_info.description
        );
        assert_eq!(
            cluster_health_report.health_information.sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            cluster_health_report.health_information.remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report =
            HealthReport::DeployedServicePackage(DeployedServicePackageHealthReport {
                application_name: "fabric:/MyApp".into(),
                service_manifest_name: "manifest_name".into(),
                node_name: "node_name".into(),
                health_information: health_info.clone(),
            });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::DeployedServicePackage(deployed_service_package_health_report) =
            health_report2
        else {
            panic!(
                "Expected DeployedServicePackageHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(
            deployed_service_package_health_report.application_name,
            "fabric:/MyApp".into()
        );
        assert_eq!(
            deployed_service_package_health_report.service_manifest_name,
            "manifest_name".into()
        );
        assert_eq!(
            deployed_service_package_health_report.node_name,
            "node_name".into()
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .source_id,
            health_info.source_id
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .property,
            health_info.property
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .state,
            health_info.state
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .description,
            health_info.description
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report = HealthReport::Application(ApplicationHealthReport {
            application_name: "fabric:/MyApp".into(),
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::Application(application_health_report) = health_report2 else {
            panic!(
                "Expected ApplicationHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(
            application_health_report.application_name,
            "fabric:/MyApp".into()
        );
        assert_eq!(
            application_health_report.health_information.source_id,
            health_info.source_id
        );
        assert_eq!(
            application_health_report.health_information.property,
            health_info.property
        );
        assert_eq!(
            application_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            application_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            application_health_report.health_information.description,
            health_info.description
        );
        assert_eq!(
            application_health_report.health_information.sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            application_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report = HealthReport::DeployedApplication(DeployedApplicationHealthReport {
            application_name: "fabric:/MyApp".into(),
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::DeployedApplication(deployed_application_health_report) = health_report2
        else {
            panic!(
                "Expected DeployedApplicationHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(
            deployed_application_health_report.application_name,
            "fabric:/MyApp".into()
        );
        assert_eq!(
            deployed_application_health_report.node_name,
            "node_name".into()
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .source_id,
            health_info.source_id
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .property,
            health_info.property
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            deployed_application_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .description,
            health_info.description
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report = HealthReport::Service(ServiceHealthReport {
            service_name: "fabric:/MyService".into(),
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::Service(service_health_report) = health_report2 else {
            panic!("Expected ServiceHealthReport, but got {:?}", health_report2);
        };

        assert_eq!(
            service_health_report.service_name,
            "fabric:/MyService".into()
        );
        assert_eq!(
            service_health_report.health_information.source_id,
            health_info.source_id
        );
        assert_eq!(
            service_health_report.health_information.property,
            health_info.property
        );
        assert_eq!(
            service_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            service_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            service_health_report.health_information.description,
            health_info.description
        );
        assert_eq!(
            service_health_report.health_information.sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            service_health_report.health_information.remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report = HealthReport::Node(NodeHealthReport {
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::Node(node_health_report) = health_report2 else {
            panic!("Expected NodeHealthReport, but got {:?}", health_report2);
        };

        assert_eq!(node_health_report.node_name, "node_name".into());
        assert_eq!(
            node_health_report.health_information.source_id,
            health_info.source_id
        );
        assert_eq!(
            node_health_report.health_information.property,
            health_info.property
        );
        assert_eq!(
            node_health_report.health_information.time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            node_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            node_health_report.health_information.description,
            health_info.description
        );
        assert_eq!(
            node_health_report.health_information.sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            node_health_report.health_information.remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report = HealthReport::Partition(PartitionHealthReport {
            partition_id: GUID::from_u128(54321),
            health_information: health_info.clone(),
        });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::Partition(partition_health_report) = health_report2 else {
            panic!(
                "Expected PartitionHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(partition_health_report.partition_id, GUID::from_u128(54321));
        assert_eq!(
            partition_health_report.health_information.source_id,
            health_info.source_id
        );
        assert_eq!(
            partition_health_report.health_information.property,
            health_info.property
        );
        assert_eq!(
            partition_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            partition_health_report.health_information.state,
            health_info.state
        );
        assert_eq!(
            partition_health_report.health_information.description,
            health_info.description
        );
        assert_eq!(
            partition_health_report.health_information.sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            partition_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report =
            HealthReport::StatelessServiceInstance(StatelessServiceInstanceHealthReport {
                partition_id: GUID::from_u128(54321),
                instance_id: 1,
                health_information: health_info.clone(),
            });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::StatelessServiceInstance(stateless_service_instance_health_report) =
            health_report2
        else {
            panic!(
                "Expected StatelessServiceInstanceHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(
            stateless_service_instance_health_report.partition_id,
            GUID::from_u128(54321)
        );
        assert_eq!(stateless_service_instance_health_report.instance_id, 1);
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .source_id,
            health_info.source_id
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .property,
            health_info.property
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .state,
            health_info.state
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .description,
            health_info.description
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
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
        let health_report =
            HealthReport::StatefulServiceReplica(StatefulServiceReplicaHealthReport {
                partition_id: GUID::from_u128(12345),
                replica_id: 1,
                health_information: health_info.clone(),
            });
        let wrapper = FabricHealthReportWrapper::from(&health_report);
        let health_report2 = HealthReport::from(wrapper.inner.as_ref().unwrap());

        let HealthReport::StatefulServiceReplica(stateful_service_replica_health_report) =
            health_report2
        else {
            panic!(
                "Expected StatefulServiceReplicaHealthReport, but got {:?}",
                health_report2
            );
        };

        assert_eq!(
            stateful_service_replica_health_report.partition_id,
            GUID::from_u128(12345),
        );
        assert_eq!(stateful_service_replica_health_report.replica_id, 1);
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .source_id,
            health_info.source_id
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .property,
            health_info.property
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .time_to_live_seconds,
            health_info.time_to_live_seconds
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .state,
            health_info.state
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .description,
            health_info.description
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .sequence_number,
            health_info.sequence_number
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .remove_when_expired,
            health_info.remove_when_expired
        );
    }

    #[test]
    fn test_cluster_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };
        let cluster_health_report = ClusterHealthReport {
            health_information: health_info.clone(),
        };
        let wrapper = FabricClusterHealthReportWrapper::from(&cluster_health_report);
        let cluster_health_report2 = ClusterHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            cluster_health_report.health_information.source_id,
            cluster_health_report2.health_information.source_id
        );
        assert_eq!(
            cluster_health_report.health_information.property,
            cluster_health_report2.health_information.property
        );
        assert_eq!(
            cluster_health_report
                .health_information
                .time_to_live_seconds,
            cluster_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            cluster_health_report.health_information.state,
            cluster_health_report2.health_information.state
        );
        assert_eq!(
            cluster_health_report.health_information.description,
            cluster_health_report2.health_information.description
        );
        assert_eq!(
            cluster_health_report.health_information.sequence_number,
            cluster_health_report2.health_information.sequence_number
        );
        assert_eq!(
            cluster_health_report.health_information.remove_when_expired,
            cluster_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_deployed_service_package_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let deployed_service_package_health_report = DeployedServicePackageHealthReport {
            application_name: "fabric:/MyApp".into(),
            service_manifest_name: "manifest_name".into(),
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        };
        let wrapper = FabricDeployedServicePackageHealthReportWrapper::from(
            &deployed_service_package_health_report,
        );
        let deployed_service_package_health_report2 =
            DeployedServicePackageHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            deployed_service_package_health_report.application_name,
            deployed_service_package_health_report2.application_name
        );
        assert_eq!(
            deployed_service_package_health_report.service_manifest_name,
            deployed_service_package_health_report2.service_manifest_name
        );
        assert_eq!(
            deployed_service_package_health_report.node_name,
            deployed_service_package_health_report2.node_name
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .source_id,
            deployed_service_package_health_report2
                .health_information
                .source_id
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .property,
            deployed_service_package_health_report2
                .health_information
                .property
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .time_to_live_seconds,
            deployed_service_package_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .state,
            deployed_service_package_health_report2
                .health_information
                .state
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .description,
            deployed_service_package_health_report2
                .health_information
                .description
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .sequence_number,
            deployed_service_package_health_report2
                .health_information
                .sequence_number
        );
        assert_eq!(
            deployed_service_package_health_report
                .health_information
                .remove_when_expired,
            deployed_service_package_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_deployed_application_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let deployed_application_health_report = DeployedApplicationHealthReport {
            application_name: "fabric:/MyApp".into(),
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        };
        let wrapper =
            FabricDeployedApplicationHealthReportWrapper::from(&deployed_application_health_report);
        let deployed_application_health_report2 =
            DeployedApplicationHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            deployed_application_health_report.application_name,
            deployed_application_health_report2.application_name
        );
        assert_eq!(
            deployed_application_health_report.node_name,
            deployed_application_health_report2.node_name
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .source_id,
            deployed_application_health_report2
                .health_information
                .source_id
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .property,
            deployed_application_health_report2
                .health_information
                .property
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .time_to_live_seconds,
            deployed_application_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            deployed_application_health_report.health_information.state,
            deployed_application_health_report2.health_information.state
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .description,
            deployed_application_health_report2
                .health_information
                .description
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .sequence_number,
            deployed_application_health_report2
                .health_information
                .sequence_number
        );
        assert_eq!(
            deployed_application_health_report
                .health_information
                .remove_when_expired,
            deployed_application_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_application_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let application_health_report = ApplicationHealthReport {
            application_name: "fabric:/MyApp".into(),
            health_information: health_info.clone(),
        };
        let wrapper = FabricApplicationHealthReportWrapper::from(&application_health_report);
        let application_health_report2 =
            ApplicationHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            application_health_report.application_name,
            application_health_report2.application_name
        );
        assert_eq!(
            application_health_report.health_information.source_id,
            application_health_report2.health_information.source_id
        );
        assert_eq!(
            application_health_report.health_information.property,
            application_health_report2.health_information.property
        );
        assert_eq!(
            application_health_report
                .health_information
                .time_to_live_seconds,
            application_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            application_health_report.health_information.state,
            application_health_report2.health_information.state
        );
        assert_eq!(
            application_health_report.health_information.description,
            application_health_report2.health_information.description
        );
        assert_eq!(
            application_health_report.health_information.sequence_number,
            application_health_report2
                .health_information
                .sequence_number
        );
        assert_eq!(
            application_health_report
                .health_information
                .remove_when_expired,
            application_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_service_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let service_health_report = ServiceHealthReport {
            service_name: "fabric:/MyService".into(),
            health_information: health_info.clone(),
        };
        let wrapper = FabricServiceHealthReportWrapper::from(&service_health_report);
        let service_health_report2 = ServiceHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            service_health_report.service_name,
            service_health_report2.service_name
        );
        assert_eq!(
            service_health_report.health_information.source_id,
            service_health_report2.health_information.source_id
        );
        assert_eq!(
            service_health_report.health_information.property,
            service_health_report2.health_information.property
        );
        assert_eq!(
            service_health_report
                .health_information
                .time_to_live_seconds,
            service_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            service_health_report.health_information.state,
            service_health_report2.health_information.state
        );
        assert_eq!(
            service_health_report.health_information.description,
            service_health_report2.health_information.description
        );
        assert_eq!(
            service_health_report.health_information.sequence_number,
            service_health_report2.health_information.sequence_number
        );
        assert_eq!(
            service_health_report.health_information.remove_when_expired,
            service_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_node_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let node_health_report = NodeHealthReport {
            node_name: "node_name".into(),
            health_information: health_info.clone(),
        };
        let wrapper = FabricNodeHealthReportWrapper::from(&node_health_report);
        let node_health_report2 = NodeHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(node_health_report.node_name, node_health_report2.node_name);
        assert_eq!(
            node_health_report.health_information.source_id,
            node_health_report2.health_information.source_id
        );
        assert_eq!(
            node_health_report.health_information.property,
            node_health_report2.health_information.property
        );
        assert_eq!(
            node_health_report.health_information.time_to_live_seconds,
            node_health_report2.health_information.time_to_live_seconds
        );
        assert_eq!(
            node_health_report.health_information.state,
            node_health_report2.health_information.state
        );
        assert_eq!(
            node_health_report.health_information.description,
            node_health_report2.health_information.description
        );
        assert_eq!(
            node_health_report.health_information.sequence_number,
            node_health_report2.health_information.sequence_number
        );
        assert_eq!(
            node_health_report.health_information.remove_when_expired,
            node_health_report2.health_information.remove_when_expired
        );
    }

    #[test]
    fn test_partition_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let partition_health_report = PartitionHealthReport {
            partition_id: GUID::from_u128(12345),
            health_information: health_info.clone(),
        };
        let wrapper = FabricPartitionHealthReportWrapper::from(&partition_health_report);
        let partition_health_report2 = PartitionHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            partition_health_report.partition_id,
            partition_health_report2.partition_id
        );
        assert_eq!(
            partition_health_report.health_information.source_id,
            partition_health_report2.health_information.source_id
        );
        assert_eq!(
            partition_health_report.health_information.property,
            partition_health_report2.health_information.property
        );
        assert_eq!(
            partition_health_report
                .health_information
                .time_to_live_seconds,
            partition_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            partition_health_report.health_information.state,
            partition_health_report2.health_information.state
        );
        assert_eq!(
            partition_health_report.health_information.description,
            partition_health_report2.health_information.description
        );
        assert_eq!(
            partition_health_report.health_information.sequence_number,
            partition_health_report2.health_information.sequence_number
        );
        assert_eq!(
            partition_health_report
                .health_information
                .remove_when_expired,
            partition_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_stateless_service_instance_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let stateless_service_instance_health_report = StatelessServiceInstanceHealthReport {
            partition_id: GUID::from_u128(12345),
            instance_id: 1,
            health_information: health_info.clone(),
        };
        let wrapper = FabricStatelessServiceInstanceHealthReportWrapper::from(
            &stateless_service_instance_health_report,
        );
        let stateless_service_instance_health_report2 =
            StatelessServiceInstanceHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            stateless_service_instance_health_report.partition_id,
            stateless_service_instance_health_report2.partition_id
        );
        assert_eq!(
            stateless_service_instance_health_report.instance_id,
            stateless_service_instance_health_report2.instance_id
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .source_id,
            stateless_service_instance_health_report2
                .health_information
                .source_id
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .property,
            stateless_service_instance_health_report2
                .health_information
                .property
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .time_to_live_seconds,
            stateless_service_instance_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .state,
            stateless_service_instance_health_report2
                .health_information
                .state
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .description,
            stateless_service_instance_health_report2
                .health_information
                .description
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .sequence_number,
            stateless_service_instance_health_report2
                .health_information
                .sequence_number
        );
        assert_eq!(
            stateless_service_instance_health_report
                .health_information
                .remove_when_expired,
            stateless_service_instance_health_report2
                .health_information
                .remove_when_expired
        );
    }

    #[test]
    fn test_stateful_service_replica_health_report_wrapper_conversion() {
        let health_info = HealthInformation {
            source_id: "source_id".into(),
            property: "property".into(),
            time_to_live_seconds: 10,
            state: crate::types::HealthState::Ok,
            description: "description".into(),
            sequence_number: 1,
            remove_when_expired: false,
        };

        let stateful_service_replica_health_report = StatefulServiceReplicaHealthReport {
            partition_id: GUID::from_u128(12345),
            replica_id: 1,
            health_information: health_info.clone(),
        };
        let wrapper = FabricStatefulServiceReplicaHealthReportWrapper::from(
            &stateful_service_replica_health_report,
        );
        let stateful_service_replica_health_report2 =
            StatefulServiceReplicaHealthReport::from(wrapper.inner.as_ref().unwrap());

        assert_eq!(
            stateful_service_replica_health_report.partition_id,
            stateful_service_replica_health_report2.partition_id
        );
        assert_eq!(
            stateful_service_replica_health_report.replica_id,
            stateful_service_replica_health_report2.replica_id
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .source_id,
            stateful_service_replica_health_report2
                .health_information
                .source_id
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .property,
            stateful_service_replica_health_report2
                .health_information
                .property
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .time_to_live_seconds,
            stateful_service_replica_health_report2
                .health_information
                .time_to_live_seconds
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .state,
            stateful_service_replica_health_report2
                .health_information
                .state
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .description,
            stateful_service_replica_health_report2
                .health_information
                .description
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .sequence_number,
            stateful_service_replica_health_report2
                .health_information
                .sequence_number
        );
        assert_eq!(
            stateful_service_replica_health_report
                .health_information
                .remove_when_expired,
            stateful_service_replica_health_report2
                .health_information
                .remove_when_expired
        );
    }
}
