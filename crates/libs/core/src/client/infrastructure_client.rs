// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_com::{
    FabricClient::IFabricInfrastructureServiceClient, FabricCommon::IFabricStringResult,
    FabricTypes::FABRIC_URI,
};

use crate::{
    PCWSTR, WString,
    runtime::executor::BoxedCancelToken,
    strings::StringResult,
    sync::{FabricReceiver, fabric_begin_end_proxy},
    types::{InfrastructureUpgradeJob, Uri},
};

#[derive(Debug, Clone)]
pub struct InfrastructureServiceClient {
    com: IFabricInfrastructureServiceClient,
}

impl From<IFabricInfrastructureServiceClient> for InfrastructureServiceClient {
    fn from(value: IFabricInfrastructureServiceClient) -> Self {
        Self { com: value }
    }
}

impl From<InfrastructureServiceClient> for IFabricInfrastructureServiceClient {
    fn from(value: InfrastructureServiceClient) -> Self {
        value.com
    }
}

impl InfrastructureServiceClient {
    fn invoke_query_internal(
        &self,
        service_name: FABRIC_URI,
        command: PCWSTR,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::Result<IFabricStringResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginInvokeInfrastructureQuery(
                    service_name,
                    command,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |context| unsafe { com2.EndInvokeInfrastructureQuery(context) },
            cancellation_token,
        )
    }

    /// Queries one Azure Infrastructure Service instance and returns all known
    /// platform or tenant jobs from its current coordinator state.
    pub async fn get_upgrade_jobs(
        &self,
        service_name: &Uri,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<Vec<InfrastructureUpgradeJob>> {
        let command = WString::from("GetCurrentState");
        let result = self
            .invoke_query_internal(
                service_name.as_raw(),
                command.as_pcwstr(),
                timeout.as_millis().try_into()?,
                cancellation_token,
            )
            .await??;
        let response = StringResult::from(&result).into_inner().to_string_lossy();
        InfrastructureUpgradeJob::from_query_response(service_name, &response)
    }
}
