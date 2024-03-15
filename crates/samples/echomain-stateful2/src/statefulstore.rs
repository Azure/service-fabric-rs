// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use log::info;
use mssf_com::FABRIC_REPLICATOR_ADDRESS;
use mssf_core::runtime::{
    executor::DefaultExecutor,
    stateful::{
        PrimaryReplicator, Replicator, StatefulServiceFactory, StatefulServicePartition,
        StatefulServiceReplica,
    },
    stateful_types::{Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, ReplicaSetQuarumMode, Role},
    store_types::ReplicatorSettings,
};
use std::{cell::Cell, sync::Mutex};
use tokio::sync::oneshot::{self, Sender};
use windows_core::{Error, HSTRING};
mod echo;

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
    async fn open(&self) -> windows::core::Result<HSTRING> {
        info!("AppFabricReplicator2::Replicator::Open");
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = HSTRING::from(addr).into();
        Ok(str_res)
    }

    async fn close(&self) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::Replicator::close");
        Ok(())
    }

    async fn change_role(&self, epoch: &Epoch, role: &Role) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::Replicator::change_role");
        Ok(())
    }

    async fn update_epoch(&self, epoch: &Epoch) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::Replicator::update_epoch");
        Ok(())
    }

    fn get_current_progress(&self) -> windows::core::Result<i64> {
        info!("AppFabricReplicator2::Replicator::get_current_progress");
        Ok(0)
    }

    fn get_catch_up_capability(&self) -> windows::core::Result<i64> {
        info!("AppFabricReplicator2::Replicator::get_catch_up_capability");
        Ok(0)
    }

    fn abort(&self) {
        info!("AppFabricReplicator2::Replicator::abort");
    }
}

// This is basic implementation of PrimaryReplicator
impl PrimaryReplicator for AppFabricReplicator {
    async fn on_data_loss(&self) -> windows::core::Result<u8> {
        info!("AppFabricReplicator2::PrimaryReplicator::on_data_loss");
        Ok(0)
    }

    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::update_catch_up_replica_set_configuration");
        Ok(())
    }

    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
    ) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::wait_for_catch_up_quorum");
        Ok(())
    }

    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::update_current_replica_set_configuration");
        Ok(())
    }

    async fn build_replica(&self, replica: &ReplicaInfo) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::build_replica");
        Ok(())
    }

    fn remove_replica(&self, replicaid: i64) -> windows::core::Result<()> {
        info!("AppFabricReplicator2::PrimaryReplicator::remove_replica");
        Ok(())
    }
}

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: &windows_core::HSTRING,
        servicename: &windows_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &windows::core::GUID,
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
        let settings = ReplicatorSettings {
            Flags: FABRIC_REPLICATOR_ADDRESS.0 as u32,
            ReplicatorAddress: HSTRING::from(get_addr(
                self.replication_port,
                self.hostname.clone(),
            )),
            ..Default::default()
        };

        info!(
            "Factory::create_replica using address {}",
            settings.ReplicatorAddress
        );

        let svc = Service::new(self.replication_port, self.hostname.clone());
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
    port_: u32,
    hostname_: HSTRING,
    //th_: Cell<Option<JoinHandle<Result<(), Error>>>>,
    tx_: Mutex<Cell<Option<Sender<()>>>>,
}

impl Service {
    pub fn new(port: u32, hostname: HSTRING) -> Service {
        Service {
            port_: port,
            hostname_: hostname,
            tx_: Mutex::new(Cell::new(None)),
            //th_: Cell::from(None),
        }
    }

    pub fn start_loop(&self) {
        let (tx, mut rx) = oneshot::channel::<()>();
        self.stop();
        self.tx_.lock().unwrap().set(Some(tx));

        let port_copy = self.port_;
        let hostname_copy = self.hostname_.clone();

        let th = std::thread::spawn(move || echo::start_echo(rx, port_copy, hostname_copy));
        //self.th_.set(Some(th));
    }

    pub fn stop(&self) {
        let mut op = self.tx_.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().send(()).unwrap()
        }
    }
}

impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<impl PrimaryReplicator + 'static> {
        // should be primary replicator
        info!("Replica::open {:?}", openmode);
        self.svc.start_loop();
        Ok(AppFabricReplicator::new(self.port_, self.hostname_.clone()))
    }
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING> {
        info!("Replica::change_role {:?}", newrole);
        if newrole == Role::Primary {
            info!("primary {:?}", self.svc.port_);
        }
        // return the address
        let addr = get_addr(self.port_, self.hostname_.clone());
        let str_res = HSTRING::from(addr).into();
        Ok(str_res)
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("Replica::close");
        self.svc.stop();
        Ok(())
    }
    fn abort(&self) {
        info!("Replica::abort");
        self.svc.stop();
    }
}
