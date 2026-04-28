// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::runtime::IStatefulServicePartition;
use mssf_core::runtime::executor::BoxedCancelToken;
use mssf_core::{Error, WString};
use mssf_core::{
    runtime::{IPrimaryReplicator, IStatefulServiceFactory, IStatefulServiceReplica},
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

use crate::control::{
    Approval, ControlMode, Decision, ReplicaController, decode_init_data, make_controller,
};
use crate::echo;
use crate::grpc::{ReflectionUrl, ReplicaRegistry};

pub struct Factory {
    replication_port: u32,
    hostname: WString,
    rt: TokioExecutor,
    grpc_port: u16,
    registry: ReplicaRegistry,
}

impl Factory {
    pub fn create(
        replication_port: u32,
        hostname: WString,
        rt: TokioExecutor,
        grpc_port: u16,
        registry: ReplicaRegistry,
    ) -> Factory {
        Factory {
            replication_port,
            hostname,
            rt,
            grpc_port,
            registry,
        }
    }
}

#[mssf_core::async_trait]
impl IStatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: mssf_core::WString,
        servicename: mssf_core::types::Uri,
        initializationdata: &[u8],
        partitionid: mssf_core::GUID,
        replicaid: i64,
    ) -> Result<Box<dyn IStatefulServiceReplica>, Error> {
        // Decide test-control mode from the bytes SF passes us. Empty
        // initdata or decode failure -> NoControl, preserving the
        // current production behavior.
        let init = decode_init_data(initializationdata);
        let mode = ControlMode::from_init_data(&init);
        let controller = make_controller(mode);

        info!(
            "Factory::create_replica type {}, service {}, init data size {}, partition {:?}, replica {}, mode {:?}",
            servicetypename,
            servicename,
            initializationdata.len(),
            partitionid,
            replicaid,
            mode,
        );

        let svc = Service::new(
            self.rt.clone(),
            self.replication_port,
            self.hostname.clone(),
        );

        if controller.is_controllable() {
            self.registry
                .add_controller(partitionid, replicaid, controller.clone());
        } else {
            self.registry.add(partitionid, replicaid);
        }

        let replica = Box::new(Replica::new(
            self.hostname.to_string(),
            self.grpc_port,
            partitionid,
            replicaid,
            self.registry.clone(),
            svc,
            controller,
            self.rt.clone(),
        ));
        Ok(replica)
    }
}

pub struct Replica {
    grpc_hostname: String,
    grpc_port: u16,
    partition_id: mssf_core::GUID,
    replica_id: i64,
    registry: ReplicaRegistry,
    svc: Service,
    ctx: ReplicaCtx,
    /// Per-replica controller. `NoopController` for production-mode
    /// replicas (one inline `Decision::Proceed` per gate);
    /// `GrpcController` for test-driven replicas.
    controller: Arc<dyn ReplicaController>,
    /// Used by `abort` to bridge sync->async into `await_approval`.
    exec: TokioExecutor,
}

impl Replica {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        grpc_hostname: String,
        grpc_port: u16,
        partition_id: mssf_core::GUID,
        replica_id: i64,
        registry: ReplicaRegistry,
        svc: Service,
        controller: Arc<dyn ReplicaController>,
        exec: TokioExecutor,
    ) -> Replica {
        Replica {
            grpc_hostname,
            grpc_port,
            partition_id,
            replica_id,
            registry,
            svc,
            ctx: ReplicaCtx::empty(),
            controller,
            exec,
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

    pub fn start_loop_in_background(&self, partition: &Arc<dyn IStatefulServicePartition>) {
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
            echo::start_load_report(token, port_copy, hostname_copy, partition_cp).await
        });
    }

    pub fn stop(&self) {
        let mut op = self.cancel.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().cancel()
        }
    }
}

