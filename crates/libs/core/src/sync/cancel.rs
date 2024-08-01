// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use mssf_com::FabricCommon::{
    IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl,
};
pub use tokio_util::sync::CancellationToken;
use windows_core::{implement, AsImpl};

use crate::{error::FabricErrorCode, runtime::executor::Executor};

/// Async operation context for bridging rust code into SF COM api that supports cancellation.
#[implement(IFabricAsyncOperationContext)]
pub struct BridgeContext3<T>
where
    T: 'static,
{
    content: Cell<Option<T>>,
    is_completed: Cell<bool>,
    is_completed_synchronously: bool,
    callback: IFabricAsyncOperationCallback,
    token: CancellationToken,
}

impl<T> BridgeContext3<T>
where
    T: Send,
{
    fn new(callback: IFabricAsyncOperationCallback, token: CancellationToken) -> Self {
        Self {
            content: Cell::new(None),
            is_completed: Cell::new(false),
            is_completed_synchronously: false,
            callback,
            token,
        }
    }

    /// Creates the context from callback, and returns a cancellation token that
    /// can be used in rust code, and the cancellation token is hooked into self,
    /// where Cancel() api cancels the operation.
    pub fn make(callback: Option<&IFabricAsyncOperationCallback>) -> (Self, CancellationToken) {
        let token = CancellationToken::new();
        let ctx = Self::new(callback.unwrap().clone(), token.clone());
        (ctx, token)
    }

    /// Spawns the future on rt.
    /// Returns a context that can be returned to SF runtime.
    /// This is intended to be used in SF Begin COM api, where
    /// rust code is spawned in background and the context is returned
    /// to caller.
    /// This api is in some sense unsafe, because the developer needs to ensure
    /// the following:
    /// * return type of the future needs to match SF COM api end return type.
    pub fn spawn<F>(
        self,
        rt: &impl Executor,
        future: F,
    ) -> crate::Result<IFabricAsyncOperationContext>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let self_cp: IFabricAsyncOperationContext = self.into();
        let self_cp2 = self_cp.clone();
        rt.spawn(async move {
            let ok = future.await;
            let self_impl: &BridgeContext3<T> = unsafe { self_cp.as_impl() };
            self_impl.set_content(ok);
            self_impl.set_complete();
            let cb = self_impl.Callback().unwrap();
            unsafe { cb.Invoke(&self_cp) };
        });
        Ok(self_cp2)
    }

    /// Get the result from the context from the SF End COM api.
    /// This api is in some sense unsafe, because the developer needs to ensure
    /// the following:
    /// * context impl type is `BridgeContext3`, and the T matches the SF end api
    /// return type.
    /// Note that if T is of Result<ICOM> type, the current function return type is
    /// Result<Result<ICOM>>, so unwrap is needed.
    pub fn result(context: Option<&IFabricAsyncOperationContext>) -> crate::Result<T> {
        let self_impl: &BridgeContext3<T> = unsafe { context.unwrap().as_impl() };
        self_impl.consume_content()
    }

    // TODO: send and comsume is expected to happend accross threads.
    // Even though we use a oneshot channel to send the signal,
    // it might be safer to add another memory barrier here.
    fn set_content(&self, content: T) {
        let prev = self.content.replace(Some(content));
        assert!(prev.is_none())
    }

    // can only be called once after set content.
    fn consume_content(&self) -> crate::Result<T> {
        let opt = self.content.take();
        match opt {
            Some(x) => Ok(x),
            None => {
                if !self.IsCompleted().as_bool() {
                    return Err(FabricErrorCode::AsyncOperationNotComplete.into());
                }
                if self.token.is_cancelled() {
                    Err(FabricErrorCode::OperationCanceled.into())
                } else {
                    panic!("content is consumed twice.")
                }
            }
        }
    }

    fn set_complete(&self) {
        self.is_completed.swap(&Cell::new(true));
    }
}

impl<T> IFabricAsyncOperationContext_Impl for BridgeContext3<T> {
    fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed.get().into()
    }

    // This always returns false because we defer all tasks in the background executuor.
    fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        self.is_completed_synchronously.into()
    }

    fn Callback(&self) -> ::windows_core::Result<IFabricAsyncOperationCallback> {
        let cp = self.callback.clone();
        Ok(cp)
    }

    fn Cancel(&self) -> ::windows_core::Result<()> {
        self.token.cancel();
        Ok(())
    }
}

// proxy impl

// Token that wraps oneshot receiver.
// The future recieve does not have error. This is designed for the use
// case where SF guarantees that sender will be called.
pub struct FabricReceiver2<T> {
    rx: tokio::sync::oneshot::Receiver<T>,
    token: Option<CancellationToken>,
    // saved ctx from SF Begin COM api for cancalling.
    ctx: Option<IFabricAsyncOperationContext>,
}

