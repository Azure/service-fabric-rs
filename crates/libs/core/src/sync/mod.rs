// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// this contains some experiments for async
#![allow(non_snake_case)]

use std::cell::Cell;

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl, IFabricAsyncOperationContext,
};
use windows_core::implement;

pub mod wait;

// This is intentional private. User should directly use bridge mod.
#[cfg(feature = "tokio_async")]
mod bridge_context;
#[cfg(feature = "tokio_async")]
pub use bridge_context::BridgeContext;

#[cfg(feature = "tokio_async")]
mod channel;
#[cfg(feature = "tokio_async")]
pub use channel::{oneshot_channel, FabricReceiver, FabricSender};

#[cfg(feature = "tokio_async")]
mod proxy;
#[cfg(feature = "tokio_async")]
pub use proxy::fabric_begin_end_proxy;
#[cfg(feature = "tokio_async")]
pub use tokio_util::sync::CancellationToken;

// fabric code begins here

pub trait Callback: FnOnce(windows_core::Ref<IFabricAsyncOperationContext>) + 'static {}
impl<T: FnOnce(windows_core::Ref<IFabricAsyncOperationContext>) + 'static> Callback for T {}

// TODO: rename.
// Fabric Callback that wraps an arbitrary Fn closure.
// Used primarily for bridging Begin and End fabric functions.
#[implement(IFabricAsyncOperationCallback)]
pub struct AwaitableCallback<F>
where
    F: Callback,
{
    callback: Cell<Option<F>>,
}

impl<F: Callback> IFabricAsyncOperationCallback_Impl for AwaitableCallback_Impl<F> {
    // notify the function has been invoked.
    fn Invoke(&self, context: windows_core::Ref<IFabricAsyncOperationContext>) {
        let cb_opt = self.callback.take();
        match cb_opt {
            Some(cb) => {
                cb(context);
            }
            None => {
                unreachable!("Invoke has been run already");
            }
        }
    }
}

impl<F: Callback> AwaitableCallback<F> {
    /// Creates a new obj and convert to the COM interface type.
    pub fn new_interface(callback: F) -> IFabricAsyncOperationCallback {
        let a = AwaitableCallback {
            callback: Cell::new(Some(callback)),
        };
        a.into()
    }
}

#[cfg(test)]
mod test{
    use mssf_com::FabricClient::IFabricClusterManagementClient3;

    #[test]
    fn local_client_create() {
        let _mgmt = crate::client::FabricClient::builder()
            .build_interface::<IFabricClusterManagementClient3>().unwrap();
    }
}
