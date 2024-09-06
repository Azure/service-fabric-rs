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
    types::ServicePartitionInformation,
};

use super::svc_mgmt_client::ResolvedServiceEndpoint;

pub trait ServiceNotificationEventHandler: 'static {
    fn on_notification(&self, notification: &ServiceNotification) -> crate::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ServiceNotification {
    pub partition_info: ServicePartitionInformation,
    pub partition_id: crate::GUID,
    pub endpoints: ServiceEndpointList,
    com: IFabricServiceNotification,
}

impl ServiceNotification {
    fn from_com(com: IFabricServiceNotification) -> Self {
        let raw = unsafe { com.get_Notification().as_ref().unwrap() };
        Self {
            partition_info: unsafe { raw.PartitionInfo.as_ref().unwrap().into() },
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

// Bridge implementation for the notification handler
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

// default implementation of ServiceNotificationEventHandler
pub struct DefaultServiceNotificationEventHandler {}

impl ServiceNotificationEventHandler for DefaultServiceNotificationEventHandler {
    fn on_notification(&self, notification: &ServiceNotification) -> crate::Result<()> {
        tracing::debug!("Got service notification {:?}", notification);
        Ok(())
    }
}
