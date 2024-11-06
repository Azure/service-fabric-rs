// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful bridge is to wrap rs types into com to expose to SF

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{runtime::stateful::ReplicatorKind, Interface, HSTRING};
use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
use tracing::info;
use windows_core::implement;

use mssf_com::{
    FabricCommon::IFabricStringResult,
    FabricRuntime::{
        IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
        IFabricReplicatorCatchupSpecificQuorum, IFabricReplicatorCatchupSpecificQuorum_Impl,
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
    sync::BridgeContext3,
    types::ReplicaRole,
};

use super::{
    executor::Executor,
    stateful::{PrimaryReplicator, Replicator, StatefulServiceFactory, StatefulServiceReplica},
};
// bridges from rs into com

// region: StatefulServiceFactoryBridge

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
        servicetypename: &crate::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &crate::GUID,
        replicaid: i64,
    ) -> crate::Result<IFabricStatefulServiceReplica> {
        info!("StatefulServiceFactoryBridge::CreateReplica");
        let p_servicename = crate::PCWSTR::from_raw(servicename.0);
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

// endregion: StatefulServiceFactoryBridge

// region: IFabricReplicatorBridge

/// bridge from safe service instance to com
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
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
    ) -> crate::Result<IFabricStringResult> {
        info!("IFabricReplicatorBridge::EndOpen");
        BridgeContext3::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let role2: ReplicaRole = (&role).into();
        info!(
            "IFabricReplicatorBridge::BeginChangeRole epoch {:?}, role {:?}",
            epoch2, role2
        );

        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.change_role(&epoch2, &role2, token).await
        })
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricReplicatorBridge::EndChangeRole");
        BridgeContext3::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        info!(
            "IFabricReplicatorBridge::BeginUpdateEpoch epoch {:?}",
            epoch2
        );
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(
            &self.rt,
            async move { inner.update_epoch(&epoch2, token).await },
        )
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricReplicatorBridge::BeginUpdateEpoch");
        BridgeContext3::result(context)?
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginClose");
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move { inner.close(token).await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricReplicatorBridge::EndClose");
        BridgeContext3::result(context)?
    }

    fn Abort(&self) {
        info!("IFabricReplicatorBridge::Abort");
        self.inner.abort();
    }

    fn GetCurrentProgress(&self) -> crate::Result<i64> {
        let lsn = self.inner.get_current_progress();
        info!("IFabricReplicatorBridge::GetCurrentProgress: {:?}", lsn);
        lsn
    }

    fn GetCatchUpCapability(&self) -> crate::Result<i64> {
        let lsn = self.inner.get_catch_up_capability();
        info!("IFabricReplicatorBridge::GetCatchUpCapability: {:?}", lsn);
        lsn
    }
}

// endregion: IFabricReplicatorBridge

// region: IFabricPrimaryReplicatorBridge

/// primary replicator bridge
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginOpen(callback)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<IFabricStringResult> {
        self.rplctr.EndOpen(context)
    }

    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginChangeRole(epoch, role, callback)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        self.rplctr.EndChangeRole(context)
    }

    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginUpdateEpoch(epoch, callback)
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        self.rplctr.EndUpdateEpoch(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginClose(callback)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        self.rplctr.EndClose(context)
    }

    fn Abort(&self) {
        self.rplctr.Abort()
    }

    fn GetCurrentProgress(&self) -> crate::Result<i64> {
        self.rplctr.GetCurrentProgress()
    }

    fn GetCatchUpCapability(&self) -> crate::Result<i64> {
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricPrimaryReplicatorBridge::BeginOnDataLoss");
        let inner = self.inner.clone();

        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move { inner.on_data_loss(token).await })
    }

    fn EndOnDataLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<u8> {
        info!("IFabricPrimaryReplicatorBridge::EndOnDataLoss");
        BridgeContext3::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::Result<()> {
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        info!(
            "IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum: mode {:?}",
            catchupmode
        );
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .wait_for_catch_up_quorum(catchupmode.into(), token)
                .await
        })
    }

    fn EndWaitForCatchUpQuorum(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum");
        BridgeContext3::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::Result<()> {
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let r = ReplicaInfo::from(unsafe { replica.as_ref().unwrap() });
        info!("IFabricPrimaryReplicatorBridge::BeginBuildReplica: {:?}", r);
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(
            &self.rt,
            async move { inner.build_replica(&r, token).await },
        )
    }

    fn EndBuildReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::EndBuildReplica");
        BridgeContext3::result(context)?
    }

    fn RemoveReplica(&self, replicaid: i64) -> crate::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::RemoveReplica");
        self.inner.remove_replica(replicaid)
    }
}

