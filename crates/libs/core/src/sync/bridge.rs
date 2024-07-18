// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};

use crate::runtime::{bridge::BridgeContext, executor::Executor};
use windows_core::AsImpl;

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

pub fn fabric_end_bridge<T>(context: Option<&IFabricAsyncOperationContext>) -> T
where
    T: 'static,
{
    let ctx_bridge: &BridgeContext<T> = unsafe { context.unwrap().as_impl() };
    ctx_bridge.consume_content()
}
