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

    // TODO: access
    // executes the future in background
    pub fn execute<F>(
        self,
        rt: &impl Executor,
        future: F,
    ) -> crate::Result<IFabricAsyncOperationContext>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let self_cp: IFabricAsyncOperationContext = self.into();
        // extra clone is necessary to avoid access violation.
        let self_cp3 = self_cp.clone();
        let self_cp2 = self_cp.clone();
        rt.spawn(async move {
            let ok = future.await;
            let self_impl: &BridgeContext3<T> = unsafe { self_cp3.as_impl() };
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

// The future differs from tokio oneshot that it will not error when awaited.
// Returns error if cancelled.
impl<T> Future for FabricReceiver2<T> {
    type Output = crate::Result<T>;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // if cancelled return error, and the sender will not send anything.

        // try to poll cancel
        if let Some(t) = &self.token {
            // this is cancel safe so we can poll it once and discard?
            let fu = t.cancelled();
            let inner = std::pin::pin!(fu).poll(_cx);
            match inner {
                Poll::Ready(_) => {
                    // operation cancelled
                    return Poll::Ready(Err(FabricErrorCode::OperationCanceled.into()));
                }
                Poll::Pending => {
                    // continue to poll rx
                }
            }
        }

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
            Poll::Pending => Poll::Pending,
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
    use std::sync::Arc;

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
        // receiver cancelled after send
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            tx.send(true);
            rx.cancel().unwrap();
            assert_eq!(
                rx.await.unwrap_err(),
                FabricErrorCode::OperationCanceled.into()
            );
        }
        // receiver cancelled before send
        {
            let (tx, rx) = oneshot_channel::<bool>(Some(CancellationToken::new()));
            rx.cancel().unwrap();
            tx.send(true);
            assert_eq!(
                rx.await.unwrap_err(),
                FabricErrorCode::OperationCanceled.into()
            );
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

    pub struct MyObj {
        data: String,
        rt: Handle,
    }

    impl MyObj {
        pub fn new(rt: Handle, data: String) -> Self {
            Self { data, rt }
        }
        // concat input with data.
        // This operation is slow and use it to test cancel.
        pub async fn get_data_slow(
            &self,
            input: String,
            token: Option<CancellationToken>,
        ) -> crate::Result<String> {
            match token {
                Some(t) => {
                    select! {
                        _ = t.cancelled() => {
                            // The token was cancelled
                            Err(FabricErrorCode::OperationCanceled.into())
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                            self.get_data(input, None).await
                        }
                    }
                }
                None => {
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                    self.get_data(input, None).await
                }
            }
        }

        pub async fn get_data(
            &self,
            input: String,
            _: Option<CancellationToken>,
        ) -> crate::Result<String> {
            let data_cp = self.data.clone();
            match self
                .rt
                .spawn(async move { format!("{}:{}", data_cp, input) })
                .await
            {
                Ok(s) => Ok(s),
                Err(_) => Err(FabricErrorCode::OperationFailed.into()),
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
            let inner = Arc::new(MyObj::new(rt.clone(), data));
            Self {
                inner,
                rt: DefaultExecutor::new(rt),
            }
        }

        pub fn begin_get_data(
            &self,
            input: String,
            callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
        ) -> crate::Result<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.execute(
                &self.rt,
                async move { inner.get_data(input, Some(token)).await },
            )
        }

        pub fn end_get_data(
            &self,
            context: ::core::option::Option<&IFabricAsyncOperationContext>,
        ) -> crate::Result<String> {
            BridgeContext3::result(context)?
        }

        pub fn begin_get_data_slow(
            &self,
            input: String,
            callback: ::core::option::Option<&IFabricAsyncOperationCallback>,
        ) -> crate::Result<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.execute(&self.rt, async move {
                inner.get_data_slow(input, Some(token)).await
            })
        }

        pub fn end_get_data_slow(
            &self,
            context: ::core::option::Option<&IFabricAsyncOperationContext>,
        ) -> crate::Result<String> {
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

        pub async fn get_data(
            &self,
            input: String,
            token: Option<CancellationToken>,
        ) -> crate::Result<String> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| com1.begin_get_data(input, callback),
                move |context| com2.end_get_data(context),
                token,
            )
            .await?
        }

        pub async fn get_data_slow(
            &self,
            input: String,
            token: Option<CancellationToken>,
        ) -> crate::Result<String> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| com1.begin_get_data_slow(input, callback),
                move |context| com2.end_get_data_slow(context),
                token,
            )
            .await?
        }
    }

    #[tokio::test]
    async fn test_cancel() {
        let h = tokio::runtime::Handle::current();
        let bridge = MyObjBridge::new(h, "mydata".to_string());
        let proxy = MyObjProxy::new(bridge);

        // no cancel
        {
            let token = CancellationToken::new();
            let out = proxy
                .get_data("myinput".to_string(), Some(token))
                .await
                .unwrap();
            assert_eq!(out, "mydata:myinput");
        }
        // cancel
        {
            let token = CancellationToken::new();
            let fu = proxy.get_data_slow("myinput".to_string(), Some(token.clone()));
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, FabricErrorCode::OperationCanceled.into());
        }
    }
}