// endregion: IFabricPrimaryReplicatorBridge

// region: IFabricPrimaryReplicatorCatchupSpecificQuorumBridge

/// Same as IFabricPrimaryReplicatorBridge but supports CatchupSpecificQuorum
/// It makes any IFabricPrimaryReplicator to implement IFabricReplicatorCatchupSpecificQuorum
/// as well.
#[implement(IFabricPrimaryReplicator, IFabricReplicatorCatchupSpecificQuorum)]
pub struct IFabricPrimaryReplicatorCatchupSpecificQuorumBridge {
    inner: IFabricPrimaryReplicator,
}

impl IFabricPrimaryReplicatorCatchupSpecificQuorumBridge {
    fn new(inner: IFabricPrimaryReplicator) -> Self {
        Self { inner }
    }
}

impl IFabricPrimaryReplicator_Impl for IFabricPrimaryReplicatorCatchupSpecificQuorumBridge {
    fn BeginOnDataLoss(
        &self,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginOnDataLoss(callback) }
    }

    fn EndOnDataLoss(&self, context: Option<&IFabricAsyncOperationContext>) -> crate::Result<u8> {
        unsafe { self.inner.EndOnDataLoss(context) }
    }

    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::Result<()> {
        unsafe {
            self.inner
                .UpdateCatchUpReplicaSetConfiguration(currentconfiguration, previousconfiguration)
        }
    }

    fn BeginWaitForCatchUpQuorum(
        &self,
        catchupmode: FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginWaitForCatchUpQuorum(catchupmode, callback) }
    }

    fn EndWaitForCatchUpQuorum(
        &self,
        context: Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        unsafe { self.inner.EndWaitForCatchUpQuorum(context) }
    }

    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::Result<()> {
        unsafe {
            self.inner
                .UpdateCurrentReplicaSetConfiguration(currentconfiguration)
        }
    }

    fn BeginBuildReplica(
        &self,
        replica: *const FABRIC_REPLICA_INFORMATION,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginBuildReplica(replica, callback) }
    }

    fn EndBuildReplica(&self, context: Option<&IFabricAsyncOperationContext>) -> crate::Result<()> {
        unsafe { self.inner.EndBuildReplica(context) }
    }

    fn RemoveReplica(&self, replicaid: i64) -> crate::Result<()> {
        unsafe { self.inner.RemoveReplica(replicaid) }
    }
}

impl IFabricReplicator_Impl for IFabricPrimaryReplicatorCatchupSpecificQuorumBridge {
    fn BeginOpen(
        &self,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginOpen(callback) }
    }

    fn EndOpen(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> crate::Result<IFabricStringResult> {
        unsafe { self.inner.EndOpen(context) }
    }

    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginChangeRole(epoch, role, callback) }
    }

    fn EndChangeRole(&self, context: Option<&IFabricAsyncOperationContext>) -> crate::Result<()> {
        unsafe { self.inner.EndChangeRole(context) }
    }

    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginUpdateEpoch(epoch, callback) }
    }

    fn EndUpdateEpoch(
        &self,
        context: Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        unsafe { self.inner.EndUpdateEpoch(context) }
    }

    fn BeginClose(
        &self,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext> {
        unsafe { self.inner.BeginClose(callback) }
    }

    fn EndClose(&self, context: Option<&IFabricAsyncOperationContext>) -> crate::Result<()> {
        unsafe { self.inner.EndClose(context) }
    }

    fn Abort(&self) {
        unsafe { self.inner.Abort() };
    }

    fn GetCurrentProgress(&self) -> crate::Result<i64> {
        unsafe { self.inner.GetCurrentProgress() }
    }

    fn GetCatchUpCapability(&self) -> crate::Result<i64> {
        unsafe { self.inner.GetCatchUpCapability() }
    }
}

impl IFabricReplicatorCatchupSpecificQuorum_Impl
    for IFabricPrimaryReplicatorCatchupSpecificQuorumBridge
{
}

// endregion: IFabricPrimaryReplicatorCatchupSpecificQuorumBridge

// region: IFabricStatefulServiceReplicaBridge

