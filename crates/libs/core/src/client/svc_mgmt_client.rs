// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![cfg_attr(
    not(feature = "tokio_async"),
    allow(unused_imports) // reason = "code configured out"
)]
use std::{ffi::c_void, time::Duration};

use mssf_com::{
    FabricClient::{IFabricResolvedServicePartitionResult, IFabricServiceManagementClient6},
    FabricTypes::{
        FABRIC_PARTITION_KEY_TYPE, FABRIC_PARTITION_KEY_TYPE_INT64,
        FABRIC_PARTITION_KEY_TYPE_INVALID, FABRIC_PARTITION_KEY_TYPE_NONE,
        FABRIC_PARTITION_KEY_TYPE_STRING, FABRIC_REMOVE_REPLICA_DESCRIPTION,
        FABRIC_RESOLVED_SERVICE_ENDPOINT, FABRIC_RESTART_REPLICA_DESCRIPTION,
        FABRIC_SERVICE_ENDPOINT_ROLE, FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        FABRIC_SERVICE_PARTITION_KIND, FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
        FABRIC_SERVICE_PARTITION_KIND_INVALID, FABRIC_SERVICE_PARTITION_KIND_NAMED,
        FABRIC_SERVICE_PARTITION_KIND_SINGLETON, FABRIC_SERVICE_ROLE_INVALID,
        FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY, FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY,
        FABRIC_SERVICE_ROLE_STATELESS, FABRIC_URI,
    },
};
use windows_core::{WString, PCWSTR};

#[cfg(feature = "tokio_async")]
use crate::sync::{fabric_begin_end_proxy2, CancellationToken, FabricReceiver2};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::WStringWrap,
    types::{
        RemoveReplicaDescription, RestartReplicaDescription, ServiceNotificationFilterDescription,
    },
};

// Service Management Client
#[derive(Debug, Clone)]
pub struct ServiceManagementClient {
    com: IFabricServiceManagementClient6,
}

impl ServiceManagementClient {
    pub fn get_com(&self) -> IFabricServiceManagementClient6 {
        self.com.clone()
    }
}
// internal implementation block
#[cfg(feature = "tokio_async")]
impl ServiceManagementClient {
    fn resolve_service_partition_internal(
        &self,
        name: FABRIC_URI,
        partition_key_type: FABRIC_PARTITION_KEY_TYPE,
        partition_key: Option<*const ::core::ffi::c_void>,
        previous_result: Option<&IFabricResolvedServicePartitionResult>, // This is different from generated code
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<IFabricResolvedServicePartitionResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginResolveServicePartition(
                    name,
                    partition_key_type,
                    partition_key.unwrap_or(std::ptr::null()),
                    previous_result,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndResolveServicePartition(ctx) },
            cancellation_token,
        )
    }

    fn restart_replica_internal(
        &self,
        desc: &FABRIC_RESTART_REPLICA_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginRestartReplica(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndRestartReplica(ctx) },
            cancellation_token,
        )
    }

    fn remove_replica_internal(
        &self,
        desc: &FABRIC_REMOVE_REPLICA_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginRemoveReplica(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndRemoveReplica(ctx) },
            cancellation_token,
        )
    }

    fn register_service_notification_filter_internal(
        &self,
        desc: &FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<i64>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginRegisterServiceNotificationFilter(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndRegisterServiceNotificationFilter(ctx) },
            cancellation_token,
        )
    }

    fn unregister_service_notification_filter_internal(
        &self,
        filterid: i64,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginUnregisterServiceNotificationFilter(
                    filterid,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndUnregisterServiceNotificationFilter(ctx) },
            cancellation_token,
        )
    }
}

impl From<IFabricServiceManagementClient6> for ServiceManagementClient {
    fn from(com: IFabricServiceManagementClient6) -> Self {
        Self { com }
    }
}

impl From<ServiceManagementClient> for IFabricServiceManagementClient6 {
    fn from(value: ServiceManagementClient) -> Self {
        value.com
    }
}

