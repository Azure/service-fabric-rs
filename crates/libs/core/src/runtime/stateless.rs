// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![deny(non_snake_case)] // this file is safe rust

use mssf_com::FabricRuntime::IFabricStatelessServicePartition;
use windows_core::HSTRING;

use crate::types::ServicePartitionInformation;

// wrap of com interface
pub struct StatelessServicePartition {
    com_impl: IFabricStatelessServicePartition,
}

impl StatelessServicePartition {
    pub fn new(com_impl: IFabricStatelessServicePartition) -> StatelessServicePartition {
        StatelessServicePartition { com_impl }
    }

    pub fn get_partition_info(&self) -> ::windows_core::Result<ServicePartitionInformation> {
        let raw = unsafe { self.com_impl.GetPartitionInfo() }?;
        let raw_ref = unsafe { raw.as_ref().unwrap() };
        assert!(!raw.is_null());
        Ok(raw_ref.into())
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
    ) -> windows_core::Result<impl StatelessServiceInstance>;
}

// safe service instance
#[trait_variant::make(StatelessServiceInstance: Send)]
pub trait LocalStatelessServiceInstance: Send + Sync + 'static {
    async fn open(&self, partition: &StatelessServicePartition) -> windows::core::Result<HSTRING>;
    async fn close(&self) -> windows::core::Result<()>;
    fn abort(&self);
}