// Bridge for stateful service replica
// Bridge from safe service instance to com
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
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let rt_cp = self.rt.clone();
        let openmode2: OpenMode = openmode.into();
        let partition2: StatefulServicePartition = partition.unwrap().into();
        info!(
            "IFabricStatefulReplicaBridge::BeginOpen: mode {:?}",
            openmode2
        );
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(openmode2, &partition2, token)
                .await
                .map(|(rplctr, kind)| {
                    let primary: IFabricPrimaryReplicator =
                        IFabricPrimaryReplicatorBridge::create(rplctr, rt_cp).into();
                    match kind {
                        // return raw bridge
                        ReplicatorKind::Default => {
                            primary.clone().cast::<IFabricReplicator>().unwrap()
                        }
                        ReplicatorKind::CatchupSpecificQuorum => {
                            // bridge with CatchupSpecificQuorum interface enabled.
                            let bridge: IFabricPrimaryReplicator =
                                IFabricPrimaryReplicatorCatchupSpecificQuorumBridge::new(primary)
                                    .into();
                            bridge.clone().cast::<IFabricReplicator>().unwrap()
                        }
                    }
                })
        })
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<IFabricReplicator> {
        info!("IFabricStatefulReplicaBridge::EndOpen");
        BridgeContext3::result(context)?
    }

    fn BeginChangeRole(
        &self,
        newrole: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let newrole2: ReplicaRole = (&newrole).into();
        info!(
            "IFabricStatefulReplicaBridge::BeginChangeRole: {:?}",
            newrole2
        );
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .change_role(newrole2, token)
                .await
                .map(|s| IFabricStringResult::from(HSTRINGWrap::from(s)))
        })
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<IFabricStringResult> {
        info!("IFabricStatefulReplicaBridge::EndChangeRole");
        BridgeContext3::result(context)?
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginClose");
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move { inner.close(token).await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        info!("IFabricStatefulReplicaBridge::EndClose");
        BridgeContext3::result(context)?
    }

    fn Abort(&self) {
        self.inner.as_ref().abort();
    }
}

// endregion: IFabricStatefulServiceReplicaBridge

#[cfg(test)]
mod tests {
    use windows_core::implement;

    use crate::runtime::executor::DefaultExecutor;
    use crate::runtime::stateful_bridge::IFabricStatefulServiceReplicaBridge;
    use crate::runtime::stateful_proxy::StatefulServiceReplicaProxy;
    use crate::types::ReplicaRole;
    use crate::HSTRING;

    use crate::runtime::stateful_types::{
        Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, ReplicaSetQuarumMode,
    };
    use crate::sync::CancellationToken;

    use crate::runtime::stateful::{
        PrimaryReplicator, Replicator, ReplicatorKind, StatefulServicePartition,
        StatefulServiceReplica,
    };

    use mssf_com::FabricRuntime::{
        IFabricStatefulServicePartition, IFabricStatefulServicePartition_Impl,
    };

    /// Replicator for testing
    /// None of the impl details is used.
    struct MyReplicator {}

    impl Replicator for MyReplicator {
        async fn open(&self, _cancellation_token: CancellationToken) -> crate::Result<HSTRING> {
            Ok(HSTRING::from("myaddr"))
        }

        async fn close(&self, _cancellation_token: CancellationToken) -> crate::Result<()> {
            todo!()
        }

