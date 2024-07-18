// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful bridge is to wrap rs types into com to expose to SF

use std::sync::Arc;

use tracing::info;
use windows::core::implement;
use windows_core::{AsImpl, Error, Interface, HSTRING};

use mssf_com::{
    FabricCommon::{
        IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl, IFabricStringResult,
    },
    FabricRuntime::{
        IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
        IFabricReplicator_Impl, IFabricStatefulServiceFactory, IFabricStatefulServiceFactory_Impl,
        IFabricStatefulServicePartition, IFabricStatefulServiceReplica,
        IFabricStatefulServiceReplica_Impl,
    },
    FabricTypes::{
        FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE, FABRIC_REPLICA_ROLE,
        FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_MODE, FABRIC_URI,
    },
};

use crate::{
    runtime::{
        bridge::BridgeContext,
        stateful::StatefulServicePartition,
        stateful_types::{Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, Role},
    },
    strings::HSTRINGWrap,
};

use super::{
    executor::Executor,
    stateful::{PrimaryReplicator, Replicator, StatefulServiceFactory, StatefulServiceReplica},
};

#[implement(IFabricStatefulServiceFactory)]
pub struct StatefulServiceFactoryBridge<E, F>
where
    E: Executor + 'static,
    F: StatefulServiceFactory + 'static,
{
    inner: F,
    rt: E,
}

impl<E, F> StatefulServiceFactoryBridge<E, F>
where
    E: Executor,
    F: StatefulServiceFactory,
{
    pub fn create(factory: F, rt: E) -> StatefulServiceFactoryBridge<E, F> {
        StatefulServiceFactoryBridge::<E, F> { inner: factory, rt }
    }
}

impl<E, F> IFabricStatefulServiceFactory_Impl for StatefulServiceFactoryBridge<E, F>
where
    E: Executor,
    F: StatefulServiceFactory,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateReplica(
        &self,
        servicetypename: &::windows_core::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows_core::GUID,
        replicaid: i64,
    ) -> ::windows_core::Result<IFabricStatefulServiceReplica> {
        info!("StatefulServiceFactoryBridge::CreateReplica");
        let p_servicename = ::windows_core::PCWSTR::from_raw(servicename.0);
        let h_servicename = HSTRING::from_wide(unsafe { p_servicename.as_wide() }).unwrap();
        let h_servicetypename = HSTRING::from_wide(unsafe { servicetypename.as_wide() }).unwrap();
        let data = unsafe {
            if !initializationdata.is_null() {
                std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
            } else {
                &[]
            }
        };

        let replica = self.inner.create_replica(
            &h_servicetypename,
            &h_servicename,
            data,
            partitionid,
            replicaid,
        )?;
        let rt = self.rt.clone();
        let replica_bridge = IFabricStatefulServiceReplicaBridge::create(replica, rt);
        Ok(replica_bridge.into())
    }
}

// bridges from rs into com

// bridge from safe service instance to com
#[implement(IFabricReplicator)]

pub struct IFabricReplicatorBridge<E, R>
where
    E: Executor,
    R: Replicator,
{
    inner: Arc<R>,
    rt: E,
}

impl<E, R> IFabricReplicatorBridge<E, R>
where
    E: Executor,
    R: Replicator,
{
    pub fn create(rplctr: R, rt: E) -> IFabricReplicatorBridge<E, R> {
        IFabricReplicatorBridge {
            inner: Arc::new(rplctr),
            rt,
        }
    }

    fn create_from_primary_replicator(replicator: Arc<R>, rt: E) -> IFabricReplicatorBridge<E, R> {
        IFabricReplicatorBridge {
            inner: replicator,
            rt,
        }
    }
}

