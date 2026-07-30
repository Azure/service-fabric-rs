// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{
    GUID,
    mem::GetRaw,
    types::{LoadMetricReport, PagingStatus, Uri},
};
use mssf_com::{
    FabricClient::{IFabricGetPartitionListResult2, IFabricGetPartitionLoadInformationResult},
    FabricTypes::{
        FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION, FABRIC_QUERY_SERVICE_PARTITION_STATUS,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING, FABRIC_SERVICE_KIND_STATEFUL,
        FABRIC_SERVICE_KIND_STATELESS, FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM,
        FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM,
        FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM,
    },
};

use crate::types::{HealthState, HealthStateFilterFlags, ServicePartitionInformation};

// Partition related types
// FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ServicePartitionQueryDescription {
    pub service_name: Uri,
    pub partition_id_filter: Option<GUID>,
    // TODO: continuation token
}

impl From<&ServicePartitionQueryDescription> for FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION {
    fn from(value: &ServicePartitionQueryDescription) -> Self {
        let filter = match value.partition_id_filter {
            Some(x) => x,
            None => GUID::zeroed(), // empty
        };
        Self {
            ServiceName: value.service_name.as_raw(),
            PartitionIdFilter: filter,
            Reserved: std::ptr::null_mut(),
        }
    }
}

pub struct ServicePartitionList {
    pub service_partitions: Vec<ServicePartitionQueryResultItem>,
    pub paging_status: Option<PagingStatus>,
}

impl From<&IFabricGetPartitionListResult2> for ServicePartitionList {
    fn from(com: &IFabricGetPartitionListResult2) -> Self {
        let service_partitions = unsafe { com.get_PartitionList().as_ref() }
            .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
            .unwrap_or_default();
        let paging_status = unsafe { com.get_PagingStatus().as_ref() }.map(|s| s.into());
        Self {
            service_partitions,
            paging_status,
        }
    }
}

// FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub enum ServicePartitionQueryResultItem {
    Invalid,
    Stateful(StatefulServicePartitionQueryResult),
    Stateless(StatelessServicePartitionQueryResult),
}

impl From<&FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM> for ServicePartitionQueryResultItem {
    fn from(value: &FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Self {
        match value.Kind {
            FABRIC_SERVICE_KIND_STATEFUL => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                Self::Stateful(raw.into())
            }
            FABRIC_SERVICE_KIND_STATELESS => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                Self::Stateless(raw.into())
            }
            _ => Self::Invalid,
        }
    }
}

