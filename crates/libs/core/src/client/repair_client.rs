// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_com::FabricClient::{IFabricGetRepairTaskListResult, IFabricRepairManagementClient};

use crate::{
    mem::{BoxPool, GetRawWithBoxPool},
    runtime::executor::BoxedCancelToken,
    sync::{FabricReceiver, fabric_begin_end_proxy},
    types::{RepairTask, RepairTaskQueryDescription},
};

#[derive(Debug, Clone)]
pub struct RepairManagementClient {
    com: IFabricRepairManagementClient,
}

impl From<IFabricRepairManagementClient> for RepairManagementClient {
    fn from(value: IFabricRepairManagementClient) -> Self {
        Self { com: value }
    }
}

impl From<RepairManagementClient> for IFabricRepairManagementClient {
    fn from(value: RepairManagementClient) -> Self {
        value.com
    }
}

impl RepairManagementClient {
    fn get_repair_task_list_internal(
        &self,
        query: &RepairTaskQueryDescription,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::Result<IFabricGetRepairTaskListResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        let mut pool = BoxPool::new();
        let query = query.get_raw_with_pool(&mut pool);
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetRepairTaskList(&query, timeout_milliseconds, callback)
            },
            move |context| unsafe { com2.EndGetRepairTaskList(context) },
            cancellation_token,
        )
    }

    /// Returns Azure platform or tenant infrastructure tasks in all Repair Manager states.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The maximum duration for the operation in milliseconds.
    /// * `cancellation_token` - An optional token for cancelling the pending
    ///   operation.
    pub async fn get_repair_task_list(
        &self,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<Vec<RepairTask>> {
        let query = RepairTaskQueryDescription::all();
        let receiver = self.get_repair_task_list_internal(
            &query,
            timeout.as_millis().try_into()?,
            cancellation_token,
        );
        let result = receiver.await??;
        Ok(RepairTask::from_result(&result))
    }
}
