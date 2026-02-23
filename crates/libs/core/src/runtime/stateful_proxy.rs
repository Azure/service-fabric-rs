// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_proxy is a wrapper layer around com api,
// making manipulating com simple.

use std::{ffi::c_void, sync::Arc};

use crate::{Interface, WString, runtime::executor::BoxedCancelToken, strings::StringResult};
use mssf_com::FabricRuntime::{
    IFabricKeyValueStoreReplica8, IFabricPrimaryReplicator, IFabricReplicator,
    IFabricReplicatorCatchupSpecificQuorum, IFabricStatefulServicePartition3,
};

use crate::{
    error::ErrorCode,
    sync::fabric_begin_end_proxy,
    types::{
        FaultType, HealthInformation, LoadMetric, LoadMetricListRef, MoveCost, ReplicaRole,
        ServicePartitionAccessStatus, ServicePartitionInformation,
    },
};

use super::{IPrimaryReplicator, IReplicator, IStatefulServiceReplica};
use crate::types::{Epoch, OpenMode, ReplicaInformation, ReplicaSetConfig, ReplicaSetQuorumMode};

pub struct StatefulServiceReplicaProxy {
    com_impl: IFabricKeyValueStoreReplica8,
}

impl StatefulServiceReplicaProxy {
    pub fn new(com_impl: IFabricKeyValueStoreReplica8) -> StatefulServiceReplicaProxy {
        StatefulServiceReplicaProxy { com_impl }
    }
}

#[async_trait::async_trait]
impl IStatefulServiceReplica for StatefulServiceReplicaProxy {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", err) // TODO: trace ret
    )]
    async fn open(
        &self,
        openmode: OpenMode,
        partition: Arc<dyn super::IStatefulServicePartition>,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<Box<dyn IPrimaryReplicator>> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginOpen(
                    openmode.into(),
                    partition.try_get_com().expect("must support com"),
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndOpen(ctx) },
            Some(cancellation_token),
        );
        let rplctr = rx.await??;

        // Check COM interface is implemented.
        let catchup_specific_quorum = rplctr
            .cast::<IFabricReplicatorCatchupSpecificQuorum>()
            .is_ok();
        assert!(
            catchup_specific_quorum,
            "mssf does not support replicator without catchup_specific_quorum interface"
        );

        // TODO: cast without clone will cause access violation on AddRef in SF runtime.
        let p_rplctr: IFabricPrimaryReplicator = rplctr.clone().cast().unwrap(); // must work
        // Replicator must impl primary replicator as well.

        let res = Box::new(PrimaryReplicatorProxy::new(p_rplctr));
        Ok(res)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<WString> {
        // replica address
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginChangeRole((&newrole).into(), callback) },
            move |ctx| unsafe { com2.EndChangeRole(ctx) },
            Some(cancellation_token),
        );
        let addr = rx.await??;
        Ok(StringResult::from(&addr).into_inner())
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginClose(callback) },
            move |ctx| unsafe { com2.EndClose(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret)
    )]
    fn abort(&self) {
        unsafe { self.com_impl.Abort() }
    }
}

pub struct ReplicatorProxy {
    com_impl: IFabricReplicator,
}

impl ReplicatorProxy {
    fn new(com_impl: IFabricReplicator) -> ReplicatorProxy {
        ReplicatorProxy { com_impl }
    }
}