impl<E, R> IFabricReplicator_Impl for IFabricReplicatorBridge<E, R>
where
    E: Executor,
    R: Replicator,
{
    fn BeginOpen(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginOpen");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.open().await;
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
        let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> =
            unsafe { context.unwrap().as_impl() };
        let content = ctx_bridge.consume_content()?;
        info!("IFabricReplicatorBridge::EndOpen addr {}", content);
        Ok(HSTRINGWrap::from(content).into())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let role2: Role = (&role).into();
        info!(
            "IFabricReplicatorBridge::BeginChangeRole epoch {:?}, role {:?}",
            epoch2, role2
        );

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.change_role(&epoch2, &role2).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::EndChangeRole");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        info!(
            "IFabricReplicatorBridge::BeginUpdateEpoch epoch {:?}",
            epoch2
        );

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.update_epoch(&epoch2).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::BeginUpdateEpoch");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginClose");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
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
        info!("IFabricReplicatorBridge::EndClose");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn Abort(&self) {
        info!("IFabricReplicatorBridge::Abort");
        self.inner.abort();
    }

    fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        let lsn = self.inner.get_current_progress();
        info!("IFabricReplicatorBridge::GetCurrentProgress: {:?}", lsn);
        lsn
    }

    fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        let lsn = self.inner.get_catch_up_capability();
        info!("IFabricReplicatorBridge::GetCatchUpCapability: {:?}", lsn);
        lsn
    }
}

// primary replicator bridge
#[implement(IFabricPrimaryReplicator)]
pub struct IFabricPrimaryReplicatorBridge<E, P>
where
    E: Executor,
    P: PrimaryReplicator,
{
    inner: Arc<P>,
    rt: E,
    rplctr: IFabricReplicatorBridge<E, P>,
}

impl<E, P> IFabricPrimaryReplicatorBridge<E, P>
where
    E: Executor,
    P: PrimaryReplicator,
{
    pub fn create(rplctr: P, rt: E) -> IFabricPrimaryReplicatorBridge<E, P> {
        let inner = Arc::new(rplctr);

        // hack to construct a replicator bridge.
        // let raw: *const Box<dyn PrimaryReplicator> = Arc::into_raw(inner.clone());
        // let raw: *const Box<dyn Replicator> = raw.cast();

        // let rpl_cast = unsafe { Arc::from_raw(raw) };
        // SAFETY: This is safe because the pointer orignally came from an Arc
        // with the same size and alignment since we've checked (via Any) that
        // the object within is the type being casted to.

        let replicator_bridge =
            IFabricReplicatorBridge::create_from_primary_replicator(inner.clone(), rt.clone());

        IFabricPrimaryReplicatorBridge {
            inner,
            rt,
            rplctr: replicator_bridge,
        }
    }
}

// TODO: this impl has duplicate code with replicator bridge
impl<E, P> IFabricReplicator_Impl for IFabricPrimaryReplicatorBridge<E, P>
where
    E: Executor,
    P: PrimaryReplicator,
{
    fn BeginOpen(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginOpen(callback)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        self.rplctr.EndOpen(context)
    }

    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginChangeRole(epoch, role, callback)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndChangeRole(context)
    }

    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginUpdateEpoch(epoch, callback)
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndUpdateEpoch(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginClose(callback)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndClose(context)
    }

    fn Abort(&self) {
        self.rplctr.Abort()
    }

    fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        self.rplctr.GetCurrentProgress()
    }

    fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        self.rplctr.GetCatchUpCapability()
    }
}