impl<T> FabricReceiver2<T> {
    fn new(
        rx: tokio::sync::oneshot::Receiver<T>,
        token: Option<CancellationToken>,
    ) -> FabricReceiver2<T> {
        FabricReceiver2 {
            rx,
            token,
            ctx: None,
        }
    }

    // This does not handle cancel. It is commented out because it is not used.
    // pub fn blocking_recv(self) -> crate::Result<T> {
    //     if let Some(t) = self.token {
    //         if t.is_cancelled() {
    //             return Err(FabricErrorCode::OperationCanceled.into());
    //         }
    //     }
    //     // sender must send stuff so that there is not error.
    //     Ok(self.rx.blocking_recv().unwrap())
    // }

    // Set the SF ctx to hook up cancellation.
    fn set_ctx(&mut self, ctx: IFabricAsyncOperationContext) {
        let prev = self.ctx.replace(ctx);
        assert!(prev.is_none());
    }

    // Cancels the inner SF operation if exists, and reset the ctx.
    fn cancel_inner_ctx(&mut self) -> crate::Result<()> {
        if let Some(ctx) = &self.ctx {
            if let Err(e) = unsafe { ctx.Cancel() } {
                // fail to cancel inner operation.
                return Err(e);
            } else {
                // clear the sf ctx to avoid cancel twice.
                self.ctx.take();
            }
        }
        Ok(())
    }
}

// Returns error if cancelled.
// If there is an inner SF ctx, cancellation signal will
// trigger cancellation of the ctx.
impl<T> Future for FabricReceiver2<T> {
    // The error code should be OperationCanceled, unless cancellation
    // of SF ctx returns other errors.
    // (TODO: observe other error code from SF, maybe some code should be ignored).
    type Output = crate::Result<T>;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the receiver first, if ready then return the output,
        // else poll the cancellation token, if cancelled return the error,
        // else return pending.
        // There can be the case that cancellation wakes the waker, but receiver
        // then got the result. The next poll will return received output rather
        // than cancelled error.

