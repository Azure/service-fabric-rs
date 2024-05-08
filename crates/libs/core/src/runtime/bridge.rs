// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::cell::Cell;

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};
use windows_core::implement;

// BridgeContext is used to pass data content from
// Begin to End operation, when implementing Rust async to COM api
// exposed to SF COM layer.
#[implement(IFabricAsyncOperationContext)]
pub struct BridgeContext<T> {
    content: Cell<Option<T>>,
    is_completed: Cell<bool>,
    is_completed_synchronously: bool,
    callback: IFabricAsyncOperationCallback,
}

impl<T> BridgeContext<T> {
    pub fn new(callback: IFabricAsyncOperationCallback) -> BridgeContext<T> {
        BridgeContext {
            content: Cell::new(None),
            is_completed: Cell::new(false),
            is_completed_synchronously: false,
            callback,
        }
    }

    // TODO: send and comsume is expected to happend accross threads.
    // Even though we use a oneshot channel to send the signal,
    // it might be safer to add another memory barrier here.
    pub fn set_content(&self, content: T) {
        let prev = self.content.replace(Some(content));
        assert!(prev.is_none())
    }

    pub fn consume_content(&self) -> T {
        self.content.take().unwrap()
    }

    pub fn set_complete(&self) {
        self.is_completed.swap(&Cell::new(true));
    }

    // This as access violation. The com layout is not safe
    // fn invoke(&mut self, ctx: &IFabricAsyncOperationContext) {
    //     assert!(!self.is_completed);
    //     self.is_completed = true;
    //     info!("callback invoke");
    //     unsafe { self.callback.Invoke(ctx) };
    // }
}

impl<T> IFabricAsyncOperationContext_Impl for BridgeContext<T> {
    fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed.get().into()
    }

    fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed_synchronously.into()
    }

    fn Callback(&self) -> ::windows_core::Result<IFabricAsyncOperationCallback> {
        let cp = self.callback.clone();
        Ok(cp)
    }

    fn Cancel(&self) -> ::windows_core::Result<()> {
        Ok(())
    }
}
