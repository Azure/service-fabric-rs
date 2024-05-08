// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricCommon::{
        FabricRuntime::{
            FabricCreateRuntime, FabricGetActivationContext, IFabricCodePackageActivationContext,
            IFabricRuntime, IFabricStatefulServiceFactory, IFabricStatelessServiceFactory,
        },
        IFabricAsyncOperationCallback, IFabricAsyncOperationContext,
    },
    FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
};
use windows_core::{Error, Interface, HSTRING, PCWSTR};

use self::{
    config::ConfigurationPackage, executor::Executor, stateful::StatefulServiceFactory,
    stateful_bridge::StatefulServiceFactoryBridge, stateless::StatelessServiceFactory,
    stateless_bridge::StatelessServiceFactoryBridge,
};

mod bridge;
pub mod config;
pub mod error;
pub mod executor;
pub mod node_context;
pub mod stateful;
pub mod stateful_bridge;
pub mod stateful_proxy;
pub mod stateful_types;
pub mod stateless;
pub mod stateless_bridge;
pub mod store;
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

// safe wrapping for runtime
pub struct Runtime<E>
where
    E: Executor,
{
    com_impl: IFabricRuntime,
    rt: E,
}

impl<E> Runtime<E>
where
    E: Executor,
{
    pub fn create(rt: E) -> ::windows_core::Result<Runtime<E>> {
        let com = create_com_runtime()?;
        Ok(Runtime { com_impl: com, rt })
    }

    pub fn register_stateless_service_factory<F>(
        &self,
        servicetypename: &HSTRING,
        factory: F,
    ) -> windows_core::Result<()>
    where
        F: StatelessServiceFactory,
    {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatelessServiceFactory =
            StatelessServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatelessServiceFactory(servicetypename, &bridge)
        }
    }

    pub fn register_stateful_service_factory(
        &self,
        servicetypename: &HSTRING,
        factory: impl StatefulServiceFactory,
    ) -> windows_core::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatefulServiceFactory =
            StatefulServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.com_impl
                .RegisterStatefulServiceFactory(servicetypename, &bridge)
        }
    }
}

#[derive(Debug)]
pub struct EndpointResourceDesc {
    pub Name: ::windows_core::HSTRING,
    pub Protocol: ::windows_core::HSTRING,
    pub Type: ::windows_core::HSTRING,
    pub Port: u32,
    pub CertificateName: ::windows_core::HSTRING,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl From<&FABRIC_ENDPOINT_RESOURCE_DESCRIPTION> for EndpointResourceDesc {
    fn from(e: &FABRIC_ENDPOINT_RESOURCE_DESCRIPTION) -> Self {
        EndpointResourceDesc {
            Name: HSTRING::from_wide(unsafe { e.Name.as_wide() }).unwrap(),
            Protocol: HSTRING::from_wide(unsafe { e.Protocol.as_wide() }).unwrap(),
            Type: HSTRING::from_wide(unsafe { e.Type.as_wide() }).unwrap(),
            Port: e.Port,
            CertificateName: HSTRING::from_wide(unsafe { e.CertificateName.as_wide() }).unwrap(),
        }
    }
}

pub struct ActivationContext {
    com_impl: IFabricCodePackageActivationContext,
}

impl ActivationContext {
    pub fn create() -> Result<ActivationContext, Error> {
        let com = get_com_activation_context()?;
        Ok(ActivationContext { com_impl: com })
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
