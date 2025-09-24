// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::ffi::c_void;

use crate::{GUID, WString};
use mssf_com::FabricTypes::{
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION, FABRIC_PARTITION_SCHEME,
    FABRIC_PARTITION_SCHEME_NAMED, FABRIC_PARTITION_SCHEME_SINGLETON,
    FABRIC_PARTITION_SCHEME_UNIFORM_INT64_RANGE, FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE,
    FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_EXCLUSIVE_PROCESS,
    FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_SHARED_PROCESS, FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_GRANTED, FABRIC_SERVICE_PARTITION_ACCESS_STATUS_INVALID,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NO_WRITE_QUORUM,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_NOT_PRIMARY,
    FABRIC_SERVICE_PARTITION_ACCESS_STATUS_RECONFIGURATION_PENDING,
    FABRIC_SERVICE_PARTITION_INFORMATION, FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
    FABRIC_SERVICE_PARTITION_KIND_INVALID, FABRIC_SERVICE_PARTITION_KIND_NAMED,
    FABRIC_SERVICE_PARTITION_KIND_SINGLETON, FABRIC_SINGLETON_PARTITION_INFORMATION,
    FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION,
};
use windows_core::PCWSTR;

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

impl ServicePartitionInformation {
    pub fn get_partition_id(&self) -> GUID {
        match self {
            ServicePartitionInformation::Invalid => GUID::zeroed(),
            ServicePartitionInformation::Singleton(info) => info.id,
            ServicePartitionInformation::Int64Range(info) => info.id,
            ServicePartitionInformation::Named(info) => info.id,
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

// Partition Schemes
// FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION
#[derive(Debug)]
pub struct UniformIn64PartitionSchemeDescription {
    internal: Box<FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION>,
}

/// SAFETY: This is thread safe because we do not use the raw pointer.
unsafe impl Send for UniformIn64PartitionSchemeDescription {}
unsafe impl Sync for UniformIn64PartitionSchemeDescription {}

impl UniformIn64PartitionSchemeDescription {
    pub fn new(partition_count: i32, low_key: i64, high_key: i64) -> Self {
        UniformIn64PartitionSchemeDescription {
            internal: Box::new(FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION {
                PartitionCount: partition_count,
                LowKey: low_key,
                HighKey: high_key,
                Reserved: std::ptr::null_mut(),
            }),
        }
    }
    /// No lifetime requirement.
    pub fn as_raw(&self) -> &FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION {
        self.internal.as_ref()
    }
}

impl Clone for UniformIn64PartitionSchemeDescription {
    fn clone(&self) -> Self {
        Self::new(
            self.internal.PartitionCount,
            self.internal.LowKey,
            self.internal.HighKey,
        )
    }
}

// FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION
#[derive(Debug)]
pub struct NamedPartitionSchemeDescription {
    _names: Vec<WString>,
    _raw_names: Vec<PCWSTR>,
    internal: Box<FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION>,
}

/// SAFETY: This is thread safe because the raw pointers points to heap allocated memory.
unsafe impl Send for NamedPartitionSchemeDescription {}
unsafe impl Sync for NamedPartitionSchemeDescription {}

impl NamedPartitionSchemeDescription {
    /// Must have lifetime as self. Can be moved.
    pub fn as_raw(&self) -> &FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION {
        self.internal.as_ref()
    }

    pub fn get_ref(&self) -> &Vec<WString> {
        &self._names
    }
}

impl NamedPartitionSchemeDescription {
    pub fn new(names: Vec<WString>) -> Self {
        let raw_names: Vec<PCWSTR> = names.iter().map(|name| name.as_pcwstr()).collect();
        let internal = Box::new(FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION {
            PartitionCount: raw_names.len() as i32,
            Names: raw_names.as_ptr() as *mut _,
            Reserved: std::ptr::null_mut(),
        });
        NamedPartitionSchemeDescription {
            _names: names,
            _raw_names: raw_names,
            internal,
        }
    }
}

impl Clone for NamedPartitionSchemeDescription {
    fn clone(&self) -> Self {
        Self::new(self._names.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ServicePackageActivationMode {
    // Invalid = 0,
    #[default]
    SharedProcess,
    ExclusiveProcess,
}

impl From<ServicePackageActivationMode> for FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE {
    fn from(mode: ServicePackageActivationMode) -> Self {
        match mode {
            ServicePackageActivationMode::SharedProcess => {
                FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_SHARED_PROCESS
            }
            ServicePackageActivationMode::ExclusiveProcess => {
                FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE_EXCLUSIVE_PROCESS
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum PartitionSchemeDescription {
    #[default]
    Invalid,
    Singleton, // This should be nullptr when passed to com.
    Int64Range(UniformIn64PartitionSchemeDescription), // FABRIC_UNIFORM_INT64_RANGE_PARTITION_SCHEME_DESCRIPTION
    Named(NamedPartitionSchemeDescription),            // FABRIC_NAMED_PARTITION_SCHEME_DESCRIPTION
}

impl PartitionSchemeDescription {
    /// Needs to have lifetime as self. Can be moved.
    pub(crate) fn as_raw(&self) -> (FABRIC_PARTITION_SCHEME, *mut c_void) {
        match self {
            PartitionSchemeDescription::Singleton => {
                (FABRIC_PARTITION_SCHEME_SINGLETON, std::ptr::null_mut())
            }
            PartitionSchemeDescription::Int64Range(scheme) => (
                FABRIC_PARTITION_SCHEME_UNIFORM_INT64_RANGE,
                scheme.as_raw() as *const _ as *mut _,
            ),
            PartitionSchemeDescription::Named(scheme) => (
                FABRIC_PARTITION_SCHEME_NAMED,
                scheme.as_raw() as *const _ as *mut _,
            ),
            PartitionSchemeDescription::Invalid => panic!("Invalid partition scheme description"),
        }
    }
}
