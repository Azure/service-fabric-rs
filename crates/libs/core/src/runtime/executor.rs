// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

use tokio::{runtime::Handle, sync::mpsc::channel};
use tracing::info;

use crate::error::FabricErrorCode;

// Executor is used by rs to post jobs to execute in the background
// Sync is needed due to we use the executor across await boundary.
pub trait Executor: Clone + Sync + Send + 'static {
    // Required functions

    // spawns the task to run in background
    fn spawn<F>(&self, future: F) -> impl JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send;

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

/// Handle can be awaited to get the success status of the task.
/// The handle is primarily needed to propagate background task error
/// back to SF.
#[trait_variant::make(JoinHandle: Send)]
pub trait LocalJoinHandle<T> {
    async fn join(self) -> crate::Result<T>;
}

#[derive(Clone)]
pub struct DefaultExecutor {
    rt: Handle,
}

/// Default implementation of the JoinHandle using tokio
pub struct DefaultJoinHandle<T> {
    inner: tokio::task::JoinHandle<T>,
}

impl DefaultExecutor {
    pub fn new(rt: Handle) -> DefaultExecutor {
        DefaultExecutor { rt }
    }
}

impl Executor for DefaultExecutor {
    fn spawn<F>(&self, future: F) -> impl JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        let h = self.rt.spawn(async move { future.await });
        DefaultJoinHandle::<F::Output> { inner: h }
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.rt.block_on(future)
    }
}

impl<T: Send> JoinHandle<T> for DefaultJoinHandle<T> {
    async fn join(self) -> crate::Result<T> {
        match self.inner.await {
            Ok(x) => Ok(x),
            Err(e) => {
                if e.is_cancelled() {
                    // we never cancel in executor
                    Err(FabricErrorCode::E_ABORT.into())
                } else if e.is_panic() {
                    Err(FabricErrorCode::E_UNEXPECTED.into())
                } else {
                    Err(FabricErrorCode::E_FAIL.into())
                }
            }
        }
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
