// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! mssf data utilities and extensions
//!

use std::sync::Arc;

use mssf_core::{
    WString,
    runtime::{
        IPrimaryReplicator, IReplicator, IStatefulServicePartition, executor::BoxedCancelToken,
    },
    types::{
        Epoch, ReplicaInformation, ReplicaRole, ReplicaSetConfig, ReplicaSetQuorumMode,
        ServicePartitionAccessStatus,
    },
};

/// An empty replicator that does nothing. Useful for services without
/// replication needs.
/// Traces are added for all methods for easier debugging.
#[derive(Clone)]
pub struct EmptyReplicator {
    name: WString,
    partition: Option<Arc<Box<dyn IStatefulServicePartition>>>,
}

impl EmptyReplicator {
    /// Get read status for tracing.
    fn read_status(&self) -> Option<ServicePartitionAccessStatus> {
        self.partition
            .as_ref()
            .map(|p| p.get_read_status().ok())
            .unwrap_or(None)
    }

    /// Get write status for tracing.
    fn write_status(&self) -> Option<ServicePartitionAccessStatus> {
        self.partition
            .as_ref()
            .map(|p| p.get_write_status().ok())
            .unwrap_or(None)
    }
}

/// Make it short for tracing purpose
impl std::fmt::Debug for EmptyReplicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmptyReplicator-{}", self.name)
    }
}

impl EmptyReplicator {
    /// Create a new empty replicator with a name for tracing purpose.
    pub fn new(
        name: WString,
        partition: Option<Box<dyn IStatefulServicePartition>>,
    ) -> EmptyReplicator {
        EmptyReplicator {
            name,
            partition: partition.map(Arc::new),
        }
    }
}

// This is basic implementation of Replicator
#[mssf_core::async_trait]
impl IReplicator for EmptyReplicator {
    #[tracing::instrument(skip(_token), err, ret)]
    async fn open(&self, _token: BoxedCancelToken) -> mssf_core::Result<WString> {
        // Empty replicator does not listen on any address
        Ok(WString::from("NoProtocol://localhost:0"))
    }

    #[tracing::instrument(skip(_token), err, ret)]
    async fn close(&self, _token: BoxedCancelToken) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(_token), fields(read_status = ?self.read_status(), write_status = ?self.write_status()), err, ret)]
    async fn change_role(
        &self,
        _epoch: Epoch,
        _role: ReplicaRole,
        _token: BoxedCancelToken,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(_token), fields(read_status = ?self.read_status(), write_status = ?self.write_status()), err, ret)]
    async fn update_epoch(&self, _epoch: Epoch, _token: BoxedCancelToken) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(err, ret)]
    fn get_current_progress(&self) -> mssf_core::Result<i64> {
        Ok(1)
    }

    #[tracing::instrument(err, ret)]
    fn get_catch_up_capability(&self) -> mssf_core::Result<i64> {
        Ok(1)
    }

    #[tracing::instrument()]
    fn abort(&self) {
        tracing::info!("abort");
    }
}

// This is basic implementation of PrimaryReplicator
#[mssf_core::async_trait]
impl IPrimaryReplicator for EmptyReplicator {
    #[tracing::instrument(skip(_token), err, ret)]
    async fn on_data_loss(&self, _token: BoxedCancelToken) -> mssf_core::Result<u8> {
        Ok(0)
    }

    #[tracing::instrument(err, ret)]
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
        previousconfiguration: ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(_token), fields(read_status = ?self.read_status(), write_status = ?self.write_status()), err, ret)]
    async fn wait_for_catch_up_quorum(
        &self,
        _catchupmode: ReplicaSetQuorumMode,
        _token: BoxedCancelToken,
    ) -> mssf_core::Result<()> {
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

    #[tracing::instrument(err, ret)]
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(_token), err, ret)]
    async fn build_replica(
        &self,
        _replica: ReplicaInformation,
        _token: BoxedCancelToken,
    ) -> mssf_core::Result<()> {
        Ok(())
    }

    #[tracing::instrument(err, ret)]
    fn remove_replica(&self, _replicaid: i64) -> mssf_core::Result<()> {
        Ok(())
    }
}