impl<E, P> IFabricPrimaryReplicator_Impl for IFabricPrimaryReplicatorBridge<E, P>
where
    E: Executor,
    P: PrimaryReplicator,
{
    fn BeginOnDataLoss(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricPrimaryReplicatorBridge::BeginOnDataLoss");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.on_data_loss().await;
            let ctx_bridge: &BridgeContext<Result<u8, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOnDataLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<u8> {
        info!("IFabricPrimaryReplicatorBridge::EndOnDataLoss");
        let ctx_bridge: &BridgeContext<Result<u8, Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        let cc = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref().unwrap() });
        let pc = ReplicaSetConfig::from(unsafe { previousconfiguration.as_ref().unwrap() });
        info!("IFabricPrimaryReplicatorBridge::UpdateCatchUpReplicaSetConfiguration: curr {:?}, prev {:?}", cc, pc);
        self.inner
            .update_catch_up_replica_set_configuration(&cc, &pc)
    }

    fn BeginWaitForCatchUpQuorum(
        &self,
        catchupmode: FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!(
            "IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum: mode {:?}",
            catchupmode
        );
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.wait_for_catch_up_quorum(catchupmode.into()).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndWaitForCatchUpQuorum(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        let c = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref() }.unwrap());
        info!(
            "IFabricPrimaryReplicatorBridge::UpdateCurrentReplicaSetConfiguration {:?}",
            c
        );
        self.inner.update_current_replica_set_configuration(&c)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginBuildReplica(
        &self,
        replica: *const FABRIC_REPLICA_INFORMATION,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let r = ReplicaInfo::from(unsafe { replica.as_ref().unwrap() });
        info!("IFabricPrimaryReplicatorBridge::BeginBuildReplica: {:?}", r);

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.build_replica(&r).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndBuildReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::EndBuildReplica");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()
    }

    fn RemoveReplica(&self, replicaid: i64) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::RemoveReplica");
        self.inner.remove_replica(replicaid)
    }
}

// bridge for replica
// bridge from safe service instance to com
#[implement(IFabricStatefulServiceReplica)]

pub struct IFabricStatefulServiceReplicaBridge<E, R>
where
    E: Executor,
    R: StatefulServiceReplica + 'static,
{
    inner: Arc<R>,
    rt: E,
}

impl<E, R> IFabricStatefulServiceReplicaBridge<E, R>
where
    E: Executor,
    R: StatefulServiceReplica,
{
    pub fn create(rplctr: R, rt: E) -> IFabricStatefulServiceReplicaBridge<E, R> {
        IFabricStatefulServiceReplicaBridge {
            inner: Arc::new(rplctr),
            rt,
        }
    }
}

impl<E, R> IFabricStatefulServiceReplica_Impl for IFabricStatefulServiceReplicaBridge<E, R>
where
    E: Executor,
    R: StatefulServiceReplica,
{
    fn BeginOpen(
        &self,
        openmode: FABRIC_REPLICA_OPEN_MODE,
        partition: ::core::option::Option<&IFabricStatefulServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let openmode2: OpenMode = openmode.into();
        let partition2: StatefulServicePartition = partition.unwrap().into();
        info!(
            "IFabricStatefulReplicaBridge::BeginOpen: mode {:?}",
            openmode2
        );

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<IFabricPrimaryReplicator, Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
        let rt_cpy = self.rt.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.open(openmode2, &partition2).await;

            let com = match ok {
                Ok(rplctr) => {
                    let bridge: IFabricPrimaryReplicator =
                        IFabricPrimaryReplicatorBridge::create(rplctr, rt_cpy).into();
                    Ok(bridge)
                }
                Err(e) => Err(e),
            };

            let ctx_bridge: &BridgeContext<Result<IFabricPrimaryReplicator, Error>> =
                unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(com);
            ctx_bridge.set_complete();
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricReplicator> {
        info!("IFabricStatefulReplicaBridge::EndOpen");
        let ctx_bridge: &BridgeContext<Result<IFabricPrimaryReplicator, Error>> =
            unsafe { context.unwrap().as_impl() };
        let rplctr = ctx_bridge.consume_content()?;
        Ok(rplctr.clone().cast::<IFabricReplicator>().unwrap())
    }

    fn BeginChangeRole(
        &self,
        newrole: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let newrole2: Role = (&newrole).into();
        info!(
            "IFabricStatefulReplicaBridge::BeginChangeRole: {:?}",
            newrole2
        );

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.change_role(newrole2).await;
            let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> =
            unsafe { context.unwrap().as_impl() };
        let addr = ctx_bridge.consume_content()?;
        info!("IFabricStatefulReplicaBridge::EndChangeRole: addr {}", addr);
        Ok(HSTRINGWrap::from(addr).into())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginClose");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
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
        info!("IFabricStatefulReplicaBridge::EndClose");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn Abort(&self) {
        self.inner.as_ref().abort();
    }
}
