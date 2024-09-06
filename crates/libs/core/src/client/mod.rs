// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use connection::{
    ClientConnectionEventHandler, ClientConnectionEventHandlerBridge,
    DefaultClientConnectionEventHandler,
};
use mssf_com::FabricClient::{
    FabricCreateLocalClient4, IFabricPropertyManagementClient2, IFabricQueryClient10,
    IFabricServiceManagementClient6,
};
use notification::{
    DefaultServiceNotificationEventHandler, ServiceNotificationEventHandler,
    ServiceNotificationEventHandlerBridge,
};
use windows_core::Interface;

use crate::types::ClientRole;

use self::{query_client::QueryClient, svc_mgmt_client::ServiceManagementClient};

mod connection;
mod notification;
pub mod query_client;
pub mod svc_mgmt_client;

#[cfg(test)]
mod tests;

// Fabric Client creation
// Creates the local client
pub fn create_local_client<T: Interface>(
    service_notification_handler: Option<impl ServiceNotificationEventHandler>,
    client_connection_handler: Option<impl ClientConnectionEventHandler>,
    client_role: Option<ClientRole>,
) -> T {
    let sn_handler =
        service_notification_handler.map(|sn| ServiceNotificationEventHandlerBridge::new_com(sn));
    let cc_handler =
        client_connection_handler.map(|cc| ClientConnectionEventHandlerBridge::new_com(cc));
    let role = client_role.unwrap_or(ClientRole::User);
    assert_ne!(
        role,
        ClientRole::Unknown,
        "Unknown role should not be used."
    );
    let raw = unsafe {
        FabricCreateLocalClient4(
            sn_handler.as_ref(),
            cc_handler.as_ref(),
            role.into(),
            &T::IID,
        )
    }
    .expect("failed to create fabric client");
    // if params are right, client should be created. There is no network call involved during obj creation.
    unsafe { T::from_raw(raw) }
}

// Used for convenience.
pub(crate) fn create_local_client_default<T: Interface>() -> T {
    create_local_client::<T>(
        None::<DefaultServiceNotificationEventHandler>,
        None::<DefaultClientConnectionEventHandler>,
        None,
    )
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

impl Default for FabricClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricClient {
    pub fn new() -> Self {
        let com = create_local_client_default::<IFabricPropertyManagementClient2>();
        Self::from_com(com)
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