// public implementation block - tokio required
#[cfg(feature = "tokio_async")]
impl ServiceManagementClient {
    // Resolve service partition
    pub async fn resolve_service_partition(
        &self,
        name: &WString,
        key_type: &PartitionKeyType,
        prev: Option<&ResolvedServicePartition>,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> windows_core::Result<ResolvedServicePartition> {
        let com = {
            let uri = FABRIC_URI(name.as_ptr() as *mut u16);
            // supply prev as null if not present
            let prev_opt = prev.map(|x| &x.com);

            let part_key_opt = key_type.get_raw_opt();

            self.resolve_service_partition_internal(
                uri,
                key_type.into(),
                part_key_opt,
                prev_opt,
                timeout.as_millis().try_into().unwrap(),
                cancellation_token,
            )
        }
        .await??;
        let res = ResolvedServicePartition::from(com);
        Ok(res)
    }

    /// Simulates a service replica failure by restarting a persisted service replica,
    /// closing the replica, and then reopening it. Use this to test your service for problems
    /// along the replica reopen path. This helps simulate the report fault temporary path through client APIs.
    /// This is only valid for replicas that belong to stateful persisted services.
    pub async fn restart_replica(
        &self,
        desc: &RestartReplicaDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<()> {
        {
            let raw: FABRIC_RESTART_REPLICA_DESCRIPTION = desc.into();
            self.restart_replica_internal(&raw, timeout.as_millis() as u32, cancellation_token)
        }
        .await?
    }

    /// This API gives a running replica the chance to cleanup its state and be gracefully shutdown.
    /// WARNING: There are no safety checks performed when this API is used.
    /// Incorrect use of this API can lead to data loss for stateful services.
    /// Remarks:
    /// For stateless services, Instance Abort is called.
    pub async fn remove_replica(
        &self,
        desc: &RemoveReplicaDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<()> {
        {
            let raw: FABRIC_REMOVE_REPLICA_DESCRIPTION = desc.into();
            self.remove_replica_internal(&raw, timeout.as_millis() as u32, cancellation_token)
        }
        .await?
    }

    /// Remarks:
    /// There is a cache of service endpoints in the client that gets updated by notifications
    /// and this same cache is used to satisfy complaint based resolution requests
    /// (see resolve_service_partition())). Applications that both register for notifications
    /// and use complaint based resolution on the same client instance typically only need to
    /// pass null for the ResolvedServicePartition argument during resolution.
    /// This will always return the endpoints in the client cache updated by the latest notification.
    /// The notification mechanism itself will keep the client cache updated when service endpoints change.
    ///
    /// Notification callback is delivered on `FabricClientBuilder::with_on_service_notification` as well.
    /// The callback contains minimum info only as a signal, user can call resolve_service_partition()
    /// again to retrieve full info from the cache.
    ///
    /// This is observed to have 1~4 secs delay compared with brute force complaint based resolve.
    pub async fn register_service_notification_filter(
        &self,
        desc: &ServiceNotificationFilterDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<FilterIdHandle> {
        let id = {
            let raw: FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION = desc.into();
            self.register_service_notification_filter_internal(
                &raw,
                timeout.as_millis() as u32,
                cancellation_token,
            )
        }
        .await??;
        Ok(FilterIdHandle { id })
    }

    /// It's not necessary to unregister individual filters if the client itself
    /// will no longer be used since all ServiceNotificationFilterDescription
    /// objects registered by the FabricClient will be automatically unregistered when client is disposed.
    pub async fn unregister_service_notification_filter(
        &self,
        filter_id_handle: FilterIdHandle,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<()> {
        self.unregister_service_notification_filter_internal(
            filter_id_handle.id,
            timeout.as_millis() as u32,
            cancellation_token,
        )
        .await?
    }
}

// Handle to the registered service notification filter
pub struct FilterIdHandle {
    id: i64,
}

// see ComFabricClient.cpp for conversion details in cpp
#[derive(Debug, PartialEq)]
pub enum PartitionKeyType {
    Int64(i64),
    Invalid,
    None,
    String(WString),
}

impl PartitionKeyType {
    fn from_raw_svc_part(svc: ServicePartitionKind, data: *const c_void) -> PartitionKeyType {
        match svc {
            ServicePartitionKind::Int64Range => {
                let x = data as *mut i64;
                assert!(!x.is_null());
                PartitionKeyType::Int64(unsafe { *x })
            }
            ServicePartitionKind::Invalid => PartitionKeyType::Invalid,
            ServicePartitionKind::Singleton => PartitionKeyType::None,
            ServicePartitionKind::Named => {
                let x = data as *mut u16;
                assert!(!x.is_null());
                let s = WStringWrap::from(PCWSTR::from_raw(x)).into();
                PartitionKeyType::String(s)
            }
        }
    }
}

impl From<&PartitionKeyType> for FABRIC_PARTITION_KEY_TYPE {
    fn from(value: &PartitionKeyType) -> Self {
        match value {
            PartitionKeyType::Int64(_) => FABRIC_PARTITION_KEY_TYPE_INT64,
            PartitionKeyType::Invalid => FABRIC_PARTITION_KEY_TYPE_INVALID,
            PartitionKeyType::None => FABRIC_PARTITION_KEY_TYPE_NONE,
            PartitionKeyType::String(_) => FABRIC_PARTITION_KEY_TYPE_STRING,
        }
    }
}

impl PartitionKeyType {
    // get raw ptr to pass in com api
    fn get_raw_opt(&self) -> Option<*const c_void> {
        match self {
            // Not sure if this is ok for i64
            PartitionKeyType::Int64(x) => Some(x as *const i64 as *const c_void),
            PartitionKeyType::Invalid => None,
            PartitionKeyType::None => None,
            PartitionKeyType::String(x) => {
                Some(PCWSTR::from_raw(x.as_ptr()).as_ptr() as *const c_void)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ServicePartitionKind {
    Int64Range,
    Invalid,
    Named,
    Singleton,
}

impl From<&ServicePartitionKind> for FABRIC_SERVICE_PARTITION_KIND {
    fn from(value: &ServicePartitionKind) -> Self {
        match value {
            ServicePartitionKind::Int64Range => FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
            ServicePartitionKind::Invalid => FABRIC_SERVICE_PARTITION_KIND_INVALID,
            ServicePartitionKind::Named => FABRIC_SERVICE_PARTITION_KIND_NAMED,
            ServicePartitionKind::Singleton => FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
        }
    }
}

impl From<FABRIC_SERVICE_PARTITION_KIND> for ServicePartitionKind {
    fn from(value: FABRIC_SERVICE_PARTITION_KIND) -> Self {
        match value {
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => ServicePartitionKind::Int64Range,
            FABRIC_SERVICE_PARTITION_KIND_INVALID => ServicePartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_NAMED => ServicePartitionKind::Named,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => ServicePartitionKind::Singleton,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("unknown type: {:?}", value);
                } else {
                    ServicePartitionKind::Invalid
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedServicePartition {
    com: IFabricResolvedServicePartitionResult,
}

impl From<IFabricResolvedServicePartitionResult> for ResolvedServicePartition {
    fn from(com: IFabricResolvedServicePartitionResult) -> Self {
        Self { com }
    }
}

#[derive(Debug)]
pub struct ResolvedServicePartitionInfo {
    pub service_name: WString,
    pub service_partition_kind: ServicePartitionKind,
    pub partition_key_type: PartitionKeyType,
}

impl ResolvedServicePartition {
    // Get the service partition info/metadata
    pub fn get_info(&self) -> ResolvedServicePartitionInfo {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        let service_name = WStringWrap::from(PCWSTR::from_raw(raw.ServiceName.0)).into();
        let kind_raw = raw.Info.Kind;
        let val = raw.Info.Value;
        let service_partition_kind: ServicePartitionKind = kind_raw.into();
        let partition_key_type = PartitionKeyType::from_raw_svc_part(service_partition_kind, val);
        ResolvedServicePartitionInfo {
            service_name,
            service_partition_kind,
            partition_key_type,
        }
    }

    // Get the list of endpoints
    pub fn get_endpoint_list(&self) -> ResolvedServiceEndpointList {
        ResolvedServiceEndpointList::from(self.com.clone())
    }

    // If compared with different partition error is returned.
    // to enable the user to identify which RSP is more
    // up-to-date. A returned value of 0 indicates that the two RSPs have the same version. 1 indicates that the other RSP has an older version.
    // -1 indicates that the other RSP has a newer version.
    pub fn compare_version(&self, other: &ResolvedServicePartition) -> windows_core::Result<i32> {
        unsafe { self.com.CompareVersion(&other.com) }
    }
}

impl PartialEq for ResolvedServicePartition {
    fn eq(&self, other: &Self) -> bool {
        match self.compare_version(other) {
            Ok(i) => i == 0,
            Err(_) => false, // error comparing different services
        }
    }
}

impl PartialOrd for ResolvedServicePartition {
    /// Compare the version of the resolved result.
    /// a > b means partial_cmp(a,b) == Some(Greater) i.e. a.compare_version(b) > 0.
    /// a is newer and up to date.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.compare_version(other) {
            Ok(i) => Some(i.cmp(&0)),
            // If you compare version of different service you get error
            Err(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ServiceEndpointRole {
    Invalid,
    StatefulPrimary,
    StatefulSecondary,
    Stateless,
}

impl From<FABRIC_SERVICE_ENDPOINT_ROLE> for ServiceEndpointRole {
    fn from(value: FABRIC_SERVICE_ENDPOINT_ROLE) -> Self {
        match value {
            FABRIC_SERVICE_ROLE_INVALID => ServiceEndpointRole::Invalid,
            FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY => ServiceEndpointRole::StatefulPrimary,
            FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY => ServiceEndpointRole::StatefulSecondary,
            FABRIC_SERVICE_ROLE_STATELESS => ServiceEndpointRole::Stateless,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("unknown type: {:?}", value);
                } else {
                    ServiceEndpointRole::Invalid
                }
            }
        }
    }
}

pub struct ResolvedServiceEndpointList {
    com: IFabricResolvedServicePartitionResult,
}

impl From<IFabricResolvedServicePartitionResult> for ResolvedServiceEndpointList {
    fn from(com: IFabricResolvedServicePartitionResult) -> Self {
        Self { com }
    }
}

impl ResolvedServiceEndpointList {
    // Get iterator for the list
    pub fn iter(&self) -> ResolvedServiceEndpointListIter {
        ResolvedServiceEndpointListIter::new(self, self)
    }
}

impl FabricListAccessor<FABRIC_RESOLVED_SERVICE_ENDPOINT> for ResolvedServiceEndpointList {
    fn get_count(&self) -> u32 {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        raw.EndpointCount
    }

    fn get_first_item(&self) -> *const FABRIC_RESOLVED_SERVICE_ENDPOINT {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        raw.Endpoints
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedServiceEndpoint {
    pub address: WString,
    pub role: ServiceEndpointRole,
}

type ResolvedServiceEndpointListIter<'a> = FabricIter<
    'a,
    FABRIC_RESOLVED_SERVICE_ENDPOINT,
    ResolvedServiceEndpoint,
    ResolvedServiceEndpointList,
>;

impl From<&FABRIC_RESOLVED_SERVICE_ENDPOINT> for ResolvedServiceEndpoint {
    fn from(value: &FABRIC_RESOLVED_SERVICE_ENDPOINT) -> Self {
        let raw = value;
        Self {
            address: WStringWrap::from(raw.Address).into(),
            role: raw.Role.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use windows_core::{WString, PCWSTR};

    use super::{PartitionKeyType, ServicePartitionKind};

    #[test]
    fn test_conversion_int() {
        let k = PartitionKeyType::Int64(99);
        // check the raw ptr is ok
        let raw = k.get_raw_opt();
        let i = unsafe { (raw.unwrap() as *const i64).as_ref().unwrap() };
        assert_eq!(*i, 99);

        let service_type = ServicePartitionKind::Int64Range;
        // restore the key
        let k2 = PartitionKeyType::from_raw_svc_part(service_type, raw.unwrap());
        assert_eq!(k, k2);
    }

    #[test]
    fn test_conversion_string() {
        let src = WString::from("mystr");
        let k = PartitionKeyType::String(src.clone());
        // check the raw ptr is ok
        let raw = k.get_raw_opt();
        let s =
            WString::from_wide(unsafe { PCWSTR::from_raw(raw.unwrap() as *const u16).as_wide() });
        assert_eq!(s, src);

        let service_type = ServicePartitionKind::Named;
        // restore the key
        let k2 = PartitionKeyType::from_raw_svc_part(service_type, raw.unwrap());
        assert_eq!(k, k2);
    }
}
