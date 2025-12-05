// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{fmt::Debug, future::Future, pin::Pin};

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
    fn spawn<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send;
}

/// Runtime independent sleep trait.
pub trait Timer: Send + Sync + 'static {
    /// Returns a future that is ready after duration.
    fn sleep(&self, duration: std::time::Duration) -> Pin<Box<dyn EventFuture>>;
}

/// Runtime independent event future.
pub trait EventFuture: Send + Future<Output = ()> {}

impl<T> EventFuture for T where T: Future<Output = ()> + Send {}

pub trait CancelToken: Send + Sync + 'static {
    /// Get a future to wait for cancellation.
    fn wait(&self) -> Pin<Box<dyn EventFuture>>;

    /// Is the token cancelled
    fn is_cancelled(&self) -> bool;

    /// Cancel the token.
    fn cancel(&self);

    /// Clone the cancel token.
    /// Because the dyn requirement, CancelToken cannot be cloned directly.
    fn clone_box(&self) -> Box<dyn CancelToken>;
}

pub type BoxedCancelToken = Box<dyn CancelToken>;

impl Clone for BoxedCancelToken {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Debug for dyn CancelToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CancelToken")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}
