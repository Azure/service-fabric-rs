// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::{Error, WString};
use mssf_core::{
    runtime::{
        executor::{DefaultExecutor, Executor},
        stateful::{PrimaryReplicator, Replicator, StatefulServiceFactory, StatefulServiceReplica},
        stateful_proxy::StatefulServicePartition,
    },
    types::{
        Epoch, OpenMode, ReplicaInformation, ReplicaRole, ReplicaSetConfig, ReplicaSetQuorumMode,
    },
};
use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::echo;

pub struct Factory {
    replication_port: u32,
    hostname: WString,
    rt: DefaultExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, hostname: WString, rt: DefaultExecutor) -> Factory {
        Factory {
            replication_port,
            hostname,
            rt,
        }
    }
}

fn get_addr(port: u32, hostname: WString) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

pub struct AppFabricReplicator {
    port_: u32,
    hostname_: WString,
    ctx: ReplicaCtx,
}

impl AppFabricReplicator {
    pub fn new(port: u32, hostname: WString, ctx: ReplicaCtx) -> AppFabricReplicator {
        AppFabricReplicator {
            port_: port,
            hostname_: hostname,
            ctx,
        }
    }
}

// This is basic implementation of Replicator
impl Replicator for AppFabricReplicator {
    async fn open(&self, _: CancellationToken) -> mssf_core::Result<WString> {
        info!(
            "AppFabricReplicator2::Replicator::Open: {:?}",
            self.ctx.get_trace_read_write_status()
        );
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = WString::from(addr);
        Ok(str_res)
    }

