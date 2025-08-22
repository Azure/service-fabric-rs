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

use mssf_core::runtime::executor::{EventFuture, Executor, Timer};
use tokio::runtime::Handle;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct TokioExecutor {
    rt: Handle,
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
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        self.rt.spawn(future);
    }
}

/// Sleep timer implementation for tokio
pub struct TokioTimer;

// TODO: the return type may be simplified if using return impl
impl Timer for TokioTimer {
    fn sleep(&self, duration: std::time::Duration) -> std::pin::Pin<Box<dyn EventFuture>> {
        Box::pin(TokioSleep::new(tokio::time::sleep(duration)))
    }
}

/// Sleep future implementation for tokio
pub struct TokioSleep {
    // May need to use pin_project
    // to remove the Pin because the inner is not Unpin
    inner: Pin<Box<tokio::time::Sleep>>,
}

impl TokioSleep {
    pub fn new(sleep: tokio::time::Sleep) -> Self {
        Self {
            inner: Box::pin(sleep),
        }
    }
}

impl Future for TokioSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the inner Sleep future
        self.inner.as_mut().poll(cx)
    }
}

/// CancelToken implementation for tokio
/// User can use tokio's token and integrate with mssf.
#[derive(Debug, Clone)]
pub struct TokioCancelToken {
    token: tokio_util::sync::CancellationToken,
}

impl mssf_core::runtime::executor::CancelToken for TokioCancelToken {
    fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    fn cancel(&self) {
        self.token.cancel()
    }

    fn wait(&self) -> Pin<Box<dyn EventFuture>> {
        let fut = self.token.clone().cancelled_owned();
        Box::pin(fut) as Pin<Box<dyn EventFuture>>
    }
}

impl TokioCancelToken {
    pub fn new() -> Self {
        TokioCancelToken {
            token: tokio_util::sync::CancellationToken::new(),
        }
    }

    pub fn get_ref(&self) -> &tokio_util::sync::CancellationToken {
        &self.token
    }
}

impl From<tokio_util::sync::CancellationToken> for TokioCancelToken {
    fn from(token: tokio_util::sync::CancellationToken) -> Self {
        TokioCancelToken { token }
    }
}

impl Default for TokioCancelToken {
    fn default() -> Self {
        Self::new()
    }
}
