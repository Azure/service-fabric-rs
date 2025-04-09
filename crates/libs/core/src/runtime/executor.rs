// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

use tokio::runtime::Handle;

use crate::error::ErrorCode;

// Executor is used by rs to post jobs to execute in the background
// Sync is needed due to we use the executor across await boundary.
pub trait Executor: Clone + Sync + Send + 'static {
    // Required functions

    /// spawns the task to run in background, and returns a join handle
    /// where the future's result can be awaited.
    /// If the future panics, the join handle should return an error code.
    /// This is primarily used by mssf Bridge to execute user app async callbacks/notifications.
    /// User app impl future may panic, and mssf propagates panic as an error in JoinHandle
    /// to SF.
    fn spawn<F>(&self, future: F) -> impl JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send;

    /// run the future on the executor until completion.
    fn block_on<F: Future>(&self, future: F) -> F::Output;
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
        let h = self.rt.spawn(future);
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
                let e = if e.is_cancelled() {
                    // we never cancel in executor
                    ErrorCode::E_ABORT
                } else if e.is_panic() {
                    ErrorCode::E_UNEXPECTED
                } else {
                    ErrorCode::E_FAIL
                };
                #[cfg(feature = "tracing")]
                tracing::error!("DefaultJoinHandle: background task failed: {e}");
                Err(e.into())
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
