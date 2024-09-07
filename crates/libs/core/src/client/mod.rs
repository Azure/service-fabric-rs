// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use connection::{
    ClientConnectionEventHandler, ClientConnectionEventHandlerBridge,
    LambdaClientConnectionNotificationHandler,
};
use mssf_com::FabricClient::{
    FabricCreateLocalClient4, IFabricClientConnectionEventHandler,
    IFabricPropertyManagementClient2, IFabricQueryClient10, IFabricServiceManagementClient6,
    IFabricServiceNotificationEventHandler,
};
use notification::{LambdaServiceNotificationHandler, ServiceNotificationEventHandlerBridge};
use windows_core::Interface;

use crate::types::ClientRole;

use self::{query_client::QueryClient, svc_mgmt_client::ServiceManagementClient};

mod connection;
mod notification;
pub mod query_client;
pub mod svc_mgmt_client;

// reexport
pub use connection::GatewayInformationResult;
pub use notification::{ServiceNotification, ServiceNotificationEventHandler};

#[cfg(test)]
mod tests;

// Fabric Client creation
fn create_local_client_internal<T: Interface>(
    service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
    client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
    client_role: Option<ClientRole>,
) -> T {
    let role = client_role.unwrap_or(ClientRole::User);
    assert_ne!(
        role,
        ClientRole::Unknown,
        "Unknown role should not be used."
    );
    let raw = unsafe {
        FabricCreateLocalClient4(
            service_notification_handler,
            client_connection_handler,
            role.into(),
            &T::IID,
        )
    }
    .expect("failed to create fabric client");
    // if params are right, client should be created. There is no network call involved during obj creation.
    unsafe { T::from_raw(raw) }
}

// Builder for FabricClient
pub struct FabricClientBuilder {
    sn_handler: Option<IFabricServiceNotificationEventHandler>,
    cc_handler: Option<IFabricClientConnectionEventHandler>,
    client_role: ClientRole,
}

impl Default for FabricClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricClientBuilder {
    pub fn new() -> Self {
        Self {
            sn_handler: None,
            cc_handler: None,
            client_role: ClientRole::User,
        }
    }

    /// Configures the service notification handler.
    pub fn with_service_notification_handler(
        mut self,
        handler: impl ServiceNotificationEventHandler,
    ) -> Self {
        self.sn_handler = Some(ServiceNotificationEventHandlerBridge::new_com(handler));
        self
    }

    /// Configures the service notification handler, but using a function.
    pub fn with_service_notification_handler_fn<T>(self, f: T) -> Self
    where
        T: Fn(&ServiceNotification) -> crate::Result<()> + 'static,
    {
        let handler = LambdaServiceNotificationHandler::new(f);
        self.with_service_notification_handler(handler)
    }

    /// Configures client connection handler.
    pub fn with_client_connection_handler(
        mut self,
        handler: impl ClientConnectionEventHandler,
    ) -> Self {
        self.cc_handler = Some(ClientConnectionEventHandlerBridge::new_com(handler));
        self
    }

    /// Configures client connection handler, but functions.
    /// f_conn and f_disconn is invoked when fabric client connects and disconnects
    /// to SF cluster respectively.
    pub fn with_client_connection_handler_fn<T, K>(self, f_conn: T, f_disconn: K) -> Self
    where
        T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
        K: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    {
        let handler = LambdaClientConnectionNotificationHandler::new(f_conn, f_disconn);
        self.with_client_connection_handler(handler)
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
        create_local_client_internal::<T>(
            self.sn_handler.as_ref(),
            self.cc_handler.as_ref(),
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
