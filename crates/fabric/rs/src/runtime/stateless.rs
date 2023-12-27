#![deny(non_snake_case)] // this file is safe rust

use async_trait::async_trait;
use fabric_base::{
    FabricCommon::FabricRuntime::IFabricStatelessServicePartition,
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
    FABRIC_SINGLETON_PARTITION_INFORMATION,
};
use windows_core::HSTRING;

// wrap of com interface
pub struct StatelessServicePartition {
    com_impl: IFabricStatelessServicePartition,
}

#[derive(Debug)]
pub struct SingletonPartitionInfo {
    pub id: ::windows_core::GUID,
}

#[derive(Debug)]
pub struct Int64PartitionInfo {
    pub id: ::windows_core::GUID,
    pub low_key: i64,
    pub high_key: i64,
}

#[derive(Debug)]
pub struct NamedPartitionInfo {
    pub id: ::windows_core::GUID,
    pub name: ::windows_core::HSTRING,
}

#[derive(Debug)]
pub enum PartitionKind {
    Invalid,
    Singleton(SingletonPartitionInfo),
    Int64Range(Int64PartitionInfo),
    Named(NamedPartitionInfo),
}

impl StatelessServicePartition {
    pub fn new(com_impl: IFabricStatelessServicePartition) -> StatelessServicePartition {
        StatelessServicePartition { com_impl }
    }

    pub fn get_partition_info(&self) -> ::windows_core::Result<PartitionKind> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        let res: PartitionKind = match raw_ref.Kind {
            FABRIC_SERVICE_PARTITION_KIND_INVALID => PartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => {
                let raw_info =
                    unsafe { &mut *(raw_ref.Value as *mut FABRIC_SINGLETON_PARTITION_INFORMATION) };
                let info = SingletonPartitionInfo { id: raw_info.Id };
                PartitionKind::Singleton(info)
            }
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => {
                let raw_info = unsafe {
                    &mut *(raw_ref.Value as *mut FABRIC_INT64_RANGE_PARTITION_INFORMATION)
                };
                let info = Int64PartitionInfo {
                    id: raw_info.Id,
                    low_key: raw_info.LowKey,
                    high_key: raw_info.HighKey,
                };
                PartitionKind::Int64Range(info)
            }
            FABRIC_SERVICE_PARTITION_KIND_NAMED => {
                let raw_info =
                    unsafe { &mut *(raw_ref.Value as *mut FABRIC_NAMED_PARTITION_INFORMATION) };
                let info = NamedPartitionInfo {
                    id: raw_info.Id,
                    name: HSTRING::from_wide(unsafe { raw_info.Name.as_wide() }).unwrap(),
                };
                PartitionKind::Named(info)
            }
            _ => PartitionKind::Invalid,
        };
        Ok(res)
    }
}

// safe factory
pub trait StatelessServiceFactory {
    fn create_instance(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &::windows::core::GUID,
        instanceid: i64,
    ) -> Box<dyn StatelessServiceInstance + Send>;
}

// safe service instance
#[async_trait]
pub trait StatelessServiceInstance: Send + Sync {
    async fn open(&self, partition: &StatelessServicePartition) -> windows::core::Result<HSTRING>;
    async fn close(&self) -> windows::core::Result<()>;
    fn abort(&self);
}
