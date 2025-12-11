// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful bridge is to wrap rs types into com to expose to SF

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{
    Interface, runtime::stateful_proxy::StatefulServicePartition, strings::StringResult, types::Uri,
};
use windows_core::implement;

use mssf_com::{
    FabricCommon::IFabricStringResult,
    FabricRuntime::{
        IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
        IFabricReplicator_Impl, IFabricReplicatorCatchupSpecificQuorum,
        IFabricReplicatorCatchupSpecificQuorum_Impl, IFabricStatefulServiceFactory,
        IFabricStatefulServiceFactory_Impl, IFabricStatefulServicePartition,
        IFabricStatefulServicePartition3, IFabricStatefulServiceReplica,
        IFabricStatefulServiceReplica_Impl,
    },
    FabricTypes::{
        FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE, FABRIC_REPLICA_ROLE,
        FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_MODE, FABRIC_URI,
    },
};

use crate::{
    WString,
    sync::BridgeContext,
    types::{Epoch, OpenMode, ReplicaInformation, ReplicaRole, ReplicaSetConfig},
};

use super::{
    IPrimaryReplicator, IReplicator, IStatefulServiceFactory, IStatefulServiceReplica,
    executor::Executor,
};
// bridges from rs into com

// region: StatefulServiceFactoryBridge

#[implement(IFabricStatefulServiceFactory)]
pub struct StatefulServiceFactoryBridge<E>
where
    E: Executor + 'static,
{
    inner: Box<dyn IStatefulServiceFactory>,
    rt: E,
}

impl<E> StatefulServiceFactoryBridge<E>
where
    E: Executor,
{
    pub fn create(
        factory: Box<dyn IStatefulServiceFactory>,
        rt: E,
    ) -> StatefulServiceFactoryBridge<E> {
        StatefulServiceFactoryBridge { inner: factory, rt }
    }
}

