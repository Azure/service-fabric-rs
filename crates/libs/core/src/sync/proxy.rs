// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};

/// Wrapper function for turning SF Begin End style api into
/// rust awaitable future.
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
pub fn fabric_begin_end_proxy<BEGIN, END, T>(
    begin: BEGIN,
    end: END,
) -> crate::sync::FabricReceiver<::windows_core::Result<T>>
where
    BEGIN: FnOnce(
        Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext>,
    END: FnOnce(Option<&IFabricAsyncOperationContext>) -> crate::Result<T> + 'static,
    T: 'static,
{
    let (tx, rx) = crate::sync::oneshot_channel();

    let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
        let res = end(ctx);
        tx.send(res);
    });
    let ctx = begin(Some(&callback));
    if ctx.is_err() {
        let (tx2, rx2) = crate::sync::oneshot_channel();
        tx2.send(Err(ctx.err().unwrap()));
        rx2
    } else {
        rx
    }
}
