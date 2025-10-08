// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::runtime::executor::BoxedCancelToken;
use mssf_core::{Error, WString};
use mssf_core::{
    runtime::{
        stateful::{PrimaryReplicator, StatefulServiceFactory, StatefulServiceReplica},
        stateful_proxy::StatefulServicePartition,
    },
    types::{OpenMode, ReplicaRole},
};
use mssf_util::data::EmptyReplicator;
use mssf_util::tokio::TokioExecutor;
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
    rt: TokioExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, hostname: WString, rt: TokioExecutor) -> Factory {
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

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: mssf_core::WString,
        servicename: mssf_core::types::Uri,
        initializationdata: &[u8],
        partitionid: mssf_core::GUID,
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
    rt: TokioExecutor,
}

impl Service {
    pub fn new(rt: TokioExecutor, tcp_port: u32, hostname: WString) -> Service {
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
        self.rt.get_ref().spawn(async move {
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
        _: BoxedCancelToken,
    ) -> mssf_core::Result<impl PrimaryReplicator> {
        self.ctx.init(partition.clone());
        info!(
            "Replica::open {openmode:?}, {:?}",
            self.ctx.get_trace_read_write_status()
        );
        self.svc.start_loop_in_background(partition);
        // Use empty replicator
        Ok(EmptyReplicator::new(
            WString::from("Stateful2"),
            Some(partition.clone()),
        ))
    }
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        _: BoxedCancelToken,
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
    async fn close(&self, _: BoxedCancelToken) -> mssf_core::Result<()> {
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
