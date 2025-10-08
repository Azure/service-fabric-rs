// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful contains rs definition of stateful traits that user needs to implement

use crate::runtime::executor::BoxedCancelToken;
use crate::types::ReplicaRole;

use crate::types::{Epoch, OpenMode, ReplicaInformation, ReplicaSetConfig, ReplicaSetQuorumMode};

use super::stateful_proxy::StatefulServicePartition;

/// Represents a stateful service factory that is responsible for creating replicas
/// of a specific type of stateful service. Stateful service factories are registered with
/// the FabricRuntime by service hosts via register_stateful_service_factory().
pub trait StatefulServiceFactory {
    /// Called by Service Fabric to create a stateful service replica for a particular service.
    fn create_replica(
        &self,
        servicetypename: crate::WString,
        servicename: crate::types::Uri,
        initializationdata: &[u8],
        partitionid: crate::GUID,
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
    /// Note:
    /// Most user calls IFabricStatefulServicePartition.CreateReplicator instead of
    /// writing their own replicator (TODO: not supported in mssf yet),
    /// or use FabricCreateKeyValueStoreReplica.
    async fn open(
        &self,
        openmode: OpenMode,
        partition: StatefulServicePartition,
        cancellation_token: BoxedCancelToken,
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
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<crate::WString>;

    /// Closes the service replica gracefully when it is being shut down.
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()>;

    /// Ungracefully terminates the service replica.
    /// Remarks: Network issues resulting in Service Fabric process shutdown
    /// and the use of ReportFault(FaultType) to report a Permanent fault are examples of ungraceful termination.
    /// When this method is invoked, the service replica should immediately release and clean up all references and return.
    fn abort(&self);
}

/// TODO: replicator has no public documentation
#[trait_variant::make(Replicator: Send)]
pub trait LocalReplicator: Send + Sync + 'static {
    /// Opens replicator, and returns the replicator address that is visible to primary
    /// in ReplicaInformation.
    /// Remarks:
    /// Replicator does not have an assigned role yet and should setup listening endpoint.
    async fn open(&self, cancellation_token: BoxedCancelToken) -> crate::Result<crate::WString>;
    async fn close(&self, cancellation_token: BoxedCancelToken) -> crate::Result<()>;

    /// Change the replicator role.
    ///
    /// Remarks:
    /// Replicator change_role is called before Replica change_role.
    async fn change_role(
        &self,
        epoch: Epoch,
        role: ReplicaRole,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()>;

    /// (TODO: This doc is from IStateProvider but not Replicator.)
    /// Indicates to a replica that the configuration of a replica set has changed due to
    /// a change or attempted change to the primary replica. The change occurs due to failure
    /// or load balancing of the previous primary replica. Epoch changes act as a barrier by
    /// segmenting operations into the exact configuration periods in which they were sent
    /// by a specific primary replica.
    ///
    /// Called only on active/idle secondary replicas. Primary replica gets new epoch via change_role call.
    async fn update_epoch(
        &self,
        epoch: Epoch,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()>;

    /// Get the current LSN, end of log, called on secondaries.
    /// SF uses this to do primary selection. It is also passed to update_catch_up_replica_set_configuration()
    /// on primary. Primary uses this for catchup.
    fn get_current_progress(&self) -> crate::Result<i64>;

    /// Get the first LSN, beginning of log.
    /// Remarks:
    /// SF uses this to determine if other replicas can catch up from this replica.
    /// Other replica's end of log must be higher than this replica's beginning of log
    /// in order for the other replica to catchup, otherwise SF needs to drop the other
    /// replica (if the current replica is chosen to be primary).
    fn get_catch_up_capability(&self) -> crate::Result<i64>;
    fn abort(&self);
}

// Remarks:
// Adding a secondary into the partition involves the following steps:
// * SF brings up an idle secondary replica S which can be empty or contain
// (partial) previous data.
// * build_replica is called on primary to copy data into the S.
// * S is changed to active secondary
// * update_catch_up_replica_set_configuration to include S in the current configuration,
// and wait_for_catch_up_quorum are called on primary for final synchronization before SF
// grants ReadStatus to S.
// * SF grants ReadStatus to S, and replica build completes.
//
// For primary failover, all active or idle secondaries gets update_epoch() call, and new
// primary gets the new epoch from change_role() call. Secondary should fence/reject
// operations from the old primary with an older epoch.

/// TODO: primary replicator has no public documentation, this is gathered unofficially and
/// is subject to change/correction.
/// IFabricPrimaryReplicator com interface wrapper.
#[trait_variant::make(PrimaryReplicator: Send)]
pub trait LocalPrimaryReplicator: Replicator {
    // SF calls this to indicate that possible data loss has occurred (write quorum loss),
    // returns is isStateChanged. If true, SF will re-create other secondaries.
    // The default SF impl might be a pass through to the state provider.
    async fn on_data_loss(&self, cancellation_token: BoxedCancelToken) -> crate::Result<u8>;

    // Remarks on replicator configuration:
    // At any time the replicator can have one or two configurations. There is always a current
    // configuration which represents the set of replicas that are participating in replication
    // along with the current write quorum. In addition there can be a previous configuration
    // which represents the set of replicas that were in the previous configuration.
    // When there is both a current and previous configuration the replicator must ensure that
    // writes are acknowledeged by a write quroum of both configurations.

    /// Informs the replicator there there is a current configuration and a previous configuration.
    /// Called on primary to inform the set of active secondary replicas that may
    /// begin to catchup. Idle secondary replicas are not included here.
    ///
    /// The total number of replica marked with must_catchup will not exceed the write quorum.
    /// Secondary to be promoted to new primary is guaranteed to have must_catchup set,
    /// i.e. it must catch up (have all the data) to be promoted to new primary.
    ///
    /// ReplicaInformation:
    /// current_progress -> The LSN of the replica. -1 if the replicator is already aware of the replica
    /// (it is in configuration or has been built) otherwise it will be the progress of the remote replica.
    /// catch_up_capability -> The first LSN of the replica. Similar to current_progress.
    /// must_catchup -> Set to true only for one replica in the current configuration.
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
        previousconfiguration: ReplicaSetConfig,
    ) -> crate::Result<()>;

    /// Informs the replicator about the current replica set configuration, and there
    /// is no longer a previous configuration.
    /// Remarks:
    /// Replicas here are not marked as must_catchup.
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: ReplicaSetConfig,
    ) -> crate::Result<()>;

    /// Called on primary to wait for replicas to catch up, before
    /// accepting writes.
    ///
    /// mssf-core enables IFabricReplicatorCatchupSpecificQuorum for replicators,
    /// so ReplicaSetQuarumMode::Write can be used.
    ///
    /// catchupmode:
    /// All -> full quorum. All replicas needs to catch up.
    /// Write -> write quorum, for replicas specified in update_catch_up_replica_set_configuration(currentconfiguration...),
    ///     a subset of replicas that can form a write quorum must catchup, and the subset must include
    ///     the replica with must_catchup set to true (primary candidate).
    ///     This is used only in primary swap case in SF, to avoid slow replica preventing/slowing down the swap.
    /// Remarks:
    /// Catchup (or quorum catchup) in SF means that the lowest LSN among all replicas (or quorum of replicas
    /// including the must catchup replica) in the current configuration is equal or greater than
    /// the current committed LSN.
    ///
    /// For swap primary case, double catchup feature is enabled by default.
    /// SF can first call this api before initiating write status revokation. SF then revoke write status,
    /// and call this again. This allows replicator to catch up with write status granted to make necessary writes for
    /// catch up. There is a chance that replicator takes forever to complete this api with mode ReplicaSetQuarumMode::All
    /// since client/user can keep writing and advancing the committed LSN, but for it most likely would not
    /// stall in mode ReplicaSetQuarumMode::Write.
    /// In other cases when client write is not impacted (like secondary restart),
    /// SF may call this api only once with write status granted.
    ///
    /// Implementor should not assume when this is called in relation to other api calls,
    /// but instead follow the semantics of what catchup should do.
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuorumMode,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()>;

    /// Transferring state up to the current quorum LSN to a new or existing replica
    /// that is outside the current configuration. (not included in update_catch_up_replica_set_configuration)
    ///
    /// replica:
    /// role is IdleSecondary
    /// status set to up or down
    /// current progress is -1
    /// catchup capability is -1
    /// must catchup is false
    ///
    /// remarks:
    /// SF can cancel the replica build operation by calling the cancellation token.
    /// Replica being built or completed built does not count towards quorum and is
    /// not part of the current configuration. Replica cannot be in build and be in the
    /// configuration at the same time. Idle replica it maybe added by SF to the configuration
    /// by calling update_x_configuration().
    async fn build_replica(
        &self,
        replica: ReplicaInformation,
        cancellation_token: BoxedCancelToken,
    ) -> crate::Result<()>;

    /// Notifies primary that an idle replica built by build_replica() api call
    /// has gone down and replicator should not send more operations to that replica
    /// and should release all resources.
    /// Remarks:
    /// Removing replicas already in the partition, update_catch_up_replica_set_configuration
    /// is called instead with ReplicaSetConfig not containng the to be removed replica.
    /// SF does not call remove_replica on the replica where build_replica is still running.
    fn remove_replica(&self, replicaid: i64) -> crate::Result<()>;
}