    async fn close(&self, _: CancellationToken) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::Replicator::close {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    async fn change_role(
        &self,
        epoch: &Epoch,
        role: &ReplicaRole,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::Replicator::change_role epoch:{epoch:?}, role:{role:?}, {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    async fn update_epoch(&self, epoch: &Epoch, _: CancellationToken) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::Replicator::update_epoch: {epoch:?}, {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    fn get_current_progress(&self) -> mssf_core::Result<i64> {
        info!(
            "AppFabricReplicator2::Replicator::get_current_progress {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(0)
    }

    fn get_catch_up_capability(&self) -> mssf_core::Result<i64> {
        info!(
            "AppFabricReplicator2::Replicator::get_catch_up_capability {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(0)
    }

    fn abort(&self) {
        info!(
            "AppFabricReplicator2::Replicator::abort {:?}",
            self.ctx.get_trace_read_write_status()
        );
    }
}

// This is basic implementation of PrimaryReplicator
impl PrimaryReplicator for AppFabricReplicator {
    async fn on_data_loss(&self, _: CancellationToken) -> mssf_core::Result<u8> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::on_data_loss {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(0)
    }

    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::update_catch_up_replica_set_configuration: curr: {currentconfiguration:?}, prev: {previousconfiguration:?},{:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuorumMode,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::wait_for_catch_up_quorum mode:{catchupmode:?} {:?}",
            self.ctx.get_trace_read_write_status()
        );
        // Before demoting a primary to active secondary in graceful failover (MovePrimary api FabricClient trigger),
        // (R:G, W:P) means read status granted, write status reconfiguration pending.
        // NA means status NotPrimary.
        // SF calls this in order:
        // * update_catch_up_replica_set_configuration
        // * wait_for_catch_up_quorum write mode, with (R:G, W:G).
        //   app should catch up making necessary writes. (For example: complete transaction?)
        //   This may take forever depends on the implementation, if write is faster than catch up.
        //   App can ignore this call and let the next catch up call handle it all, if the app
        //   does not need to do write while catching up.
        // * update epoch,(R:G, W:P). SF revokes write status for the service.
        // * update_catch_up_replica_set_configuration, with (R:G, W:P)
        // * wait_for_catch_up_quorum, with (R:G, W:P).
        //   app should catch up knowing that user/client is not able to write.
        // * change_role from Primary to ActiveSecondary, with the same epoch from update epoch. (R:NA,W:NA)

        // For newly created or promoted Primary, status starts with ChangeRole Primary (R:P, W:P)
        // * update_catch_up_replica_set_configuration (R:P, W:P)
        // * wait_for_catch_up_quorum (R:P, W:P)
        // * update_current_replica_set_configuration (R:G, W:G)
        Ok(())
    }

    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::update_current_replica_set_configuration {currentconfiguration:?} {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    async fn build_replica(
        &self,
        replica: &ReplicaInformation,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::build_replica: info: {replica:?} {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }

    fn remove_replica(&self, _replicaid: i64) -> mssf_core::Result<()> {
        info!(
            "AppFabricReplicator2::PrimaryReplicator::remove_replica {:?}",
            self.ctx.get_trace_read_write_status()
        );
        Ok(())
    }
}

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: &mssf_core::WString,
        servicename: &mssf_core::WString,
        initializationdata: &[u8],
        partitionid: &mssf_core::GUID,
        replicaid: i64,
    ) -> Result<impl StatefulServiceReplica + 'static, Error> {
        info!(
            "Factory::create_replica type {}, service {}, init data size {}, partition {:?}, replica {}",
            servicetypename,
            servicename,
            initializationdata.len(),
            partitionid,
            replicaid
        );

        let svc = Service::new(
            self.rt.clone(),
            self.replication_port,
            self.hostname.clone(),
        );
        let replica = Replica::new(self.replication_port, self.hostname.clone(), svc);
        Ok(replica)
    }
}

pub struct Replica {
    port_: u32,
    hostname_: WString,
    svc: Service,
    ctx: ReplicaCtx,
}

impl Replica {
    pub fn new(port: u32, hostname: WString, svc: Service) -> Replica {
        Replica {
            port_: port,
            hostname_: hostname,
            svc,
            ctx: ReplicaCtx::empty(),
        }
    }
}
pub struct Service {
    tcp_port: u32,
    hostname_: WString,

    cancel: Mutex<Cell<Option<CancellationToken>>>,
    rt: DefaultExecutor,
}

impl Service {
    pub fn new(rt: DefaultExecutor, tcp_port: u32, hostname: WString) -> Service {
        Service {
            tcp_port,
            hostname_: hostname,
            cancel: Mutex::new(Cell::new(None)),
            rt,
        }
    }

    pub fn start_loop_in_background(&self, partition: &StatefulServicePartition) {
        info!("Service::start_loop_in_background");
        self.stop();
        let token = CancellationToken::new();
        self.cancel.lock().unwrap().set(Some(token.clone()));
        let port_copy = self.tcp_port;
        let hostname_copy = self.hostname_.clone();
        let partition_cp = partition.clone();
        // start the echo server in background
        self.rt.spawn(async move {
            info!("Service: start echo");
            echo::start_echo(token, port_copy, hostname_copy, partition_cp).await
        });
    }

    pub fn stop(&self) {
        let mut op = self.cancel.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().cancel()
        }
    }
}

impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
        _: CancellationToken,
    ) -> mssf_core::Result<impl PrimaryReplicator> {
        self.ctx.init(partition.clone());
        info!(
            "Replica::open {openmode:?}, {:?}",
            self.ctx.get_trace_read_write_status()
        );
        self.svc.start_loop_in_background(partition);
        Ok(AppFabricReplicator::new(
            self.port_,
            self.hostname_.clone(),
            self.ctx.clone(),
        ))
    }
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        _: CancellationToken,
    ) -> mssf_core::Result<WString> {
        info!(
            "Replica::change_role {newrole:?}, {:?}",
            self.ctx.get_trace_read_write_status()
        );
        if newrole == ReplicaRole::Primary {
            info!("primary {:?}", self.svc.tcp_port);
        }
        // return the address
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = WString::from(addr);
        Ok(str_res)
    }
    async fn close(&self, _: CancellationToken) -> mssf_core::Result<()> {
        info!(
            "Replica::close: {:?}",
            self.ctx.get_trace_read_write_status()
        );
        self.svc.stop();
        Ok(())
    }
    fn abort(&self) {
        info!(
            "Replica::abort: {:?}",
            self.ctx.get_trace_read_write_status()
        );
        self.svc.stop();
    }
}

/// Stores info shared between replica and replicator
#[derive(Clone)]
pub struct ReplicaCtx {
    pub partition: Arc<Mutex<Option<StatefulServicePartition>>>,
}

impl ReplicaCtx {
    fn empty() -> Self {
        Self {
            partition: Arc::new(Mutex::new(None)),
        }
    }
    fn init(&self, partition: StatefulServicePartition) {
        let prev = self.partition.lock().unwrap().replace(partition);
        assert!(prev.is_none())
    }

    fn get_partition(&self) -> StatefulServicePartition {
        self.partition
            .lock()
            .unwrap()
            .as_ref()
            .expect("option null")
            .clone()
    }

    fn get_trace_read_write_status(&self) -> String {
        let p = self.get_partition();
        format!(
            "read: {:?}, write {:?}",
            p.get_read_status(),
            p.get_write_status()
        )
    }
}
