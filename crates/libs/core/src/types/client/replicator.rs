// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Module that defines replicator-related types through FabricClient

use std::time::SystemTime;

use mssf_com::FabricTypes::{
    FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT, FABRIC_REMOTE_REPLICATOR_STATUS,
    FABRIC_REMOTE_REPLICATOR_STATUS_LIST, FABRIC_REPLICATOR_QUEUE_STATUS,
    FABRIC_REPLICATOR_STATUS_QUERY_RESULT, FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
    FABRIC_REPLICA_ROLE_IDLE_SECONDARY, FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
    FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT,
};

use crate::iter::FabricListAccessor;

// FABRIC_REMOTE_REPLICATOR_STATUS_LIST
#[derive(Debug, Clone)]
pub struct RemoteReplicatorStatusList {
    pub count: u32,
    pub items: *const FABRIC_REMOTE_REPLICATOR_STATUS,
}

impl From<&FABRIC_REMOTE_REPLICATOR_STATUS_LIST> for RemoteReplicatorStatusList {
    fn from(value: &FABRIC_REMOTE_REPLICATOR_STATUS_LIST) -> Self {
        Self {
            count: value.Count,
            items: value.Items,
        }
    }
}

impl FabricListAccessor<FABRIC_REMOTE_REPLICATOR_STATUS> for RemoteReplicatorStatusList {
    fn get_count(&self) -> u32 {
        self.count
    }

    fn get_first_item(&self) -> *const FABRIC_REMOTE_REPLICATOR_STATUS {
        self.items
    }
}

// FABRIC_REPLICATOR_QUEUE_STATUS
#[derive(Debug, Clone)]
pub struct ReplicatorQueueStatus {
    pub queue_utilization_percentage: u32,
    pub queue_memory_size: i64,
    pub first_sequence_number: i64,
    pub completed_sequence_number: i64,
    pub committed_sequence_number: i64,
    pub last_sequence_number: i64,
    // pub reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_REPLICATOR_QUEUE_STATUS> for ReplicatorQueueStatus {
    fn from(value: &FABRIC_REPLICATOR_QUEUE_STATUS) -> Self {
        Self {
            queue_utilization_percentage: value.QueueUtilizationPercentage,
            queue_memory_size: value.QueueMemorySize,
            first_sequence_number: value.FirstSequenceNumber,
            completed_sequence_number: value.CompletedSequenceNumber,
            committed_sequence_number: value.CommittedSequenceNumber,
            last_sequence_number: value.LastSequenceNumber,
        }
    }
}

// FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT
#[derive(Debug, Clone)]
pub struct PrimaryReplicatorStatus {
    pub replication_queue_status: ReplicatorQueueStatus,
    pub remote_replicators: RemoteReplicatorStatusList,
    // pub reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT> for PrimaryReplicatorStatus {
    fn from(value: &FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT) -> Self {
        Self {
            replication_queue_status: unsafe {
                ReplicatorQueueStatus::from(&*value.ReplicationQueueStatus)
            },
            remote_replicators: unsafe {
                RemoteReplicatorStatusList::from(&*value.RemoteReplicators)
            },
        }
    }
}

// FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT
#[derive(Debug, Clone)]
pub struct SecondaryReplicatorStatus {
    pub replication_queue_status: ReplicatorQueueStatus,
    pub last_replication_operation_received_time_utc: SystemTime,
    pub is_in_build: bool,
    pub copy_queue_status: ReplicatorQueueStatus,
    pub last_copy_operation_received_time_utc: SystemTime,
    pub last_acknowledgement_sent_time_utc: SystemTime,
    // pub reserved: *mut core::ffi::c_void,
}

impl From<&FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT> for SecondaryReplicatorStatus {
    fn from(value: &FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT) -> Self {
        Self {
            replication_queue_status: unsafe {
                ReplicatorQueueStatus::from(&*value.ReplicationQueueStatus)
            },
            last_replication_operation_received_time_utc: SystemTime::UNIX_EPOCH, // TODO: convert Win32 FILETIME to SystemTime in Unix or Win32 depending on the platform
            is_in_build: value.IsInBuild,
            copy_queue_status: unsafe { ReplicatorQueueStatus::from(&*value.CopyQueueStatus) },
            last_copy_operation_received_time_utc: SystemTime::UNIX_EPOCH, // TODO: convert Win32 FILETIME to SystemTime in Unix or Win32 depending on the platform
            last_acknowledgement_sent_time_utc: SystemTime::UNIX_EPOCH, // TODO: convert Win32 FILETIME to SystemTime in Unix or Win32 depending on the platform
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReplicatorStatus {
    ActiveSecondary(SecondaryReplicatorStatus),
    IdleSecondary(SecondaryReplicatorStatus),
    None,
    Primary(PrimaryReplicatorStatus),
    Unknown,
}

impl From<&FABRIC_REPLICATOR_STATUS_QUERY_RESULT> for ReplicatorStatus {
    fn from(value: &FABRIC_REPLICATOR_STATUS_QUERY_RESULT) -> Self {
        match value.Role {
            FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT)
                        .as_ref()
                        .unwrap()
                };
                let secondary_replicator_status = SecondaryReplicatorStatus::from(raw);
                Self::ActiveSecondary(secondary_replicator_status)
            }
            FABRIC_REPLICA_ROLE_IDLE_SECONDARY => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_SECONDARY_REPLICATOR_STATUS_QUERY_RESULT)
                        .as_ref()
                        .unwrap()
                };
                let secondary_replicator_status = SecondaryReplicatorStatus::from(raw);
                Self::IdleSecondary(secondary_replicator_status)
            }
            FABRIC_REPLICA_ROLE_NONE => Self::None,
            FABRIC_REPLICA_ROLE_PRIMARY => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_PRIMARY_REPLICATOR_STATUS_QUERY_RESULT)
                        .as_ref()
                        .unwrap()
                };
                let primary_replicator_status = PrimaryReplicatorStatus::from(raw);
                Self::Primary(primary_replicator_status)
            }
            _ => Self::Unknown,
        }
    }
}
