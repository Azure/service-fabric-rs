use std::{ffi::c_void, time::Duration};

use mssf_com::{
    FabricCommon::FabricClient::{
        IFabricResolvedServicePartitionResult, IFabricServiceManagementClient6,
    },
    FABRIC_PARTITION_KEY_TYPE, FABRIC_PARTITION_KEY_TYPE_INT64, FABRIC_PARTITION_KEY_TYPE_INVALID,
    FABRIC_PARTITION_KEY_TYPE_NONE, FABRIC_PARTITION_KEY_TYPE_STRING,
    FABRIC_RESOLVED_SERVICE_ENDPOINT, FABRIC_SERVICE_ENDPOINT_ROLE, FABRIC_SERVICE_PARTITION_KIND,
    FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
    FABRIC_SERVICE_ROLE_INVALID, FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY,
    FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY, FABRIC_SERVICE_ROLE_STATELESS,
};
use windows_core::{Interface, HSTRING, PCWSTR};

use super::gen::svc::IFabricServiceManagementClient6Wrap;

// Service Management Client
pub struct ServiceManagementClient {
    gen_wrap: IFabricServiceManagementClient6Wrap,
}

impl ServiceManagementClient {
    pub fn from_com(com: IFabricServiceManagementClient6) -> Self {
        Self {
            gen_wrap: IFabricServiceManagementClient6Wrap::from_com(com),
        }
    }

    // Resolve service partition
    pub async fn resolve_service_partition(
        &self,
        name: &HSTRING,
        key_type: &PartitionKeyType,
        prev: Option<&ResolvedServicePartition>,
        timeout: Duration,
    ) -> windows_core::Result<ResolvedServicePartition> {
        let uri = unsafe { name.as_ptr().as_ref().unwrap() };
        // supply prev as null if not present
        let prev_ptr = std::ptr::null_mut();
        let prev_empty = unsafe { IFabricResolvedServicePartitionResult::from_raw(prev_ptr) };
        let mut prev_ref = &prev_empty;
        if prev.is_some() {
            prev_ref = &prev.unwrap().com;
        }

        let part_key_raw_ptr = key_type.get_raw();
        let part_key_raw = unsafe { &*part_key_raw_ptr };

        let fu = self.gen_wrap.ResolveServicePartition(
            uri,
            key_type.into(),
            part_key_raw,
            prev_ref,
            timeout.as_millis().try_into().unwrap(),
        );
        // Do not run destruct/drop. TODO: find a better way to construct null prev result.
        // prev_empty is constructed with a null ptr inside, and drop will try do Release()
        // which dereferences the null ptr.
        std::mem::forget(prev_empty);

        let com = fu.await?;
        let res = ResolvedServicePartition::from_com(com);
        Ok(res)
    }
}

// see ComFabricClient.cpp for conversion details in cpp
#[derive(Debug, PartialEq)]
pub enum PartitionKeyType {
    Int64(i64),
    Invalid,
    None,
    String(HSTRING),
}

impl PartitionKeyType {
    fn from_raw_svc_part(svc: ServicePartitionKind, data: *const c_void) -> PartitionKeyType {
        match svc {
            ServicePartitionKind::Int64Range => {
                let x = data as *mut i64;
                assert!(!x.is_null());
                PartitionKeyType::Int64(unsafe { *x })
            }
            ServicePartitionKind::Invalid => PartitionKeyType::Invalid,
            ServicePartitionKind::Singleton => PartitionKeyType::None,
            ServicePartitionKind::Named => {
                let x = data as *mut u16;
                assert!(!x.is_null());
                let s = HSTRING::from_wide(unsafe { PCWSTR::from_raw(x).as_wide() }).unwrap();
                PartitionKeyType::String(s)
            }
        }
    }
}

impl From<&PartitionKeyType> for FABRIC_PARTITION_KEY_TYPE {
    fn from(value: &PartitionKeyType) -> Self {
        match value {
            PartitionKeyType::Int64(_) => FABRIC_PARTITION_KEY_TYPE_INT64,
            PartitionKeyType::Invalid => FABRIC_PARTITION_KEY_TYPE_INVALID,
            PartitionKeyType::None => FABRIC_PARTITION_KEY_TYPE_NONE,
            PartitionKeyType::String(_) => FABRIC_PARTITION_KEY_TYPE_STRING,
        }
    }
}

