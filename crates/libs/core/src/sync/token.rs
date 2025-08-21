// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use crate::runtime::executor::{CancelToken, EventFuture};

/// A simple cancel token implementation
#[derive(Clone, Debug)]
pub struct SimpleCancelToken {
    inner: Arc<TokenInner>,
}

#[derive(Debug)]
struct TokenInner {
    cancelled: AtomicBool,
    wakers: Mutex<Vec<Waker>>,
}

impl SimpleCancelToken {
    pub fn new() -> Self {
        SimpleCancelToken {
            inner: Arc::new(TokenInner {
                cancelled: AtomicBool::new(false),
                wakers: Mutex::new(Vec::new()),
            }),
        }
    }

    pub fn cancel(&self) {
        // Set the cancelled flag
        self.inner.cancelled.store(true, Ordering::Release);

        // Wake all waiting tasks
        let mut wakers = self.inner.wakers.lock().unwrap();
        for waker in wakers.drain(..) {
            waker.wake();
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.inner.cancelled.load(Ordering::Acquire)
    }

    /// Returns a future that completes when cancellation is triggered
    pub fn cancelled(&self) -> CancelledFuture {
        CancelledFuture {
            token: self.clone(),
        }
    }
}

/// This future is cancel safe.
pub struct CancelledFuture {
    token: SimpleCancelToken,
}

impl Future for CancelledFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.token.is_cancelled() {
            return Poll::Ready(());
        }

        // Register this task's waker to be notified when cancelled
        let mut wakers = self.token.inner.wakers.lock().unwrap();

        // Double-check after acquiring the lock
        if self.token.is_cancelled() {
            return Poll::Ready(());
        }

        // Store the waker to be called when cancel() is invoked
        wakers.push(cx.waker().clone());
        Poll::Pending
    }
}

impl Default for SimpleCancelToken {
    fn default() -> Self {
        Self::new()
    }
}

/// Integrate with mssf trait system.
impl CancelToken for SimpleCancelToken {
    fn cancel(&self) {
        self.cancel();
    }

    fn is_cancelled(&self) -> bool {
        self.is_cancelled()
    }

    fn wait(&self) -> Pin<Box<dyn EventFuture>> {
        Box::pin(self.cancelled())
    }
}

pub const NONE_CANCEL_TOKEN: Option<SimpleCancelToken> = None;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_token() {
        let token = SimpleCancelToken::new();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[tokio::test]
    async fn test_cancel_token_async() {
        let token = SimpleCancelToken::new();
        let h = tokio::spawn({
            let token = token.clone();
            async move {
                token.wait().await;
            }
        });
        token.cancel();
        assert!(token.is_cancelled());
        h.await.unwrap();
    }

    #[tokio::test]
    async fn test_cancel_token_multi() {
        let token = SimpleCancelToken::new();
        let mut join_set = tokio::task::JoinSet::new();

        for _ in 0..10 {
            let token = token.clone();
            join_set.spawn(async move {
                token.wait().await;
            });
            tokio::task::yield_now().await;
        }
        token.cancel();
        assert!(token.is_cancelled());
        join_set.join_all().await;
    }
}
