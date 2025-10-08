// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::cell::Cell;
use std::sync::Arc;

use mssf_core::WString;
use mssf_core::runtime::executor::BoxedCancelToken;
use mssf_core::runtime::{StatelessServicePartition, stateless::StatelessServiceInstance};
use mssf_core::sync::SimpleCancelToken;
use mssf_core::types::{HealthInformation, ServicePartitionInformation};
use tokio::sync::Mutex;
use tokio::sync::oneshot::{self, Sender};
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
        partition: StatelessServicePartition,
        _: BoxedCancelToken,
    ) -> mssf_core::Result<WString> {
        info!("open");
        let info = partition.get_partition_info().unwrap();
        let ServicePartitionInformation::Singleton(s) = info else {
            panic!("paritionkind not match manifeset: {info:?}");
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
        // send health report
        send_instance_health_report(&partition);
        Ok(WString::from(self.ctx.get_addr()))
    }

    #[tracing::instrument(skip(self))]
    async fn close(&self, _: BoxedCancelToken) -> mssf_core::Result<()> {
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
            self.close(SimpleCancelToken::new_boxed()).await.unwrap();
        });
    }
}

/// Send health ok to SF to validate health reporting code
fn send_instance_health_report(p: &StatelessServicePartition) {
    let healthinfo = HealthInformation {
        source_id: WString::from("echoapp"),
        property: WString::from("echo-opened"),
        time_to_live_seconds: 300,
        state: mssf_core::types::HealthState::Ok,
        description: WString::from("echo instance opened"),
        sequence_number: 1,
        remove_when_expired: true,
    };
    if let Err(e) = p.report_instance_health(&healthinfo) {
        tracing::error!("report instance health failed: {e:?}");
    }
}
