// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use log::info;
use mssf_core::runtime::stateless::{
    PartitionKind, StatelessServiceFactory, StatelessServiceInstance, StatelessServicePartition,
};
use windows_core::HSTRING;

#[derive(Default)]
pub struct Factory {}

impl StatelessServiceFactory for Factory {
    fn create_instance(
        &self,
        servicetypename: &windows_core::HSTRING,
        servicename: &windows_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &windows::core::GUID,
        instanceid: i64,
    ) -> windows_core::Result<impl StatelessServiceInstance> {
        info!(
            "Factory::create_instance, servicetype {}, service {}, init len {}, ptid {:?}, iid {}",
            servicetypename,
            servicename,
            initializationdata.len(),
            partitionid,
            instanceid
        );
        Ok(Instance::default())
    }
}

#[derive(Default)]
pub struct Instance {}

impl StatelessServiceInstance for Instance {
    async fn open(&self, partition: &StatelessServicePartition) -> windows::core::Result<HSTRING> {
        info!("Instance::open");
        let info = partition.get_partition_info().unwrap();
        if let PartitionKind::Singleton(s) = info {
            info!("Instance::open parition id {:?}", s.id);
        } else {
            panic!("paritionkind not match manifeset: {:?}", info);
        }

        Ok(HSTRING::from("MyAddress"))
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("Instance::close");
        Ok(())
    }
    fn abort(&self) {
        info!("Instance::abort")
    }
}