        async fn change_role(
            &self,
            _epoch: &Epoch,
            _role: &ReplicaRole,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<()> {
            todo!()
        }

        async fn update_epoch(
            &self,
            _epoch: &Epoch,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<()> {
            todo!()
        }

        fn get_current_progress(&self) -> crate::Result<i64> {
            todo!()
        }

        fn get_catch_up_capability(&self) -> crate::Result<i64> {
            todo!()
        }

        fn abort(&self) {
            todo!()
        }
    }

    impl PrimaryReplicator for MyReplicator {
        async fn on_data_loss(&self, _cancellation_token: CancellationToken) -> crate::Result<u8> {
            todo!()
        }

        fn update_catch_up_replica_set_configuration(
            &self,
            _currentconfiguration: &ReplicaSetConfig,
            _previousconfiguration: &ReplicaSetConfig,
        ) -> crate::Result<()> {
            todo!()
        }

        async fn wait_for_catch_up_quorum(
            &self,
            _catchupmode: ReplicaSetQuarumMode,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<()> {
            todo!()
        }

        fn update_current_replica_set_configuration(
            &self,
            _currentconfiguration: &ReplicaSetConfig,
        ) -> crate::Result<()> {
            todo!()
        }

        async fn build_replica(
            &self,
            _replica: &ReplicaInfo,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<()> {
            todo!()
        }

        fn remove_replica(&self, _replicaid: i64) -> crate::Result<()> {
            todo!()
        }
    }

    /// Replica for testing.
    /// Opens replicator based on catchup specific quorum flag
    struct MyServiceReplica {
        catchup_specific_quorum: bool,
    }

    impl StatefulServiceReplica for MyServiceReplica {
        async fn open(
            &self,
            _openmode: OpenMode,
            _partition: &StatefulServicePartition,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<(impl PrimaryReplicator, ReplicatorKind)> {
            let kind = match self.catchup_specific_quorum {
                true => ReplicatorKind::CatchupSpecificQuorum,
                false => ReplicatorKind::Default,
            };
            Ok((MyReplicator {}, kind))
        }
        async fn change_role(
            &self,
            _newrole: ReplicaRole,
            _cancellation_token: CancellationToken,
        ) -> crate::Result<crate::HSTRING> {
            todo!()
        }

        async fn close(&self, _cancellation_token: CancellationToken) -> crate::Result<()> {
            todo!()
        }

        fn abort(&self) {
            todo!()
        }
    }

    /// Partition COM obj for testing. None of the function is called.
    #[implement(IFabricStatefulServicePartition)]
    struct MyComPartition {}

    impl IFabricStatefulServicePartition_Impl for MyComPartition {
        fn GetPartitionInfo(
            &self,
        ) -> crate::Result<
            *mut mssf_com::ServiceFabric::FabricTypes::FABRIC_SERVICE_PARTITION_INFORMATION,
        > {
            todo!()
        }

        fn GetReadStatus(
            &self,
        ) -> crate::Result<
            mssf_com::ServiceFabric::FabricTypes::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
        > {
            todo!()
        }

        fn GetWriteStatus(
            &self,
        ) -> crate::Result<
            mssf_com::ServiceFabric::FabricTypes::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
        > {
            todo!()
        }

        fn CreateReplicator(
            &self,
            _stateprovider: Option<&mssf_com::FabricRuntime::IFabricStateProvider>,
            _replicatorsettings: *const mssf_com::ServiceFabric::FabricTypes::FABRIC_REPLICATOR_SETTINGS,
            _replicator: *mut Option<mssf_com::FabricRuntime::IFabricReplicator>,
        ) -> crate::Result<mssf_com::FabricRuntime::IFabricStateReplicator> {
            todo!()
        }

        fn ReportLoad(
            &self,
            _metriccount: u32,
            _metrics: *const mssf_com::ServiceFabric::FabricTypes::FABRIC_LOAD_METRIC,
        ) -> crate::Result<()> {
            todo!()
        }

        fn ReportFault(
            &self,
            _faulttype: mssf_com::ServiceFabric::FabricTypes::FABRIC_FAULT_TYPE,
        ) -> crate::Result<()> {
            todo!()
        }
    }

    #[tokio::test]
    async fn replicator_bridge_test() {
        replicator_bridge_test_inner(true).await;
        replicator_bridge_test_inner(false).await;
    }

    /// Use service replica proxy to create replica and check
    /// the replica kind is currectly bridged and proxyed.
    async fn replicator_bridge_test_inner(catchup_specific_quorum: bool) {
        let rt = DefaultExecutor::new(tokio::runtime::Handle::current());
        // make a mock service replica
        let replica = MyServiceReplica {
            catchup_specific_quorum,
        };
        let replica_bridge = IFabricStatefulServiceReplicaBridge::create(replica, rt).into();
        let replica_proxy = StatefulServiceReplicaProxy::new(replica_bridge);

        let com_partition: IFabricStatefulServicePartition = MyComPartition {}.into();
        let partition = StatefulServicePartition::from(&com_partition);

        // create replicator from replica and check replicator kind
        let (rplctr, kind) = replica_proxy
            .open(OpenMode::Existing, &partition, CancellationToken::new())
            .await
            .unwrap();

        match catchup_specific_quorum {
            true => assert_eq!(kind, ReplicatorKind::CatchupSpecificQuorum),
            false => assert_eq!(kind, ReplicatorKind::Default),
        }
        let addr = rplctr.open(CancellationToken::new()).await.unwrap();
        assert_eq!(addr, "myaddr");
    }
}
