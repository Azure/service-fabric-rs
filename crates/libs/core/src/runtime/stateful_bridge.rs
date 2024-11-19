// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful bridge is to wrap rs types into com to expose to SF

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{runtime::stateful_proxy::StatefulServicePartition, Interface, HSTRING};
use tracing::info;
use windows_core::implement;

use mssf_com::{
    FabricCommon::IFabricStringResult,
    FabricRuntime::{
        IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
        IFabricReplicatorCatchupSpecificQuorum, IFabricReplicatorCatchupSpecificQuorum_Impl,
        IFabricReplicator_Impl, IFabricStatefulServiceFactory, IFabricStatefulServiceFactory_Impl,
        IFabricStatefulServicePartition, IFabricStatefulServicePartition3,
        IFabricStatefulServiceReplica, IFabricStatefulServiceReplica_Impl,
    },
    FabricTypes::{
        FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE, FABRIC_REPLICA_ROLE,
        FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_MODE, FABRIC_URI,
    },
};

use crate::{
    strings::HSTRINGWrap,
    sync::BridgeContext3,
    types::{Epoch, OpenMode, ReplicaInformation, ReplicaRole, ReplicaSetConfig},
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

/// Primary replicator bridge.
/// mssf_core only supports primary replicator with IFabricReplicatorCatchupSpecificQuorum enabled,
/// which allows an IReplicator to indicate that it supports catching up specific quorums with the
/// use of the MustCatchup flag in ReplicaInformation.
// Nearly all replicators in cpp and csharp all enables CatchupSpecificQuorum, and not enabling it
// is a rare case.
#[implement(IFabricPrimaryReplicator, IFabricReplicatorCatchupSpecificQuorum)]
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
        let catchupmode = catchupmode.into();
        info!(
            "IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum: mode {:?}",
            catchupmode
        );
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.wait_for_catch_up_quorum(catchupmode, token).await
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
        let r = ReplicaInformation::from(unsafe { replica.as_ref().unwrap() });
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
        info!("IFabricPrimaryReplicatorBridge::RemoveReplica: replicaid {replicaid}");
        self.inner.remove_replica(replicaid)
    }
}

impl<E, P> IFabricReplicatorCatchupSpecificQuorum_Impl for IFabricPrimaryReplicatorBridge<E, P>
where
    E: Executor,
    P: PrimaryReplicator,
{
}
// endregion: IFabricPrimaryReplicatorBridge

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
        let com_partition = partition
            .unwrap()
            .cast::<IFabricStatefulServicePartition3>()
            .expect("cannot query interface");
        let partition = StatefulServicePartition::from(&com_partition);
        info!(
            "IFabricStatefulReplicaBridge::BeginOpen: mode {:?}",
            openmode2
        );
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.open(openmode2, &partition, token).await.map(|s| {
                let bridge: IFabricPrimaryReplicator =
                    IFabricPrimaryReplicatorBridge::create(s, rt_cp).into();
                bridge.clone().cast::<IFabricReplicator>().unwrap()
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
