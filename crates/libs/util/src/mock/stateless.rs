// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;

use mssf_core::{runtime::IStatelessServicePartition, types::ServicePartitionInformation};

/// Mock for IStatelessServicePartition
/// Currently does not react to any reports
pub struct StatelessServicePartitionMock {
    info: ServicePartitionInformation,
}

impl StatelessServicePartitionMock {
    /// Create a new mock with given partition info
    pub fn new(info: ServicePartitionInformation) -> Self {
        Self { info }
    }
    pub fn new_arc(info: ServicePartitionInformation) -> Arc<dyn IStatelessServicePartition> {
        Arc::new(Self::new(info))
    }
}

impl IStatelessServicePartition for StatelessServicePartitionMock {
    fn get_partition_info(&self) -> mssf_core::Result<ServicePartitionInformation> {
        Ok(self.info.clone())
    }

    fn report_load(&self, _metrics: &[mssf_core::types::LoadMetric]) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_fault(&self, _fault_type: mssf_core::types::FaultType) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_move_cost(&self, _move_cost: mssf_core::types::MoveCost) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_partition_health(
        &self,
        _healthinfo: &mssf_core::types::HealthInformation,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    fn report_instance_health(
        &self,
        _health_info: &mssf_core::types::HealthInformation,
    ) -> mssf_core::Result<()> {
        Ok(())
    }
}
