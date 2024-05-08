// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::cell::Cell;

use mssf_core::runtime::stateless::{
    PartitionKind, StatelessServiceFactory, StatelessServiceInstance, StatelessServicePartition,
};
use mssf_core::HSTRING;
use tokio::runtime::Handle;
use tokio::sync::oneshot::{self, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use crate::echo;

pub struct Factory {
    port: u32,
    hostname: HSTRING,
    rt: Handle,
}

impl Factory {
    pub fn new(port: u32, hostname: HSTRING, rt: Handle) -> Self {
        Self { port, hostname, rt }
    }
}

impl StatelessServiceFactory for Factory {
    fn create_instance(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &mssf_core::GUID,
        instanceid: i64,
    ) -> mssf_core::Result<impl StatelessServiceInstance> {
        info!(
            "Factory::create_instance, servicetype {}, service {}, init len {}, ptid {:?}, iid {}",
            servicetypename,
            servicename,
            initializationdata.len(),
            partitionid,
            instanceid
        );
        Ok(Instance::new(
            self.port,
            self.hostname.clone(),
            self.rt.clone(),
        ))
    }
}

#[allow(clippy::type_complexity)]
pub struct Instance {
    port: u32,
    hostname: HSTRING,
    tx_: Mutex<Cell<Option<Sender<()>>>>, // hack to use this mutably
    task_: Mutex<Cell<Option<JoinHandle<Result<(), std::io::Error>>>>>,
    rt: Handle,
}

impl Instance {
    pub fn new(port: u32, hostname: HSTRING, rt: Handle) -> Self {
        Self {
            port,
            hostname,
            tx_: Mutex::new(Cell::from(None)),
            rt,
            task_: Mutex::new(Cell::from(None)),
        }
    }
}

impl StatelessServiceInstance for Instance {
    async fn open(&self, partition: &StatelessServicePartition) -> mssf_core::Result<HSTRING> {
        info!("Instance::open");
        let info = partition.get_partition_info().unwrap();
        if let PartitionKind::Singleton(s) = info {
            info!("Instance::open parition id {:?}", s.id);
        } else {
            panic!("paritionkind not match manifeset: {:?}", info);
        }

        let port_copy = self.port;
        let hostname_copy = self.hostname.clone();

        let (tx, rx) = oneshot::channel::<()>();
        // start echo
        let t = self
            .rt
            .spawn(async move { echo::start_echo(rx, port_copy, hostname_copy).await });
        self.task_.lock().await.set(Some(t));
        self.tx_.lock().await.set(Some(tx));
        let addr = echo::get_addr(self.port, self.hostname.clone());
        Ok(HSTRING::from(addr))
    }
    async fn close(&self) -> mssf_core::Result<()> {
        info!("Instance::close");
        if let Some(sender) = self.tx_.lock().await.take() {
            info!("AppInstance:: Triggering shutdown");
            let res = sender.send(());
            match res {
                Ok(_) => {
                    if let Some(th) = self.task_.lock().await.take() {
                        let res2 = th.await.unwrap();
                        match res2 {
                            Ok(_) => {
                                info!("AppInstance:: Background thread terminated");
                            }
                            Err(e) => {
                                info!("AppInstance:: Background thread failed to join. {e}")
                            }
                        }
                    }
                }
                Err(_) => {
                    info!("AppInstance:: failed to send");
                }
            }
        } else {
            info!("AppInstance:: sender is None");
        }
        Ok(())
    }
    fn abort(&self) {
        info!("Instance::abort");
        // It is ok to block since we are on a fabric thread.
        self.rt.block_on(async {
            self.close().await.unwrap();
        });
    }
}