impl<E> IFabricStatefulServiceFactory_Impl for StatefulServiceFactoryBridge_Impl<E>
where
    E: Executor,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn CreateReplica(
        &self,
        servicetypename: &crate::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &crate::GUID,
        replicaid: i64,
    ) -> crate::WinResult<IFabricStatefulServiceReplica> {
        let h_servicename = Uri::from(servicename);
        let h_servicetypename = WString::from(*servicetypename);
        let data = unsafe {
            if !initializationdata.is_null() {
                std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
            } else {
                &[]
            }
        };

        let replica = self.inner.create_replica(
            h_servicetypename,
            h_servicename,
            data,
            *partitionid,
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

pub struct IFabricReplicatorBridge<E>
where
    E: Executor,
{
    inner: Arc<Box<dyn IReplicator>>,
    rt: E,
}

impl<E> IFabricReplicatorBridge<E>
where
    E: Executor,
{
    pub fn create(rplctr: Box<dyn IReplicator>, rt: E) -> IFabricReplicatorBridge<E> {
        IFabricReplicatorBridge {
            inner: Arc::new(rplctr),
            rt,
        }
    }

    fn create_from_primary_replicator(
        replicator: Arc<Box<dyn IReplicator>>,
        rt: E,
    ) -> IFabricReplicatorBridge<E> {
        IFabricReplicatorBridge {
            inner: replicator,
            rt,
        }
    }
}

impl<E> IFabricReplicator_Impl for IFabricReplicatorBridge_Impl<E>
where
    E: Executor,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginOpen(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(token)
                .await
                .map(|s| IFabricStringResult::from(StringResult::new(s)))
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndOpen(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<IFabricStringResult> {
        BridgeContext::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let role2: ReplicaRole = (&role).into();

        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .change_role(epoch2, role2, token)
                .await
                .map_err(crate::WinError::from)
        })
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndChangeRole(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .update_epoch(epoch2, token)
                .await
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndUpdateEpoch(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginClose(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.close(token).await.map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndClose(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"))
    )]
    fn Abort(&self) {
        self.inner.abort();
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn GetCurrentProgress(&self) -> crate::WinResult<i64> {
        let lsn = self.inner.get_current_progress();
        lsn.map_err(crate::WinError::from)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn GetCatchUpCapability(&self) -> crate::WinResult<i64> {
        let lsn = self.inner.get_catch_up_capability();
        lsn.map_err(crate::WinError::from)
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
pub struct IFabricPrimaryReplicatorBridge<E>
where
    E: Executor,
{
    inner: Arc<Box<dyn IPrimaryReplicator>>,
    rt: E,
    rplctr: IFabricReplicator,
}

impl<E> IFabricPrimaryReplicatorBridge<E>
where
    E: Executor,
{
    pub fn create(rplctr: Box<dyn IPrimaryReplicator>, rt: E) -> IFabricPrimaryReplicatorBridge<E> {
        let inner = Arc::new(rplctr);

        // hack to construct a replicator bridge.
        let raw: *const Box<dyn IPrimaryReplicator> = Arc::into_raw(inner.clone());
        let raw: *const Box<dyn IReplicator> = raw.cast();

        // SAFETY: This is safe because the pointer orignally came from an Arc
        // with the same size and alignment since we've checked (via Any) that
        // the object within is the type being casted to.
        let rpl_cast = unsafe { Arc::from_raw(raw) };

        let replicator_bridge =
            IFabricReplicatorBridge::create_from_primary_replicator(rpl_cast, rt.clone());

        IFabricPrimaryReplicatorBridge {
            inner,
            rt,
            rplctr: replicator_bridge.into(),
        }
    }
}

// TODO: this impl has duplicate code with replicator bridge
impl<E> IFabricReplicator_Impl for IFabricPrimaryReplicatorBridge_Impl<E>
where
    E: Executor,
{
    fn BeginOpen(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        unsafe { self.rplctr.BeginOpen(callback.as_ref()) }
    }

    fn EndOpen(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<IFabricStringResult> {
        unsafe { self.rplctr.EndOpen(context.as_ref()) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        unsafe { self.rplctr.BeginChangeRole(epoch, role, callback.as_ref()) }
    }

    fn EndChangeRole(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        unsafe { self.rplctr.EndChangeRole(context.as_ref()) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        unsafe { self.rplctr.BeginUpdateEpoch(epoch, callback.as_ref()) }
    }

    fn EndUpdateEpoch(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        unsafe { self.rplctr.EndUpdateEpoch(context.as_ref()) }
    }

    fn BeginClose(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        unsafe { self.rplctr.BeginClose(callback.as_ref()) }
    }

    fn EndClose(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        unsafe { self.rplctr.EndClose(context.as_ref()) }
    }

    fn Abort(&self) {
        unsafe { self.rplctr.Abort() }
    }

    fn GetCurrentProgress(&self) -> crate::WinResult<i64> {
        unsafe { self.rplctr.GetCurrentProgress() }
    }

    fn GetCatchUpCapability(&self) -> crate::WinResult<i64> {
        unsafe { self.rplctr.GetCatchUpCapability() }
    }
}

impl<E> IFabricPrimaryReplicator_Impl for IFabricPrimaryReplicatorBridge_Impl<E>
where
    E: Executor,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginOnDataLoss(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();

        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .on_data_loss(token)
                .await
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndOnDataLoss(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<u8> {
        BridgeContext::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::WinResult<()> {
        let cc = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref().unwrap() });
        let pc = ReplicaSetConfig::from(unsafe { previousconfiguration.as_ref().unwrap() });
        self.inner
            .update_catch_up_replica_set_configuration(cc, pc)
            .map_err(crate::WinError::from)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginWaitForCatchUpQuorum(
        &self,
        catchupmode: FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let catchupmode = catchupmode.into();
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .wait_for_catch_up_quorum(catchupmode, token)
                .await
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndWaitForCatchUpQuorum(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> crate::WinResult<()> {
        let c = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref() }.unwrap());
        self.inner
            .update_current_replica_set_configuration(c)
            .map_err(crate::WinError::from)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginBuildReplica(
        &self,
        replica: *const FABRIC_REPLICA_INFORMATION,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let r = ReplicaInformation::from(unsafe { replica.as_ref().unwrap() });
        // check the parameter requirements from SF
        debug_assert_eq!(r.role, ReplicaRole::IdleSecondary);
        debug_assert_eq!(r.catch_up_capability, -1);
        debug_assert_eq!(r.current_progress, -1);

        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .build_replica(r, token)
                .await
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndBuildReplica(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn RemoveReplica(&self, replicaid: i64) -> crate::WinResult<()> {
        self.inner
            .remove_replica(replicaid)
            .map_err(crate::WinError::from)
    }
}

impl<E> IFabricReplicatorCatchupSpecificQuorum_Impl for IFabricPrimaryReplicatorBridge_Impl<E> where
    E: Executor
{
}
// endregion: IFabricPrimaryReplicatorBridge

// region: IFabricStatefulServiceReplicaBridge

// Bridge for stateful service replica
// Bridge from safe service instance to com
#[implement(IFabricStatefulServiceReplica)]

pub struct IFabricStatefulServiceReplicaBridge<E>
where
    E: Executor,
{
    inner: Arc<Box<dyn IStatefulServiceReplica>>,
    rt: E,
}

impl<E> IFabricStatefulServiceReplicaBridge<E>
where
    E: Executor,
{
    pub fn create(
        rplctr: Box<dyn IStatefulServiceReplica>,
        rt: E,
    ) -> IFabricStatefulServiceReplicaBridge<E> {
        IFabricStatefulServiceReplicaBridge {
            inner: Arc::new(rplctr),
            rt,
        }
    }
}

impl<E> IFabricStatefulServiceReplica_Impl for IFabricStatefulServiceReplicaBridge_Impl<E>
where
    E: Executor,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginOpen(
        &self,
        openmode: FABRIC_REPLICA_OPEN_MODE,
        partition: windows_core::Ref<IFabricStatefulServicePartition>,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let rt_cp = self.rt.clone();
        let openmode2: OpenMode = openmode.into();
        let com_partition = partition
            .unwrap()
            .cast::<IFabricStatefulServicePartition3>()
            .expect("cannot query interface");
        let partition = Arc::new(StatefulServicePartition::from(&com_partition));
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(openmode2, partition, token)
                .await
                .map(|s| {
                    let bridge: IFabricPrimaryReplicator =
                        IFabricPrimaryReplicatorBridge::create(s, rt_cp).into();
                    bridge.clone().cast::<IFabricReplicator>().unwrap()
                })
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndOpen(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<IFabricReplicator> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginChangeRole(
        &self,
        newrole: FABRIC_REPLICA_ROLE,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let newrole2: ReplicaRole = (&newrole).into();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .change_role(newrole2, token)
                .await
                .map(|s| IFabricStringResult::from(StringResult::new(s)))
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndChangeRole(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<IFabricStringResult> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginClose(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.close(token).await.map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndClose(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"))
    )]
    fn Abort(&self) {
        self.inner.as_ref().abort();
    }
}

// endregion: IFabricStatefulServiceReplicaBridge
