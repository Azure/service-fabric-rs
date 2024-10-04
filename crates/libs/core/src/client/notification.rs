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
    strings::HSTRINGWrap,
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
    pub service_name: crate::HSTRING,
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
            service_name: HSTRINGWrap::from(crate::PCWSTR(raw.ServiceName.0)).into(),
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

    /// TODO: documentation.
    pub fn compare(&self, other: &ServiceEndpointsVersion) -> crate::Result<i32> {
        unsafe { self.com.Compare(&other.com) }
    }
}

// Bridge implementation for the notification handler to turn rust code into SF com object.
#[windows_core::implement(IFabricServiceNotificationEventHandler)]
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

impl<T> IFabricServiceNotificationEventHandler_Impl for ServiceNotificationEventHandlerBridge<T>
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
