// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};

use crate::runtime::executor::Executor;
use windows_core::AsImpl;

use super::bridge_context::BridgeContext;

/// Bridge for SF Begin com api.
/// This launches the future task on rt, and triggers the callback when
/// the future completes.
/// The returned context must be passed up to the Begin api's return.
/// The End api must use fabric_end_bridge() to retrieve the async result from
/// the returned context.
/// Remarks:
/// This api in a sense is unsafe because the develper needs to ensure the following:
///     The return type of the future must match the return type of the SF End api.
/// SF Begin api should use this function to launch the future in the background,
/// and return the context. When future completes,the result is saved in the context and
/// callback is triggerd to SF. SF guarantees to call End function to retrieve the
/// future's result.
/// See examples in *_bridge.rs runtime mod files for example usage.
pub fn fabric_begin_bridge<F>(
    rt: &impl Executor,
    callback: Option<&IFabricAsyncOperationCallback>,
    future: F,
) -> crate::Result<IFabricAsyncOperationContext>
where
    F: Future + Send + 'static,
{
    let cb = callback.unwrap().clone();
    let ctx: IFabricAsyncOperationContext = BridgeContext::<F::Output>::new(cb).into();
    let ctx_cpy = ctx.clone();
    rt.spawn(async move {
        let ok = future.await;
        let ctx_bridge: &BridgeContext<F::Output> = unsafe { ctx_cpy.as_impl() };
        ctx_bridge.set_content(ok);
        let cb = ctx_bridge.Callback().unwrap();
        unsafe { cb.Invoke(&ctx_cpy) };
    });
    Ok(ctx)
}

/// Retrieves the future result obtained from fabric_begin_bridge in context.
/// Must be used in SF End api if its Begin api uses fabric_begin_bridge().
/// See fabric_begin_bridge for details.
pub fn fabric_end_bridge<T>(context: Option<&IFabricAsyncOperationContext>) -> T
where
    T: 'static,
{
    let ctx_bridge: &BridgeContext<T> = unsafe { context.unwrap().as_impl() };
    ctx_bridge.consume_content()
}
