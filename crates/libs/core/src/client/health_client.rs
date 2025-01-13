// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricClient::IFabricHealthClient4,
    FabricTypes::{
        FABRIC_APPLICATION_HEALTH_REPORT, FABRIC_CLUSTER_HEALTH_REPORT,
        FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT,
        FABRIC_HEALTH_INFORMATION, FABRIC_HEALTH_REPORT, FABRIC_HEALTH_REPORT_KIND_APPLICATION,
        FABRIC_HEALTH_REPORT_KIND_CLUSTER, FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
        FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE, FABRIC_HEALTH_REPORT_KIND_INVALID,
        FABRIC_HEALTH_REPORT_KIND_NODE, FABRIC_HEALTH_REPORT_KIND_PARTITION,
        FABRIC_HEALTH_REPORT_KIND_SERVICE, FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
        FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE, FABRIC_NODE_HEALTH_REPORT,
        FABRIC_PARTITION_HEALTH_REPORT, FABRIC_SERVICE_HEALTH_REPORT,
        FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT,
        FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT, FABRIC_URI,
    },
};

use crate::types::HealthReport;

/// Provides functionality to perform health related operations, like report and query health.
/// See C# API [here](https://docs.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.healthclient?view=azure-dotnet).
///
/// TODO: Implement full functionality of the HealthClient.
#[derive(Debug, Clone)]
pub struct HealthClient {
    com: IFabricHealthClient4,
}

impl From<IFabricHealthClient4> for HealthClient {
    fn from(value: IFabricHealthClient4) -> Self {
        Self { com: value }
    }
}

impl From<HealthClient> for IFabricHealthClient4 {
    fn from(value: HealthClient) -> Self {
        value.com
    }
}

// Public implementation block
impl HealthClient {
    /// Reports health on a Service Fabric entity. See C# API [here](https://docs.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.healthclient.reporthealth?view=azure-dotnet).
    ///
    /// Remarks:
    /// When a cluster is secured, the health client needs administrator permission to be able to send the reports.
    /// Read more about [connecting to a cluster using the FabricClient APIs](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-connect-to-secure-cluster).
    /// For more information about health reporting, see [Service Fabric health monitoring](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-health-introduction).
    pub fn report_health(&self, health_report: &HealthReport) -> windows_core::Result<()> {
        match health_report {
            HealthReport::Invalid => {
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_INVALID,
                    Value: std::ptr::null_mut(),
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::StatefulServiceReplica(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT {
                    PartitionId: health_report.partition_id,
                    ReplicaId: health_report.replica_id,
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_STATEFUL_SERVICE_REPLICA,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::StatelessServiceInstance(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT {
                    PartitionId: health_report.partition_id,
                    InstanceId: health_report.instance_id,
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_STATELESS_SERVICE_INSTANCE,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::Partition(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_PARTITION_HEALTH_REPORT {
                    PartitionId: health_report.partition_id,
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_PARTITION,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::Node(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_NODE_HEALTH_REPORT {
                    NodeName: health_report.node_name.as_pcwstr(),
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_NODE,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::Service(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_SERVICE_HEALTH_REPORT {
                    ServiceName: FABRIC_URI(health_report.service_name.as_ptr() as *mut u16),
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_SERVICE,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::Application(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_APPLICATION_HEALTH_REPORT {
                    ApplicationName: FABRIC_URI(health_report.application_name.as_ptr() as *mut u16),
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_APPLICATION,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::DeployedApplication(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT {
                    ApplicationName: FABRIC_URI(health_report.application_name.as_ptr() as *mut u16),
                    NodeName: health_report.node_name.as_pcwstr(),
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_APPLICATION,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::DeployedServicePackage(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT {
                    ApplicationName: FABRIC_URI(health_report.application_name.as_ptr() as *mut u16),
                    ServiceManifestName: health_report.service_manifest_name.as_pcwstr(),
                    NodeName: health_report.node_name.as_pcwstr(),
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_DEPLOYED_SERVICE_PACKAGE,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
            HealthReport::Cluster(health_report) => {
                let fabric_health_info =
                    FABRIC_HEALTH_INFORMATION::from(&health_report.health_information);
                let fabric_health_report_value = FABRIC_CLUSTER_HEALTH_REPORT {
                    HealthInformation: &fabric_health_info,
                    Reserved: std::ptr::null_mut(),
                };
                let fabric_health_report = FABRIC_HEALTH_REPORT {
                    Kind: FABRIC_HEALTH_REPORT_KIND_CLUSTER,
                    Value: &fabric_health_report_value as *const _ as *mut _,
                };
                unsafe { self.com.ReportHealth(&fabric_health_report) }
            }
        }
    }
}
