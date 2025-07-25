// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::SystemTime;

use crate::{GUID, PCWSTR, WString, types::ServicePartitionAccessStatus};
use mssf_com::{
    FabricClient::{IFabricGetDeployedServiceReplicaDetailResult, IFabricGetReplicaListResult2},
    FabricTypes::{
        FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
        FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM,
        FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS, FABRIC_QUERY_SERVICE_REPLICA_STATUS_DOWN,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_DROPPED, FABRIC_QUERY_SERVICE_REPLICA_STATUS_INBUILD,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_INVALID, FABRIC_QUERY_SERVICE_REPLICA_STATUS_READY,
        FABRIC_QUERY_SERVICE_REPLICA_STATUS_STANDBY, FABRIC_REMOVE_REPLICA_DESCRIPTION,
        FABRIC_RESTART_REPLICA_DESCRIPTION, FABRIC_SERVICE_KIND_STATEFUL,
        FABRIC_SERVICE_KIND_STATELESS, FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        FABRIC_SERVICE_REPLICA_QUERY_RESULT_ITEM,
        FABRIC_STATEFUL_SERVICE_REPLICA_QUERY_RESULT_ITEM,
        FABRIC_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM,
    },
};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::WStringWrap,
    types::{HealthState, ReplicaRole},
};

use super::{QueryReplicatorOperationName, QueryServiceOperationName};

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct StatefulServiceReplicaQueryResult {
    pub replica_id: i64,
    pub replica_role: ReplicaRole,
    pub replica_status: QueryServiceReplicaStatus,
    pub aggregated_health_state: HealthState,
    pub replica_address: WString,
    pub node_name: WString,
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
            replica_address: WStringWrap::from(value.ReplicaAddress).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            last_in_build_duration_in_seconds: value.LastInBuildDurationInSeconds,
        }
    }
}

// FABRIC_QUERY_SERVICE_REPLICA_STATUS
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone)]
pub struct StatelessServiceInstanceQueryResult {
    pub instance_id: i64,
    pub replica_status: QueryServiceReplicaStatus,
    pub aggregated_health_state: HealthState,
    pub replica_address: WString,
    pub node_name: WString,
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
            replica_address: WStringWrap::from(value.ReplicaAddress).into(),
            node_name: WStringWrap::from(value.NodeName).into(),
            last_in_build_duration_in_seconds: value.LastInBuildDurationInSeconds,
        }
    }
}

// FABRIC_RESTART_REPLICA_DESCRIPTION
pub struct RestartReplicaDescription {
    pub node_name: WString,
    pub partition_id: GUID,
    pub replica_or_instance_id: i64,
}

