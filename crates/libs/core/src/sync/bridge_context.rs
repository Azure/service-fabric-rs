// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{cell::Cell, future::Future};

use crate::{
    error::ErrorCode,
    runtime::executor::{Executor, JoinHandle},
};
use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};
use windows_core::{implement, AsImpl};

use crate::sync::CancellationToken;

/// Async operation context for bridging rust code into SF COM api that supports cancellation.
#[implement(IFabricAsyncOperationContext)]
pub struct BridgeContext<T>
where
    T: 'static,
{
    /// The task result. Initially it is None.
    /// If the task panics, the error is propagated here.
    content: Cell<Option<crate::Result<T>>>,
    /// Indicates the async operation has completed or not.
    /// This is a memory barrier for making the content available
    /// from writer thread to the reader thread. It is needed because
    /// in SF COM API, the caller can call Begin operation, poll on this
    /// status until complete, and End operation without barriers.
    is_completed: std::sync::atomic::AtomicBool,
    /// mssf never completes async operations synchronously.
    /// This is always false.
    is_completed_synchronously: bool,
    callback: IFabricAsyncOperationCallback,
    token: CancellationToken,
}

impl<T> BridgeContext<T>
where
    T: Send,
{
    fn new(callback: IFabricAsyncOperationCallback, token: CancellationToken) -> Self {
        Self {
            content: Cell::new(None),
            is_completed: std::sync::atomic::AtomicBool::new(false),
            is_completed_synchronously: false,
            callback,
            token,
        }
    }

    /// Creates the context from callback, and returns a cancellation token that
    /// can be used in rust code, and the cancellation token is hooked into self,
    /// where Cancel() api cancels the operation.
    pub fn make(
        callback: windows_core::Ref<IFabricAsyncOperationCallback>,
    ) -> (Self, CancellationToken) {
        let token = CancellationToken::new();
        let ctx = Self::new(callback.unwrap().clone(), token.clone());
        (ctx, token)
    }

    /// Spawns the future on rt.
    /// Returns a context that can be returned to SF runtime.
    /// This is intended to be used in SF Begin COM api, where
    /// rust code is spawned in background and the context is returned
    /// to caller.
    /// If the future panics, an error is set in the resulting content,
    /// caller will still get callback and receive an error in the End api.
    /// This api is in some sense unsafe, because the developer needs to ensure
    /// the following:
    /// * return type of the future needs to match SF COM api end return type.
    pub fn spawn<F>(
        self,
        rt: &impl Executor,
        future: F,
    ) -> crate::WinResult<IFabricAsyncOperationContext>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let self_cp: IFabricAsyncOperationContext = self.into();
        let self_cp2 = self_cp.clone();
        let rt_cp = rt.clone();
        let task = async move {
            // Run user code in a task and wait on its status.
            // If user code panics we propagate the error back to SF.
            let task_res = rt_cp.spawn(future).join().await;
            // TODO: maybe it is good to report health to SF here the same way that sf dotnet app works.

            // We trust the code in mssf here to not panic, or we have bigger problem (memory corruption etc.).
            let self_impl: &BridgeContext<T> = unsafe { self_cp.as_impl() };
            self_impl.set_content(task_res);
            let cb = unsafe { self_cp.Callback().unwrap() };
            unsafe { cb.Invoke(&self_cp) };
        };
        /// Propagate the span so that the executor has the right trace.
        /// The trace would likely have BeginXXX as the function where spawn()
        /// is called.
        #[cfg(feature = "tracing")]
        use tracing::Instrument;
        #[cfg(feature = "tracing")]
        let task = task.in_current_span();
        rt.spawn(task);
        Ok(self_cp2)
    }

    /// Get the result from the context from the SF End COM api.
    /// This api is in some sense unsafe, because the developer needs to ensure
    /// the following:
    /// * context impl type is `BridgeContext3`, and the T matches the SF end api
    ///   return type.
    ///
    /// Note that if T is of Result<ICOM> type, the current function return type is
    /// Result<Result<ICOM>>, so unwrap is needed.
    pub fn result(context: windows_core::Ref<IFabricAsyncOperationContext>) -> crate::Result<T> {
        let self_impl: &BridgeContext<T> = unsafe { context.unwrap().as_impl() };
        self_impl.consume_content()
    }

    /// Set the content for the ctx.
    /// Marks the ctx as completed.
    fn set_content(&self, content: crate::Result<T>) {
        let prev = self.content.replace(Some(content));
        assert!(prev.is_none());
        self.set_complete();
    }

    /// Consumes the content set by set_content().
    /// can only be called once after set content.
    fn consume_content(&self) -> crate::Result<T> {
        match self.check_complete() {
            true => self.content.take().expect("content is consumed twice."),
            false => {
                if self.token.is_cancelled() {
                    Err(ErrorCode::E_ABORT.into())
                } else {
                    Err(ErrorCode::FABRIC_E_OPERATION_NOT_COMPLETE.into())
                }
            }
        }
    }

    /// Set the ctx as completed. Requires the ctx content to be set. Makes
    /// the content available for access from other threads using barrier.
    fn set_complete(&self) {
        self.is_completed
            .store(true, std::sync::atomic::Ordering::Release);
    }

    /// Checks ctx is completed.
    /// Makes sure content sets by other threads is visible from this thread.
    fn check_complete(&self) -> bool {
        self.is_completed.load(std::sync::atomic::Ordering::Acquire)
    }
}

impl<T> IFabricAsyncOperationContext_Impl for BridgeContext_Impl<T> {
    fn IsCompleted(&self) -> bool {
        self.is_completed.load(std::sync::atomic::Ordering::Relaxed)
    }

    // This always returns false because we defer all tasks in the background executuor.
    fn CompletedSynchronously(&self) -> bool {
        self.is_completed_synchronously
    }

    fn Callback(&self) -> crate::WinResult<IFabricAsyncOperationCallback> {
        let cp = self.callback.clone();
        Ok(cp)
    }

    fn Cancel(&self) -> crate::WinResult<()> {
        self.token.cancel();
        Ok(())
    }
}
