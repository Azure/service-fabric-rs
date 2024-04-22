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
use windows_core::{HSTRING, PCWSTR};

use crate::iter::{FabricIter, FabricListAccessor};

use super::gen::svc::IFabricServiceManagementClient6Wrap;

// Service Management Client
pub struct ServiceManagementClient {
    com: IFabricServiceManagementClient6,
    _gen_wrap: IFabricServiceManagementClient6Wrap,
}

impl ServiceManagementClient {
    pub fn from_com(com: IFabricServiceManagementClient6) -> Self {
        Self {
            com: com.clone(),
            _gen_wrap: IFabricServiceManagementClient6Wrap::from_com(com),
        }
    }

    fn resolve_service_partition_internal(
        &self,
        name: &u16,
        partitionKeyType: FABRIC_PARTITION_KEY_TYPE,
        partitionKey: &::core::ffi::c_void,
        previousResult: Option<&IFabricResolvedServicePartitionResult>, // This is different from generated code
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<IFabricResolvedServicePartitionResult>>
    {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndResolveServicePartition(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginResolveServicePartition(
                name,
                partitionKeyType,
                partitionKey,
                previousResult,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
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
        let prev_opt = prev.map(|x| &x.com);

        let part_key_raw_ptr = key_type.get_raw();
        let part_key_raw = unsafe { &*part_key_raw_ptr };

        let fu = self.resolve_service_partition_internal(
            uri,
            key_type.into(),
            part_key_raw,
            prev_opt,
            timeout.as_millis().try_into().unwrap(),
        );

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
    pub service_name: HSTRING,
    pub service_partition_kind: ServicePartitionKind,
    pub partition_key_type: PartitionKeyType,
}

impl ResolvedServicePartition {
    // Get the service partition info/metadata
    pub fn get_info(&self) -> ResolvedServicePartitionInfo {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        let service_name =
            HSTRING::from_wide(unsafe { PCWSTR::from_raw(raw.ServiceName).as_wide() }).unwrap();
        let kind_raw = raw.Info.Kind;
        let val = raw.Info.Value;
        let service_partition_kind: ServicePartitionKind = kind_raw.into();
        let partition_key_type = PartitionKeyType::from_raw_svc_part(service_partition_kind, val);
        ResolvedServicePartitionInfo {
            service_name,
            service_partition_kind,
            partition_key_type,
        }
    }

    // Get the list of endpoints
    pub fn get_endpoint_list(&self) -> ResolvedServiceEndpointList {
        ResolvedServiceEndpointList::from_com(self.com.clone())
    }

    // If compared with different partition error is returned.
    // to enable the user to identify which RSP is more
    // up-to-date. A returned value of 0 indicates that the two RSPs have the same version. 1 indicates that the other RSP has an older version.
    // -1 indicates that the other RSP has a newer version.
    pub fn compare_version(&self, other: &ResolvedServicePartition) -> windows_core::Result<i32> {
        unsafe { self.com.CompareVersion(&other.com) }
    }
}

#[derive(Debug)]
pub enum ServiceEndpointRole {
    Invalid,
    StatefulPrimary,
    StatefulSecondary,
    Stateless,
}

impl From<FABRIC_SERVICE_ENDPOINT_ROLE> for ServiceEndpointRole {
    fn from(value: FABRIC_SERVICE_ENDPOINT_ROLE) -> Self {
        match value {
            FABRIC_SERVICE_ROLE_INVALID => ServiceEndpointRole::Invalid,
            FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY => ServiceEndpointRole::StatefulPrimary,
            FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY => ServiceEndpointRole::StatefulSecondary,
            FABRIC_SERVICE_ROLE_STATELESS => ServiceEndpointRole::Stateless,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("unknown type: {:?}", value);
                } else {
                    ServiceEndpointRole::Invalid
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
        ResolvedServiceEndpointListIter::new(self, self)
    }
}

impl FabricListAccessor<FABRIC_RESOLVED_SERVICE_ENDPOINT> for ResolvedServiceEndpointList {
    fn get_count(&self) -> u32 {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        raw.EndpointCount
    }

    fn get_first_item(&self) -> *const FABRIC_RESOLVED_SERVICE_ENDPOINT {
        let raw = unsafe { self.com.get_Partition().as_ref().unwrap() };
        raw.Endpoints
    }
}

#[derive(Debug)]
pub struct ResolvedServiceEndpoint {
    pub address: HSTRING,
    pub role: ServiceEndpointRole,
}

type ResolvedServiceEndpointListIter<'a> = FabricIter<
    'a,
    FABRIC_RESOLVED_SERVICE_ENDPOINT,
    ResolvedServiceEndpoint,
    ResolvedServiceEndpointList,
>;

impl From<&FABRIC_RESOLVED_SERVICE_ENDPOINT> for ResolvedServiceEndpoint {
    fn from(value: &FABRIC_RESOLVED_SERVICE_ENDPOINT) -> Self {
        let raw = value;
        Self {
            address: HSTRING::from_wide(unsafe { raw.Address.as_wide() }).unwrap(),
            role: raw.Role.into(),
        }
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
