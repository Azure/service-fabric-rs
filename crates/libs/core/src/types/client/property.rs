// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![cfg_attr(
    not(feature = "tokio_async"),
    allow(unused_imports, reason = "code configured out"),
    allow(dead_code, reason = "code configured out")
)]

use mssf_com::{
    FabricClient::IFabricNameEnumerationResult,
    FabricTypes::{
        FABRIC_ENUMERATION_BEST_EFFORT_FINISHED, FABRIC_ENUMERATION_BEST_EFFORT_MASK,
        FABRIC_ENUMERATION_BEST_EFFORT_MORE_DATA, FABRIC_ENUMERATION_CONSISTENT_FINISHED,
        FABRIC_ENUMERATION_CONSISTENT_MASK, FABRIC_ENUMERATION_CONSISTENT_MORE_DATA,
        FABRIC_ENUMERATION_FINISHED_MASK, FABRIC_ENUMERATION_INVALID,
        FABRIC_ENUMERATION_MORE_DATA_MASK, FABRIC_ENUMERATION_STATUS,
        FABRIC_NAMED_PROPERTY_METADATA,
    },
};
use windows_core::WString;

use crate::{strings::FabricStringListAccessorIter, types::Uri};

pub struct NameEnumerationResult {
    com: IFabricNameEnumerationResult,
}

impl NameEnumerationResult {
    pub(crate) fn from_com(com: IFabricNameEnumerationResult) -> Self {
        Self { com }
    }

    pub(crate) fn as_com(&self) -> &IFabricNameEnumerationResult {
        &self.com
    }

    pub fn get_enumeration_status(&self) -> EnumerationStatus {
        unsafe { self.com.get_EnumerationStatus().into() }
    }

