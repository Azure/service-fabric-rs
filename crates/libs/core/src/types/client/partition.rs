// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{GUID, HSTRING};
use mssf_com::{
    FabricClient::IFabricGetPartitionListResult2,
    FabricTypes::{
        FABRIC_QUERY_SERVICE_PARTITION_STATUS, FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY,
        FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING, FABRIC_SERVICE_KIND_STATEFUL,
        FABRIC_SERVICE_KIND_STATELESS, FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM,
        FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM,
        FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM, FABRIC_URI,
    },
};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    types::{HealthState, ServicePartitionInformation},
};

// Partition related types
// FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION
pub struct ServicePartitionQueryDescription {
    pub service_name: HSTRING,
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
            ServiceName: FABRIC_URI(value.service_name.as_ptr() as *mut u16),
            PartitionIdFilter: filter,
            Reserved: std::ptr::null_mut(),
        }
    }
}

pub struct ServicePartitionList {
    com: IFabricGetPartitionListResult2,
}

type ServicePartitionListIter<'a> = FabricIter<
    'a,
    FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM,
    ServicePartition,
    ServicePartitionList,
>;

impl FabricListAccessor<FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM> for ServicePartitionList {
    fn get_count(&self) -> u32 {
        let raw = unsafe { self.com.get_PartitionList().as_ref() };
        raw.unwrap().Count
    }

    fn get_first_item(&self) -> *const FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM {
        let raw = unsafe { self.com.get_PartitionList().as_ref() };
        raw.unwrap().Items
    }
}

impl ServicePartitionList {
    pub fn new(com: IFabricGetPartitionListResult2) -> Self {
        Self { com }
    }

    pub fn iter(&self) -> ServicePartitionListIter {
        ServicePartitionListIter::new(self, self)
    }
}

// FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM
pub enum ServicePartition {
    Invalid,
    Stateful(StatefulServicePartition),
    Stateless(StatelessServicePartition),
}

impl From<&FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM> for ServicePartition {
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

#[derive(Debug, PartialEq, Clone)]
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
pub struct StatefulServicePartition {
    pub partition_information: ServicePartitionInformation,
    pub target_replica_set_size: u32,
    pub min_replica_set_size: u32,
    pub health_state: HealthState,
    pub partition_status: ServicePartitionStatus,
    pub last_quorum_loss_duration_in_seconds: i64,
    // TODO: reserved fields
    //pub Reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM> for StatefulServicePartition {
    fn from(value: &FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Self {
        Self {
            partition_information: unsafe { value.PartitionInformation.as_ref().unwrap().into() },
            target_replica_set_size: value.TargetReplicaSetSize,
            min_replica_set_size: value.MinReplicaSetSize,
            health_state: (&value.HealthState).into(),
            partition_status: (&value.PartitionStatus).into(),
            last_quorum_loss_duration_in_seconds: value.LastQuorumLossDurationInSeconds,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatelessServicePartition {
    pub partition_information: ServicePartitionInformation,
    pub instance_count: u32,
    pub health_state: HealthState,
    pub partition_status: ServicePartitionStatus,
    // TODO: reserved fields
    // pub Reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM> for StatelessServicePartition {
    fn from(value: &FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Self {
        Self {
            partition_information: unsafe { value.PartitionInformation.as_ref().unwrap().into() },
            instance_count: value.InstanceCount,
            health_state: (&value.HealthState).into(),
            partition_status: (&value.PartitionStatus).into(),
        }
    }
}
