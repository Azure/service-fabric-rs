// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::ErrorCode;
use crate::types::{
    FaultType, HealthInformation, LoadMetric, LoadMetricListRef, MoveCost,
    SelfReconfiguringConfigurationReport, ServicePartitionInformation,
};
use mssf_com::FabricRuntime::IFabricSelfReconfiguringServicePartition;

/// Wrapper for the `IFabricSelfReconfiguringServicePartition` COM interface.
///
/// Unlike the stateless partition, this interface has no versioned successor, so
/// the base interface is wrapped directly.
#[derive(Debug, Clone)]
pub struct SelfReconfiguringServicePartition {
    com_impl: IFabricSelfReconfiguringServicePartition,
}

impl SelfReconfiguringServicePartition {
    pub fn new(
        com_impl: IFabricSelfReconfiguringServicePartition,
    ) -> SelfReconfiguringServicePartition {
        SelfReconfiguringServicePartition { com_impl }
    }

    pub fn get_com(&self) -> &IFabricSelfReconfiguringServicePartition {
        &self.com_impl
    }
}

impl crate::runtime::ISelfReconfiguringServicePartition for SelfReconfiguringServicePartition {
    fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation> {
        // Treat a null partition info pointer as an error rather than panicking,
        // following the stateful partition proxy pattern.
        unsafe { self.com_impl.GetPartitionInfo()?.as_ref() }
            .ok_or(ErrorCode::E_POINTER.into())
            .map(ServicePartitionInformation::from)
    }

    fn report_load(&self, metrics: &[LoadMetric]) -> crate::Result<()> {
        let metrics_ref = LoadMetricListRef::from_slice(metrics);
        let raw = metrics_ref.as_raw_slice();
        unsafe { self.com_impl.ReportLoad(raw) }.map_err(crate::Error::from)
    }

    fn report_fault(&self, fault_type: FaultType) -> crate::Result<()> {
        unsafe { self.com_impl.ReportFault(fault_type.into()) }.map_err(crate::Error::from)
    }

    fn report_move_cost(&self, move_cost: MoveCost) -> crate::Result<()> {
        unsafe { self.com_impl.ReportMoveCost(move_cost.into()) }.map_err(crate::Error::from)
    }

    fn report_instance_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe {
            self.com_impl
                .ReportInstanceHealth(healthinfo_ref, std::ptr::null())
        }
        .map_err(crate::Error::from)
    }

    fn report_partition_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe {
            self.com_impl
                .ReportPartitionHealth(healthinfo_ref, std::ptr::null())
        }
        .map_err(crate::Error::from)
    }

    fn report_configuration(
        &self,
        report: &SelfReconfiguringConfigurationReport,
    ) -> crate::Result<()> {
        let view = report.get_view();
        unsafe { self.com_impl.ReportConfiguration(view.get_raw()) }.map_err(crate::Error::from)
    }
}
