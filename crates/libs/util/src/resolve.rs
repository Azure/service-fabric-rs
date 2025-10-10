// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::retry::OperationRetryer;
use mssf_core::{
    ErrorCode,
    client::{
        FabricClient,
        svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition, ServiceManagementClient},
    },
    runtime::executor::BoxedCancelToken,
    types::Uri,
};
use std::time::Duration;

/// The same as dotnet sdk:
/// https://github.com/microsoft/service-fabric-services-and-actors-dotnet/blob/develop/src/Microsoft.ServiceFabric.Services/Client/ServicePartitionResolver.cs
/// But this does not register notification on resolve success.
/// User needs to register notification manually on the FabricClient before creating this resolver.
pub struct ServicePartitionResolver {
    sm: ServiceManagementClient,
    retryer: OperationRetryer,
}

impl ServicePartitionResolver {
    pub fn new(fc: FabricClient, retryer: OperationRetryer) -> Self {
        ServicePartitionResolver {
            sm: fc.get_service_manager().clone(),
            retryer,
        }
    }

    /// Resolve the service partition by name and key type.
    /// It retries all transient errors and timeouts.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, fields(uri = %name, timeout = ?timeout), err)
    )]
    pub async fn resolve(
        &self,
        name: &Uri,
        key_type: &PartitionKeyType,
        prev: Option<&ResolvedServicePartition>,
        timeout: Option<Duration>, // Total timeout for the operation
        token: Option<BoxedCancelToken>,
    ) -> mssf_core::Result<ResolvedServicePartition> {
        // tracing span is auto propagated in async context
        self.retryer
            .run(
                async |t, tk| {
                    let rsp = self
                        .sm
                        .resolve_service_partition(name, key_type, prev, t, tk)
                        .await?;

                    // Check rsp is valid and save in the cache.
                    // Sometimes endpoint is empty (may due to service removed), so we need to retry.
                    if !rsp.endpoints.is_empty() {
                        Ok(rsp)
                    } else {
                        Err(ErrorCode::FABRIC_E_SERVICE_OFFLINE.into())
                    }
                },
                timeout,
                token,
            )
            .await
    }
}
