// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricRuntime::{
        FabricCreateRuntime, FabricGetActivationContext, IFabricCodePackageActivationContext,
        IFabricRuntime,
    },
    FabricTypes::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
};
use windows_core::{Error, Interface, HSTRING, PCWSTR};

#[cfg(feature = "tokio_async")]
use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};

use self::config::ConfigurationPackage;

#[cfg(feature = "tokio_async")]
pub use self::runtime_wrapper::Runtime;

#[cfg(feature = "tokio_async")]
mod bridge;
pub mod config;
pub mod error;
#[cfg(feature = "tokio_async")]
pub mod executor;
#[cfg(feature = "tokio_async")]
pub mod node_context;
#[cfg(feature = "tokio_async")]
pub mod runtime_wrapper;
pub mod stateful;
#[cfg(feature = "tokio_async")]
pub mod stateful_bridge;
#[cfg(feature = "tokio_async")]
pub mod stateful_proxy;
pub mod stateful_types;
pub mod stateless;
#[cfg(feature = "tokio_async")]
pub mod stateless_bridge;
pub mod store;
#[cfg(feature = "tokio_async")]
pub mod store_proxy;
pub mod store_types;

// creates fabric runtime
pub fn create_com_runtime() -> ::windows_core::Result<IFabricRuntime> {
    let rawruntime = unsafe { FabricCreateRuntime(&IFabricRuntime::IID)? };
    let runtime = unsafe { IFabricRuntime::from_raw(rawruntime) };
    Ok(runtime)
}

pub fn get_com_activation_context() -> ::windows_core::Result<IFabricCodePackageActivationContext> {
    let raw_activation_ctx =
        unsafe { FabricGetActivationContext(&IFabricCodePackageActivationContext::IID)? };

    let activation_ctx =
        unsafe { IFabricCodePackageActivationContext::from_raw(raw_activation_ctx) };
    Ok(activation_ctx)
}

#[derive(Debug)]
pub struct EndpointResourceDesc {
    pub name: ::windows_core::HSTRING,
    pub protocol: ::windows_core::HSTRING,
    pub r#type: ::windows_core::HSTRING,
    pub port: u32,
    pub certificate_name: ::windows_core::HSTRING,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl From<&FABRIC_ENDPOINT_RESOURCE_DESCRIPTION> for EndpointResourceDesc {
    fn from(e: &FABRIC_ENDPOINT_RESOURCE_DESCRIPTION) -> Self {
        EndpointResourceDesc {
            name: HSTRING::from_wide(unsafe { e.Name.as_wide() }).unwrap(),
            protocol: HSTRING::from_wide(unsafe { e.Protocol.as_wide() }).unwrap(),
            r#type: HSTRING::from_wide(unsafe { e.Type.as_wide() }).unwrap(),
            port: e.Port,
            certificate_name: HSTRING::from_wide(unsafe { e.CertificateName.as_wide() }).unwrap(),
        }
    }
}

pub struct ActivationContext {
    com_impl: IFabricCodePackageActivationContext,
}

impl ActivationContext {
    pub fn create() -> Result<ActivationContext, Error> {
        let com = get_com_activation_context()?;
        Ok(Self::from(com))
    }

    pub fn get_endpoint_resource(
        &self,
        serviceendpointresourcename: &HSTRING,
    ) -> Result<EndpointResourceDesc, Error> {
        let rs = unsafe {
            self.com_impl.GetServiceEndpointResource(PCWSTR::from_raw(
                serviceendpointresourcename.as_ptr(),
            ))?
        };
        let res_ref = unsafe { rs.as_ref().unwrap() };
        let desc = EndpointResourceDesc::from(res_ref);
        Ok(desc)
    }

    pub fn get_configuration_package(
        &self,
        configpackagename: &HSTRING,
    ) -> windows_core::Result<ConfigurationPackage> {
        let c = unsafe { self.com_impl.GetConfigurationPackage(configpackagename) }?;
        Ok(ConfigurationPackage::from_com(c))
    }

    pub fn get_com(&self) -> IFabricCodePackageActivationContext {
        self.com_impl.clone()
    }
}

impl From<IFabricCodePackageActivationContext> for ActivationContext {
    fn from(value: IFabricCodePackageActivationContext) -> Self {
        ActivationContext { com_impl: value }
    }
}
