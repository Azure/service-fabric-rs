// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(non_snake_case)]

use std::cell::Cell;
use std::convert::TryInto;
use std::io::Error;
use std::thread::JoinHandle;

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricStringResult,
};
use mssf_com::FabricRuntime::{
    IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
    IFabricReplicator_Impl, IFabricRuntime, IFabricStatefulServiceFactory,
    IFabricStatefulServiceFactory_Impl, IFabricStatefulServicePartition,
    IFabricStatefulServiceReplica, IFabricStatefulServiceReplica_Impl,
};
use mssf_com::FabricTypes::{
    FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE,
    FABRIC_REPLICA_OPEN_MODE_INVALID, FABRIC_REPLICA_ROLE, FABRIC_REPLICA_SET_CONFIGURATION,
    FABRIC_REPLICA_SET_QUORUM_MODE, FABRIC_URI,
};
use mssf_core::WString;
use mssf_core::{strings::WStringWrap, sync::wait::AsyncContext};
use tokio::sync::oneshot::{self, Sender};
use tracing::info;
use windows_core::implement;

mod echo;

pub fn run(runtime: &IFabricRuntime, port: u32, hostname: WString) {
    info!("port: {}, host: {:?}", port, hostname);

    let factory: IFabricStatefulServiceFactory = StatefulServiceFactory::new(port, hostname).into();
    let service_type_name = mssf_core::WString::from("StatefulEchoAppService");
    unsafe { runtime.RegisterStatefulServiceFactory(service_type_name.as_pcwstr(), &factory) }
        .expect("register failed");
}

#[derive(Debug)]
#[implement(IFabricStatefulServiceFactory)]
pub struct StatefulServiceFactory {
    port_: u32,
    hostname_: WString,
}

impl StatefulServiceFactory {
    pub fn new(port: u32, hostname: WString) -> StatefulServiceFactory {
        StatefulServiceFactory {
            port_: port,
            hostname_: hostname,
        }
    }
}