#[async_trait::async_trait]
impl IReplicator for ReplicatorProxy {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn open(&self, cancellation_token: BoxedCancelToken) -> crate::Result<WString> {
        // replicator address
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginOpen(callback) },
            move |ctx| unsafe { com2.EndOpen(ctx) },
            Some(cancellation_token),
        );
        let addr = rx.await??;
        Ok(StringResult::from(&addr).into_inner())
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginClose(callback) },
            move |ctx| unsafe { com2.EndClose(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn change_role(
        &self,
        epoch: Epoch,
        role: ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let fabric_epoch: mssf_com::FabricTypes::FABRIC_EPOCH = (&epoch).into();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginChangeRole(&fabric_epoch, (&role).into(), callback)
            },
            move |ctx| unsafe { com2.EndChangeRole(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn update_epoch(
        &self,
        epoch: Epoch,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let fabric_epoch: mssf_com::FabricTypes::FABRIC_EPOCH = (&epoch).into();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginUpdateEpoch(&fabric_epoch, callback) },
            move |ctx| unsafe { com2.EndUpdateEpoch(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    fn get_current_progress(&self) -> crate::Result<i64> {
        unsafe { self.com_impl.GetCurrentProgress() }.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    fn get_catch_up_capability(&self) -> crate::Result<i64> {
        unsafe { self.com_impl.GetCatchUpCapability() }.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret)
    )]
    fn abort(&self) {
        unsafe { self.com_impl.Abort() }
    }
}

pub struct PrimaryReplicatorProxy {
    com_impl: IFabricPrimaryReplicator,
    parent: ReplicatorProxy,
}

impl PrimaryReplicatorProxy {
    pub fn new(com_impl: IFabricPrimaryReplicator) -> PrimaryReplicatorProxy {
        let parent = ReplicatorProxy::new(com_impl.clone().cast().unwrap());
        PrimaryReplicatorProxy { com_impl, parent }
    }
}

#[async_trait::async_trait]
impl IReplicator for PrimaryReplicatorProxy {
    async fn open(&self, cancellation_token: BoxedCancelToken) -> crate::Result<WString> {
        self.parent.open(cancellation_token).await
    }
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()> {
        self.parent.close(cancellation_token).await
    }
    async fn change_role(
        &self,
        epoch: Epoch,
        role: ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        self.parent
            .change_role(epoch, role, cancellation_token)
            .await
    }
    async fn update_epoch(
        &self,
        epoch: Epoch,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        self.parent.update_epoch(epoch, cancellation_token).await
    }
    fn get_current_progress(&self) -> crate::Result<i64> {
        self.parent.get_current_progress()
    }
    fn get_catch_up_capability(&self) -> crate::Result<i64> {
        self.parent.get_catch_up_capability()
    }
    fn abort(&self) {
        self.parent.abort()
    }
}

#[async_trait::async_trait]
impl IPrimaryReplicator for PrimaryReplicatorProxy {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn on_data_loss(&self, cancellation_token: BoxedCancelToken) -> crate::Result<u8> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginOnDataLoss(callback) },
            move |ctx| unsafe { com2.EndOnDataLoss(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
        previousconfiguration: ReplicaSetConfig,
    ) -> crate::Result<()> {
        let cc_view = currentconfiguration.get_view();
        let pc_view = previousconfiguration.get_view();
        unsafe {
            self.com_impl
                .UpdateCatchUpReplicaSetConfiguration(cc_view.get_raw(), pc_view.get_raw())
        }
        .map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuorumMode,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginWaitForCatchUpQuorum(catchupmode.into(), callback) },
            move |ctx| unsafe { com2.EndWaitForCatchUpQuorum(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
    ) -> crate::Result<()> {
        unsafe {
            self.com_impl
                .UpdateCurrentReplicaSetConfiguration(currentconfiguration.get_view().get_raw())
        }
        .map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    async fn build_replica(
        &self,
        replica: ReplicaInformation,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| {
                let (mut info, ex1) = replica.get_raw_parts();
                info.Reserved = std::ptr::addr_of!(ex1) as *mut c_void;
                unsafe { com1.BeginBuildReplica(&info, callback) }
            },
            move |ctx| unsafe { com2.EndBuildReplica(ctx) },
            Some(cancellation_token),
        );
        rx.await?.map_err(crate::Error::from)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret, err)
    )]
    fn remove_replica(&self, replicaid: i64) -> crate::Result<()> {
        unsafe { self.com_impl.RemoveReplica(replicaid) }.map_err(crate::Error::from)
    }
}

/// Proxy COM object IFabricStatefulServicePartition3
#[derive(Debug, Clone)]
pub struct StatefulServicePartition {
    com_impl: IFabricStatefulServicePartition3,
}

impl super::IStatefulServicePartition for StatefulServicePartition {
    fn get_partition_information(&self) -> crate::Result<ServicePartitionInformation> {
        unsafe { self.com_impl.GetPartitionInfo()?.as_ref() }
            .ok_or(ErrorCode::E_POINTER.into())
            .map(ServicePartitionInformation::from)
    }

    fn get_read_status(&self) -> crate::Result<ServicePartitionAccessStatus> {
        unsafe { self.com_impl.GetReadStatus() }
            .map(ServicePartitionAccessStatus::from)
            .map_err(crate::Error::from)
    }

    fn get_write_status(&self) -> crate::Result<ServicePartitionAccessStatus> {
        unsafe { self.com_impl.GetWriteStatus() }
            .map(ServicePartitionAccessStatus::from)
            .map_err(crate::Error::from)
    }

    /// TODO: not implemented
    fn create_replicator(&self) -> crate::Result<Box<dyn IPrimaryReplicator>> {
        Err(ErrorCode::E_NOTIMPL.into())
    }

    fn report_load(&self, metrics: &[LoadMetric]) -> crate::Result<()> {
        let metrics_ref = LoadMetricListRef::from_slice(metrics);
        let raw = metrics_ref.as_raw_slice();
        unsafe { self.com_impl.ReportLoad(raw) }.map_err(crate::Error::from)
    }

    fn report_fault(&self, fault_type: FaultType) -> crate::Result<()> {
        unsafe { self.com_impl.ReportFault(fault_type.into()) }.map_err(crate::Error::from)
    }

    fn report_move_cost(&self, move_cost: MoveCost) -> crate::Result<()> {
        unsafe { self.com_impl.ReportMoveCost(move_cost.into()) }.map_err(crate::Error::from)
    }

    fn report_partition_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportPartitionHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }

    fn report_replica_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportReplicaHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }

    fn try_get_com(
        &self,
    ) -> crate::Result<&mssf_com::FabricRuntime::IFabricStatefulServicePartition> {
        Ok(&self.com_impl)
    }
}

impl From<&IFabricStatefulServicePartition3> for StatefulServicePartition {
    fn from(e: &IFabricStatefulServicePartition3) -> Self {
        StatefulServicePartition {
            com_impl: e.clone(),
        }
    }
}
