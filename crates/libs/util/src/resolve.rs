// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{pin::Pin, time::Duration};

use mssf_core::ErrorCode;
use mssf_core::runtime::executor::{BoxedCancelToken, Timer};
use mssf_core::types::Uri;

use mssf_core::client::{
    FabricClient,
    svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition, ServiceManagementClient},
};

/// The same as dotnet sdk:
/// https://github.com/microsoft/service-fabric-services-and-actors-dotnet/blob/develop/src/Microsoft.ServiceFabric.Services/Client/ServicePartitionResolver.cs
/// But this does not register notification on resolve success.
/// User needs to register notification manually on the FabricClient before creating this resolver.
pub struct ServicePartitionResolver {
    sm: ServiceManagementClient,
    timer: Box<dyn Timer>,
    default_timeout: Duration,
    max_retry_interval: Duration,
}

/// TimeCounter is used to track elapsed time and remaining time for operations.
struct TimeCounter {
    timeout: Duration,
    start: std::time::Instant,
}

impl TimeCounter {
    pub fn new(timeout: Duration) -> Self {
        TimeCounter {
            timeout,
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn remaining(&self) -> mssf_core::Result<Duration> {
        if self.elapsed() < self.timeout {
            Ok(self.timeout - self.elapsed())
        } else {
            Err(ErrorCode::FABRIC_E_TIMEOUT.into())
        }
    }

    /// returns a future that will sleep until the remaining time is up.
    pub fn sleep_until_remaining(
        &self,
        timer: &dyn Timer,
    ) -> mssf_core::Result<impl Future<Output = ()>> {
        let remaining = self.remaining()?;
        Ok(timer.sleep(remaining))
    }
}

pub struct ServicePartitionResolverBuilder {
    fc: FabricClient,
    timer: Option<Box<dyn Timer>>,
    default_timeout: Option<Duration>,
    default_max_retry_interval: Option<Duration>,
}

impl ServicePartitionResolverBuilder {
    pub fn new(fc: FabricClient) -> Self {
        ServicePartitionResolverBuilder {
            fc,
            timer: None,
            default_timeout: None,
            default_max_retry_interval: None,
        }
    }

    /// With a runtime timer to use for sleeping.
    pub fn with_timer(mut self, timer: Box<dyn Timer>) -> Self {
        self.timer = Some(timer);
        self
    }

    pub fn build(self) -> ServicePartitionResolver {
        ServicePartitionResolver {
            sm: self.fc.get_service_manager().clone(),
            timer: self.timer.unwrap_or(Box::new(crate::tokio::TokioTimer)),
            default_timeout: self.default_timeout.unwrap_or(Duration::from_secs(30)),
            max_retry_interval: self
                .default_max_retry_interval
                .unwrap_or(Duration::from_secs(5)),
        }
    }
}

impl ServicePartitionResolver {
    pub fn builder(fc: FabricClient) -> ServicePartitionResolverBuilder {
        ServicePartitionResolverBuilder::new(fc)
    }

    /// Resolve the service partition by name and key type.
    /// It retries all transient errors and timeouts.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, fields(uri = %name, timeout = ?timeout.unwrap_or(self.default_timeout)), err)
    )]
    pub async fn resolve(
        &self,
        name: &Uri,
        key_type: &PartitionKeyType,
        prev: Option<&ResolvedServicePartition>,
        timeout: Option<Duration>, // Total timeout for the operation
        token: Option<BoxedCancelToken>,
    ) -> mssf_core::Result<ResolvedServicePartition> {
        let timeout = timeout.unwrap_or(self.default_timeout);
        let timer = TimeCounter::new(timeout);
        let mut cancel: Pin<Box<dyn std::future::Future<Output = ()> + Send>> =
            if let Some(t) = &token {
                t.wait()
            } else {
                Box::pin(std::future::pending())
            };
        loop {
            let rsp_res = tokio::select! {
                _ = timer.sleep_until_remaining(self.timer.as_ref())? => {
                    // Timeout reached, return error.
                    return Err(ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(ErrorCode::E_ABORT.into());
                }
                rsp_opt = self
                    .sm
                    .resolve_service_partition(name, key_type, prev, timer.remaining()?, token.clone()) => rsp_opt,
            };
            let rsp_opt = match rsp_res {
                Ok(partition) => Some(partition),
                Err(e) => match e.try_as_fabric_error_code() {
                    Ok(ec) => {
                        if ec == ErrorCode::FABRIC_E_TIMEOUT || ec.is_transient() {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "Service partition transient error {ec}. Remaining time {:?}. Retrying...",
                                timer.remaining()?
                            );
                            // do nothing, retry.
                            None
                        } else {
                            return Err(e);
                        }
                    }
                    _ => return Err(e),
                },
            };

            // Check rsp is valid and save in the cache.
            // Sometimes endpoint is empty (may due to service removed), so we need to retry.
            if let Some(rsp) = rsp_opt
                && rsp.get_endpoint_list().iter().count() > 0
            {
                return Ok(rsp);
            }
            // sleep for a while before retrying.
            tokio::select! {
                _ = self.timer.sleep(self.max_retry_interval) => {},
                _ = timer.sleep_until_remaining(self.timer.as_ref())? => {
                    // Timeout reached, return error.
                    return Err(ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(ErrorCode::E_ABORT.into());
                }
            }
        }
    }
}
