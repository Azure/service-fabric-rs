// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// Sync implementation for SF Callback and Contexts.
use std::sync::{Arc, Condvar, Mutex};

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};
use windows_core::implement;

#[derive(Debug)]
#[implement(IFabricAsyncOperationCallback)]
pub struct WaitableCallback {
    pair_: Arc<(Mutex<bool>, Condvar)>,
}

pub struct WaitableToken {
    pair_: Arc<(Mutex<bool>, Condvar)>,
}

impl Default for WaitableCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl WaitableCallback {
    pub fn channel() -> (WaitableToken, IFabricAsyncOperationCallback) {
        let callback = WaitableCallback::new();
        let token = WaitableToken {
            pair_: callback.pair_.clone(),
        };
        let i_callbaack = callback.into();
        (token, i_callbaack)
    }

    pub fn new() -> WaitableCallback {
        WaitableCallback {
            pair_: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }
}

impl IFabricAsyncOperationCallback_Impl for WaitableCallback_Impl {
    // notify the function has been invoked.
    fn Invoke(&self, _context: windows_core::Ref<IFabricAsyncOperationContext>) {
        //println!("WaitableCallback Invoke.");
        let (lock, cvar) = &*self.pair_;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }
}

impl WaitableToken {
    pub fn wait(&self) {
        //println!("WaitableCallback wait.");
        // Wait for callback to be invoked
        let (lock, cvar) = &*self.pair_;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}

// The basic implementation of async context
// which use needs to trigger callback synchronously
#[derive(Debug)]
#[implement(IFabricAsyncOperationContext)]
pub struct AsyncContext {
    callback_: IFabricAsyncOperationCallback,
}

impl AsyncContext {
    // construct ctx. Note: caller needs to invoke callback.
    // This is different from cpp impl.
    pub fn new(callback: core::option::Option<&IFabricAsyncOperationCallback>) -> AsyncContext {
        let callback_copy: IFabricAsyncOperationCallback = callback.expect("msg").clone();

        AsyncContext {
            callback_: callback_copy,
        }
    }
}

impl IFabricAsyncOperationContext_Impl for AsyncContext_Impl {
    fn IsCompleted(&self) -> bool {
        true
    }

    fn CompletedSynchronously(&self) -> bool {
        true
    }

    fn Callback(&self) -> crate::WinResult<IFabricAsyncOperationCallback> {
        // get a view of the callback
        let callback_copy: IFabricAsyncOperationCallback = self.callback_.clone();
        Ok(callback_copy)
    }

    fn Cancel(&self) -> crate::WinResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mssf_com::FabricCommon::IFabricAsyncOperationContext;

    use super::{AsyncContext, WaitableCallback};

    #[test]
    fn test_waitable_callback() {
        let (token, callback) = WaitableCallback::channel();
        let ctx: IFabricAsyncOperationContext = AsyncContext::new(Some(&callback)).into();
        unsafe { ctx.Callback().unwrap().Invoke(&ctx) };
        token.wait();
    }
}