impl ServicePartitionQueryResultItem {
    pub fn get_partition_id(&self) -> GUID {
        match self {
            ServicePartitionQueryResultItem::Stateful(stateful) => {
                stateful.partition_information.get_partition_id()
            }
            ServicePartitionQueryResultItem::Stateless(stateless) => {
                stateless.partition_information.get_partition_id()
            }
            ServicePartitionQueryResultItem::Invalid => GUID::zeroed(),
        }
    }
    pub fn get_health_state(&self) -> HealthState {
        match self {
            ServicePartitionQueryResultItem::Stateful(stateful) => stateful.health_state,
            ServicePartitionQueryResultItem::Stateless(stateless) => stateless.health_state,
            ServicePartitionQueryResultItem::Invalid => HealthState::Invalid,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServicePartitionStatus {
    Invalid,
    Ready,
    NotReady,
    InQuorumLoss,
    Reconfiguring,
    Deleting,
}

impl From<&FABRIC_QUERY_SERVICE_PARTITION_STATUS> for ServicePartitionStatus {
    fn from(value: &FABRIC_QUERY_SERVICE_PARTITION_STATUS) -> Self {
        match *value {
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID => Self::Invalid,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY => Self::Ready,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY => Self::NotReady,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS => Self::InQuorumLoss,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING => Self::Reconfiguring,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING => Self::Deleting,
            _ => Self::Invalid,
        }
    }
}

// FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct StatefulServicePartitionQueryResult {
    pub partition_information: ServicePartitionInformation,
    pub target_replica_set_size: u32,
    pub min_replica_set_size: u32,
    pub health_state: HealthState,
    pub partition_status: ServicePartitionStatus,
    pub last_quorum_loss_duration_in_seconds: i64,
    pub primary_epoch: crate::types::Epoch,
    pub auxiliary_replica_count: u32,
}

impl From<&FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM>
    for StatefulServicePartitionQueryResult
{
    fn from(value: &FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Self {
        let ex1 = unsafe {
            (value.Reserved as *const mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX1).as_ref()
        }.unwrap();
        let primary_epoch = (&ex1.PrimaryEpoch).into();
        let ex2 = unsafe {
            (ex1.Reserved as *const mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM_EX2).as_ref()
        }.unwrap();
        let auxiliary_replica_count = ex2.AuxiliaryReplicaCount;
        Self {
            partition_information: unsafe { value.PartitionInformation.as_ref().unwrap().into() },
            target_replica_set_size: value.TargetReplicaSetSize,
            min_replica_set_size: value.MinReplicaSetSize,
            health_state: (&value.HealthState).into(),
            partition_status: (&value.PartitionStatus).into(),
            last_quorum_loss_duration_in_seconds: value.LastQuorumLossDurationInSeconds,
            primary_epoch,
            auxiliary_replica_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatelessServicePartitionQueryResult {
    pub partition_information: ServicePartitionInformation,
    pub instance_count: u32,
    pub health_state: HealthState,
    pub partition_status: ServicePartitionStatus,
    // TODO: reserved fields
    // pub Reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM>
    for StatelessServicePartitionQueryResult
{
    fn from(value: &FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Self {
        Self {
            partition_information: unsafe { value.PartitionInformation.as_ref().unwrap().into() },
            instance_count: value.InstanceCount,
            health_state: (&value.HealthState).into(),
            partition_status: (&value.PartitionStatus).into(),
        }
    }
}

/// Wrapper around FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION
pub struct PartitionLoadInformationQueryDescription {
    pub partition_id: GUID,
}

impl From<&PartitionLoadInformationQueryDescription>
    for FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION
{
    fn from(value: &PartitionLoadInformationQueryDescription) -> Self {
        Self {
            PartitionId: value.partition_id,
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// Wrapper around FABRIC_PARTITION_LOAD_INFORMATION
///
/// Note that the COM object IFabricGetPartitionLoadInformationResult passed in from the constructor is
/// the owner of this structure, and therefore the primary load metric reports and the secondary load
/// metric reports must be retreived from the owner COM object and cannot be owned by this wrapper structure.
/// Therefore, we created PrimaryLoadMetricReportList and SecondaryLoadMetricReportList to represent the
/// load metric reports for primary and secondary respectively, instead of using a same wrapper type for
///  FABRIC_LOAD_METRIC_REPORT_LIST.
pub struct GetPartitionLoadInformationResult {
    pub partition_id: GUID,
    pub primary_load_metric_reports: Vec<LoadMetricReport>,
    pub secondary_load_metric_reports: Vec<LoadMetricReport>,
    // TODO: implement Reserved
}

impl From<&IFabricGetPartitionLoadInformationResult> for GetPartitionLoadInformationResult {
    fn from(com: &IFabricGetPartitionLoadInformationResult) -> Self {
        let raw = unsafe { com.get_PartitionLoadInformation().as_ref().unwrap() };
        let partition_id = raw.PartitionId;
        let primary_load_metric_reports = unsafe {
            raw.PrimaryLoadMetricReports
                .as_ref()
                .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
                .unwrap_or_default()
        };
        let secondary_load_metric_reports = unsafe {
            raw.SecondaryLoadMetricReports
                .as_ref()
                .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
                .unwrap_or_default()
        };
        Self {
            partition_id,
            primary_load_metric_reports,
            secondary_load_metric_reports,
        }
    }
}

// FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct PartitionHealthQueryDescription {
    pub partition_id: GUID,
    pub health_policy: Option<super::ApplicationHealthPolicy>,
    pub events_filter: Option<super::HealthEventsFilter>,
    pub replicas_filter: Option<super::ReplicaHealthStatesFilter>,
    // TODO: other fields
    // pub health_statistics_filter: Option<super::HealthStatisticsFilter>,
}

impl crate::mem::GetRawWithBoxPool<mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION>
    for PartitionHealthQueryDescription
{
    fn get_raw_with_pool(
        &self,
        pool: &mut crate::mem::BoxPool,
    ) -> mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION {
        let health_policy = self
            .health_policy
            .as_ref()
            .map(|p| {
                let b = Box::new(p.get_raw_with_pool(pool));
                pool.push(b)
            })
            .unwrap_or_default();

        let events_filter = self
            .events_filter
            .as_ref()
            .map(|f| {
                let b = Box::new(f.get_raw());
                pool.push(b)
            })
            .unwrap_or_default();

        let replicas_filter = self
            .replicas_filter
            .as_ref()
            .map(|f| {
                let b = Box::new(f.get_raw());
                pool.push(b)
            })
            .unwrap_or_default();

        let ex1 = pool.push(Box::new(
            mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION_EX1 {
                Reserved: std::ptr::null_mut(),
                HealthStatisticsFilter: std::ptr::null_mut(),
            },
        ));

        mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION {
            EventsFilter: events_filter,
            HealthPolicy: health_policy,
            PartitionId: self.partition_id,
            ReplicasFilter: replicas_filter,
            Reserved: ex1 as *mut _,
        }
    }
}

// IFabricPartitionHealthResult
#[derive(Debug, Clone)]
pub struct PartitionHealthResult {
    pub partition_id: GUID,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<super::HealthEvent>,
    pub replica_health_states: Vec<super::ReplicaHealthState>,
    // TODO: other fields
    // pub health_statistics: super::HealthStatistics,
    // pub unhealthy_evaluations: Vec<super::HealthEvaluation>,
}

impl From<&mssf_com::FabricClient::IFabricPartitionHealthResult> for PartitionHealthResult {
    fn from(value: &mssf_com::FabricClient::IFabricPartitionHealthResult) -> Self {
        let raw = unsafe { value.get_PartitionHealth().as_ref().unwrap() };
        raw.into()
    }
}

impl From<&mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH> for PartitionHealthResult {
    fn from(raw: &mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH) -> Self {
        let health_event_list = unsafe { raw.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        let replica_health_states = unsafe { raw.ReplicaHealthStates.as_ref() }
            .map_or(vec![], |list| {
                crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
            });
        Self {
            partition_id: raw.PartitionId,
            aggregated_health_state: (&raw.AggregatedHealthState).into(),
            health_events: health_event_list,
            replica_health_states,
        }
    }
}

// FABRIC_PARTITION_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct PartitionHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags,
}

impl GetRaw<mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_STATES_FILTER>
    for PartitionHealthStatesFilter
{
    fn get_raw(&self) -> mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_STATES_FILTER {
        mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_PARTITION_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct PartitionHealthState {
    pub partition_id: GUID,
    pub aggregated_health_state: HealthState,
}

impl From<&mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_STATE> for PartitionHealthState {
    fn from(value: &mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH_STATE) -> Self {
        Self {
            partition_id: value.PartitionId,
            aggregated_health_state: (&value.AggregatedHealthState).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::{BoxPool, GetRawWithBoxPool};
    use crate::types::{ReplicaHealthState, ReplicaHealthStatesFilter};

    #[test]
    fn test_partition_health_query_description_replicas_filter_raw() {
        let desc = PartitionHealthQueryDescription {
            partition_id: GUID::zeroed(),
            replicas_filter: Some(ReplicaHealthStatesFilter {
                health_state_filter: HealthStateFilterFlags::ERROR,
            }),
            ..Default::default()
        };
        let mut pool = BoxPool::new();
        let raw = desc.get_raw_with_pool(&mut pool);
        assert!(!raw.ReplicasFilter.is_null());
        assert_eq!(
            unsafe { (*raw.ReplicasFilter).HealthStateFilter },
            HealthStateFilterFlags::ERROR.bits() as u32
        );
    }

    #[test]
    fn test_partition_health_query_description_no_replicas_filter_raw() {
        let desc = PartitionHealthQueryDescription {
            partition_id: GUID::zeroed(),
            ..Default::default()
        };
        let mut pool = BoxPool::new();
        let raw = desc.get_raw_with_pool(&mut pool);
        assert!(raw.ReplicasFilter.is_null());
    }

    #[test]
    fn test_partition_health_result_replica_health_states() {
        let replica_state = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_STATE {
            PartitionId: GUID::zeroed(),
            ReplicaId: 42,
            AggregatedHealthState: mssf_com::FabricTypes::FABRIC_HEALTH_STATE_OK,
            Reserved: std::ptr::null_mut(),
        };
        let replica_health_state = mssf_com::FabricTypes::FABRIC_REPLICA_HEALTH_STATE {
            Kind: mssf_com::FabricTypes::FABRIC_SERVICE_KIND_STATEFUL,
            Value: &replica_state as *const _ as *mut _,
        };
        let list = mssf_com::FabricTypes::FABRIC_REPLICA_HEALTH_STATE_LIST {
            Count: 1,
            Items: &replica_health_state,
        };
        let raw = mssf_com::FabricTypes::FABRIC_PARTITION_HEALTH {
            PartitionId: GUID::zeroed(),
            AggregatedHealthState: mssf_com::FabricTypes::FABRIC_HEALTH_STATE_OK,
            HealthEvents: std::ptr::null(),
            ReplicaHealthStates: &list,
            Reserved: std::ptr::null_mut(),
        };
        let result = PartitionHealthResult::from(&raw);
        assert_eq!(result.replica_health_states.len(), 1);
        match &result.replica_health_states[0] {
            ReplicaHealthState::Stateful(s) => assert_eq!(s.replica_id, 42),
            _ => panic!("expected stateful replica health state"),
        }
    }
}
