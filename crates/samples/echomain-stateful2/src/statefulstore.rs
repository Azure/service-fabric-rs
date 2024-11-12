// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

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
use mssf_core::{Error, HSTRING};
use std::{cell::Cell, sync::Mutex};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::echo;

pub struct Factory {
    replication_port: u32,
    hostname: HSTRING,
    rt: DefaultExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, hostname: HSTRING, rt: DefaultExecutor) -> Factory {
        Factory {
            replication_port,
            hostname,
            rt,
        }
    }
}

fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

pub struct AppFabricReplicator {
    port_: u32,
    hostname_: HSTRING,
}

impl AppFabricReplicator {
    pub fn new(port: u32, hostname: HSTRING) -> AppFabricReplicator {
        AppFabricReplicator {
            port_: port,
            hostname_: hostname,
        }
    }
}

// This is basic implementation of Replicator
impl Replicator for AppFabricReplicator {
    async fn open(&self, _: CancellationToken) -> mssf_core::Result<HSTRING> {
        info!("AppFabricReplicator2::Replicator::Open");
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = HSTRING::from(addr);
        Ok(str_res)
    }

    async fn close(&self, _: CancellationToken) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::Replicator::close");
        Ok(())
    }

    async fn change_role(
        &self,
        _epoch: &Epoch,
        _role: &ReplicaRole,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::Replicator::change_role");
        Ok(())
    }

    async fn update_epoch(&self, _epoch: &Epoch, _: CancellationToken) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::Replicator::update_epoch");
        Ok(())
    }

    fn get_current_progress(&self) -> mssf_core::Result<i64> {
        info!("AppFabricReplicator2::Replicator::get_current_progress");
        Ok(0)
    }

    fn get_catch_up_capability(&self) -> mssf_core::Result<i64> {
        info!("AppFabricReplicator2::Replicator::get_catch_up_capability");
        Ok(0)
    }

    fn abort(&self) {
        info!("AppFabricReplicator2::Replicator::abort");
    }
}

// This is basic implementation of PrimaryReplicator
impl PrimaryReplicator for AppFabricReplicator {
    async fn on_data_loss(&self, _: CancellationToken) -> mssf_core::Result<u8> {
        info!("AppFabricReplicator2::PrimaryReplicator::on_data_loss");
        Ok(0)
    }

    fn update_catch_up_replica_set_configuration(
        &self,
        _currentconfiguration: &ReplicaSetConfig,
        _previousconfiguration: &ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::update_catch_up_replica_set_configuration");
        Ok(())
    }

    async fn wait_for_catch_up_quorum(
        &self,
        _catchupmode: ReplicaSetQuorumMode,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::wait_for_catch_up_quorum");
        Ok(())
    }

    fn update_current_replica_set_configuration(
        &self,
        _currentconfiguration: &ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::update_current_replica_set_configuration");
        Ok(())
    }

    async fn build_replica(
        &self,
        _replica: &ReplicaInformation,
        _: CancellationToken,
    ) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::build_replica");
        Ok(())
    }

    fn remove_replica(&self, _replicaid: i64) -> mssf_core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::remove_replica");
        Ok(())
    }
}

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: &mssf_core::HSTRING,
        servicename: &mssf_core::HSTRING,
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
    hostname_: HSTRING,
    svc: Service,
}

impl Replica {
    pub fn new(port: u32, hostname: HSTRING, svc: Service) -> Replica {
        Replica {
            port_: port,
            hostname_: hostname,
            svc,
        }
    }
}
pub struct Service {
    tcp_port: u32,
    hostname_: HSTRING,

    cancel: Mutex<Cell<Option<CancellationToken>>>,
    rt: DefaultExecutor,
}

impl Service {
    pub fn new(rt: DefaultExecutor, tcp_port: u32, hostname: HSTRING) -> Service {
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
        // should be primary replicator
        info!("Replica::open {:?}", openmode);
        self.svc.start_loop_in_background(partition);
        Ok(AppFabricReplicator::new(self.port_, self.hostname_.clone()))
    }
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        _: CancellationToken,
    ) -> mssf_core::Result<HSTRING> {
        info!("Replica::change_role {:?}", newrole);
        if newrole == ReplicaRole::Primary {
            info!("primary {:?}", self.svc.tcp_port);
        }
        // return the address
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = HSTRING::from(addr);
        Ok(str_res)
    }
    async fn close(&self, _: CancellationToken) -> mssf_core::Result<()> {
        info!("Replica::close");
        self.svc.stop();
        Ok(())
    }
    fn abort(&self) {
        info!("Replica::abort");
        self.svc.stop();
    }
}
