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
        IFabricAsyncOperationContext_Impl,
    },
    FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
};
use std::cell::Cell;
use windows::core::implement;
use windows_core::{Error, Interface, HSTRING, PCWSTR};

use self::{
    executor::Executor, stateful::StatefulServiceFactory,
    stateful_bridge::StatefulServiceFactoryBridge, stateless::StatelessServiceFactory,
    stateless_bridge::StatelessServiceFactoryBridge,
};

pub mod error;
pub mod executor;
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
}

pub struct NodeContext {}

#[implement(IFabricAsyncOperationContext)]
struct BridgeContext<T> {
    content: Cell<Option<T>>,
    is_completed: Cell<bool>,
    is_completed_synchronously: bool,
    callback: IFabricAsyncOperationCallback,
}

impl<T> BridgeContext<T> {
    fn new(callback: IFabricAsyncOperationCallback) -> BridgeContext<T> {
        BridgeContext {
            content: Cell::new(None),
            is_completed: Cell::new(false),
            is_completed_synchronously: false,
            callback,
        }
    }

    // fn new_i(callback: IFabricAsyncOperationCallback) -> IFabricAsyncOperationContext {
    //     Self::new(callback).into()
    // }

    fn set_content(&self, content: T) {
        let prev = self.content.replace(Some(content));
        assert!(prev.is_none())
    }

    fn consume_content(&self) -> T {
        self.content.take().unwrap()
    }

    fn set_complete(&self) {
        self.is_completed.swap(&Cell::new(true));
    }

    // This as access violation. The com layout is not safe
    // fn invoke(&mut self, ctx: &IFabricAsyncOperationContext) {
    //     assert!(!self.is_completed);
    //     self.is_completed = true;
    //     info!("callback invoke");
    //     unsafe { self.callback.Invoke(ctx) };
    // }
}

impl<T> IFabricAsyncOperationContext_Impl for BridgeContext<T> {
    fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed.get().into()
    }

    fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed_synchronously.into()
    }

    fn Callback(&self) -> ::windows_core::Result<IFabricAsyncOperationCallback> {
        let cp = self.callback.clone();
        Ok(cp)
    }

    fn Cancel(&self) -> ::windows_core::Result<()> {
        Ok(())
    }
}
