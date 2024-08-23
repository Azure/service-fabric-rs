// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful bridge is to wrap rs types into com to expose to SF

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use tracing::info;
use windows::core::implement;
use windows_core::{Interface, HSTRING};

use mssf_com::{
    FabricCommon::IFabricStringResult,
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
        stateful::StatefulServicePartition,
        stateful_types::{Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig},
    },
    strings::HSTRINGWrap,
    sync::{fabric_begin_bridge, fabric_end_bridge, BridgeContext3},
    types::ReplicaRole,
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
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(token)
                .await
                .map(|s| IFabricStringResult::from(HSTRINGWrap::from(s)))
        })
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricReplicatorBridge::EndOpen");
        BridgeContext3::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let role2: ReplicaRole = (&role).into();
        info!(
            "IFabricReplicatorBridge::BeginChangeRole epoch {:?}, role {:?}",
            epoch2, role2
        );

        fabric_begin_bridge(&self.rt, callback, async move {
            inner.change_role(&epoch2, &role2).await
        })
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::EndChangeRole");
        fabric_end_bridge(context)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        info!(
            "IFabricReplicatorBridge::BeginUpdateEpoch epoch {:?}",
            epoch2
        );
        fabric_begin_bridge(&self.rt, callback, async move {
            inner.update_epoch(&epoch2).await
        })
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::BeginUpdateEpoch");
        fabric_end_bridge(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginClose");
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move { inner.close(token).await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::EndClose");
        BridgeContext3::result(context)?
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
        let inner = self.inner.clone();

        fabric_begin_bridge(
            &self.rt,
            callback,
            async move { inner.on_data_loss().await },
        )
    }

    fn EndOnDataLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<u8> {
        info!("IFabricPrimaryReplicatorBridge::EndOnDataLoss");
        fabric_end_bridge(context)
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
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move {
            inner.wait_for_catch_up_quorum(catchupmode.into()).await
        })
    }

    fn EndWaitForCatchUpQuorum(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum");
        fabric_end_bridge(context)
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
        let inner = self.inner.clone();
        let r = ReplicaInfo::from(unsafe { replica.as_ref().unwrap() });
        info!("IFabricPrimaryReplicatorBridge::BeginBuildReplica: {:?}", r);
        fabric_begin_bridge(
            &self.rt,
            callback,
            async move { inner.build_replica(&r).await },
        )
    }

    fn EndBuildReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::EndBuildReplica");
        fabric_end_bridge(context)
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
        let inner = self.inner.clone();
        let rt_cp = self.rt.clone();
        let openmode2: OpenMode = openmode.into();
        let partition2: StatefulServicePartition = partition.unwrap().into();
        info!(
            "IFabricStatefulReplicaBridge::BeginOpen: mode {:?}",
            openmode2
        );
        fabric_begin_bridge(&self.rt, callback, async move {
            inner.open(openmode2, &partition2).await.map(|s| {
                let bridge: IFabricPrimaryReplicator =
                    IFabricPrimaryReplicatorBridge::create(s, rt_cp).into();
                bridge.clone().cast::<IFabricReplicator>().unwrap()
            })
        })
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricReplicator> {
        info!("IFabricStatefulReplicaBridge::EndOpen");
        fabric_end_bridge(context)
    }

    fn BeginChangeRole(
        &self,
        newrole: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let newrole2: ReplicaRole = (&newrole).into();
        info!(
            "IFabricStatefulReplicaBridge::BeginChangeRole: {:?}",
            newrole2
        );
        fabric_begin_bridge(&self.rt, callback, async move {
            inner
                .change_role(newrole2)
                .await
                .map(|s| IFabricStringResult::from(HSTRINGWrap::from(s)))
        })
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricStatefulReplicaBridge::EndChangeRole");
        fabric_end_bridge(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginClose");
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move { inner.close().await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricStatefulReplicaBridge::EndClose");
        fabric_end_bridge(context)
    }

    fn Abort(&self) {
        self.inner.as_ref().abort();
    }
}
