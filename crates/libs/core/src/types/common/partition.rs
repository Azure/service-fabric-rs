// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{GUID, HSTRING};
use mssf_com::FabricTypes::{
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_SERVICE_PARTITION_INFORMATION, FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
    FABRIC_SERVICE_PARTITION_KIND_INVALID, FABRIC_SERVICE_PARTITION_KIND_NAMED,
    FABRIC_SERVICE_PARTITION_KIND_SINGLETON, FABRIC_SINGLETON_PARTITION_INFORMATION,
};

use crate::strings::HSTRINGWrap;

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
    pub name: HSTRING,
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
            name: HSTRINGWrap::from(value.Name).into(),
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