#[mssf_core::async_trait]
impl IStatefulServiceReplica for Replica {
    #[tracing::instrument(skip(self,_token), fields(read_status = ?self.ctx.read_status(), write_status = ?self.ctx.write_status()), err, ret)]
    async fn open(
        &self,
        _openmode: OpenMode,
        partition: Arc<dyn IStatefulServicePartition>,
        _token: BoxedCancelToken,
    ) -> mssf_core::Result<Box<dyn IPrimaryReplicator>> {
        match self.controller.await_approval(Approval::Open).await {
            Decision::Proceed => {}
            Decision::Fail(e) => return Err(e),
        }
        self.ctx.init(partition.clone());
        self.svc.start_loop_in_background(&partition);
        // Use empty replicator
        Ok(Box::new(EmptyReplicator::new(
            WString::from("Stateful2"),
            Some(partition),
        )))
    }
    #[tracing::instrument(skip(self,_token), fields(read_status = ?self.ctx.read_status(), write_status = ?self.ctx.write_status()), err, ret)]
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        _token: BoxedCancelToken,
    ) -> mssf_core::Result<WString> {
        match self
            .controller
            .await_approval(Approval::ChangeRole(newrole))
            .await
        {
            Decision::Proceed => {}
            Decision::Fail(e) => return Err(e),
        }
        self.registry
            .update_role(self.partition_id, self.replica_id, newrole);
        // return the gRPC address with partition and replica id as query params
        let reflection_url = ReflectionUrl::new(
            &self.grpc_hostname,
            self.grpc_port,
            self.partition_id,
            self.replica_id,
        );
        Ok(WString::from(reflection_url.to_url_string()))
    }
    #[tracing::instrument(skip(self,_token), fields(read_status = ?self.ctx.read_status(), write_status = ?self.ctx.write_status()), err, ret)]
    async fn close(&self, _token: BoxedCancelToken) -> mssf_core::Result<()> {
        match self.controller.await_approval(Approval::Close).await {
            Decision::Proceed => {}
            Decision::Fail(e) => return Err(e),
        }
        self.registry.remove(self.partition_id, self.replica_id);
        self.svc.stop();
        Ok(())
    }
    #[tracing::instrument(skip(self), fields(read_status = ?self.ctx.read_status(), write_status = ?self.ctx.write_status()))]
    fn abort(&self) {
        info!("abort",);
        // Sync->async bridge for the abort gate. Decision is
        // intentionally ignored: IStatefulServiceReplica::abort
        // returns () and cannot propagate an error. Under
        // NoopController this resolves immediately; under
        // GrpcController this may queue at gate_lock if a previous
        // lifecycle method (e.g. close) is still parked.
        let controller = self.controller.clone();
        self.exec.block_on_any(async move {
            let _ = controller.await_approval(Approval::Abort).await;
        });
        self.registry.remove(self.partition_id, self.replica_id);
        self.svc.stop();
    }
}

/// Stores info shared between replica and replicator
#[derive(Clone)]
pub struct ReplicaCtx {
    pub partition: Arc<Mutex<Option<Arc<dyn IStatefulServicePartition>>>>,
}

impl ReplicaCtx {
    fn empty() -> Self {
        Self {
            partition: Arc::new(Mutex::new(None)),
        }
    }
    fn init(&self, partition: Arc<dyn IStatefulServicePartition>) {
        let prev = self.partition.lock().unwrap().replace(partition);
        assert!(prev.is_none())
    }

    fn get_partition(&self) -> Option<Arc<dyn IStatefulServicePartition>> {
        self.partition.lock().unwrap().as_ref().map(|p| p.clone())
    }

    /// Get read status for tracing.
    fn read_status(&self) -> Option<mssf_core::types::ServicePartitionAccessStatus> {
        let p = self.get_partition();
        p.and_then(|p| p.get_read_status().ok())
    }

    /// Get write status for tracing.
    fn write_status(&self) -> Option<mssf_core::types::ServicePartitionAccessStatus> {
        let p = self.get_partition();
        p.and_then(|p| p.get_write_status().ok())
    }
}
