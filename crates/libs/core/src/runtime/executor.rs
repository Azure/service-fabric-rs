// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

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
    fn sleep(&self, duration: std::time::Duration) -> std::pin::Pin<Box<dyn Sleep>>;
}

/// Runtime independent sleep future
pub trait Sleep: Send + Sync + Future<Output = ()> {}
