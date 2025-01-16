// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::cell::Cell;
use std::sync::Arc;

use mssf_core::runtime::stateless::{StatelessServiceInstance, StatelessServicePartition};
use mssf_core::sync::CancellationToken;
use mssf_core::types::ServicePartitionInformation;
use mssf_core::WString;
use tokio::sync::oneshot::{self, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use crate::app::AppContext;
use crate::echo;

/// Stateless instance
#[allow(clippy::type_complexity)]
pub struct ServiceInstance {
    ctx: Arc<AppContext>,
    tx_: Mutex<Cell<Option<Sender<()>>>>, // hack to use this mutably
    task_: Mutex<Cell<Option<JoinHandle<Result<(), std::io::Error>>>>>,
}

impl ServiceInstance {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self {
            ctx,
            tx_: Mutex::new(Cell::from(None)),
            task_: Mutex::new(Cell::from(None)),
        }
    }
}

impl StatelessServiceInstance for ServiceInstance {
    #[tracing::instrument(skip(self, partition))]
    async fn open(
        &self,
        partition: &StatelessServicePartition,
        _: CancellationToken,
    ) -> mssf_core::Result<WString> {
        info!("open");
        let info = partition.get_partition_info().unwrap();
        let ServicePartitionInformation::Singleton(s) = info else {
            panic!("paritionkind not match manifeset: {:?}", info);
        };
        info!("open parition id {:?}", s.id);

        let addr = self.ctx.get_addr();

        let (tx, rx) = oneshot::channel::<()>();
        // start echo
        let t = self
            .ctx
            .rt
            .spawn(async move { echo::start_echo(rx, addr).await });
        self.task_.lock().await.set(Some(t));
        self.tx_.lock().await.set(Some(tx));
        Ok(WString::from(self.ctx.get_addr()))
    }

    #[tracing::instrument(skip(self))]
    async fn close(&self, _: CancellationToken) -> mssf_core::Result<()> {
        info!("close");
        if let Some(sender) = self.tx_.lock().await.take() {
            info!("Triggering shutdown");
            let res = sender.send(());
            match res {
                Ok(_) => {
                    if let Some(th) = self.task_.lock().await.take() {
                        let res2 = th.await.unwrap();
                        match res2 {
                            Ok(_) => {
                                info!("Background thread terminated");
                            }
                            Err(e) => {
                                info!("Background thread failed to join. {e}")
                            }
                        }
                    }
                }
                Err(_) => {
                    info!("failed to send");
                }
            }
        } else {
            info!("sender is None");
        }
        Ok(())
    }
    #[tracing::instrument(skip(self))]
    fn abort(&self) {
        info!("abort");
        // It is ok to block since we are on a fabric thread.
        self.ctx.rt.block_on(async {
            // never cancel
            self.close(CancellationToken::new()).await.unwrap();
        });
    }
}
