use std::ffi::OsString;
use std::os::windows::prelude::OsStrExt;
use std::{convert::TryInto, ptr::null};

use log::info;
use service_fabric_rs::FabricCommon::FabricRuntime::{
    IFabricRuntime, IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
    IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
};
use service_fabric_rs::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
    IFabricStringResult, IFabricStringResult_Impl,
};
use windows::core::implement;
use windows::w;

mod echo;

pub fn run(runtime: &IFabricRuntime, port: u32, hostname: OsString) {
    info!("port: {}, host: {:?}", port, hostname);

    let factory: IFabricStatelessServiceFactory = ServiceFactory::new(port, hostname).into();
    let service_type_name = w!("EchoAppService");
    unsafe { runtime.RegisterStatelessServiceFactory(service_type_name, &factory) }
        .expect("register failed");
}

#[derive(Debug)]
#[implement(IFabricStatelessServiceFactory)]
pub struct ServiceFactory {
    port_: u32,
    hostname_: OsString,
}

impl ServiceFactory {
    pub fn new(port: u32, hostname: OsString) -> ServiceFactory {
        ServiceFactory {
            port_: port,
            hostname_: hostname,
        }
    }
}

impl IFabricStatelessServiceFactory_Impl for ServiceFactory {
    fn CreateInstance(
        &self,
        servicetypename: &windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &windows::core::GUID,
        instanceid: i64,
    ) -> windows::core::Result<
        service_fabric_rs::FabricCommon::FabricRuntime::IFabricStatelessServiceInstance,
    > {
        let mut init_data: String = "".to_string();
        if initializationdata != null() && initializationdatalength != 0 {
            init_data = unsafe {
                String::from_utf8_lossy(std::slice::from_raw_parts(
                    initializationdata,
                    initializationdatalength.try_into().unwrap(),
                ))
                .to_string()
            };
        }
        info!("servicetypename: {}, servicename: {:?}, initdata: {}, partitionid: {:?}, instanceid {}", 
            unsafe{servicetypename.display()},
            servicename,
            init_data,
            partitionid,
            instanceid
        );
        let port_copy = self.port_.clone();
        let hostname_copy = self.hostname_.clone();
        let instance = AppInstance::new(port_copy, hostname_copy);
        return Ok(instance.into());
    }
}

#[derive(Debug)]
#[implement(IFabricStatelessServiceInstance)]

pub struct AppInstance {
    port_: u32,
    hostname_: OsString,
}

impl AppInstance {
    pub fn new(port: u32, hostname: OsString) -> AppInstance {
        return AppInstance {
            port_: port,
            hostname_: hostname,
        };
    }
}

impl IFabricStatelessServiceInstance_Impl for AppInstance {
    fn BeginOpen(
        &self,
        partition: &core::option::Option<
            service_fabric_rs::FabricCommon::FabricRuntime::IFabricStatelessServicePartition,
        >,
        callback: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback,
        >,
    ) -> windows::core::Result<service_fabric_rs::FabricCommon::IFabricAsyncOperationContext> {
        let p = partition.as_ref().expect("get partition failed");
        let info = unsafe { p.GetPartitionInfo() }.expect("getpartition info failed");
        info!("AppInstance::BeginOpen partition kind {:#?}", info);

        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };

        // TODO: emplement stop thread.
        let port_copy = self.port_.clone();
        let hostname_copy = self.hostname_.clone();
        std::thread::spawn(move || echo::start_echo(port_copy, hostname_copy));
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationContext,
        >,
    ) -> windows::core::Result<service_fabric_rs::FabricCommon::IFabricStringResult> {
        info!("AppInstance::EndOpen");
        let completed = unsafe {
            context
                .as_ref()
                .expect("not ctx")
                .CompletedSynchronously()
                .as_bool()
        };
        if !completed {
            info!("AppInstance::EndOpen callback not completed");
        }

        let addr = echo::get_addr(self.port_, self.hostname_.clone());

        let str_res: IFabricStringResult = StringResult::new(OsString::from(addr)).into();
        Ok(str_res)
    }

    fn BeginClose(
        &self,
        callback: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback,
        >,
    ) -> windows::core::Result<service_fabric_rs::FabricCommon::IFabricAsyncOperationContext> {
        info!("AppInstance::BeginClose");

        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }

    fn EndClose(
        &self,
        context: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationContext,
        >,
    ) -> windows::core::Result<()> {
        info!("AppInstance::EndClose");
        let completed = unsafe {
            context
                .as_ref()
                .expect("not ctx")
                .CompletedSynchronously()
                .as_bool()
        };
        if !completed {
            info!("AppInstance::EndClose callback not completed");
        }
        Ok(())
    }

    fn Abort(&self) {
        info!("AppInstance::Abort");
    }
}

#[derive(Debug)]
#[implement(IFabricAsyncOperationContext)]
struct AsyncContext {
    callback_: IFabricAsyncOperationCallback,
}

impl AsyncContext {
    // construct ctx. Note: caller needs to invoke callback.
    // This is different from cpp impl.
    pub fn new(
        callback: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback,
        >,
    ) -> AsyncContext {
        info!("AsyncContext::new");
        let callback_copy: IFabricAsyncOperationCallback = callback.clone().expect("msg");

        let ctx = AsyncContext {
            callback_: callback_copy,
        };
        return ctx;
    }
}

impl IFabricAsyncOperationContext_Impl for AsyncContext {
    fn IsCompleted(&self) -> windows::Win32::Foundation::BOOLEAN {
        return windows::Win32::Foundation::BOOLEAN::from(true);
    }

    fn CompletedSynchronously(&self) -> windows::Win32::Foundation::BOOLEAN {
        return windows::Win32::Foundation::BOOLEAN::from(true);
    }

    fn Callback(
        &self,
    ) -> windows::core::Result<service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback> {
        info!("AsyncContext::Callback");
        // get a view of the callback
        let callback_copy: IFabricAsyncOperationCallback = self.callback_.clone();
        Ok(callback_copy)
    }

    fn Cancel(&self) -> windows::core::Result<()> {
        info!("AsyncContext::Cancel");
        Ok(())
    }
}

#[derive(Debug)]
#[implement(IFabricStringResult)]
struct StringResult {
    vec_: Vec<u16>,
}

impl StringResult {
    pub fn new(data: OsString) -> StringResult {
        let data_vec = data
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let ret = StringResult { vec_: data_vec };
        return ret;
    }
}

impl IFabricStringResult_Impl for StringResult {
    fn get_String(&self) -> windows::core::PWSTR {
        // This is some hack to get the raw pointer out.
        let ptr: *mut u16 = self.vec_.as_ptr() as *mut u16;
        return windows::core::PWSTR::from_raw(ptr);
    }
}
