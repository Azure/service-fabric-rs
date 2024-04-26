// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_proxy is a wrapper layer around com api,
// making manipulating com simple.

use log::info;
use mssf_com::{
    FabricCommon::FabricRuntime::{
        IFabricPrimaryReplicator, IFabricReplicator, IFabricStatefulServiceReplica,
    },
    FABRIC_EPOCH,
};
use windows_core::{Interface, HSTRING};

use crate::strings::HSTRINGWrap;

use super::{
    stateful::{PrimaryReplicator, Replicator, StatefulServicePartition, StatefulServiceReplica},
    stateful_types::{Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, ReplicaSetQuarumMode, Role},
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
    ) -> windows::core::Result<impl PrimaryReplicator> {
        info!("StatefulServiceReplicaProxy::open with mode {:?}", openmode);
        // replicator address
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndOpen(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });

        let _ = unsafe {
            self.com_impl
                .BeginOpen(openmode.into(), partition.get_com(), &callback)?
        };
        let rplctr = rx.await.unwrap()?;
        // TODO: cast without clone will cause access violation on AddRef in SF runtime.
        let p_rplctr: IFabricPrimaryReplicator = rplctr.clone().cast().unwrap(); // must work
                                                                                 // Replicator must impl primary replicator as well.
        let res = PrimaryReplicatorProxy::new(p_rplctr);
        Ok(res)
    }
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING> {
        // replica address
        info!("StatefulServiceReplicaProxy::change_role {:?}", newrole);
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndChangeRole(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });

        let _ = unsafe { self.com_impl.BeginChangeRole(newrole.into(), &callback)? };
        let addr = rx.await.unwrap()?;
        Ok(HSTRINGWrap::from(&addr).into())
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("StatefulServiceReplicaProxy::close");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndClose(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });

        let _ = unsafe { self.com_impl.BeginClose(&callback)? };
        rx.await.unwrap()?;
        Ok(())
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
    async fn open(&self) -> ::windows_core::Result<HSTRING> {
        info!("ReplicatorProxy::open");
        // replicator address
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndOpen(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let _ = unsafe { self.com_impl.BeginOpen(&callback)? };
        let addr = rx.await.unwrap()?;
        Ok(HSTRINGWrap::from(&addr).into())
    }
    async fn close(&self) -> ::windows_core::Result<()> {
        info!("ReplicatorProxy::close");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndClose(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let _ = unsafe { self.com_impl.BeginClose(&callback)? };
        rx.await.unwrap()
    }
    async fn change_role(&self, epoch: &Epoch, role: &Role) -> ::windows_core::Result<()> {
        info!("ReplicatorProxy::change_role");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndChangeRole(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        {
            let epoch2: FABRIC_EPOCH = epoch.clone().into();
            let _ = unsafe {
                self.com_impl
                    .BeginChangeRole(&epoch2, role.clone().into(), &callback)?
            };
        }
        rx.await.unwrap()
    }
    async fn update_epoch(&self, epoch: &Epoch) -> ::windows_core::Result<()> {
        info!("ReplicatorProxy::update_epoch");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndUpdateEpoch(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        {
            let epoch2: FABRIC_EPOCH = epoch.clone().into();
            let _ = unsafe { self.com_impl.BeginUpdateEpoch(&epoch2, &callback)? };
        }
        rx.await.unwrap()
    }
    fn get_current_progress(&self) -> ::windows_core::Result<i64> {
        info!("ReplicatorProxy::get_current_progress");
        unsafe { self.com_impl.GetCurrentProgress() }
    }
    fn get_catch_up_capability(&self) -> ::windows_core::Result<i64> {
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
}

impl Replicator for PrimaryReplicatorProxy {
    async fn open(&self) -> ::windows_core::Result<HSTRING> {
        self.parent.open().await
    }
    async fn close(&self) -> ::windows_core::Result<()> {
        self.parent.close().await
    }
    async fn change_role(&self, epoch: &Epoch, role: &Role) -> ::windows_core::Result<()> {
        self.parent.change_role(epoch, role).await
    }
    async fn update_epoch(&self, epoch: &Epoch) -> ::windows_core::Result<()> {
        self.parent.update_epoch(epoch).await
    }
    fn get_current_progress(&self) -> ::windows_core::Result<i64> {
        self.parent.get_current_progress()
    }
    fn get_catch_up_capability(&self) -> ::windows_core::Result<i64> {
        self.parent.get_catch_up_capability()
    }
    fn abort(&self) {
        self.parent.abort()
    }
}

impl PrimaryReplicator for PrimaryReplicatorProxy {
    async fn on_data_loss(&self) -> ::windows_core::Result<u8> {
        info!("PrimaryReplicatorProxy::on_data_loss");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndOnDataLoss(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        {
            let _ = unsafe { self.com_impl.BeginOnDataLoss(&callback)? };
        }
        rx.await.unwrap()
    }
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()> {
        info!("PrimaryReplicatorProxy::update_catch_up_replica_set_configuration");
        let cc = currentconfiguration.get_raw();
        let pc = previousconfiguration.get_raw();
        unsafe { self.com_impl.UpdateCatchUpReplicaSetConfiguration(&cc, &pc) }
    }
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
    ) -> ::windows_core::Result<()> {
        info!("PrimaryReplicatorProxy::wait_for_catch_up_quorum");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndWaitForCatchUpQuorum(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        {
            let _ = unsafe {
                self.com_impl
                    .BeginWaitForCatchUpQuorum(catchupmode.into(), &callback)?
            };
        }
        rx.await.unwrap()
    }
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()> {
        info!("PrimaryReplicatorProxy::update_current_replica_set_configuration");
        unsafe {
            self.com_impl
                .UpdateCurrentReplicaSetConfiguration(&currentconfiguration.get_raw())
        }
    }
    async fn build_replica(&self, replica: &ReplicaInfo) -> ::windows_core::Result<()> {
        info!("PrimaryReplicatorProxy::build_replica");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndBuildReplica(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        {
            let _ = unsafe {
                self.com_impl
                    .BeginBuildReplica(&replica.get_raw(), &callback)?
            };
        }
        rx.await.unwrap()
    }
    fn remove_replica(&self, replicaid: i64) -> ::windows_core::Result<()> {
        info!("PrimaryReplicatorProxy::remove_replica");
        unsafe { self.com_impl.RemoveReplica(replicaid) }
    }
}
