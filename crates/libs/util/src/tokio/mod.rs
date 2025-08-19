// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! tokio utilites

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use mssf_core::{
    ErrorCode,
    runtime::executor::{Executor, JoinHandle, Sleep, Timer},
};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct TokioExecutor {
    rt: Handle,
}

/// Default implementation of the JoinHandle using tokio
pub struct TokioJoinHandle<T> {
    inner: tokio::task::JoinHandle<T>,
}

impl TokioExecutor {
    pub fn new(rt: Handle) -> TokioExecutor {
        TokioExecutor { rt }
    }

    /// Returns a reference to the tokio runtime handle.
    pub fn get_ref(&self) -> &Handle {
        &self.rt
    }

    /// Block on the current task safely.
    /// Usually regular tokio block_on panics if it is already on the tokio task.
    /// This allows block on tokio task, using spawn_blocking.
    /// Note: This only works on multi-threaded runtime.
    pub fn block_on_any<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        match tokio::runtime::Handle::try_current() {
            Ok(h) => {
                // Currently on tokio thread.
                // Need to block the task.
                tokio::task::block_in_place(move || h.block_on(future))
            }
            Err(_) => {
                // Not on tokio thread, safe to block it directly
                self.rt.block_on(future)
            }
        }
    }

    /// Block the current thread until Ctrl+C is received.
    /// This is typically used in SF app main function.
    pub fn block_until_ctrlc(&self) {
        self.rt.block_on(async {
            tokio::signal::ctrl_c().await.expect("fail to get ctrl-c");
        });
    }
}

impl Executor for TokioExecutor {
    fn spawn<F>(&self, future: F) -> impl JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        let h = self.rt.spawn(future);
        TokioJoinHandle::<F::Output> { inner: h }
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.rt.block_on(future)
    }
}

impl<T: Send> JoinHandle<T> for TokioJoinHandle<T> {
    async fn join(self) -> mssf_core::Result<T> {
        match self.inner.await {
            Ok(x) => Ok(x),
            Err(e) => {
                let ec = if e.is_cancelled() {
                    // we never cancel in executor
                    ErrorCode::E_ABORT
                } else if e.is_panic() {
                    ErrorCode::E_UNEXPECTED
                } else {
                    ErrorCode::E_FAIL
                };
                #[cfg(feature = "tracing")]
                tracing::error!("DefaultJoinHandle: background task failed: {ec}, msg: {e}");
                Err(ec.into())
            }
        }
    }
}

pub struct TokioTimer;

impl Timer for TokioTimer {
    fn sleep(&self, duration: std::time::Duration) -> std::pin::Pin<Box<dyn Sleep>> {
        Box::pin(TokioSleep::new(tokio::time::sleep(duration)))
    }
}

pub struct TokioSleep {
    inner: Pin<Box<tokio::time::Sleep>>,
}

impl TokioSleep {
    pub fn new(sleep: tokio::time::Sleep) -> Self {
        Self {
            inner: Box::pin(sleep),
        }
    }
}

// Default sleep implementation for tokio
impl Sleep for TokioSleep {}

impl Future for TokioSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the inner Sleep future
        self.inner.as_mut().poll(cx)
    }
}
