// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

use tokio::{runtime::Handle, sync::mpsc::channel};
use tracing::info;

// Executor is used by rs to post jobs to execute in the background
// Sync is needed due to we use the executor across await boundary.
pub trait Executor: Clone + Sync + Send + 'static {
    // Required functions

    // spawns the task to run in background
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static;

    // run the future on the executor until completion.
    fn block_on<F: Future>(&self, future: F) -> F::Output;

    // provided functions

    // Run the executor and block the current thread until ctrl-c event is
    // Received.
    fn run_until_ctrl_c(&self) {
        info!("DefaultExecutor: setting up ctrl-c event.");
        // set ctrc event
        let (tx, mut rx) = channel(1);
        let handler = move || {
            tx.blocking_send(())
                .expect("Could not send signal on channel.")
        };
        ctrlc::set_handler(handler).expect("Error setting Ctrl-C handler");

        // wait for ctrl-c signal.
        self.block_on(async move {
            info!("DefaultExecutor: Waiting for Ctrl-C...");
            rx.recv().await.expect("Could not receive from channel.");
            info!("DefaultExecutor: Got Ctrl-C! Exiting...");
        });
    }
}

#[derive(Clone)]
pub struct DefaultExecutor {
    rt: Handle,
}

impl DefaultExecutor {
    pub fn new(rt: Handle) -> DefaultExecutor {
        DefaultExecutor { rt }
    }
}

impl Executor for DefaultExecutor {
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
    {
        let h = self.rt.spawn(async move {
            future.await;
        });

        // Monitor user future.
        // If user task has panic, exit the process.
        // TODO: expose a config to control this behavior.
        // It is observed that if user task panics, sf operation are stuck.
        self.rt.spawn(async move {
            let ok = h.await;
            if ok.is_err() {
                info!(
                    "DefaultExecutor: User spawned future paniced {}",
                    ok.unwrap_err()
                );
                std::process::exit(1);
            }
        });
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.rt.block_on(future)
    }
}

#[cfg(test)]
mod test {
    use super::DefaultExecutor;

    #[test]
    fn test_executor() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let _ex = DefaultExecutor::new(rt.handle().clone());
        // let b_ex: Box<dyn Executor> = Box::new(ex);
    }
}
