// stateful contains rs definition of stateful traits that user needs to implement

use async_trait::async_trait;
use fabric_base::FabricCommon::FabricRuntime::IFabricStatefulServicePartition;
use windows_core::{Error, HSTRING};

use super::stateful_types::{
    Epoch, OpenMode, ReplicaInfo, ReplicaSetConfig, ReplicaSetQuarumMode, Role,
};

pub trait StatefulServiceFactory {
    fn create_replica(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &::windows::core::GUID,
        replicaid: i64,
    ) -> Result<Box<dyn StatefulServiceReplica>, Error>;
}

// safe service instance
#[async_trait]
pub trait StatefulServiceReplica: Send + Sync {
    // Note that open returns PrimaryReplicator instead of Replicator.
    // The replicator that gives to SF has to implement primary replicator all the time.
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<Box<dyn PrimaryReplicator>>;
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING>; // replica address
    async fn close(&self) -> windows::core::Result<()>;
    fn abort(&self);
}

pub struct StatefulServicePartition {
    _com_impl: IFabricStatefulServicePartition,
}

impl StatefulServicePartition {
    pub fn get_com(&self) -> &IFabricStatefulServicePartition {
        &self._com_impl
    }
}

impl From<&IFabricStatefulServicePartition> for StatefulServicePartition {
    fn from(e: &IFabricStatefulServicePartition) -> Self {
        StatefulServicePartition {
            _com_impl: e.clone(),
        }
    }
}

#[async_trait]
pub trait Replicator: Send + Sync {
    async fn open(&self) -> ::windows_core::Result<HSTRING>; // replicator address
    async fn close(&self) -> ::windows_core::Result<()>;
    async fn change_role(&self, epoch: &Epoch, role: &Role) -> ::windows_core::Result<()>;
    async fn update_epoch(&self, epoch: &Epoch) -> ::windows_core::Result<()>;
    fn get_current_progress(&self) -> ::windows_core::Result<i64>;
    fn get_catch_up_capability(&self) -> ::windows_core::Result<i64>;
    fn abort(&self);
}

#[async_trait]
pub trait PrimaryReplicator: Replicator {
    // SF calls this to indicate that possible data loss has occurred (write quorum loss),
    // returns is isStateChanged. If true, SF will re-create other secondaries.
    // The default SF impl might be a pass through to the state provider.
    async fn on_data_loss(&self) -> ::windows_core::Result<u8>;
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()>;
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
    ) -> ::windows_core::Result<()>;
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()>;
    async fn build_replica(&self, replica: &ReplicaInfo) -> ::windows_core::Result<()>;
    fn remove_replica(&self, replicaid: i64) -> ::windows_core::Result<()>;
}
