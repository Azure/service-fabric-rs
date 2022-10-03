use std::sync::{Arc, Condvar, Mutex};

use service_fabric_rs::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationCallback_Vtbl, IFabricAsyncOperationContext,
};
use windows::core::{implement, PCWSTR};

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

pub fn pwstr_to_string(p: PCWSTR) -> String {
    if p.0.is_null() {
        return String::new();
    }

    let mut end = p.0;
    unsafe {
        while *end != 0 {
            end = end.add(1);
        }
    }
    let ret: String = unsafe {
        String::from_utf16_lossy(std::slice::from_raw_parts(p.0, end.offset_from(p.0) as _))
    };
    return ret;
}
