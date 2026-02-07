// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use crate::runtime::executor::{BoxedCancelToken, CancelToken, EventFuture};

/// A simple cancel token implementation
#[derive(Clone, Debug)]
pub struct SimpleCancelToken {
    inner: Arc<TokenInner>,
}

#[derive(Debug)]
struct TokenInner {
    cancelled: AtomicBool,
    wakers: Mutex<Vec<Waker>>,
    child: Mutex<Option<BoxedCancelToken>>,
}

impl SimpleCancelToken {
    pub fn new() -> Self {
        SimpleCancelToken {
            inner: Arc::new(TokenInner {
                cancelled: AtomicBool::new(false),
                wakers: Mutex::new(Vec::new()),
                child: Mutex::new(None),
            }),
        }
    }

    pub fn new_boxed() -> BoxedCancelToken {
        Box::new(Self::new())
    }

    pub fn cancel(&self) {
        // Set the cancelled flag
        self.inner.cancelled.store(true, Ordering::Release);

        // Wake all waiting tasks
        let mut wakers = self.inner.wakers.lock().unwrap();
        for waker in wakers.drain(..) {
            waker.wake();
        }
        drop(wakers);

        // Take and cancel the child, releasing the lock
        // before calling cancel to avoid deadlock on self-attach or circular chains.
        let child = self.inner.child.lock().unwrap().take();
        if let Some(child) = child {
            child.cancel();
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

    fn attach_child(&self, child: BoxedCancelToken) {
        if self.is_cancelled() {
            child.cancel();
            return;
        }
        let mut slot = self.inner.child.lock().unwrap();
        // Double-check after acquiring the lock
        if self.is_cancelled() {
            drop(slot);
            child.cancel();
        } else {
            assert!(slot.is_none(), "a child has already been attached");
            *slot = Some(child);
        }
    }

    fn clone_box(&self) -> Box<dyn CancelToken> {
        Box::new(self.clone())
    }
}
