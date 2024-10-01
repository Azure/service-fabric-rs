// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use connection::{ClientConnectionEventHandlerBridge, LambdaClientConnectionNotificationHandler};
use mssf_com::FabricClient::{
    FabricCreateLocalClient3, FabricCreateLocalClient4, IFabricClientConnectionEventHandler,
    IFabricPropertyManagementClient2, IFabricQueryClient10, IFabricServiceManagementClient6,
    IFabricServiceNotificationEventHandler,
};
use notification::{
    LambdaServiceNotificationHandler, ServiceNotificationEventHandler,
    ServiceNotificationEventHandlerBridge,
};
use windows_core::Interface;

use crate::types::ClientRole;

use self::{query_client::QueryClient, svc_mgmt_client::ServiceManagementClient};

mod connection;
mod notification;
pub mod query_client;
pub mod svc_mgmt_client;

// reexport
pub use connection::GatewayInformationResult;
pub use notification::ServiceNotification;

#[cfg(test)]
mod tests;

/// Creates FabricClient com object using SF com API.
fn create_local_client_internal<T: Interface>(
    service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
    client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
    client_role: Option<ClientRole>,
) -> T {
    let role = client_role.unwrap_or(ClientRole::Unknown);
    let raw = if role == ClientRole::Unknown {
        // unknown role should use the SF function without role param.
        unsafe {
            FabricCreateLocalClient3(
                service_notification_handler,
                client_connection_handler,
                &T::IID,
            )
        }
    } else {
        unsafe {
            FabricCreateLocalClient4(
                service_notification_handler,
                client_connection_handler,
                role.into(),
                &T::IID,
            )
        }
    }
    .expect("failed to create fabric client");
    // if params are right, client should be created. There is no network call involved during obj creation.
    unsafe { T::from_raw(raw) }
}

// Builder for FabricClient
pub struct FabricClientBuilder {
    sn_handler: Option<IFabricServiceNotificationEventHandler>,
    cc_handler: Option<LambdaClientConnectionNotificationHandler>,
    client_role: ClientRole,
}

impl Default for FabricClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricClientBuilder {
    /// Creates the builder.
    pub fn new() -> Self {
        Self {
            sn_handler: None,
            cc_handler: None,
            client_role: ClientRole::Unknown,
        }
    }

    /// Configures the service notification handler internally.
    fn with_service_notification_handler(
        mut self,
        handler: impl ServiceNotificationEventHandler,
    ) -> Self {
        self.sn_handler = Some(ServiceNotificationEventHandlerBridge::new_com(handler));
        self
    }

    /// Configures the service notification handler.
    /// See details in `register_service_notification_filter` API.
    /// If the service endpoint change matches the registered filter,
    /// this notification is invoked
    pub fn with_on_service_notification<T>(self, f: T) -> Self
    where
        T: Fn(&ServiceNotification) -> crate::Result<()> + 'static,
    {
        let handler = LambdaServiceNotificationHandler::new(f);
        self.with_service_notification_handler(handler)
    }

    /// When FabricClient connects to the SF cluster, this callback is invoked.
    pub fn with_on_client_connect<T>(mut self, f: T) -> Self
    where
        T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    {
        if self.cc_handler.is_none() {
            self.cc_handler = Some(LambdaClientConnectionNotificationHandler::new());
        }
        if let Some(cc) = self.cc_handler.as_mut() {
            cc.set_f_conn(f)
        }
        self
    }

    /// When FabricClient disconnets to the SF cluster, this callback is called.
    /// This callback is not called on Drop of FabricClient.
    pub fn with_on_client_disconnect<T>(mut self, f: T) -> Self
    where
        T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    {
        if self.cc_handler.is_none() {
            self.cc_handler = Some(LambdaClientConnectionNotificationHandler::new());
        }
        if let Some(cc) = self.cc_handler.as_mut() {
            cc.set_f_disconn(f)
        }
        self
    }

    /// Sets the role of the client connection. Default is User if not set.
    pub fn with_client_role(mut self, role: ClientRole) -> Self {
        self.client_role = role;
        self
    }

    /// Build the fabricclient
    /// Remarks: FabricClient connect to SF cluster when
    /// the first API call is triggered. Build/create of the object does not
    /// establish connection.
    pub fn build(self) -> FabricClient {
        let c = Self::build_interface(self);
        FabricClient::from_com(c)
    }

    /// Build the specific com interface of the fabric client.
    pub fn build_interface<T: Interface>(self) -> T {
        let cc_handler = self
            .cc_handler
            .map(ClientConnectionEventHandlerBridge::new_com);
        create_local_client_internal::<T>(
            self.sn_handler.as_ref(),
            cc_handler.as_ref(),
            Some(self.client_role),
        )
    }
}

// FabricClient safe wrapper
// The design of FabricClient follows from the csharp client:
// https://github.com/microsoft/service-fabric/blob/master/src/prod/src/managed/Api/src/System/Fabric/FabricClient.cs
#[derive(Debug, Clone)]
pub struct FabricClient {
    com_property_client: IFabricPropertyManagementClient2,
    com_service_client: IFabricServiceManagementClient6,
    com_query_client: IFabricQueryClient10,
}

impl FabricClient {
    /// Get a builder
    pub fn builder() -> FabricClientBuilder {
        FabricClientBuilder::new()
    }

    // Get a copy of COM object
    pub fn get_com(&self) -> IFabricPropertyManagementClient2 {
        self.com_property_client.clone()
    }

    // Creates from com directly. This gives the user freedom to create com from
    // custom code and pass it in.
    // For the final state of FabricClient, this function should be private.
    pub fn from_com(com: IFabricPropertyManagementClient2) -> Self {
        let com_property_client = com.clone();
        let com_service_client = com
            .clone()
            .cast::<IFabricServiceManagementClient6>()
            .unwrap();
        let com_query_client = com.clone().cast::<IFabricQueryClient10>().unwrap();
        Self {
            com_property_client,
            com_service_client,
            com_query_client,
        }
    }

    // Get the client for managing Fabric Properties in Naming Service
    pub fn get_property_manager(&self) -> PropertyManagementClient {
        PropertyManagementClient {
            _com: self.com_property_client.clone(),
        }
    }

    // Get the client for quering SF info.
    pub fn get_query_manager(&self) -> QueryClient {
        QueryClient::from_com(self.com_query_client.clone())
    }

    // Get the client for managing service info and lifecycles.
    pub fn get_service_manager(&self) -> ServiceManagementClient {
        ServiceManagementClient::from_com(self.com_service_client.clone())
    }
}

pub struct PropertyManagementClient {
    _com: IFabricPropertyManagementClient2,
}
