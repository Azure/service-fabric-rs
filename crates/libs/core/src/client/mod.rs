// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use connection::{ClientConnectionEventHandlerBridge, LambdaClientConnectionNotificationHandler};
use health_client::HealthClient;
use mssf_com::FabricClient::{
    IFabricClientConnectionEventHandler, IFabricHealthClient4, IFabricPropertyManagementClient2,
    IFabricQueryClient10, IFabricServiceManagementClient6, IFabricServiceNotificationEventHandler,
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

// Export public client modules
pub mod health_client;
pub mod query_client;
pub mod svc_mgmt_client;

// reexport
pub use connection::GatewayInformationResult;
pub use notification::ServiceNotification;

#[cfg(test)]
mod tests;

/// Creates FabricClient com object using SF com API.
fn create_local_client_internal<T: Interface>(
    connection_strings: Option<&Vec<crate::WString>>,
    service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
    client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
    client_role: Option<ClientRole>,
) -> T {
    let role = client_role.unwrap_or(ClientRole::Unknown);

    // create raw conn str ptrs.
    let connection_strings_ptrs = connection_strings.map(|addrs| {
        addrs
            .iter()
            .map(|s| crate::PCWSTR(s.as_ptr()))
            .collect::<Vec<_>>()
    });

    match connection_strings_ptrs {
        Some(addrs) => {
            assert!(
                role == ClientRole::Unknown,
                "ClientRole is for local client only and cannot be used for connecting to remote cluster."
            );
            crate::API_TABLE.fabric_create_client3::<T>(
                &addrs,
                service_notification_handler,
                client_connection_handler,
            )
        },
        None => {
            if role == ClientRole::Unknown {
                // unknown role should use the SF function without role param.
                    crate::API_TABLE.fabric_create_local_client3::<T>(
                        service_notification_handler,
                        client_connection_handler,
                    )
            } else {
                    crate::API_TABLE.fabric_create_local_client4::<T>(
                        service_notification_handler,
                        client_connection_handler,
                        role.into(),
                    )
            }
        }
    }
    // if params are right, client should be created. There is no network call involved during obj creation.
    .expect("failed to create fabric client")
}

// Builder for FabricClient
pub struct FabricClientBuilder {
    sn_handler: Option<IFabricServiceNotificationEventHandler>,
    cc_handler: Option<LambdaClientConnectionNotificationHandler>,
    client_role: ClientRole,
    connection_strings: Option<Vec<crate::WString>>,
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
            connection_strings: None,
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
    /// this notification is invoked.
    ///
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

    /// Sets the role of the client connection. Default is Unknown if not set.
    /// Unknown role cannot be used for remote client connection.
    /// If connection strings are set, only Unknown is allowed.
    pub fn with_client_role(mut self, role: ClientRole) -> Self {
        self.client_role = role;
        self
    }

    /// Sets the client connection strings.
    /// Example value: localhost:19000
    pub fn with_connection_strings(mut self, addrs: Vec<crate::WString>) -> Self {
        self.connection_strings = Some(addrs);
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
            self.connection_strings.as_ref(),
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
    property_client: PropertyManagementClient,
    service_client: ServiceManagementClient,
    query_client: QueryClient,
    health_client: HealthClient,
}

impl FabricClient {
    /// Get a builder
    pub fn builder() -> FabricClientBuilder {
        FabricClientBuilder::new()
    }

    /// Creates from com directly. This gives the user freedom to create com from
    /// custom code and pass it in.
    /// For the final state of FabricClient, this function should be private.
    pub fn from_com(com: IFabricPropertyManagementClient2) -> Self {
        let com_property_client = com.clone();
        let com_service_client = com
            .clone()
            .cast::<IFabricServiceManagementClient6>()
            .unwrap();
        let com_query_client = com.clone().cast::<IFabricQueryClient10>().unwrap();
        let com_health_client = com.clone().cast::<IFabricHealthClient4>().unwrap();
        Self {
            property_client: PropertyManagementClient::from_com(com_property_client),
            service_client: ServiceManagementClient::from_com(com_service_client),
            query_client: QueryClient::from_com(com_query_client),
            health_client: HealthClient::from_com(com_health_client),
        }
    }

    /// Get the client for managing Fabric Properties in Naming Service
    pub fn get_property_manager(&self) -> &PropertyManagementClient {
        &self.property_client
    }

    /// Get the client for quering Service Fabric information.
    pub fn get_query_manager(&self) -> &QueryClient {
        &self.query_client
    }

    /// Get the client for managing service information and lifecycles.
    pub fn get_service_manager(&self) -> &ServiceManagementClient {
        &self.service_client
    }

    /// Get the client for get/set Service Fabric health properties.
    pub fn get_health_manager(&self) -> &HealthClient {
        &self.health_client
    }
}

#[derive(Debug, Clone)]
pub struct PropertyManagementClient {
    com: IFabricPropertyManagementClient2,
}

impl PropertyManagementClient {
    /// Get a copy of COM object
    pub fn get_com(&self) -> IFabricPropertyManagementClient2 {
        self.com.clone()
    }

    fn from_com(com: IFabricPropertyManagementClient2) -> Self {
        Self { com }
    }
}
