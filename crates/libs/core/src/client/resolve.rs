// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{collections::HashMap, pin::Pin, sync::Mutex, time::Duration};

use crate::{client::svc_mgmt_client::FilterIdHandle, runtime::executor::Timer};
use tokio_util::sync::CancellationToken;
use windows_core::WString;

use crate::{
    client::{
        FabricClient,
        svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition, ServiceManagementClient},
    },
    iter::FabricListAccessor,
};

/// The same as dotnet sdk:
/// https://github.com/microsoft/service-fabric-services-and-actors-dotnet/blob/develop/src/Microsoft.ServiceFabric.Services/Client/ServicePartitionResolver.cs
pub struct ServicePartitionResolver {
    sm: ServiceManagementClient,
    timer: Box<dyn Timer>,
    default_timeout: Duration,
    max_retry_interval: Duration,
    use_notification: bool,
    registration_cache: RegistrationCache,
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

    pub fn remaining(&self) -> crate::Result<Duration> {
        if self.elapsed() < self.timeout {
            Ok(self.timeout - self.elapsed())
        } else {
            Err(crate::ErrorCode::FABRIC_E_TIMEOUT.into())
        }
    }

    /// returns a future that will sleep until the remaining time is up.
    pub fn sleep_until_remaining(
        &self,
        timer: &Box<dyn Timer>,
    ) -> crate::Result<impl Future<Output = ()>> {
        let remaining = self.remaining()?;
        Ok(timer.sleep(remaining))
    }
}

pub struct ServicePartitionResolverBuilder {
    fc: FabricClient,
    timer: Option<Box<dyn Timer>>,
    use_notification: bool,
    default_timeout: Option<Duration>,
    default_max_retry_interval: Option<Duration>,
}

impl ServicePartitionResolverBuilder {
    pub fn new(fc: FabricClient) -> Self {
        ServicePartitionResolverBuilder {
            fc,
            timer: None,
            use_notification: true,
            default_timeout: None,
            default_max_retry_interval: None,
        }
    }

    pub fn with_timer(mut self, timer: Box<dyn Timer>) -> Self {
        self.timer = Some(timer);
        self
    }

    pub fn with_notification(mut self, use_notification: bool) -> Self {
        self.use_notification = use_notification;
        self
    }

    pub fn build(self) -> ServicePartitionResolver {
        ServicePartitionResolver {
            sm: self.fc.get_service_manager().clone(),
            timer: self
                .timer
                .unwrap_or(Box::new(crate::runtime::executor::DefaultTimer)),
            default_timeout: self.default_timeout.unwrap_or(Duration::from_secs(30)),
            max_retry_interval: self
                .default_max_retry_interval
                .unwrap_or(Duration::from_secs(5)),
            use_notification: self.use_notification,
            registration_cache: RegistrationCache::default(),
        }
    }
}

impl ServicePartitionResolver {
    pub fn builder(fc: FabricClient) -> ServicePartitionResolverBuilder {
        ServicePartitionResolverBuilder::new(fc)
    }

    pub async fn resolve(
        &self,
        name: &WString,
        key_type: &PartitionKeyType,
        prev: Option<&ResolvedServicePartition>,
        timeout: Option<Duration>, // Total timeout for the operation
        token: Option<CancellationToken>,
    ) -> crate::Result<ResolvedServicePartition> {
        let timeout = timeout.unwrap_or(self.default_timeout);
        let timer = TimeCounter::new(timeout);
        let mut cancel: Pin<Box<dyn std::future::Future<Output = ()> + Send>> =
            if let Some(t) = &token {
                Box::pin(t.cancelled())
            } else {
                Box::pin(std::future::pending())
            };
        loop {
            let rsp_res = tokio::select! {
                _ = timer.sleep_until_remaining(&self.timer)? => {
                    // Timeout reached, return error.
                    return Err(crate::ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(crate::ErrorCode::E_ABORT.into());
                }
                rsp_opt = self
                    .sm
                    .resolve_service_partition(name, key_type, prev, timer.remaining()?, token.clone()) => rsp_opt,
            };
            let rsp_opt = match rsp_res {
                Ok(partition) => Some(partition),
                Err(e) => match e.try_as_fabric_error_code() {
                    Ok(ec) => {
                        if ec == crate::ErrorCode::FABRIC_E_TIMEOUT || ec.is_transient() {
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
            // Sometimes endpoint is empty, so we need to retry.
            if let Some(rsp) = rsp_opt
                && rsp.get_endpoint_list().get_count() != 0
            {
                if self.use_notification {
                    let timeout = timer.remaining()?;
                    let token_cp = token.clone();
                    let sm = self.sm.clone();
                    let reg_fn = |name: WString| async move {
                        let desc = crate::types::ServiceNotificationFilterDescription {
                            name,
                            flags: crate::types::ServiceNotificationFilterFlags::NamePrefix,
                        };
                        sm.register_service_notification_filter(&desc, timeout, token_cp)
                            .await
                    };
                    self.handle_notification(name, reg_fn).await?;
                }
                return Ok(rsp);
            }
            // sleep for a while before retrying.
            tokio::select! {
                _ = self.timer.sleep(self.max_retry_interval) => {},
                _ = timer.sleep_until_remaining(&self.timer)? => {
                    // Timeout reached, return error.
                    return Err(crate::ErrorCode::FABRIC_E_TIMEOUT.into());
                }
                _ = &mut cancel => {
                    // Cancellation requested, return error.
                    return Err(crate::ErrorCode::E_ABORT.into());
                }
            }
        }
    }

    async fn handle_notification<F, Fut>(&self, name: &WString, reg_fn: F) -> crate::Result<()>
    where
        F: FnOnce(WString) -> Fut,
        Fut: Future<Output = crate::Result<FilterIdHandle>> + Send + 'static,
    {
        // Put in the cache if not already present.
        // This makes sure other threads cannot register.
        // There is a chance that 2 threads try to register at the same time, but one fails, and the svc is not registered,
        // But there is a failure returned to user.
        {
            let mut cache = self.registration_cache.0.lock().unwrap();
            if !cache.contains_key(&name) {
                cache.insert(name.clone(), None);
            }
        }
        // Call the registration function
        match reg_fn(name.clone()).await {
            Ok(handle) => {
                let prev = self
                    .registration_cache
                    .0
                    .lock()
                    .unwrap()
                    .insert(name.clone(), Some(handle));
                assert!(prev.is_none(), "Filter already registered for {name}");
                Ok(())
            }
            Err(e) => {
                let prev = self
                    .registration_cache
                    .0
                    .lock()
                    .unwrap()
                    .remove(&name)
                    .expect("none id should be present");
                assert!(prev.is_none(), "Filter should be none on failure.");
                Err(e)
            }
        }
    }
}

// TODO: once notification is added, we don't ever unregister it, but we only rely on FabricClient drop to do all the cleanup.
#[derive(Debug, Default)]
pub struct RegistrationCache(Mutex<HashMap<WString, Option<FilterIdHandle>>>);
