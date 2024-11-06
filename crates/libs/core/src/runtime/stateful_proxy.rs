// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_proxy is a wrapper layer around com api,
// making manipulating com simple.

use std::ffi::c_void;

use mssf_com::FabricRuntime::{
    IFabricPrimaryReplicator, IFabricReplicator, IFabricReplicatorCatchupSpecificQuorum,
    IFabricStatefulServiceReplica,
};
use tracing::info;
use windows_core::{Interface, HSTRING};

use crate::{
    strings::HSTRINGWrap,
    sync::{fabric_begin_end_proxy2, CancellationToken},
    types::ReplicaRole,
};

use super::{
    stateful::{
        PrimaryReplicator, Replicator, ReplicatorKind, StatefulServicePartition,
        StatefulServiceReplica,
    },
    stateful_types::{Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, ReplicaSetQuarumMode},
};

pub struct StatefulServiceReplicaProxy {
    com_impl: IFabricStatefulServiceReplica,
}

impl StatefulServiceReplicaProxy {
    pub fn new(com_impl: IFabricStatefulServiceReplica) -> StatefulServiceReplicaProxy {
        StatefulServiceReplicaProxy { com_impl }
    }
}

impl StatefulServiceReplica for StatefulServiceReplicaProxy {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
        cancellation_token: CancellationToken,
    ) -> crate::Result<(impl PrimaryReplicator, ReplicatorKind)> {
        info!("StatefulServiceReplicaProxy::open with mode {:?}", openmode);
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginOpen(openmode.into(), partition.get_com(), callback)
            },
            move |ctx| unsafe { com2.EndOpen(ctx) },
            Some(cancellation_token),
        );
        let rplctr = rx.await??;
        // TODO: cast without clone will cause access violation on AddRef in SF runtime.
        let p_rplctr: IFabricPrimaryReplicator = rplctr.clone().cast().unwrap(); // must work
                                                                                 // Replicator must impl primary replicator as well.
        let rplctr_kind = match p_rplctr
            .cast::<IFabricReplicatorCatchupSpecificQuorum>()
            .is_ok()
        {
            true => ReplicatorKind::CatchupSpecificQuorum,
            false => ReplicatorKind::Default,
        };
        let res = PrimaryReplicatorProxy::new(p_rplctr);
        Ok((res, rplctr_kind))
    }
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        cancellation_token: CancellationToken,
    ) -> crate::Result<HSTRING> {
        // replica address
        info!("StatefulServiceReplicaProxy::change_role {:?}", newrole);
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginChangeRole((&newrole).into(), callback) },
            move |ctx| unsafe { com2.EndChangeRole(ctx) },
            Some(cancellation_token),
        );
        let addr = rx.await??;
        Ok(HSTRINGWrap::from(&addr).into())
    }
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()> {
        info!("StatefulServiceReplicaProxy::close");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginClose(callback) },
            move |ctx| unsafe { com2.EndClose(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    fn abort(&self) {
        info!("StatefulServiceReplicaProxy::abort");
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
    async fn open(&self, cancellation_token: CancellationToken) -> crate::Result<HSTRING> {
        info!("ReplicatorProxy::open");
        // replicator address
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginOpen(callback) },
            move |ctx| unsafe { com2.EndOpen(ctx) },
            Some(cancellation_token),
        );
        let addr = rx.await??;
        Ok(HSTRINGWrap::from(&addr).into())
    }
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()> {
        info!("ReplicatorProxy::close");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginClose(callback) },
            move |ctx| unsafe { com2.EndClose(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    async fn change_role(
        &self,
        epoch: &Epoch,
        role: &ReplicaRole,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()> {
        info!("ReplicatorProxy::change_role");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginChangeRole(&epoch.into(), role.into(), callback) },
            move |ctx| unsafe { com2.EndChangeRole(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    async fn update_epoch(
        &self,
        epoch: &Epoch,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()> {
        info!("ReplicatorProxy::update_epoch");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginUpdateEpoch(&epoch.into(), callback) },
            move |ctx| unsafe { com2.EndUpdateEpoch(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    fn get_current_progress(&self) -> crate::Result<i64> {
        info!("ReplicatorProxy::get_current_progress");
        unsafe { self.com_impl.GetCurrentProgress() }
    }
    fn get_catch_up_capability(&self) -> crate::Result<i64> {
        info!("ReplicatorProxy::get_catch_up_capability");
        unsafe { self.com_impl.GetCatchUpCapability() }
    }
    fn abort(&self) {
        info!("ReplicatorProxy::abort");
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

    pub fn get_com(&self) -> &IFabricPrimaryReplicator {
        &self.com_impl
    }
}

impl Replicator for PrimaryReplicatorProxy {
    async fn open(&self, cancellation_token: CancellationToken) -> crate::Result<HSTRING> {
        self.parent.open(cancellation_token).await
    }
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()> {
        self.parent.close(cancellation_token).await
    }
    async fn change_role(
        &self,
        epoch: &Epoch,
        role: &ReplicaRole,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()> {
        self.parent
            .change_role(epoch, role, cancellation_token)
            .await
    }
    async fn update_epoch(
        &self,
        epoch: &Epoch,
        cancellation_token: CancellationToken,
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
    async fn on_data_loss(&self, cancellation_token: CancellationToken) -> crate::Result<u8> {
        info!("PrimaryReplicatorProxy::on_data_loss");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginOnDataLoss(callback) },
            move |ctx| unsafe { com2.EndOnDataLoss(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> crate::Result<()> {
        info!("PrimaryReplicatorProxy::update_catch_up_replica_set_configuration");
        let cc_view = currentconfiguration.get_view();
        let pc_view = previousconfiguration.get_view();
        unsafe {
            self.com_impl
                .UpdateCatchUpReplicaSetConfiguration(cc_view.get_raw(), pc_view.get_raw())
        }
    }
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()> {
        info!("PrimaryReplicatorProxy::wait_for_catch_up_quorum");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| unsafe { com1.BeginWaitForCatchUpQuorum(catchupmode.into(), callback) },
            move |ctx| unsafe { com2.EndWaitForCatchUpQuorum(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> crate::Result<()> {
        info!("PrimaryReplicatorProxy::update_current_replica_set_configuration");
        unsafe {
            self.com_impl
                .UpdateCurrentReplicaSetConfiguration(currentconfiguration.get_view().get_raw())
        }
    }
    async fn build_replica(
        &self,
        replica: &ReplicaInfo,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()> {
        info!("PrimaryReplicatorProxy::build_replica");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy2(
            move |callback| {
                let (mut info, ex1) = replica.get_raw_parts();
                info.Reserved = std::ptr::addr_of!(ex1) as *mut c_void;
                unsafe { com1.BeginBuildReplica(&info, callback) }
            },
            move |ctx| unsafe { com2.EndBuildReplica(ctx) },
            Some(cancellation_token),
        );
        rx.await?
    }
    fn remove_replica(&self, replicaid: i64) -> crate::Result<()> {
        info!("PrimaryReplicatorProxy::remove_replica");
        unsafe { self.com_impl.RemoveReplica(replicaid) }
    }
}
