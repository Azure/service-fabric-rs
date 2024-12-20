// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{WString, GUID};
use mssf_com::FabricTypes::{
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS, FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING,
    FABRIC_SERVICE_PARTITION_INFORMATION, FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
    FABRIC_SERVICE_PARTITION_KIND_INVALID, FABRIC_SERVICE_PARTITION_KIND_NAMED,
    FABRIC_SERVICE_PARTITION_KIND_SINGLETON, FABRIC_SINGLETON_PARTITION_INFORMATION,
};

use crate::strings::WStringWrap;

// FABRIC_SERVICE_PARTITION_INFORMATION
#[derive(Debug, Clone)]
pub enum ServicePartitionInformation {
    Invalid,
    Singleton(SingletonPartitionInfomation),
    Int64Range(Int64PartitionInfomation),
    Named(NamedPartitionInfomation),
}

#[derive(Debug, Clone)]
pub struct SingletonPartitionInfomation {
    pub id: GUID,
}

#[derive(Debug, Clone)]
pub struct Int64PartitionInfomation {
    pub id: GUID,
    pub low_key: i64,
    pub high_key: i64,
}

#[derive(Debug, Clone)]
pub struct NamedPartitionInfomation {
    pub id: GUID,
    pub name: WString,
}

impl From<&FABRIC_SINGLETON_PARTITION_INFORMATION> for SingletonPartitionInfomation {
    fn from(value: &FABRIC_SINGLETON_PARTITION_INFORMATION) -> Self {
        Self { id: value.Id }
    }
}

impl From<&FABRIC_INT64_RANGE_PARTITION_INFORMATION> for Int64PartitionInfomation {
    fn from(value: &FABRIC_INT64_RANGE_PARTITION_INFORMATION) -> Self {
        Self {
            high_key: value.HighKey,
            id: value.Id,
            low_key: value.LowKey,
        }
    }
}

impl From<&FABRIC_NAMED_PARTITION_INFORMATION> for NamedPartitionInfomation {
    fn from(value: &FABRIC_NAMED_PARTITION_INFORMATION) -> Self {
        Self {
            id: value.Id,
            name: WStringWrap::from(value.Name).into(),
        }
    }
}

impl From<&FABRIC_SERVICE_PARTITION_INFORMATION> for ServicePartitionInformation {
    fn from(value: &FABRIC_SERVICE_PARTITION_INFORMATION) -> Self {
        match value.Kind {
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_SINGLETON_PARTITION_INFORMATION)
                        .as_ref()
                        .unwrap()
                };
                Self::Singleton(raw.into())
            }
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_INT64_RANGE_PARTITION_INFORMATION)
                        .as_ref()
                        .unwrap()
                };
                Self::Int64Range(raw.into())
            }
            FABRIC_SERVICE_PARTITION_KIND_NAMED => {
                let raw = unsafe {
                    (value.Value as *const FABRIC_NAMED_PARTITION_INFORMATION)
                        .as_ref()
                        .unwrap()
                };
                Self::Named(raw.into())
            }
            FABRIC_SERVICE_PARTITION_KIND_INVALID => Self::Invalid,
            _ => Self::Invalid,
        }
    }
}

/// FABRIC_SERVICE_PARTITION_ACCESS_STATUS
/// Remarks:
/// PartitionAccessStatus is used to check that a read or write operation is allowed.
/// When service replicas handle a client request, they should verify that the system is
/// in a state that allows processing. By checking the ReadStatus or WriteStatus as appropriate,
/// the replica can be notified of conditions that prevent correct operation.
/// Note that write operations might still see an exception from the replicator for one of these
/// conditions, because the condition might change between the WriteStatus check and the call
/// to StateReplicator.Replicate() (Not yet supported in mssf).
#[derive(Debug, Clone, PartialEq)]
pub enum ServicePartitionAccessStatus {
    Invalid,
    /// Indicates that the read or write operation access is granted and the operation is allowed.
    Granted,
    /// Indicates that the client should try again later, because a reconfiguration is in progress.
    /// After the reconfiguration is completed, a new status is returned that gives further instructions.
    /// The client should retry the operation at this replica
    ReconfigurationPending,
    /// Indicates that this client request was received by a replica that is not a Primary replica.
    /// The read or write operation cannot be performed at this replica.
    /// The client should attempt to use the naming service to identify the correct primary replica.
    NotPrimary,
    /// Indicates that no write quorum is available and, therefore, no write operation can be accepted.
    /// The client should retry the operation at this replica.
    NoWriteQuorum,
}

impl From<FABRIC_SERVICE_PARTITION_ACCESS_STATUS> for ServicePartitionAccessStatus {
    fn from(value: FABRIC_SERVICE_PARTITION_ACCESS_STATUS) -> Self {
        match value {
            FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID => Self::Invalid,
            FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED => Self::Granted,
            FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY => Self::NotPrimary,
            FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM => Self::NoWriteQuorum,
            FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING => {
                Self::ReconfigurationPending
            }
            _ => Self::Invalid,
        }
    }
}

impl From<ServicePartitionAccessStatus> for FABRIC_SERVICE_PARTITION_ACCESS_STATUS {
    fn from(value: ServicePartitionAccessStatus) -> Self {
        match value {
            ServicePartitionAccessStatus::Invalid => FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID,
            ServicePartitionAccessStatus::Granted => FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED,
            ServicePartitionAccessStatus::ReconfigurationPending => {
                FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING
            }
            ServicePartitionAccessStatus::NotPrimary => {
                FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY
            }
            ServicePartitionAccessStatus::NoWriteQuorum => {
                FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM
            }
        }
    }
}
