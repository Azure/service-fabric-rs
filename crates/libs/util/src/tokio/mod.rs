// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! tokio utilites

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use mssf_core::runtime::executor::{BoxedCancelToken, CancelToken, EventFuture, Executor, Timer};
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
    pub fn block_on_any<F: Future>(&self, future: F) -> F::Output {
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
#[derive(Clone)]
pub struct TokioCancelToken {
    token: tokio_util::sync::CancellationToken,
    #[allow(clippy::type_complexity)]
    callback: Arc<Mutex<Option<Box<dyn FnOnce() + Send + Sync>>>>,
}

impl std::fmt::Debug for TokioCancelToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokioCancelToken")
            .field("token", &self.token)
            .field("has_callback", &self.callback.lock().unwrap().is_some())
            .finish()
    }
}

impl CancelToken for TokioCancelToken {
    fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    fn cancel(&self) {
        self.token.cancel();

        // Take and invoke the callback, releasing the lock
        // before calling it to avoid deadlock.
        let callback = self.callback.lock().unwrap().take();
        if let Some(cb) = callback {
            cb();
        }
    }

    fn wait(&self) -> Pin<Box<dyn EventFuture>> {
        let fut = self.token.clone().cancelled_owned();
        Box::pin(fut) as Pin<Box<dyn EventFuture>>
    }

    fn on_cancel(&self, callback: Box<dyn FnOnce() + Send + Sync>) {
        if self.token.is_cancelled() {
            callback();
            return;
        }
        let mut slot = self.callback.lock().unwrap();
        // Double-check after acquiring the lock
        if self.token.is_cancelled() {
            drop(slot);
            callback();
        } else {
            debug_assert!(slot.is_none(), "a callback has already been registered");
            *slot = Some(callback);
        }
    }

    fn clone_box(&self) -> BoxedCancelToken {
        Box::new(self.clone())
    }
}

impl TokioCancelToken {
    pub fn new() -> Self {
        TokioCancelToken {
            token: tokio_util::sync::CancellationToken::new(),
            callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn new_boxed() -> BoxedCancelToken {
        Box::new(Self::new())
    }

    pub fn boxed_from(token: tokio_util::sync::CancellationToken) -> BoxedCancelToken {
        Box::new(Self::from(token))
    }

    pub fn get_ref(&self) -> &tokio_util::sync::CancellationToken {
        &self.token
    }
}

impl From<tokio_util::sync::CancellationToken> for TokioCancelToken {
    fn from(token: tokio_util::sync::CancellationToken) -> Self {
        TokioCancelToken {
            token,
            callback: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for TokioCancelToken {
    fn default() -> Self {
        Self::new()
    }
}
