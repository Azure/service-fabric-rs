// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricClient::{
        IFabricServiceEndpointsVersion, IFabricServiceNotification,
        IFabricServiceNotificationEventHandler, IFabricServiceNotificationEventHandler_Impl,
    },
    FabricTypes::FABRIC_RESOLVED_SERVICE_ENDPOINT,
};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::WStringWrap,
    types::ServicePartitionInformation,
};

use super::svc_mgmt_client::ResolvedServiceEndpoint;

/// Rust trait to turn rust code into IFabricServiceNotificationEventHandler.
/// Not exposed to user
pub trait ServiceNotificationEventHandler: 'static {
    fn on_notification(&self, notification: &ServiceNotification) -> crate::Result<()>;
}

/// Content of the service notification callback.
/// Remarks:
/// If endpoint list is empty, the service is removed.
#[derive(Debug, Clone)]
pub struct ServiceNotification {
    pub service_name: crate::WString,
    pub partition_info: Option<ServicePartitionInformation>,
    pub partition_id: crate::GUID,
    pub endpoints: ServiceEndpointList,
    com: IFabricServiceNotification,
}

impl ServiceNotification {
    fn from_com(com: IFabricServiceNotification) -> Self {
        // SF guarantees this is not null.
        let raw = unsafe { com.get_Notification().as_ref().unwrap() };
        Self {
            service_name: WStringWrap::from(crate::PCWSTR(raw.ServiceName.0)).into(),
            partition_info: unsafe {
                // It is possible for partition info to be null,
                // that is why we make the field as an option.
                // See: https://github.com/microsoft/service-fabric/blob/93545a62e8f6c2407bd538c0f092b33419f77c30/src/prod/src/client/ServiceNotificationResult.cpp#L120
                raw.PartitionInfo
                    .as_ref()
                    .map(ServicePartitionInformation::from)
            },
            partition_id: raw.PartitionId,
            endpoints: ServiceEndpointList { com: com.clone() },
            com,
        }
    }

    pub fn get_version(&self) -> crate::Result<ServiceEndpointsVersion> {
        let version = unsafe { self.com.GetVersion() }?;
        Ok(ServiceEndpointsVersion::from_com(version))
    }
}

#[derive(Debug, Clone)]
pub struct ServiceEndpointList {
    com: IFabricServiceNotification,
}

impl ServiceEndpointList {
    // Get iterator for the list
    pub fn iter(&self) -> ServiceEndpointListIter {
        ServiceEndpointListIter::new(self, self)
    }
}

/// mssf_core iterator infrastructure implementation
impl FabricListAccessor<FABRIC_RESOLVED_SERVICE_ENDPOINT> for ServiceEndpointList {
    fn get_count(&self) -> u32 {
        let raw = unsafe { self.com.get_Notification().as_ref().unwrap() };
        raw.EndpointCount
    }

    fn get_first_item(&self) -> *const FABRIC_RESOLVED_SERVICE_ENDPOINT {
        let raw = unsafe { self.com.get_Notification().as_ref().unwrap() };
        raw.Endpoints
    }
}

type ServiceEndpointListIter<'a> =
    FabricIter<'a, FABRIC_RESOLVED_SERVICE_ENDPOINT, ResolvedServiceEndpoint, ServiceEndpointList>;

/// IFabricServiceEndpointsVersion wrapper.
pub struct ServiceEndpointsVersion {
    com: IFabricServiceEndpointsVersion,
}

impl ServiceEndpointsVersion {
    fn from_com(com: IFabricServiceEndpointsVersion) -> Self {
        Self { com }
    }

    /// CSharp doc: Zero if this and other are equivalent,
    /// a negative value if this is less than other, and a positive value if this is greater than other.
    ///
    /// This is not usually used in CSharp apps, but the implementation is provided here for completeness.
    /// Ideally one should use mssf_core::client::svc_mgmt_client::ResolvedServicePartition instead, by
    /// doing an additional FabricClient resolve call to retrieve from FabricClient cache.
    pub fn compare(&self, other: &ServiceEndpointsVersion) -> crate::Result<i32> {
        unsafe { self.com.Compare(&other.com) }
    }
}

impl PartialEq for ServiceEndpointsVersion {
    fn eq(&self, other: &Self) -> bool {
        match self.compare(other) {
            Ok(i) => i == 0,
            Err(_) => false, // error comparing different services
        }
    }
}

impl PartialOrd for ServiceEndpointsVersion {
    /// Compare the version of the resolved result.
    /// a > b means partial_cmp(a,b) == Some(Greater) i.e. a.compare_version(b) > 0.
    /// a is newer and up to date.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.compare(other) {
            Ok(i) => Some(i.cmp(&0)),
            // If you compare version of different service you get error
            Err(_) => None,
        }
    }
}

// Bridge implementation for the notification handler to turn rust code into SF com object.
#[windows_core::implement(IFabricServiceNotificationEventHandler)]
#[allow(non_camel_case_types)] // Suppress lint for _Impl struct
pub struct ServiceNotificationEventHandlerBridge<T>
where
    T: ServiceNotificationEventHandler,
{
    inner: T,
}

impl<T> ServiceNotificationEventHandlerBridge<T>
where
    T: ServiceNotificationEventHandler,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn new_com(inner: T) -> IFabricServiceNotificationEventHandler {
        Self::new(inner).into()
    }
}

impl<T> IFabricServiceNotificationEventHandler_Impl
    for ServiceNotificationEventHandlerBridge_Impl<T>
where
    T: ServiceNotificationEventHandler,
{
    fn OnNotification(
        &self,
        notification: Option<&IFabricServiceNotification>,
    ) -> crate::Result<()> {
        let com = notification.unwrap();
        let msg = ServiceNotification::from_com(com.to_owned());
        self.inner.on_notification(&msg)
    }
}

/// Lambda implemnentation of ServiceNotificationEventHandler trait.
/// This is used in FabricClientBuilder to build function into handler.
/// Not exposed to user.
/// This isn't strictly required by the implementation as written. But it leaves open the door to non-lambda implementations in future.
pub struct LambdaServiceNotificationHandler<T>
where
    T: Fn(&ServiceNotification) -> crate::Result<()> + 'static,
{
    f: T,
}

impl<T> LambdaServiceNotificationHandler<T>
where
    T: Fn(&ServiceNotification) -> crate::Result<()> + 'static,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> ServiceNotificationEventHandler for LambdaServiceNotificationHandler<T>
where
    T: Fn(&ServiceNotification) -> crate::Result<()> + 'static,
{
    fn on_notification(&self, notification: &ServiceNotification) -> crate::Result<()> {
        (self.f)(notification)
    }
}
