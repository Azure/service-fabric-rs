use std::{cell::Cell, sync::Arc};

use async_trait::async_trait;
use fabric_base::FabricCommon::{
    FabricRuntime::{
        FabricCreateRuntime, FabricGetActivationContext, IFabricCodePackageActivationContext,
        IFabricRuntime, IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
        IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
        IFabricStatelessServicePartition,
    },
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};
use log::info;
use tokio::{runtime::Handle, sync::Mutex};
use windows::core::implement;
use windows_core::{AsImpl, ComInterface, Error, Interface, HSTRING};

use crate::StringResult;

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
pub struct Runtime {
    comImpl: IFabricRuntime,
    rt: Handle,
}

impl Runtime {
    pub fn create(rt: Handle) -> ::windows_core::Result<Runtime> {
        let com = create_com_runtime()?;
        Ok(Runtime { comImpl: com, rt })
    }

    pub fn register_stateless_service_factory(
        &self,
        servicetypename: &HSTRING,
        factory: Box<dyn StatelessServiceFactory>,
    ) -> windows_core::Result<()> {
        let rt_cp = self.rt.clone();
        let bridge: IFabricStatelessServiceFactory =
            StatelessServiceFactoryBridge::create(factory, rt_cp).into();
        unsafe {
            self.comImpl
                .RegisterStatelessServiceFactory(servicetypename, &bridge)
        }
    }
}

#[implement(IFabricAsyncOperationContext)]
struct BridgeContext<T> {
    content: Cell<Option<T>>,
    is_completed: bool,
    is_completed_synchronously: bool,
    callback: IFabricAsyncOperationCallback,
}

impl<T> BridgeContext<T> {
    fn new(callback: IFabricAsyncOperationCallback) -> BridgeContext<T> {
        BridgeContext {
            content: Cell::new(None),
            is_completed: false,
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
        self.is_completed.into()
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

pub struct StatelessServicePartition {}

// safe factory
pub trait StatelessServiceFactory {
    fn create_instance(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &::windows::core::GUID,
        instanceid: i64,
    ) -> Box<dyn StatelessServiceInstance + Send>;
}

#[implement(IFabricStatelessServiceFactory)]
pub struct StatelessServiceFactoryBridge {
    inner: Box<dyn StatelessServiceFactory>,
    rt: Handle,
}

impl StatelessServiceFactoryBridge {
    pub fn create(
        factory: Box<dyn StatelessServiceFactory>,
        rt: Handle,
    ) -> StatelessServiceFactoryBridge {
        StatelessServiceFactoryBridge { inner: factory, rt }
    }
}

impl IFabricStatelessServiceFactory_Impl for StatelessServiceFactoryBridge {
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
// safe service instance
#[async_trait]
pub trait StatelessServiceInstance {
    async fn open(&self, partition: &StatelessServicePartition) -> windows::core::Result<HSTRING>;
    async fn close(&self) -> windows::core::Result<()>;
    fn abort(&self);
}

// bridge from safe service instance to com
#[implement(IFabricStatelessServiceInstance)]

struct IFabricStatelessServiceInstanceBridge {
    inner: Arc<Mutex<Box<dyn StatelessServiceInstance + Send>>>,
    rt: Handle,
}

impl IFabricStatelessServiceInstanceBridge {
    pub fn create(
        instance: Box<dyn StatelessServiceInstance + Send>,
        rt: Handle,
    ) -> IFabricStatelessServiceInstanceBridge {
        IFabricStatelessServiceInstanceBridge {
            inner: Arc::new(Mutex::new(instance)),
            rt,
        }
    }
}

impl IFabricStatelessServiceInstance_Impl for IFabricStatelessServiceInstanceBridge {
    fn BeginOpen(
        &self,
        _partition: ::core::option::Option<&IFabricStatelessServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatelessServiceInstanceBridge::BeginOpen");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            info!("IFabricStatelessServiceInstanceBridge::BeginOpen spawn");
            let partition_bridge = StatelessServicePartition {};
            let ok = inner_cp.lock().await.open(&partition_bridge).await;
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
    ) -> ::windows_core::Result<super::IFabricStringResult> {
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
            let ok = inner_cp.lock().await.close().await;
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
