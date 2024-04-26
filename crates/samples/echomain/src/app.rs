// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(non_snake_case)]

use std::cell::Cell;
use std::convert::TryInto;
use std::io::Error;
use std::thread::JoinHandle;

use log::info;
use mssf_com::FabricCommon::FabricRuntime::{
    IFabricRuntime, IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
    IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
    IFabricStatelessServicePartition,
};
use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricStringResult,
};
use mssf_core::{strings::HSTRINGWrap, AsyncContext};
use tokio::sync::oneshot::{self, Sender};
use windows::core::implement;
use windows::core::w;
use windows_core::HSTRING;

mod echo;

pub fn run(runtime: &IFabricRuntime, port: u32, hostname: HSTRING) {
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
    hostname_: HSTRING,
}

impl ServiceFactory {
    pub fn new(port: u32, hostname: HSTRING) -> ServiceFactory {
        ServiceFactory {
            port_: port,
            hostname_: hostname,
        }
    }
}

impl IFabricStatelessServiceFactory_Impl for ServiceFactory {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateInstance(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows::core::GUID,
        instanceid: i64,
    ) -> ::windows::core::Result<IFabricStatelessServiceInstance> {
        let mut init_data: String = "".to_string();
        if initializationdata.is_null() && initializationdatalength != 0 {
            init_data = unsafe {
                String::from_utf8_lossy(std::slice::from_raw_parts(
                    initializationdata,
                    initializationdatalength.try_into().unwrap(),
                ))
                .to_string()
            };
        }
        info!(
            "servicetypename: {}, servicename: {:?}, initdata: {}, partitionid: {:?}, instanceid {}",
            unsafe { servicetypename.display() },
            servicename,
            init_data,
            partitionid,
            instanceid
        );
        let port_copy = self.port_;
        let hostname_copy = self.hostname_.clone();
        let instance = AppInstance::new(port_copy, hostname_copy);
        Ok(instance.into())
    }
}

//#[derive(Debug)]
#[implement(IFabricStatelessServiceInstance)]

pub struct AppInstance {
    port_: u32,
    hostname_: HSTRING,
    tx_: Cell<Option<Sender<()>>>, // hack to use this mutably
    th_: Cell<Option<JoinHandle<Result<(), Error>>>>,
}

impl AppInstance {
    pub fn new(port: u32, hostname: HSTRING) -> AppInstance {
        AppInstance {
            port_: port,
            hostname_: hostname,
            tx_: Cell::from(None),
            th_: Cell::from(None),
        }
    }
}

impl IFabricStatelessServiceInstance_Impl for AppInstance {
    fn BeginOpen(
        &self,
        partition: core::option::Option<&IFabricStatelessServicePartition>,
        callback: core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> windows::core::Result<IFabricAsyncOperationContext> {
        let p = partition.as_ref().expect("get partition failed");
        let info = unsafe { p.GetPartitionInfo() }.expect("getpartition info failed");
        info!("AppInstance::BeginOpen partition kind {:#?}", info);

        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };

        // TODO: emplement stop thread.

        let port_copy = self.port_;
        let hostname_copy = self.hostname_.clone();

        let (tx, rx) = oneshot::channel::<()>();

        // owns tx which is to be used when shutdown.
        self.tx_.set(Some(tx));
        let th = std::thread::spawn(move || echo::start_echo(rx, port_copy, hostname_copy));
        self.th_.set(Some(th));

        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: core::option::Option<&IFabricAsyncOperationContext>,
    ) -> windows::core::Result<IFabricStringResult> {
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

        let str_res: IFabricStringResult = HSTRINGWrap::from(HSTRING::from(addr)).into();
        Ok(str_res)
    }

    fn BeginClose(
        &self,
        callback: core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> windows::core::Result<IFabricAsyncOperationContext> {
        info!("AppInstance::BeginClose");

        // triggers shutdown to tokio
        if let Some(sender) = self.tx_.take() {
            info!("AppInstance:: Triggering shutdown");
            let res = sender.send(());
            match res {
                Ok(_) => {
                    if let Some(th) = self.th_.take() {
                        let res2 = th.join();
                        match res2 {
                            Ok(_) => {
                                info!("AppInstance:: Background thread terminated");
                            }
                            Err(_) => {
                                info!("AppInstance:: Background thread failed to join.")
                            }
                        }
                    }
                }
                Err(_) => {
                    info!("AppInstance:: failed to send");
                }
            }
        } else {
            info!("AppInstance:: sender is None");
        }

        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }

    fn EndClose(
        &self,
        context: core::option::Option<&IFabricAsyncOperationContext>,
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
