// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_com::{
    FabricClient::IFabricInfrastructureServiceClient, FabricCommon::IFabricStringResult,
};

use crate::{
    WString, runtime::executor::BoxedCancelToken, strings::StringResult,
    sync::fabric_begin_end_proxy, types::Uri,
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
    /// Invokes a query on one Infrastructure Service instance and returns the
    /// response payload without interpreting it.
    ///
    /// # Arguments
    ///
    /// * `service_name` - The Fabric URI of the Infrastructure Service instance
    ///   that will receive the query.
    /// * `command` - The query command passed to the Infrastructure Service.
    /// * `timeout` - The maximum duration for the operation in milliseconds
    /// * `cancellation_token` - An optional token for cancelling the pending
    ///   operation.
    pub async fn invoke_infrastructure_query(
        &self,
        service_name: &Uri,
        command: &WString,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<WString> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        let result: IFabricStringResult = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginInvokeInfrastructureQuery(
                    service_name.as_raw(),
                    command.as_pcwstr(),
                    timeout.as_millis().try_into()?,
                    callback,
                )
            },
            move |context| unsafe { com2.EndInvokeInfrastructureQuery(context) },
            cancellation_token,
        )
        .await??;
        Ok(StringResult::from(&result).into_inner())
    }
}