    /// returns all SF names from the enumeration result.
    pub fn get_names(&self) -> crate::Result<Vec<Uri>> {
        let mut count = 0;
        let first_ptr = unsafe { self.com.GetNames(&mut count)? };
        let l = crate::strings::FabricStringListAccessor {
            itemcount: count,
            first_str: first_ptr as *mut _,
            phantom: std::marker::PhantomData,
        };
        let itr = FabricStringListAccessorIter::new(&l, &l);
        let data = itr.map(Uri::new).collect::<Vec<_>>();
        Ok(data)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumerationStatus {
    BestEffortFinished,
    BestEffortMask,
    BestEffortMoreData,
    ConsistentFinished,
    ConsistentMask,
    ConsistentMoreData,
    FinishedMask,
    Invalid,
    MoreDataMask,
}

impl From<FABRIC_ENUMERATION_STATUS> for EnumerationStatus {
    fn from(value: FABRIC_ENUMERATION_STATUS) -> Self {
        match value {
            FABRIC_ENUMERATION_BEST_EFFORT_FINISHED => Self::BestEffortFinished,
            FABRIC_ENUMERATION_BEST_EFFORT_MASK => Self::BestEffortMask,
            FABRIC_ENUMERATION_INVALID => Self::Invalid,
            FABRIC_ENUMERATION_BEST_EFFORT_MORE_DATA => Self::BestEffortMoreData,
            FABRIC_ENUMERATION_CONSISTENT_FINISHED => Self::ConsistentFinished,
            FABRIC_ENUMERATION_CONSISTENT_MASK => Self::ConsistentMask,
            FABRIC_ENUMERATION_CONSISTENT_MORE_DATA => Self::ConsistentMoreData,
            FABRIC_ENUMERATION_FINISHED_MASK => Self::FinishedMask,
            FABRIC_ENUMERATION_MORE_DATA_MASK => Self::MoreDataMask,
            _ => Self::Invalid,
        }
    }
}

/// Metadata for a named property.
pub struct NamedPropertyMetadata {
    pub property_name: WString,
    pub property_type_id: PropertyTypeId,
    pub value_size: i32,
    pub sequence_number: i64,
    pub last_modified_utc: windows_core::Win32::Foundation::FILETIME,
    pub name: Uri,
}

impl NamedPropertyMetadata {
    pub(crate) fn from_raw(ptr: &FABRIC_NAMED_PROPERTY_METADATA) -> Self {
        Self {
            property_name: ptr.PropertyName.into(),
            property_type_id: ptr.TypeId.into(),
            value_size: ptr.ValueSize,
            sequence_number: ptr.SequenceNumber,
            last_modified_utc: ptr.LastModifiedUtc,
            name: Uri::new(windows_core::PCWSTR(ptr.Name.0).into()),
        }
    }
}

pub struct PropertyMetadataResult {
    com: mssf_com::FabricClient::IFabricPropertyMetadataResult,
}

impl PropertyMetadataResult {
    pub(crate) fn from_com(com: mssf_com::FabricClient::IFabricPropertyMetadataResult) -> Self {
        Self { com }
    }

    pub fn get_metadata(&self) -> crate::Result<NamedPropertyMetadata> {
        let ptr = unsafe { self.com.get_Metadata().as_ref() }.unwrap();

        Ok(NamedPropertyMetadata::from_raw(ptr))
    }
}

// See: https://github.com/microsoft/service-fabric/blob/master/src/prod/src/managed/Api/src/System/Fabric/CheckValuePropertyOperation.cs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyTypeId {
    Invalid = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_INVALID.0 as isize,
    Binary = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_BINARY.0 as isize,
    Int64 = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_INT64.0 as isize,
    Double = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_DOUBLE.0 as isize,
    WString = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_WSTRING.0 as isize,
    Guid = mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_GUID.0 as isize,
}

impl From<mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_ID> for PropertyTypeId {
    fn from(value: mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_ID) -> Self {
        match value {
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_INVALID => PropertyTypeId::Invalid,
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_BINARY => PropertyTypeId::Binary,
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_INT64 => PropertyTypeId::Int64,
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_DOUBLE => PropertyTypeId::Double,
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_WSTRING => PropertyTypeId::WString,
            mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_GUID => PropertyTypeId::Guid,
            _ => PropertyTypeId::Invalid,
        }
    }
}

impl From<PropertyTypeId> for mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_ID {
    fn from(kind: PropertyTypeId) -> Self {
        mssf_com::FabricTypes::FABRIC_PROPERTY_TYPE_ID(kind as i32)
    }
}

// TODO: batch operations is net yet supported.
// // FABRIC_PROPERTY_BATCH_OPERATION
// pub enum PropertyBatchOperation {
//     Invalid,
//     Put{name: WString, type_id: PropertyTypeId, value: WString},
//     Get,
//     CheckExists,
//     CheckSequence,
//     Delete,
//     PutCustom,
//     CheckValue,
// }

// FABRIC_CHECK_EXISTS_PROPERTY_OPERATION

/// Represents the result of a property value operation.
pub struct PropertyValueResult {
    com: mssf_com::FabricClient::IFabricPropertyValueResult,
}

/// Converts a raw buffer into a `Vec<u8>` by making a copy.
fn raw_to_vec(ptr: *const u8, size: i32) -> Vec<u8> {
    if size <= 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(ptr, size as usize).to_vec() }
}

impl PropertyValueResult {
    pub(crate) fn from_com(com: mssf_com::FabricClient::IFabricPropertyValueResult) -> Self {
        Self { com }
    }

    /// Get the property as raw bytes regardless of type.
    pub fn get_named_property(&self) -> (NamedPropertyMetadata, Vec<u8>) {
        let ptr = unsafe { self.com.get_Property().as_ref() }.unwrap();
        let meta = NamedPropertyMetadata::from_raw(unsafe { ptr.Metadata.as_ref().unwrap() });
        let data = raw_to_vec(ptr.Value, meta.value_size);
        (meta, data)
    }

    pub fn get_value_as_binary(&self) -> crate::Result<Vec<u8>> {
        let mut bytecount = 0_u32;
        let data = unsafe { self.com.GetValueAsBinary(&mut bytecount as *mut u32) }?;

        Ok(raw_to_vec(data, bytecount as i32))
    }

    pub fn get_value_as_int64(&self) -> crate::Result<i64> {
        let value = unsafe { self.com.GetValueAsInt64()? };
        Ok(value)
    }
    pub fn get_value_as_double(&self) -> crate::Result<f64> {
        let value = unsafe { self.com.GetValueAsDouble()? };
        Ok(value)
    }
    pub fn get_value_as_wstring(&self) -> crate::Result<WString> {
        let value = unsafe { self.com.GetValueAsWString() }?;
        Ok(value.into())
    }
    pub fn get_value_as_guid(&self) -> crate::Result<windows_core::GUID> {
        let value = unsafe { self.com.GetValueAsGuid() }?;
        Ok(value)
    }
}
