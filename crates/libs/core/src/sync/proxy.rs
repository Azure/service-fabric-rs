// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::runtime::executor::BoxedCancelToken;
use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};

use super::{FabricReceiver, oneshot_channel};

// proxy impl
// Tests for this is in mssf_util crate.

/// Wrapper function for turning SF Begin End style api into
/// rust awaitable future.
/// Cancellation token cancels the operation.
/// begin is a function/closure taking a callback and returns the context.
/// end is a function/closure taking a context and returns the result type.
/// See example usage in FabricClient wrappers.
///
/// Remarks:
/// The main work of the closures are for aligning the raw params and return values from SF api.
/// Due to the complexity and irregularity of the begin and end function signatures,
/// the begin and end closure needs to be manually written.
///
/// Begin closure is initiated/called, and FabricReceiver is returned to the user. FabricSender
/// is supposed to send the async result obtaind from the end closure to the user.
/// End closure is wrapped in an awaitable callback (together with a FabricSender),
/// and such callback is passed to SF begin api and is invoked when
/// the (begin) initiated operation completes.
///
/// If receiver is dropped before the result is ready, the inner SF operation will be
/// auto cancelled (if the token is not already cancelled).
/// The user-passed token is not cancelled by the receiver's Drop, as it is user-owned.
/// Cancelling the token will propagate to the inner SF operation during polling.
/// After cancellation is triggered, the receiver future should finish in a short time,
/// with an error code operation cancelled, or other code if cancel failed.
/// If the result is ready before the cancellation is triggered, the success result will
/// be the output of the receiver future.
///
/// Cancellation best practice:
/// User should always poll the receiver future to completion even after cancellation is triggered,
/// to ensure the cancellation signal is properly propagated to SF and resources are cleaned up in a timely manner.
pub fn fabric_begin_end_proxy<BEGIN, END, T>(
    begin: BEGIN,
    end: END,
    token: Option<BoxedCancelToken>,
) -> FabricReceiver<crate::WinResult<T>>
where
    BEGIN: FnOnce(
        Option<&IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<IFabricAsyncOperationContext>,
    END: FnOnce(Option<&IFabricAsyncOperationContext>) -> crate::WinResult<T> + 'static,
    T: 'static,
{
    let (tx, mut rx) = oneshot_channel(token);

    let callback = crate::sync::AwaitableCallback::new_interface(move |ctx| {
        let res = end(ctx.as_ref());
        tx.send(res);
    });
    let ctx = begin(Some(&callback));
    match ctx {
        Ok(c) => {
            // attach the inner ctx to rx for cancellation integration.
            rx.set_ctx(c);
            rx
        }
        Err(e) => {
            let (tx2, rx2) = oneshot_channel(None);
            tx2.send(Err(e));
            rx2
        }
    }
}
