// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricHealthClient4;

use crate::types::{FabricHealthReportWrapper, HealthReport};

/// Provides functionality to perform health related operations, like report and query health.
/// See C# API [here](https://docs.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.healthclient?view=azure-dotnet).
///
/// TODO: Implement full functionality of the HealthClient.
#[derive(Debug, Clone)]
pub struct HealthClient {
    com: IFabricHealthClient4,
}

// Public implementation block
impl HealthClient {
    pub fn from_com(com: IFabricHealthClient4) -> Self {
        Self { com: com.clone() }
    }

    /// Reports health on a Service Fabric entity. See C# API [here](https://docs.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.healthclient.reporthealth?view=azure-dotnet).
    ///
    /// Remarks:
    /// When a cluster is secured, the health client needs administrator permission to be able to send the reports.
    /// Read more about [connecting to a cluster using the FabricClient APIs](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-connect-to-secure-cluster).
    /// For more information about health reporting, see [Service Fabric health monitoring](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-health-introduction).
    pub fn report_health(&self, health_report: &HealthReport) -> windows_core::Result<()> {
        // The following call will construct a FabricHealthReportWrapper from the HealthReport.
        // It does a few heap memory allocations, but it is necessary to pass the health_report to the COM API.
        // The memory allocations are then freed by the FabricHealthReportWrapper when it goes out of scope.
        let health_report_wrapper = FabricHealthReportWrapper::from(health_report);
        unsafe { self.com.ReportHealth(&health_report_wrapper.inner.unwrap()) }
    }
}
