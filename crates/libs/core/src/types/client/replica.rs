use mssf_com::{
    FabricClient::IFabricGetReplicaListResult2,
    FabricTypes::{
        FABRIC_QUERY_SERVICE_REPLICA_STATUS, FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED, FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID, FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY, FABRIC_SERVICE_KIND_STATEFUL,
        FABRIC_SERVICE_KIND_STATELESS, FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM,
        FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM,
        FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM,
    },
};
use windows_core::{GUID, HSTRING};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::HSTRINGWrap,
    types::{HealthState, ReplicaRole},
};

// FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION
pub struct ServiceReplicaQueryDescription {
    pub partition_id: GUID,
    pub replica_id_or_instance_id_filter: Option<i64>, // TODO: reserved fields
}

impl From<&ServiceReplicaQueryDescription> for FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION {
    fn from(value: &ServiceReplicaQueryDescription) -> Self {
        let id_filter = value.replica_id_or_instance_id_filter.unwrap_or_default();
        Self {
            PartitionId: value.partition_id,
            ReplicaIdOrInstanceIdFilter: id_filter,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// IFabricGetReplicaListResult2
pub struct ServiceReplicaList {
    com: IFabricGetReplicaListResult2,
}

impl ServiceReplicaList {
    pub fn new(com: IFabricGetReplicaListResult2) -> Self {
        Self { com }
    }

    pub fn iter(&self) -> ServiceReplicaListIter {
        ServiceReplicaListIter::new(self, self)
    }
}

type ServiceReplicaListIter<'a> = FabricIter<
    'a,
    FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM,
    ServiceReplicaQueryResult,
    ServiceReplicaList,
>;

impl FabricListAccessor<FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM> for ServiceReplicaList {
    fn get_count(&self) -> u32 {
        let raw = unsafe { self.com.get_ReplicaList().as_ref() };
        raw.unwrap().Count
    }

    fn get_first_item(&self) -> *const FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM {
        let raw = unsafe { self.com.get_ReplicaList().as_ref() };
        raw.unwrap().Items
    }
}

// FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM
pub enum ServiceReplicaQueryResult {
    Invalid,
    Stateful(StatefulServiceReplicaQueryResult),
    Stateless(StatelessServiceInstanceQueryResult),
}

impl From<&FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM> for ServiceReplicaQueryResult {
    fn from(value: &FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM) -> Self {
        match value.Kind {
            FABRIC_SERVICE_KIND_STATEFUL => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                Self::Stateful(raw.into())
            }
            FABRIC_SERVICE_KIND_STATELESS => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                Self::Stateless(raw.into())
            }
            _ => Self::Invalid,
        }
    }
}

// FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM
pub struct StatefulServiceReplicaQueryResult {
    pub replica_id: i64,
    pub replica_role: ReplicaRole,
    pub replica_status: QueryServiceReplicaStatus,
    pub aggregated_health_state: HealthState,
    pub replica_address: HSTRING,
    pub node_name: HSTRING,
    pub last_in_build_duration_in_seconds: i64,
    // pub Reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM>
    for StatefulServiceReplicaQueryResult
{
    fn from(value: &FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM) -> Self {
        Self {
            replica_id: value.ReplicaId,
            replica_role: (&value.ReplicaRole).into(),
            replica_status: (&value.ReplicaStatus).into(),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
            replica_address: HSTRINGWrap::from(value.ReplicaAddress).into(),
            node_name: HSTRINGWrap::from(value.NodeName).into(),
            last_in_build_duration_in_seconds: value.LastInBuildDurationInSeconds,
        }
    }
}

// FABRIC_QUERY_SERVICE_REPLICA_STATUS
#[derive(Debug, PartialEq)]
pub enum QueryServiceReplicaStatus {
    Invalid,
    Inbuild,
    Standby,
    Ready,
    Down,
    Dropped,
}

impl From<&FABRIC_QUERY_SERVICE_REPLICA_STATUS> for QueryServiceReplicaStatus {
    fn from(value: &FABRIC_QUERY_SERVICE_REPLICA_STATUS) -> Self {
        match *value {
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID => Self::Invalid,
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD => Self::Inbuild,
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY => Self::Standby,
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY => Self::Ready,
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN => Self::Down,
            FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED => Self::Dropped,
            _ => Self::Invalid,
        }
    }
}

//FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM
pub struct StatelessServiceInstanceQueryResult {
    pub instance_id: i64,
    pub replica_status: QueryServiceReplicaStatus,
    pub aggregated_health_state: HealthState,
    pub replica_address: HSTRING,
    pub node_name: HSTRING,
    pub last_in_build_duration_in_seconds: i64,
    // pub Reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM>
    for StatelessServiceInstanceQueryResult
{
    fn from(value: &FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM) -> Self {
        Self {
            instance_id: value.InstanceId,
            replica_status: (&value.ReplicaStatus).into(),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
            replica_address: HSTRINGWrap::from(value.ReplicaAddress).into(),
            node_name: HSTRINGWrap::from(value.NodeName).into(),
            last_in_build_duration_in_seconds: value.LastInBuildDurationInSeconds,
        }
    }
}
