// lib that contains all common extensions for the raw fabric apis.

pub mod fasync;

use std::os::windows::prelude::OsStrExt;
use std::{
    ffi::OsString,
    sync::{Arc, Condvar, Mutex},
};

use log::info;
use service_fabric_rs::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationCallback_Vtbl, IFabricAsyncOperationContext,
    IFabricAsyncOperationContext_Impl, IFabricStringResult, IFabricStringResult_Impl,
};
use windows::core::implement;

// Interface for waitable async callback.
// This is a common use case to combine fabric Begin* and End* apis.
#[windows::core::interface("ce5d1e03-90f0-44a3-9d87-849973970761")]
pub unsafe trait IFabricWaitableCallback: IFabricAsyncOperationCallback {
    pub unsafe fn wait(&self);
}

#[derive(Debug)]
#[implement(IFabricWaitableCallback, IFabricAsyncOperationCallback)]
pub struct WaitableCallback {
    pair_: Arc<(Mutex<bool>, Condvar)>,
}

impl WaitableCallback {
    pub fn new() -> WaitableCallback {
        return WaitableCallback {
            pair_: Arc::new((Mutex::new(false), Condvar::new())),
        };
    }
}

impl IFabricAsyncOperationCallback_Impl for WaitableCallback {
    // notify the function has been invoked.
    fn Invoke(&self, _context: &core::option::Option<IFabricAsyncOperationContext>) {
        //println!("WaitableCallback Invoke.");
        let (lock, cvar) = &*self.pair_;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }
}

impl IFabricWaitableCallback_Impl for WaitableCallback {
    unsafe fn wait(&self) {
        //println!("WaitableCallback wait.");
        // Wait for callback to be invoked
        let (lock, cvar) = &*self.pair_;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}

// pub fn pwstr_to_string(p: PCWSTR) -> String {
//     if p.0.is_null() {
//         return String::new();
//     }

//     let mut end = p.0;
//     unsafe {
//         while *end != 0 {
//             end = end.add(1);
//         }
//     }
//     let ret: String = unsafe {
//         String::from_utf16_lossy(std::slice::from_raw_parts(p.0, end.offset_from(p.0) as _))
//     };
//     return ret;
// }

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
    pub fn new(
        callback: &core::option::Option<
            service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback,
        >,
    ) -> AsyncContext {
        info!("AsyncContext::new");
        let callback_copy: IFabricAsyncOperationCallback = callback.clone().expect("msg");

        let ctx = AsyncContext {
            callback_: callback_copy,
        };
        return ctx;
    }
}

impl IFabricAsyncOperationContext_Impl for AsyncContext {
    fn IsCompleted(&self) -> windows::Win32::Foundation::BOOLEAN {
        return windows::Win32::Foundation::BOOLEAN::from(true);
    }

    fn CompletedSynchronously(&self) -> windows::Win32::Foundation::BOOLEAN {
        return windows::Win32::Foundation::BOOLEAN::from(true);
    }

    fn Callback(
        &self,
    ) -> windows::core::Result<service_fabric_rs::FabricCommon::IFabricAsyncOperationCallback> {
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
    vec_: Vec<u16>,
}

impl StringResult {
    pub fn new(data: OsString) -> StringResult {
        let data_vec = data
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let ret = StringResult { vec_: data_vec };
        return ret;
    }
}

impl IFabricStringResult_Impl for StringResult {
    fn get_String(&self) -> windows::core::PWSTR {
        // This is some hack to get the raw pointer out.
        let ptr: *mut u16 = self.vec_.as_ptr() as *mut u16;
        return windows::core::PWSTR::from_raw(ptr);
    }
}
