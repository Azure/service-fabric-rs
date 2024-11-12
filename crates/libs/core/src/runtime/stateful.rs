// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful contains rs definition of stateful traits that user needs to implement
use mssf_com::FabricRuntime::IFabricStatefulServicePartition;

use crate::sync::CancellationToken;
use crate::types::{LoadMetric, LoadMetricListRef, ReplicaRole};

use crate::types::{Epoch, OpenMode, ReplicaInformation, ReplicaSetConfig, ReplicaSetQuarumMode};

/// Represents a stateful service factory that is responsible for creating replicas
/// of a specific type of stateful service. Stateful service factories are registered with
/// the FabricRuntime by service hosts via register_stateful_service_factory().
pub trait StatefulServiceFactory {
    /// Called by Service Fabric to create a stateful service replica for a particular service.
    fn create_replica(
        &self,
        servicetypename: &crate::HSTRING,
        servicename: &crate::HSTRING,
        initializationdata: &[u8],
        partitionid: &crate::GUID,
        replicaid: i64,
    ) -> crate::Result<impl StatefulServiceReplica>;
}

/// Defines behavior that governs the lifecycle of a replica, such as startup, initialization, role changes, and shutdown.
/// Remarks:
/// Stateful service types must implement this interface. The logic of a stateful service type includes behavior that is
/// invoked on primary replicas and behavior that is invoked on secondary replicas.
#[trait_variant::make(StatefulServiceReplica: Send)]
pub trait LocalStatefulServiceReplica: Send + Sync + 'static {
    /// Opens an initialized service replica so that additional actions can be taken.
    /// Returns PrimaryReplicator that is used by the stateful service.
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
        cancellation_token: CancellationToken,
    ) -> crate::Result<impl PrimaryReplicator>;

    /// Changes the role of the service replica to one of the ReplicaRole.
    /// Returns the service’s new connection address that is to be associated with the replica via Service Fabric Naming.
    /// Remarks:
    /// The new role is indicated as a parameter. When the service transitions to the new role,
    /// the service has a chance to update its current listening address. The listening address is the address
    /// where clients connect to it and the one returned via the ResolveAsync API. This enables the service when
    /// it is a primary replica to only claim some resources such as ports when communication from clients is expected.
    async fn change_role(
        &self,
        newrole: ReplicaRole,
        cancellation_token: CancellationToken,
    ) -> crate::Result<crate::HSTRING>;

    /// Closes the service replica gracefully when it is being shut down.
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()>;

    /// Ungracefully terminates the service replica.
    /// Remarks: Network issues resulting in Service Fabric process shutdown
    /// and the use of ReportFault(FaultType) to report a Permanent fault are examples of ungraceful termination.
    /// When this method is invoked, the service replica should immediately release and clean up all references and return.
    fn abort(&self);
}

#[derive(Debug, Clone)]
pub struct StatefulServicePartition {
    com_impl: IFabricStatefulServicePartition,
}

impl StatefulServicePartition {
    pub fn get_com(&self) -> &IFabricStatefulServicePartition {
        &self.com_impl
    }

    /// Reports load for the current replica in the partition.
    pub fn report_load(&self, metrics: &[LoadMetric]) -> crate::Result<()> {
        let metrics_ref = LoadMetricListRef::from_slice(metrics);
        let raw = metrics_ref.as_raw_slice();
        unsafe { self.com_impl.ReportLoad(raw) }
    }
}

impl From<&IFabricStatefulServicePartition> for StatefulServicePartition {
    fn from(e: &IFabricStatefulServicePartition) -> Self {
        StatefulServicePartition {
            com_impl: e.clone(),
        }
    }
}

/// TODO: replicator has no public documentation
#[trait_variant::make(Replicator: Send)]
pub trait LocalReplicator: Send + Sync + 'static {
    async fn open(&self, cancellation_token: CancellationToken) -> crate::Result<crate::HSTRING>; // replicator address
    async fn close(&self, cancellation_token: CancellationToken) -> crate::Result<()>;
    async fn change_role(
        &self,
        epoch: &Epoch,
        role: &ReplicaRole,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()>;

    /// (TODO: This doc is from IStateProvider but not Replicator.)
    /// Indicates to a replica that the configuration of a replica set has changed due to
    /// a change or attempted change to the primary replica. The change occurs due to failure
    /// or load balancing of the previous primary replica. Epoch changes act as a barrier by
    /// segmenting operations into the exact configuration periods in which they were sent
    /// by a specific primary replica.
    ///
    /// Called only on active secondary replicas. Primary replica gets new epoch via change_role call.
    async fn update_epoch(
        &self,
        epoch: &Epoch,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()>;
    fn get_current_progress(&self) -> crate::Result<i64>;
    fn get_catch_up_capability(&self) -> crate::Result<i64>;
    fn abort(&self);
}

/// TODO: primary replicator has no public documentation
/// IFabricPrimaryReplicator com interface wrapper.
#[trait_variant::make(PrimaryReplicator: Send)]
pub trait LocalPrimaryReplicator: Replicator {
    // SF calls this to indicate that possible data loss has occurred (write quorum loss),
    // returns is isStateChanged. If true, SF will re-create other secondaries.
    // The default SF impl might be a pass through to the state provider.
    async fn on_data_loss(&self, cancellation_token: CancellationToken) -> crate::Result<u8>;
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> crate::Result<()>;
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()>;
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> crate::Result<()>;
    async fn build_replica(
        &self,
        replica: &ReplicaInformation,
        cancellation_token: CancellationToken,
    ) -> crate::Result<()>;
    fn remove_replica(&self, replicaid: i64) -> crate::Result<()>;
}