impl From<&RestartReplicaDescription> for FABRIC_RESTART_REPLICA_DESCRIPTION {
    fn from(value: &RestartReplicaDescription) -> Self {
        Self {
            NodeName: PCWSTR(value.node_name.as_ptr()),
            PartitionId: value.partition_id,
            ReplicaOrInstanceId: value.replica_or_instance_id,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_REMOVE_REPLICA_DESCRIPTION
pub struct RemoveReplicaDescription {
    pub node_name: WString,
    pub partition_id: GUID,
    pub replica_or_instance_id: i64,
    // TODO: support force flag
}

impl From<&RemoveReplicaDescription> for FABRIC_REMOVE_REPLICA_DESCRIPTION {
    fn from(value: &RemoveReplicaDescription) -> Self {
        Self {
            NodeName: PCWSTR(value.node_name.as_ptr()),
            PartitionId: value.partition_id,
            ReplicaOrInstanceId: value.replica_or_instance_id,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct DeployedStatefulServiceReplicaDetailQueryResult {
    pub service_name: WString,
    pub partition_id: GUID,
    pub replica_id: i64,
    pub current_service_operation: QueryServiceOperationName,
    pub current_service_operation_start_time_utc: SystemTime,
    pub current_replicator_operation: QueryReplicatorOperationName,
    pub read_status: ServicePartitionAccessStatus,
    pub write_status: ServicePartitionAccessStatus,
    // pub reported_load: LoadMetricReportList, // TODO: convert value.ReportedLoad to LoadMetricReportList
    // pub replicator_status: ReplicatorStatus, // TODO: convert value.ReplicatorStatus to ReplicatorStatus
    // pub Reserved: *mut core::ffi::c_void,
}

impl DeployedStatefulServiceReplicaDetailQueryResult {
    pub fn new(value: &FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM) -> Self {
        Self {
            service_name: WString::from(PCWSTR(value.ServiceName.0)),
            partition_id: value.PartitionId,
            replica_id: value.ReplicaId,
            current_service_operation: (value.CurrentServiceOperation).into(),
            current_service_operation_start_time_utc: SystemTime::UNIX_EPOCH, // TODO: convert Win32 FILETIME to SystemTime in Unix or Win32 depending on the platform
            current_replicator_operation: (value.CurrentReplicatorOperation).into(),
            read_status: (value.ReadStatus).into(),
            write_status: (value.WriteStatus).into(),
        }
    }
}

// FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct DeployedStatelessServiceInstanceQueryResult {
    pub service_name: WString,
    pub service_type_name: WString,
    pub service_manifest_version: WString,
    pub code_package_name: WString,
    pub partition_id: GUID,
    pub instance_id: i64,
    pub replica_status: QueryServiceReplicaStatus,
    pub address: WString,
    // pub reserved: *mut core::ffi::c_void,
}

impl DeployedStatelessServiceInstanceQueryResult {
    pub fn new(value: &FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM) -> Self {
        Self {
            service_name: WString::from(PCWSTR(value.ServiceName.0)),
            service_type_name: WString::from(value.ServiceTypeName),
            service_manifest_version: WString::from(value.ServiceManifestVersion),
            code_package_name: WString::from(value.CodePackageName),
            partition_id: value.PartitionId,
            instance_id: value.InstanceId,
            replica_status: (&value.ReplicaStatus).into(),
            address: WString::from(value.Address),
        }
    }
}

// FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION
#[derive(Debug, Clone)]
pub struct DeployedServiceReplicaDetailQueryDescription {
    pub node_name: WString,
    pub partition_id: GUID,
    pub replica_id: i64,
}

impl From<&DeployedServiceReplicaDetailQueryDescription>
    for FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION
{
    fn from(value: &DeployedServiceReplicaDetailQueryDescription) -> Self {
        Self {
            NodeName: PCWSTR(value.node_name.as_ptr()),
            PartitionId: value.partition_id,
            ReplicaId: value.replica_id,
            Reserved: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeployedServiceReplicaDetailQueryResultValue {
    Invalid,
    Stateful(DeployedStatefulServiceReplicaDetailQueryResult),
    Stateless(DeployedStatelessServiceInstanceQueryResult),
}

// FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM
pub struct DeployedServiceReplicaDetailQueryResult {
    pub value: DeployedServiceReplicaDetailQueryResultValue,
}

impl DeployedServiceReplicaDetailQueryResult {
    pub fn new(com: IFabricGetDeployedServiceReplicaDetailResult) -> Self {
        let replica_detail = unsafe { com.get_ReplicaDetail().as_ref().unwrap() };
        let value = match replica_detail.Kind {
            FABRIC_SERVICE_KIND_STATEFUL => {
                let raw = unsafe {
                    (replica_detail.Value
                        as *const FABRIC_DEPLOYED_STATEFUL_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                DeployedServiceReplicaDetailQueryResultValue::Stateful(
                    DeployedStatefulServiceReplicaDetailQueryResult::new(raw),
                )
            }
            FABRIC_SERVICE_KIND_STATELESS => {
                let raw = unsafe {
                    (replica_detail.Value
                        as *const FABRIC_DEPLOYED_STATELESS_SERVICE_INSTANCE_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                DeployedServiceReplicaDetailQueryResultValue::Stateless(
                    DeployedStatelessServiceInstanceQueryResult::new(raw),
                )
            }
            _ => DeployedServiceReplicaDetailQueryResultValue::Invalid,
        };
        Self { value }
    }
}
