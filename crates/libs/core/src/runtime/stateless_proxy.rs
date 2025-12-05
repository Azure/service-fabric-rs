// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::types::{
    FaultType, HealthInformation, LoadMetric, LoadMetricListRef, MoveCost,
    ServicePartitionInformation,
};
use mssf_com::FabricRuntime::{
    IFabricStatelessServicePartition, IFabricStatelessServicePartition3,
};
use windows_core::Interface;
// wrap of com interface
#[derive(Debug, Clone)]
pub struct StatelessServicePartition {
    com_impl: IFabricStatelessServicePartition3,
}

impl StatelessServicePartition {
    pub fn new(com_impl: IFabricStatelessServicePartition) -> StatelessServicePartition {
        StatelessServicePartition {
            com_impl: com_impl.cast().unwrap(),
        } // cast to the newer version.
    }

    pub fn get_com(&self) -> &IFabricStatelessServicePartition3 {
        &self.com_impl
    }
}

impl crate::runtime::IStatelessServicePartition for StatelessServicePartition {
    fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        Ok(raw_ref.into())
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

    fn report_partition_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportPartitionHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }

    fn report_instance_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportInstanceHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }
}
