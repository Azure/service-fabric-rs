#![deny(non_snake_case)] // this file is safe rust

use std::sync::Arc;

use crate::{runtime::BridgeContext, StringResult};
use async_trait::async_trait;
use fabric_base::{
    FabricCommon::{
        FabricRuntime::{
            IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
            IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
            IFabricStatelessServicePartition,
        },
        IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl, IFabricStringResult,
    },
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
    FABRIC_SINGLETON_PARTITION_INFORMATION,
};
use log::info;
use tokio::{runtime::Handle, sync::Mutex};
use windows::core::implement;
use windows_core::{AsImpl, Error, HSTRING};

// wrap of com interface
pub struct StatelessServicePartition {
    com_impl: IFabricStatelessServicePartition,
}

#[derive(Debug)]
pub struct SingletonPartitionInfo {
    pub id: ::windows_core::GUID,
}

#[derive(Debug)]
pub struct Int64PartitionInfo {
    pub id: ::windows_core::GUID,
    pub low_key: i64,
    pub high_key: i64,
}

#[derive(Debug)]
pub struct NamedPartitionInfo {
    pub id: ::windows_core::GUID,
    pub name: ::windows_core::HSTRING,
}

#[derive(Debug)]
pub enum PartitionKind {
    Invalid,
    Singleton(SingletonPartitionInfo),
    Int64Range(Int64PartitionInfo),
    Named(NamedPartitionInfo),
}

impl StatelessServicePartition {
    pub fn new(com_impl: IFabricStatelessServicePartition) -> StatelessServicePartition {
        StatelessServicePartition { com_impl }
    }

    pub fn get_partition_info(&self) -> ::windows_core::Result<PartitionKind> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        let res: PartitionKind = match raw_ref.Kind {
            FABRIC_SERVICE_PARTITION_KIND_INVALID => PartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => {
                let raw_info =
                    unsafe { &mut *(raw_ref.Value as *mut FABRIC_SINGLETON_PARTITION_INFORMATION) };
                let info = SingletonPartitionInfo { id: raw_info.Id };
                PartitionKind::Singleton(info)
            }
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => {
                let raw_info = unsafe {
                    &mut *(raw_ref.Value as *mut FABRIC_INT64_RANGE_PARTITION_INFORMATION)
                };
                let info = Int64PartitionInfo {
                    id: raw_info.Id,
                    low_key: raw_info.LowKey,
                    high_key: raw_info.HighKey,
                };
                PartitionKind::Int64Range(info)
            }
            FABRIC_SERVICE_PARTITION_KIND_NAMED => {
                let raw_info =
                    unsafe { &mut *(raw_ref.Value as *mut FABRIC_NAMED_PARTITION_INFORMATION) };
                let info = NamedPartitionInfo {
                    id: raw_info.Id,
                    name: HSTRING::from_wide(unsafe { raw_info.Name.as_wide() }).unwrap(),
                };
                PartitionKind::Named(info)
            }
            _ => PartitionKind::Invalid,
        };
        Ok(res)
    }
}

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
