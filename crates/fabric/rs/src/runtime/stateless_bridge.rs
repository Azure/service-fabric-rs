#![deny(non_snake_case)] // this file is safe rust

use std::{marker::PhantomData, sync::Arc};

use crate::{
    runtime::{stateless::StatelessServicePartition, BridgeContext},
    StringResult,
};
use fabric_base::FabricCommon::{
    FabricRuntime::{
        IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
        IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
        IFabricStatelessServicePartition,
    },
    IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl, IFabricStringResult,
};
use log::info;
use windows::core::implement;
use windows_core::{AsImpl, Error, HSTRING};

use super::{
    executor::Executor,
    stateless::{StatelessServiceFactory, StatelessServiceInstance},
};

#[implement(IFabricStatelessServiceFactory)]
pub struct StatelessServiceFactoryBridge<E, F, S>
where
    E: Executor,
    F: StatelessServiceFactory<S>,
    S: StatelessServiceInstance + 'static,
{
    inner: F,
    rt: E,
    phantom: PhantomData<S>,
}

impl<E, F, S> StatelessServiceFactoryBridge<E, F, S>
where
    E: Executor,
    F: StatelessServiceFactory<S>,
    S: StatelessServiceInstance,
{
    pub fn create(factory: F, rt: E) -> StatelessServiceFactoryBridge<E, F, S> {
        StatelessServiceFactoryBridge::<E, F, S> {
            inner: factory,
            rt,
            phantom: PhantomData,
        }
    }
}

impl<E, F, S> IFabricStatelessServiceFactory_Impl for StatelessServiceFactoryBridge<E, F, S>
where
    E: Executor,
    F: StatelessServiceFactory<S>,
    S: StatelessServiceInstance + 'static,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateInstance(
        &self,
        servicetypename: &::windows_core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows_core::GUID,
        instanceid: i64,
    ) -> ::windows_core::Result<IFabricStatelessServiceInstance> {
        info!("StatelessServiceFactoryBridge::CreateInstance");
        let p_servicename = ::windows_core::PCWSTR::from_raw(servicename);
        let h_servicename = HSTRING::from_wide(unsafe { p_servicename.as_wide() }).unwrap();
        let h_servicetypename = HSTRING::from_wide(unsafe { servicetypename.as_wide() }).unwrap();
        let data = unsafe {
            std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
        };

        let instance = self.inner.create_instance(
            &h_servicetypename,
            &h_servicename,
            data,
            partitionid,
            instanceid,
        );
        let rt = self.rt.clone();
        let instance_bridge = IFabricStatelessServiceInstanceBridge::create(instance, rt);

        Ok(instance_bridge.into())
    }
}

// bridge from safe service instance to com
#[implement(IFabricStatelessServiceInstance)]

struct IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance + 'static,
{
    inner: Arc<S>,
    rt: E,
}

impl<E, S> IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance,
{
    pub fn create(instance: S, rt: E) -> IFabricStatelessServiceInstanceBridge<E, S>
    where
        S: StatelessServiceInstance,
    {
        IFabricStatelessServiceInstanceBridge {
            inner: Arc::new(instance),
            rt,
        }
    }
}

impl<E, S> IFabricStatelessServiceInstance_Impl for IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance + 'static,
{
    fn BeginOpen(
        &self,
        partition: ::core::option::Option<&IFabricStatelessServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatelessServiceInstanceBridge::BeginOpen");
        let partition_cp = partition.unwrap().clone();
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            info!("IFabricStatelessServiceInstanceBridge::BeginOpen spawn");
            let partition_bridge = StatelessServicePartition::new(partition_cp);
            let ok = inner_cp.open(&partition_bridge).await;
            let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricStatelessServiceInstanceBridge::EndOpen");
        let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> =
            unsafe { context.unwrap().as_impl() };

        let content = ctx_bridge.consume_content()?;
        Ok(StringResult::new(content).into())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatelessServiceInstanceBridge::BeginClose");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy: IFabricAsyncOperationContext = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.close().await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricStatelessServiceInstanceBridge::EndClose");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn Abort(&self) {
        info!("IFabricStatelessServiceInstanceBridge::Abort")
    }
}
