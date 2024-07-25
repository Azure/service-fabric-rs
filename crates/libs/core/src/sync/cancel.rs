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

    pub fn make(callback: Option<&IFabricAsyncOperationCallback>) -> (Self, CancellationToken) {
        let token = CancellationToken::new();
        let ctx = Self::new(callback.unwrap().clone(), token.clone());
        (ctx, token)
    }

    // executes the future in background
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
}

impl<T> FabricReceiver2<T> {
    fn new(
        rx: tokio::sync::oneshot::Receiver<T>,
        token: Option<CancellationToken>,
    ) -> FabricReceiver2<T> {
        FabricReceiver2 { rx, token }
    }

    pub fn blocking_recv(self) -> crate::Result<T> {
        if let Some(t) = self.token {
            if t.is_cancelled() {
                return Err(FabricErrorCode::OperationCanceled.into());
            }
        }
        // sender must send stuff so that there is not error.
        Ok(self.rx.blocking_recv().unwrap())
    }

    // cancels the operation
    pub fn cancel(&self) -> crate::Result<()> {
        match &self.token {
            Some(t) => {
                t.cancel();
                Ok(())
            }
            None => Err(FabricErrorCode::OperationNotSupported.into()),
        }
    }
}

// Returns error if cancelled.
impl<T> Future for FabricReceiver2<T> {
    // The error will only have error code OperationCanceled
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

// Creates a fabric oneshot channel.
pub fn oneshot_channel<T>(
    token: Option<CancellationToken>,
) -> (FabricSender2<T>, FabricReceiver2<T>) {
    let (tx, rx) = tokio::sync::oneshot::channel::<T>();
    (
        FabricSender2::new(tx, token.clone()),
        FabricReceiver2::new(rx, token),
    )
}

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
    let (tx, rx) = oneshot_channel(token);

    let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
        let res = end(ctx);
        tx.send(res);
    });
    let ctx = begin(Some(&callback));
    if ctx.is_err() {
        let (tx2, rx2) = oneshot_channel(None);
        tx2.send(Err(ctx.err().unwrap()));
        rx2
    } else {
        rx
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
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            tx.send(true);
            rx.cancel().unwrap();
            assert!(rx.await.unwrap(),);
        }
        // receiver cancelled before send, still received the result.
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            rx.cancel().unwrap();
            tx.send(true);
            assert!(rx.await.unwrap(),);
        }
        // receiver cancelled and droped, send is no op
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            rx.cancel().unwrap();
            std::mem::drop(rx);
            tx.send(true);
        }
        // receiver cancelled and sender dropped. receiver get error
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            rx.cancel().unwrap();
            std::mem::drop(tx);
            assert_eq!(
                rx.await.unwrap_err(),
                FabricErrorCode::OperationCanceled.into()
            );
        }
    }

    /// Test Obj for cancellation
    pub struct MyObj {
        data: Mutex<Cell<String>>,
    }

    impl MyObj {
        pub fn new(data: String) -> Self {
            Self {
                data: Mutex::new(Cell::new(data)),
            }
        }
        // Get the data inside
        // This operation will wait for duration of delay before performing work.
        pub async fn get_data_delay(
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

        fn get_data(&self) -> String {
            self.data.lock().unwrap().get_mut().clone()
        }

        fn set_data(&self, input: String) {
            self.data.lock().unwrap().replace(input);
        }

        pub async fn set_data_delay(
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

    #[derive(Clone)]
    pub struct MyObjBridge {
        inner: Arc<MyObj>,
        rt: DefaultExecutor,
    }

    impl MyObjBridge {
        pub fn new(rt: Handle, data: String) -> Self {
            let inner = Arc::new(MyObj::new(data));
            Self {
                inner,
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

    pub struct MyObjProxy {
        com: MyObjBridge,
    }

    impl MyObjProxy {
        pub fn new(com: MyObjBridge) -> Self {
            Self { com }
        }

        pub async fn get_data_delay(
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

        pub async fn set_data_delay(
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
        let bridge = MyObjBridge::new(h, expected_data1.to_string());
        let proxy = MyObjProxy::new(bridge);
        // get with no cancel
        {
            let token = CancellationToken::new();
            let out = proxy
                .get_data_delay(Duration::ZERO, Some(token))
                .await
                .unwrap();
            assert_eq!(out, expected_data1);
        }
        // get with cancel
        {
            let token = CancellationToken::new();
            let fu = proxy.get_data_delay(Duration::from_secs(5), Some(token.clone()));
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, FabricErrorCode::OperationCanceled.into());
        }
        // set with cancel
        {
            let token = CancellationToken::new();
            let fu = proxy.set_data_delay(
                "random_data".to_string(),
                Duration::from_secs(5),
                Some(token.clone()),
            );
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, FabricErrorCode::OperationCanceled.into());
        }
        // because of cancel, data should not be changed.
        {
            let out = proxy.get_data_delay(Duration::ZERO, None).await.unwrap();
            assert_eq!(out, expected_data1);
        }
        let expected_data2 = "mydata2";
        // set without cancel
        {
            proxy
                .set_data_delay(expected_data2.to_string(), Duration::from_millis(1), None)
                .await
                .expect("fail to set data");
        }
        // read the set.
        {
            let out = proxy.get_data_delay(Duration::ZERO, None).await.unwrap();
            assert_eq!(out, expected_data2);
        }
    }
}
