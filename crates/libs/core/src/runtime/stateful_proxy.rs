// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_proxy is a wrapper layer around com api,
// making manipulating com simple.

use std::ffi::c_void;

use crate::{Interface, WString, runtime::executor::BoxedCancelToken};
use mssf_com::FabricRuntime::{
    IFabricPrimaryReplicator, IFabricReplicator, IFabricReplicatorCatchupSpecificQuorum,
    IFabricStatefulServicePartition3, IFabricStatefulServiceReplica,
};

use crate::{
    error::ErrorCode,
    strings::WStringWrap,
    sync::fabric_begin_end_proxy,
    types::{
        FaultType, HealthInformation, LoadMetric, LoadMetricListRef, MoveCost, ReplicaRole,
        ServicePartitionAccessStatus, ServicePartitionInformation,
    },
};

use super::stateful::{PrimaryReplicator, Replicator, StatefulServiceReplica};
use crate::types::{Epoch, OpenMode, ReplicaInformation, ReplicaSetConfig, ReplicaSetQuorumMode};

pub struct StatefulServiceReplicaProxy {
    com_impl: IFabricStatefulServiceReplica,
}

impl StatefulServiceReplicaProxy {
    pub fn new(com_impl: IFabricStatefulServiceReplica) -> StatefulServiceReplicaProxy {
        StatefulServiceReplicaProxy { com_impl }
    }
}

impl StatefulServiceReplica for StatefulServiceReplicaProxy {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", err) // TODO: trace ret
    )]
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<impl PrimaryReplicator> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginOpen(openmode.into(), partition.get_com(), callback)
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

        let res = PrimaryReplicatorProxy::new(p_rplctr);
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
        Ok(WStringWrap::from(&addr).into())
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

impl Replicator for ReplicatorProxy {
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
        Ok(WStringWrap::from(&addr).into())
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
        epoch: &Epoch,
        role: &ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginChangeRole(&epoch.into(), role.into(), callback) },
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
        epoch: &Epoch,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginUpdateEpoch(&epoch.into(), callback) },
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

impl Replicator for PrimaryReplicatorProxy {
    async fn open(&self, cancellation_token: BoxedCancelToken) -> crate::Result<WString> {
        self.parent.open(cancellation_token).await
    }
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()> {
        self.parent.close(cancellation_token).await
    }
    async fn change_role(
        &self,
        epoch: &Epoch,
        role: &ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()> {
        self.parent
            .change_role(epoch, role, cancellation_token)
            .await
    }
    async fn update_epoch(
        &self,
        epoch: &Epoch,
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

impl PrimaryReplicator for PrimaryReplicatorProxy {
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
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
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
        currentconfiguration: &ReplicaSetConfig,
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
        replica: &ReplicaInformation,
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

impl StatefulServicePartition {
    pub fn get_com(&self) -> &IFabricStatefulServicePartition3 {
        &self.com_impl
    }

    /// Provides access to the ServicePartitionInformation of the service, which contains the partition type and ID.
    pub fn get_partition_information(&self) -> crate::Result<ServicePartitionInformation> {
        unsafe { self.com_impl.GetPartitionInfo()?.as_ref() }
            .ok_or(ErrorCode::E_POINTER.into())
            .map(ServicePartitionInformation::from)
    }

    /// Used to check the readiness of the replica in regard to read operations.
    /// The ReadStatus should be checked before the replica is servicing a customer request that is a read operation.
    pub fn get_read_status(&self) -> crate::Result<ServicePartitionAccessStatus> {
        unsafe { self.com_impl.GetReadStatus() }
            .map(ServicePartitionAccessStatus::from)
            .map_err(crate::Error::from)
    }

    /// Used to check the readiness of the partition in regard to write operations.
    /// The WriteStatus should be checked before the replica services a customer request that is a write operation.
    pub fn get_write_status(&self) -> crate::Result<ServicePartitionAccessStatus> {
        unsafe { self.com_impl.GetWriteStatus() }
            .map(ServicePartitionAccessStatus::from)
            .map_err(crate::Error::from)
    }

    /// TODO: not implemented
    /// Creates a FabricReplicator with the specified settings and returns it to the replica.
    pub fn create_replicator(&self) -> crate::Result<()> {
        Err(ErrorCode::E_NOTIMPL.into())
    }

    /// Reports load for the current replica in the partition.
    /// Remarks:
    /// The reported metrics should correspond to those that are provided in the ServiceLoadMetricDescription
    /// as a part of the ServiceDescription that is used to create the service. Load metrics that are not
    /// present in the description are ignored. Reporting custom metrics allows Service Fabric to balance
    /// services that are based on additional custom information.
    pub fn report_load(&self, metrics: &[LoadMetric]) -> crate::Result<()> {
        let metrics_ref = LoadMetricListRef::from_slice(metrics);
        let raw = metrics_ref.as_raw_slice();
        unsafe { self.com_impl.ReportLoad(raw) }.map_err(crate::Error::from)
    }

    /// Enables the replica to report a fault to the runtime and indicates that it has encountered
    /// an error from which it cannot recover and must either be restarted or removed.
    pub fn report_fault(&self, fault_type: FaultType) -> crate::Result<()> {
        unsafe { self.com_impl.ReportFault(fault_type.into()) }.map_err(crate::Error::from)
    }

    /// Reports the move cost for a replica.
    /// Remarks:
    /// Services can report move cost of a replica using this method.
    /// While the Service Fabric Resource Balances searches for the best balance in the cluster,
    /// it examines both load information and move cost of each replica.
    /// Resource balances will prefer to move replicas with lower cost in order to achieve balance.
    pub fn report_move_cost(&self, move_cost: MoveCost) -> crate::Result<()> {
        unsafe { self.com_impl.ReportMoveCost(move_cost.into()) }.map_err(crate::Error::from)
    }

    // Remarks:
    // The health information describes the report details, like the source ID, the property,
    // the health state and other relevant details. The partition uses an internal health client
    // to send the reports to the health store. The client optimizes messages to Health Manager
    // by batching reports per a configured duration (Default: 30 seconds). If the report has high priority,
    // you can specify send options to send it immediately.

    /// Reports current partition health.
    pub fn report_partition_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportPartitionHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }

    /// Reports health on the current stateful service replica of the partition.
    pub fn report_replica_health(&self, healthinfo: &HealthInformation) -> crate::Result<()> {
        let healthinfo_ref = &healthinfo.into();
        unsafe { self.com_impl.ReportReplicaHealth(healthinfo_ref) }.map_err(crate::Error::from)
    }
}

impl From<&IFabricStatefulServicePartition3> for StatefulServicePartition {
    fn from(e: &IFabricStatefulServicePartition3) -> Self {
        StatefulServicePartition {
            com_impl: e.clone(),
        }
    }
}