impl PartitionKeyType {
    // get raw ptr to pass in com api
    fn get_raw(&self) -> *const c_void {
        match self {
            // Not sure if this is ok for i64
            PartitionKeyType::Int64(x) => x as *const i64 as *const c_void,
            PartitionKeyType::Invalid => std::ptr::null(),
            PartitionKeyType::None => std::ptr::null(),
            PartitionKeyType::String(x) => PCWSTR::from_raw(x.as_ptr()).as_ptr() as *const c_void,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ServicePartitionKind {
    Int64Range,
    Invalid,
    Named,
    Singleton,
}

impl From<&ServicePartitionKind> for FABRIC_SERVICE_PARTITION_KIND {
    fn from(value: &ServicePartitionKind) -> Self {
        match value {
            ServicePartitionKind::Int64Range => FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE,
            ServicePartitionKind::Invalid => FABRIC_SERVICE_PARTITION_KIND_INVALID,
            ServicePartitionKind::Named => FABRIC_SERVICE_PARTITION_KIND_NAMED,
            ServicePartitionKind::Singleton => FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
        }
    }
}

impl From<FABRIC_SERVICE_PARTITION_KIND> for ServicePartitionKind {
    fn from(value: FABRIC_SERVICE_PARTITION_KIND) -> Self {
        match value {
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => ServicePartitionKind::Int64Range,
            FABRIC_SERVICE_PARTITION_KIND_INVALID => ServicePartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_NAMED => ServicePartitionKind::Named,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => ServicePartitionKind::Singleton,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("unknown type: {:?}", value);
                } else {
                    ServicePartitionKind::Invalid
                }
            }
        }
    }
}

pub struct ResolvedServicePartition {
    com: IFabricResolvedServicePartitionResult,
}

impl ResolvedServicePartition {
    fn from_com(com: IFabricResolvedServicePartitionResult) -> Self {
        Self { com }
    }
}

#[derive(Debug)]
pub struct ResolvedServicePartitionInfo {
    pub name: HSTRING,
    pub service_partition_kind: ServicePartitionKind,
    pub partition_key_type: PartitionKeyType,
}

impl ResolvedServicePartition {
    // Get the service partition info/metadata
    pub fn get_info(&self) -> ResolvedServicePartitionInfo {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        let name =
            HSTRING::from_wide(unsafe { PCWSTR::from_raw(raw.ServiceName).as_wide() }).unwrap();
        let kind_raw = raw.Info.Kind;
        let val = raw.Info.Value;
        let service_partition_kind: ServicePartitionKind = kind_raw.into();
        let partition_key_type = PartitionKeyType::from_raw_svc_part(service_partition_kind, val);
        ResolvedServicePartitionInfo {
            name,
            service_partition_kind,
            partition_key_type,
        }
    }

    // Get the list of endpoints
    pub fn get_endpoint_list(&self) -> ResolvedServiceEndpointList {
        ResolvedServiceEndpointList::from_com(self.com.clone())
    }
}

#[derive(Debug)]
pub enum ServiceRole {
    Invalid,
    StatefulPrimary,
    StatefulSecondary,
    Stateless,
}

impl From<FABRIC_SERVICE_ENDPOINT_ROLE> for ServiceRole {
    fn from(value: FABRIC_SERVICE_ENDPOINT_ROLE) -> Self {
        match value {
            FABRIC_SERVICE_ROLE_INVALID => ServiceRole::Invalid,
            FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY => ServiceRole::StatefulPrimary,
            FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY => ServiceRole::StatefulSecondary,
            FABRIC_SERVICE_ROLE_STATELESS => ServiceRole::Stateless,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("unknown type: {:?}", value);
                } else {
                    ServiceRole::Invalid
                }
            }
        }
    }
}

pub struct ResolvedServiceEndpointList {
    com: IFabricResolvedServicePartitionResult,
}

impl ResolvedServiceEndpointList {
    fn from_com(com: IFabricResolvedServicePartitionResult) -> Self {
        Self { com }
    }
    // Get iterator for the list
    pub fn iter(&self) -> ResolvedServiceEndpointListIter {
        ResolvedServiceEndpointListIter::new(self.com.clone())
    }
}

#[derive(Debug)]
pub struct ResolvedServiceEndpoint {
    pub address: HSTRING,
    pub role: ServiceRole,
}

pub struct ResolvedServiceEndpointListIter {
    _owner: IFabricResolvedServicePartitionResult,
    count: u32, // total
    index: u32,
    curr: *const FABRIC_RESOLVED_SERVICE_ENDPOINT,
}

impl ResolvedServiceEndpointListIter {
    fn new(com: IFabricResolvedServicePartitionResult) -> Self {
        let raw = unsafe { com.get_Partition().as_ref().unwrap() };
        let count = raw.EndpointCount;
        let item = raw.Endpoints;
        Self {
            _owner: com,
            count,
            index: 0,
            curr: item,
        }
    }
}

impl Iterator for ResolvedServiceEndpointListIter {
    type Item = ResolvedServiceEndpoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        // get the curr out
        let raw = unsafe { self.curr.as_ref().unwrap() };
        let res = Self::Item {
            address: HSTRING::from_wide(unsafe { raw.Address.as_wide() }).unwrap(),
            role: raw.Role.into(),
        };
        self.index += 1;
        self.curr = unsafe { self.curr.offset(1) };
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use windows_core::{HSTRING, PCWSTR};

    use super::{PartitionKeyType, ServicePartitionKind};

    #[test]
    fn test_conversion_int() {
        let k = PartitionKeyType::Int64(99);
        // check the raw ptr is ok
        let raw = k.get_raw();
        let i = unsafe { (raw as *const i64).as_ref().unwrap() };
        assert_eq!(*i, 99);

        let service_type = ServicePartitionKind::Int64Range;
        // restore the key
        let k2 = PartitionKeyType::from_raw_svc_part(service_type, raw);
        assert_eq!(k, k2);
    }

    #[test]
    fn test_conversion_string() {
        let src = HSTRING::from("mystr");
        let k = PartitionKeyType::String(src.clone());
        // check the raw ptr is ok
        let raw = k.get_raw();
        let s =
            HSTRING::from_wide(unsafe { PCWSTR::from_raw(raw as *const u16).as_wide() }).unwrap();
        assert_eq!(s, src);

        let service_type = ServicePartitionKind::Named;
        // restore the key
        let k2 = PartitionKeyType::from_raw_svc_part(service_type, raw);
        assert_eq!(k, k2);
    }
}