impl IFabricStatefulServiceFactory_Impl for StatefulServiceFactory_Impl {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateReplica(
        &self,
        servicetypename: &mssf_core::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &mssf_core::GUID,
        instanceid: i64,
    ) -> mssf_core::Result<IFabricStatefulServiceReplica> {
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
            mssf_core::strings::WStringWrap::from(*servicetypename).into_wstring(),
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

#[implement(IFabricReplicator, IFabricPrimaryReplicator)]
pub struct AppFabricReplicator {
    port_: u32,
    hostname_: WString,
}

impl AppFabricReplicator {
    pub fn new(port: u32, hostname: WString) -> AppFabricReplicator {
        AppFabricReplicator {
            port_: port,
            hostname_: hostname,
        }
    }
}

// This is basic implementation of Replicator
impl IFabricReplicator_Impl for AppFabricReplicator_Impl {
    fn BeginOpen(
        &self,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginOpen");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> mssf_core::Result<IFabricStringResult> {
        info!("AppFabricReplicator::EndOpen");
        let addr = echo::get_addr(self.port_, self.hostname_.clone());
        info!("AppFabricReplicator::EndOpen {}", addr);
        let str_res: IFabricStringResult = WStringWrap::from(WString::from(addr)).into();
        Ok(str_res)
    }

    fn BeginChangeRole(
        &self,
        _epoch: *const FABRIC_EPOCH,
        _role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginChangeRole");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator::EndChangeRole");
        Ok(())
    }

    fn BeginUpdateEpoch(
        &self,
        _epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginUpdateEpoch");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }
    fn EndUpdateEpoch(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::EndUpdateEpoch");
        Ok(())
    }
    fn BeginClose(
        &self,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginClose");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }
    fn EndClose(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::EndClose");
        Ok(())
    }
    fn Abort(&self) {
        info!("AppFabricReplicator::Abort");
    }
    fn GetCurrentProgress(&self) -> ::mssf_core::Result<i64> {
        info!("AppFabricReplicator::GetCurrentProgress");
        let v = 0;
        Ok(v)
    }
    fn GetCatchUpCapability(&self) -> ::mssf_core::Result<i64> {
        info!("AppFabricReplicator::GetCatchUpCapability");
        let v = 0;
        Ok(v)
    }
}

// This is basic implementation of PrimaryReplicator
impl IFabricPrimaryReplicator_Impl for AppFabricReplicator_Impl {
    fn BeginOnDataLoss(
        &self,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginOnDataLoss");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }
    fn EndOnDataLoss(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<u8> {
        info!("AppFabricReplicator::EndOnDataLoss");
        let v = 0;
        Ok(v)
    }
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        _currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        _previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::UpdateCatchUpReplicaSetConfiguration");
        Ok(())
    }
    fn BeginWaitForCatchUpQuorum(
        &self,
        _catchupmode: FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginWaitForCatchUpQuorum");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }
    fn EndWaitForCatchUpQuorum(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::EndWaitForCatchUpQuorum");
        Ok(())
    }
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        _currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::UpdateCurrentReplicaSetConfiguration");
        Ok(())
    }
    fn BeginBuildReplica(
        &self,
        _replica: *const FABRIC_REPLICA_INFORMATION,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppFabricReplicator::BeginBuildReplica");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }
    fn EndBuildReplica(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<()> {
        info!("AppFabricReplicator::EndBuildReplica");
        Ok(())
    }
    fn RemoveReplica(&self, replicaid: i64) -> ::mssf_core::Result<()> {
        info!(
            "AppFabricReplicator::UpdateCurrentReplicaSetConfiguration {} ",
            replicaid
        );
        Ok(())
    }
}

//#[derive(Debug)]
#[implement(IFabricStatefulServiceReplica)]
pub struct AppInstance {
    port_: u32,
    hostname_: WString,
    tx_: Cell<Option<Sender<()>>>, // hack to use this mutably
    th_: Cell<Option<JoinHandle<Result<(), Error>>>>,
    // role_: Cell<mssf_com::FABRIC_REPLICA_ROLE>,
    // replicator_: Cell<Option<IFabricReplicator>>,
}

impl AppInstance {
    pub fn new(port: u32, hostname: WString) -> AppInstance {
        AppInstance {
            port_: port,
            hostname_: hostname,
            tx_: Cell::from(None),
            th_: Cell::from(None),
            // role_: Cell::from(mssf_com::FABRIC_REPLICA_ROLE_UNKNOWN),
            // replicator_: Cell::from(None),
        }
    }
}

impl IFabricStatefulServiceReplica_Impl for AppInstance_Impl {
    fn BeginOpen(
        &self,
        openmode: FABRIC_REPLICA_OPEN_MODE,
        partition: ::core::option::Option<&IFabricStatefulServicePartition>,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("echo_replica::BeginOpen");

        if openmode == FABRIC_REPLICA_OPEN_MODE_INVALID {
            //TODO: return error
        }

        info!("open mode: {:?}", openmode);

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
        context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<IFabricReplicator> {
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

        Ok(AppFabricReplicator::new(self.port_, self.hostname_.clone()).into())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
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
        context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<()> {
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

    fn BeginChangeRole(
        &self,
        _newrole: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
    ) -> ::mssf_core::Result<IFabricAsyncOperationContext> {
        info!("AppInstance::BeginChangeRole");
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(callback).into();
        // invoke callback right away
        unsafe { ctx.Callback().expect("cannot get callback").Invoke(&ctx) };
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        _context: ::core::option::Option<&IFabricAsyncOperationContext>,
    ) -> ::mssf_core::Result<IFabricStringResult> {
        info!("AppInstance::EndChangeRole");
        let addr = echo::get_addr(self.port_, self.hostname_.clone());
        info!("AppInstance::EndChangeRole {}", addr);
        let str_res: IFabricStringResult = WStringWrap::from(WString::from(addr)).into();
        Ok(str_res)
    }
}
