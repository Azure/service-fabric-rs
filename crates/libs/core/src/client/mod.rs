// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricCommon::FabricClient::{
    IFabricPropertyManagementClient2, IFabricQueryClient10, IFabricServiceManagementClient6,
};
use windows_core::Interface;

use self::{
    gen::{property::IFabricPropertyManagementClient2Wrap, query::IFabricQueryClient10Wrap},
    query_client::QueryClient,
    svc_mgmt_client::ServiceManagementClient,
};

// TODO: make private. Currently exposed for backward compat
pub mod gen;

pub mod query_client;
pub mod svc_mgmt_client;

#[cfg(test)]
mod tests;

// FabricClient safe wrapper
// The design of FabricClient follows from the csharp client:
// https://github.com/microsoft/service-fabric/blob/master/src/prod/src/managed/Api/src/System/Fabric/FabricClient.cs

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
        let com = crate::sync::CreateLocalClient::<IFabricPropertyManagementClient2>();
        Self::from_com(com)
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
            _gen_wrap: IFabricPropertyManagementClient2Wrap::from_com(
                self.com_property_client.clone(),
            ),
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
    _gen_wrap: IFabricPropertyManagementClient2Wrap,
}