        // Try to receive the value from the sender
        let innner =
            <tokio::sync::oneshot::Receiver<T> as Future>::poll(Pin::new(&mut self.rx), _cx);
        match innner {
            Poll::Ready(x) => {
                // error only happens when sender is dropped without sending.
                match x {
                    Ok(data) => Poll::Ready(Ok(data)),
                    Err(_) => {
                        if let Some(t) = self.token.as_ref() {
                            if t.is_cancelled() {
                                // cancel the SF ctx
                                if let Err(e) = self.cancel_inner_ctx() {
                                    return Poll::Ready(Err(e));
                                }
                                return Poll::Ready(Err(FabricErrorCode::OperationCanceled.into()));
                            }
                        }
                        panic!("sender dropped without sending")
                    }
                }
            }
            Poll::Pending => {
                // If the action is canceled we can safely stop and return canceled error.
                if let Some(t) = &self.token {
                    // this is cancel safe so we can poll it once and discard
                    let fu = t.cancelled();
                    let inner = std::pin::pin!(fu).poll(_cx);
                    match inner {
                        Poll::Ready(_) => {
                            // operation cancelled
                            if let Err(e) = self.cancel_inner_ctx() {
                                return Poll::Ready(Err(e));
                            }
                            Poll::Ready(Err(FabricErrorCode::OperationCanceled.into()))
                        }
                        Poll::Pending => Poll::Pending,
                    }
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

pub struct FabricSender2<T> {
    tx: tokio::sync::oneshot::Sender<T>,
    token: Option<CancellationToken>,
}

impl<T> FabricSender2<T> {
    fn new(
        tx: tokio::sync::oneshot::Sender<T>,
        token: Option<CancellationToken>,
    ) -> FabricSender2<T> {
        FabricSender2 { tx, token }
    }

    pub fn send(self, data: T) {
        let e = self.tx.send(data);
        if e.is_err() {
            // In SF use case receiver should not be dropped by user.
            // If it acctually dropped by user, it is ok to ignore because user
            // does not want to want the value any more. But too bad SF has done
            // the work to get the value.

            // receiver should never be dropped if operation is not cancelled.
            if let Some(t) = self.token {
                if !t.is_cancelled() {
                    panic!("receiver dropped.");
                }
            }
        }
    }
}

/// Creates a fabric oneshot channel.
/// Operation can be cancelled by cancelling the token.
pub fn oneshot_channel<T>(
    token: Option<CancellationToken>,
) -> (FabricSender2<T>, FabricReceiver2<T>) {
    let (tx, rx) = tokio::sync::oneshot::channel::<T>();
    (
        FabricSender2::new(tx, token.clone()),
        FabricReceiver2::new(rx, token),
    )
}

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
/// Cancelling the token will in turn cancalling the fabric operation. Caller needs to
/// poll/run the receiver future to completion (even if operation intends to cancel),
/// or else cancellation signal might not propagate to SF.
/// After cancellation is triggered, the receiver future should finish in a short time,
/// with an error code opeartion cancelled, or other code if cancel failed.
/// If the result is ready before the cancellation is triggered, the success result will
/// be the output of the receiver future.
pub fn fabric_begin_end_proxy2<BEGIN, END, T>(
    begin: BEGIN,
    end: END,
    token: Option<CancellationToken>,
) -> FabricReceiver2<::windows_core::Result<T>>
where
    BEGIN: FnOnce(
        Option<&IFabricAsyncOperationCallback>,
    ) -> crate::Result<IFabricAsyncOperationContext>,
    END: FnOnce(Option<&IFabricAsyncOperationContext>) -> crate::Result<T> + 'static,
    T: 'static,
{
    let (tx, mut rx) = oneshot_channel(token);

    let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
        let res = end(ctx);
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

#[cfg(test)]
mod test {
    use std::{
        cell::Cell,
        sync::{Arc, Mutex},
        time::Duration,
    };

    use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
    use tokio::{runtime::Handle, select};
    use tokio_util::sync::CancellationToken;

    use crate::{
        error::FabricErrorCode,
        runtime::executor::DefaultExecutor,
        sync::cancel::{oneshot_channel, BridgeContext3},
    };

    use super::fabric_begin_end_proxy2;

    #[tokio::test]
    async fn test_channel() {
        // success send
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            tx.send(true);
            assert!(rx.await.unwrap());
        }
        // receiver cancelled after send, still received the result.
        {
            let token = CancellationToken::new();
            let (tx, rx) = oneshot_channel::<bool>(Some(token.clone()));
            tx.send(true);
            token.cancel();
            assert!(rx.await.unwrap());
        }
        // receiver cancelled before send, still received the result.
        {
            let token = CancellationToken::new();
            let (tx, rx) = oneshot_channel::<bool>(Some(token.clone()));
            token.cancel();
            tx.send(true);
            assert!(rx.await.unwrap(),);
        }
        // receiver cancelled and droped, send is no op
        {
            let token = CancellationToken::new();
            let (tx, rx) = oneshot_channel::<bool>(Some(token.clone()));
            token.cancel();
            std::mem::drop(rx);
            tx.send(true);
        }
        // receiver cancelled and sender dropped. receiver get error
        {
            let token = CancellationToken::new();
            let (tx, rx) = oneshot_channel::<bool>(Some(token.clone()));
            token.cancel();
            std::mem::drop(tx);
            assert_eq!(
                rx.await.unwrap_err(),
                FabricErrorCode::OperationCanceled.into()
            );
        }
    }

    // Test interface for cancellation
    #[allow(dead_code)]
    #[trait_variant::make(IMyObj: Send)]
    pub trait LocalIMyObj: Send + Sync + 'static {
        // Get the data inside
        // This operation will wait for duration of delay before performing work.
        async fn get_data_delay(
            &self,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<String>;

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<()>;
    }

    /// Test Obj for cancellation
    pub struct MyObj {
        data: Mutex<Cell<String>>,
    }

    impl IMyObj for MyObj {
        async fn get_data_delay(
            &self,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<String> {
            if delay.is_zero() {
                return Ok(self.get_data());
            }
            match token {
                Some(t) => {
                    select! {
                        _ = t.cancelled() => {
                            // The token was cancelled
                            Err(FabricErrorCode::OperationCanceled.into())
                        }
                        _ = tokio::time::sleep(delay) => {
                            Ok(self.get_data())
                        }
                    }
                }
                None => {
                    tokio::time::sleep(delay).await;
                    Ok(self.get_data())
                }
            }
        }

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<()> {
            if delay.is_zero() {
                return Ok(self.set_data(input));
            }
            match token {
                Some(t) => {
                    select! {
                        _ = t.cancelled() => {
                            // The token was cancelled
                            Err(FabricErrorCode::OperationCanceled.into())
                        }
                        _ = tokio::time::sleep(delay) => {
                            Ok(self.set_data(input))
                        }
                    }
                }
                None => {
                    tokio::time::sleep(delay).await;
                    Ok(self.set_data(input))
                }
            }
        }
    }

    impl MyObj {
        pub fn new(data: String) -> Self {
            Self {
                data: Mutex::new(Cell::new(data)),
            }
        }

        fn get_data(&self) -> String {
            self.data.lock().unwrap().get_mut().clone()
        }

        fn set_data(&self, input: String) {
            self.data.lock().unwrap().replace(input);
        }
    }

    pub struct MyObjBridge<T: IMyObj> {
        inner: Arc<T>,
        rt: DefaultExecutor,
    }

    impl<T: IMyObj> Clone for MyObjBridge<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
                rt: self.rt.clone(),
            }
        }
    }

    impl<T: IMyObj> MyObjBridge<T> {
        pub fn new(rt: Handle, inner: T) -> Self {
            Self {
                inner: Arc::new(inner),
                rt: DefaultExecutor::new(rt),
            }
        }

        pub fn begin_get_data_delay(
            &self,
            delay: Duration,
            callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
        ) -> crate::Result<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.spawn(&self.rt, async move {
                inner.get_data_delay(delay, Some(token)).await
            })
        }

        pub fn end_get_data_delay(
            &self,
            context: ::core::option::Option<&IFabricAsyncOperationContext>,
        ) -> crate::Result<String> {
            BridgeContext3::result(context)?
        }

        pub fn begin_set_data_delay(
            &self,
            input: String,
            delay: Duration,
            callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
        ) -> crate::Result<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.spawn(&self.rt, async move {
                inner.set_data_delay(input, delay, Some(token)).await
            })
        }

        pub fn end_set_data_delay(
            &self,
            context: ::core::option::Option<&IFabricAsyncOperationContext>,
        ) -> crate::Result<()> {
            BridgeContext3::result(context)?
        }
    }

