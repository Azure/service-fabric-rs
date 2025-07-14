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

    pub fn get_partition_info(&self) -> crate::Result<ServicePartitionInformation> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        Ok(raw_ref.into())
    }

    /// Reports load for the current replica in the partition.
    /// Remarks:
    /// The reported metrics should correspond to those that are provided in the ServiceLoadMetricDescription
    /// as a part of the ServiceDescription that is used to create the service. Load metrics that are not
    /// present in the description are ignored. Reporting custom metrics allows Service Fabric to balance
    /// services that are based on additional custom information.
    pub fn report_load(&self, metrics: &[LoadMetric]) -> crate::Result<()> {
        let metrics_ref = LoadMetricListRef::from_slice(metrics);
        let raw = metrics_ref.as_raw_slice();
        unsafe { self.com_impl.ReportLoad(raw) }.map_err(crate::Error::from)
    }

    /// Enables the replica to report a fault to the runtime and indicates that it has encountered
    /// an error from which it cannot recover and must either be restarted or removed.
    pub fn report_fault(&self, fault_type: FaultType) -> crate::Result<()> {
        unsafe { self.com_impl.ReportFault(fault_type.into()) }.map_err(crate::Error::from)
    }

    /// Reports the move cost for a replica.
    /// Remarks:
    /// Services can report move cost of a replica using this method.
    /// While the Service Fabric Resource Balances searches for the best balance in the cluster,
    /// it examines both load information and move cost of each replica.
    /// Resource balances will prefer to move replicas with lower cost in order to achieve balance.
    pub fn report_move_cost(&self, move_cost: MoveCost) -> crate::Result<()> {
        unsafe { self.com_impl.ReportMoveCost(move_cost.into()) }.map_err(crate::Error::from)
    }

    /// Reports current partition health.
    pub fn report_partition_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportPartitionHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }

    /// Reports health on the current stateless service instance of the partition.
    pub fn report_instance_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportInstanceHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }
}
