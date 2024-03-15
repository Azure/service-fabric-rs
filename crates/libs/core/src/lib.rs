// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(non_snake_case)]

// lib that contains all common extensions for the raw fabric apis.

pub mod client;
pub mod debug;
pub mod runtime;
pub mod sync;

use std::sync::{Arc, Condvar, Mutex};

use log::info;
use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl, IFabricStringResult,
    IFabricStringResult_Impl,
};
use windows::core::implement;
use windows_core::HSTRING;

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

impl IFabricAsyncOperationCallback_Impl for WaitableCallback {
    // notify the function has been invoked.
    fn Invoke(&self, _context: ::core::option::Option<&IFabricAsyncOperationContext>) {
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
        info!("AsyncContext::new");
        let callback_copy: IFabricAsyncOperationCallback = callback.expect("msg").clone();

        AsyncContext {
            callback_: callback_copy,
        }
    }
}

impl IFabricAsyncOperationContext_Impl for AsyncContext {
    fn IsCompleted(&self) -> windows::Win32::Foundation::BOOLEAN {
        windows::Win32::Foundation::BOOLEAN::from(true)
    }

    fn CompletedSynchronously(&self) -> windows::Win32::Foundation::BOOLEAN {
        windows::Win32::Foundation::BOOLEAN::from(true)
    }

    fn Callback(&self) -> windows::core::Result<IFabricAsyncOperationCallback> {
        info!("AsyncContext::Callback");
        // get a view of the callback
        let callback_copy: IFabricAsyncOperationCallback = self.callback_.clone();
        Ok(callback_copy)
    }

    fn Cancel(&self) -> windows::core::Result<()> {
        info!("AsyncContext::Cancel");
        Ok(())
    }
}

// Basic implementation of fabric string result
// usually used as string return value to fabric runtime.
#[derive(Debug)]
#[implement(IFabricStringResult)]
pub struct StringResult {
    data: HSTRING,
}

impl StringResult {
    pub fn new(data: HSTRING) -> StringResult {
        let ret = StringResult { data };
        ret
    }
}

impl IFabricStringResult_Impl for StringResult {
    fn get_String(&self) -> windows::core::PCWSTR {
        // This is some hack to get the raw pointer out.
        windows::core::PCWSTR::from_raw(self.data.as_ptr())
    }
}

pub fn IFabricStringResultToHString(s: &IFabricStringResult) -> HSTRING {
    let content = unsafe { s.get_String() };
    HSTRING::from_wide(unsafe { content.as_wide() }).unwrap()
}

#[cfg(test)]
mod test {
    use super::{IFabricStringResultToHString, StringResult};
    use mssf_com::FabricCommon::IFabricStringResult;
    use windows_core::HSTRING;

    #[test]
    fn test_str_addr() {
        // Test the addr returned to SF is right.
        let addr = "1.2.3.4:1234";

        // Check hstring len.
        let haddr = HSTRING::from(addr);
        let haddr_slice = haddr.as_wide();
        assert_eq!(haddr_slice.len(), 12);

        // check StringResult len.
        let com_addr: IFabricStringResult = StringResult::new(haddr.clone()).into();
        let raw = unsafe { com_addr.get_String() };
        let slice = unsafe { raw.as_wide() };
        assert_eq!(slice.len(), 12);

        // check StringResult conversion is right
        let haddr2 = IFabricStringResultToHString(&com_addr);
        assert_eq!(haddr, haddr2);
    }
}