    pub struct MyObjProxy<T: IMyObj> {
        com: MyObjBridge<T>,
    }

    impl<T: IMyObj> MyObjProxy<T> {
        pub fn new(rt: Handle, inner: T) -> Self {
            let bridge = MyObjBridge::new(rt, inner);
            Self { com: bridge }
        }
    }

    impl<T: IMyObj> IMyObj for MyObjProxy<T> {
        async fn get_data_delay(
            &self,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<String> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| com1.begin_get_data_delay(delay, callback),
                move |context| com2.end_get_data_delay(context),
                token,
            )
            .await?
        }

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::Result<()> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| com1.begin_set_data_delay(input, delay, callback),
                move |context| com2.end_set_data_delay(context),
                token,
            )
            .await?
        }
    }

    #[tokio::test]
    async fn test_cancel() {
        let h = tokio::runtime::Handle::current();
        let expected_data1 = "mydata1";
        // test the plain obj
        let inner = MyObj::new(expected_data1.to_string());
        test_cancel_interface(&inner, expected_data1).await;
        let proxy = MyObjProxy::new(h.clone(), inner);
        test_cancel_interface(&proxy, expected_data1).await;
        // proxy in another layer
        let proxy2 = MyObjProxy::new(h.clone(), proxy);
        test_cancel_interface(&proxy2, expected_data1).await;
        let proxy3 = MyObjProxy::new(h, proxy2);
        test_cancel_interface(&proxy3, expected_data1).await;
    }

    async fn test_cancel_interface(obj: &impl IMyObj, init_data: &str) {
        // get with no cancel
        {
            let token = CancellationToken::new();
            let out = obj
                .get_data_delay(Duration::ZERO, Some(token))
                .await
                .unwrap();
            assert_eq!(out, init_data);
        }
        // get with cancel
        {
            let token = CancellationToken::new();
            let fu = obj.get_data_delay(Duration::from_secs(5), Some(token.clone()));
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, FabricErrorCode::OperationCanceled.into());
        }
        // set with cancel
        {
            let token = CancellationToken::new();
            let fu = obj.set_data_delay(
                "random_data".to_string(),
                Duration::from_millis(15),
                Some(token.clone()),
            );
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, FabricErrorCode::OperationCanceled.into());
        }
        // because of cancel, data should not be changed.
        {
            // sleep past the delay time to observe the final state
            tokio::time::sleep(Duration::from_millis(20)).await;
            let out = obj.get_data_delay(Duration::ZERO, None).await.unwrap();
            assert_eq!(out, init_data);
        }
        let expected_data2 = "mydata2";
        // set without cancel
        {
            obj.set_data_delay(expected_data2.to_string(), Duration::from_millis(1), None)
                .await
                .expect("fail to set data");
        }
        // read the set.
        {
            let out = obj.get_data_delay(Duration::ZERO, None).await.unwrap();
            assert_eq!(out, expected_data2);
        }
        // restore the data to the original data
        // So that the next test can reuse the obj.
        {
            {
                obj.set_data_delay(init_data.to_string(), Duration::ZERO, None)
                    .await
                    .expect("fail to set data");
            }
        }
    }
}
