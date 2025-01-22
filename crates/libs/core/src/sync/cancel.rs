// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
pub use tokio_util::sync::CancellationToken;

use crate::error::ErrorCode;

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
    //             return Err(ErrorCode::OperationCanceled.into());
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
    fn cancel_inner_ctx(&mut self) -> crate::WinResult<()> {
        if let Some(ctx) = &self.ctx {
            if let Err(e) = unsafe { ctx.Cancel() } {
                // fail to cancel inner operation.
                return Err(e);
            } else {
                // clear the sf ctx to avoid cancel twice.
                self.ctx.take();
            }
        } else {
            // The inner ctx can be empty after we already cancelled the inner ctx.
            // This can happen because we cancel during polling, and polling can
            // happen many times.
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
    type Output = crate::WinResult<T>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the receiver first, if ready then return the output,
        // else poll the cancellation token, if cancelled propergate the cancel to SF ctx,
        // and return pending. SF task should continue finish execute in the background,
        // and finish with error code OperationCancelled
        // and send the error code from FabricSender.
        //
        // There can be the case that cancellation wakes the waker, but receiver
        // then got the result. The next poll will return received output rather
        // than cancelled error.

        // Try to receive the value from the sender
        let inner = <tokio::sync::oneshot::Receiver<T> as Future>::poll(Pin::new(&mut self.rx), cx);
        match (inner, self.token.as_ref()) {
            (Poll::Ready(Ok(data)), _) => Poll::Ready(Ok(data)),
            (Poll::Ready(Err(_)), Some(t)) => {
                if t.is_cancelled() {
                    // clear the token since we only propergate the signal once.
                    self.token.take();
                    // cancel the SF ctx and clear it.
                    if let Err(e) = self.cancel_inner_ctx() {
                        Poll::Ready(Err(e))
                    } else {
                        Poll::Ready(Err(ErrorCode::E_ABORT.into()))
                    }
                } else {
                    panic!("sender dropped without sending")
                }
            }
            (Poll::Ready(Err(_)), None) => {
                panic!("sender dropped without sending")
            }
            (Poll::Pending, Some(t)) => {
                // If the action is canceled we can safely stop and return canceled error.
                // this is cancel safe so we can poll it once and discard
                let fu = t.cancelled();
                let inner = std::pin::pin!(fu).poll(cx);
                match inner {
                    Poll::Ready(_) => {
                        // clear the token since we only propergate the signal once.
                        self.token.take();
                        // operation cancelled. Propergate to inner sf ctx.
                        if let Err(e) = self.cancel_inner_ctx() {
                            Poll::Ready(Err(e))
                        } else {
                            // The cancellation is propergated to sf task,
                            // the receiver from now on should wait for the
                            // final result from the sf task. (as we have cleared the token)
                            // Most likely the task finishes with OperationCancelled error code.
                            Poll::Pending
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            (Poll::Pending, None) => Poll::Pending,
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
                debug_assert!(
                    t.is_cancelled(),
                    "task should be cancelled when receiver dropped."
                );
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
) -> FabricReceiver2<crate::WinResult<T>>
where
    BEGIN: FnOnce(
        Option<&IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<IFabricAsyncOperationContext>,
    END: FnOnce(Option<&IFabricAsyncOperationContext>) -> crate::WinResult<T> + 'static,
    T: 'static,
{
    let (tx, mut rx) = oneshot_channel(token);

    let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
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

#[cfg(test)]
mod test {
    use std::{
        cell::Cell,
        sync::{atomic::AtomicBool, Arc, Mutex},
        time::Duration,
    };

    use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
    use tokio::{runtime::Handle, select};
    use tokio_util::sync::CancellationToken;

    use crate::{
        error::ErrorCode, runtime::executor::DefaultExecutor, sync::bridge_context::BridgeContext3,
        sync::cancel::oneshot_channel,
    };

    use super::fabric_begin_end_proxy2;

    /// Test various cancellation cases for the channel used
    /// to send data in proxy layer.
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
            assert_eq!(rx.await.unwrap_err(), ErrorCode::E_ABORT.into());
        }
    }

    /// Test trait for cancellation
    /// The whole test focuses on testing cancelation propergation from SF api to rust api
    /// and also from rust api to sf api.
    /// The Proxy and Bridge layers all implementes this trait and the same test can run for
    /// this trait. Proxy and Bridge layers can have arbitrary nesting.
    #[allow(dead_code)]
    #[trait_variant::make(IMyObj: Send)]
    pub trait LocalIMyObj: Send + Sync + 'static {
        // Get the data inside
        // This operation will wait for duration of delay before performing work.
        async fn get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool, // ignores the token
            token: Option<CancellationToken>,
        ) -> crate::WinResult<String>;

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::WinResult<()>;
    }

    /// Test Obj for cancellation
    pub struct MyObj {
        data: Mutex<Cell<String>>,
        panic: AtomicBool,
    }

    // Implement the test trait
    impl IMyObj for MyObj {
        async fn get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool,
            token: Option<CancellationToken>,
        ) -> crate::WinResult<String> {
            if self.panic.load(std::sync::atomic::Ordering::Relaxed) {
                panic!("test panic is set")
            }
            if delay.is_zero() {
                // This is needed to make future is breakable in bench test in select
                tokio::task::yield_now().await;
                return Ok(self.get_data());
            }
            match (token, ignore_cancel) {
                (Some(t), false) => {
                    select! {
                        _ = t.cancelled() => {
                            // The token was cancelled
                            Err(ErrorCode::E_ABORT.into())
                        }
                        _ = tokio::time::sleep(delay) => {
                            Ok(self.get_data())
                        }
                    }
                }
                // token is empty or ignore cancel.
                _ => {
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
        ) -> crate::WinResult<()> {
            if self.panic.load(std::sync::atomic::Ordering::Relaxed) {
                panic!("test panic is set")
            }
            if delay.is_zero() {
                // This is needed to make future is breakable in bench test in select
                tokio::task::yield_now().await;
                self.set_data(input);
                return Ok(());
            }
            match token {
                Some(t) => {
                    select! {
                        _ = t.cancelled() => {
                            // The token was cancelled
                            Err(ErrorCode::E_ABORT.into())
                        }
                        _ = tokio::time::sleep(delay) => {
                            self.set_data(input);
                            Ok(())
                        }
                    }
                }
                None => {
                    tokio::time::sleep(delay).await;
                    self.set_data(input);
                    Ok(())
                }
            }
        }
    }

    impl MyObj {
        pub fn new(data: String) -> Self {
            Self {
                data: Mutex::new(Cell::new(data)),
                panic: AtomicBool::new(false),
            }
        }

        fn get_data(&self) -> String {
            self.data.lock().unwrap().get_mut().clone()
        }

        fn set_data(&self, input: String) {
            self.data.lock().unwrap().replace(input);
        }
    }

    /// This is a bridge to turn the test interface
    /// into a SF Async Begin and End api.
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
            ignore_cancel: bool,
            callback: windows_core::Ref<IFabricAsyncOperationCallback>,
        ) -> crate::WinResult<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.spawn(&self.rt, async move {
                inner
                    .get_data_delay(delay, ignore_cancel, Some(token))
                    .await
            })
        }

        pub fn end_get_data_delay(
            &self,
            context: windows_core::Ref<IFabricAsyncOperationContext>,
        ) -> crate::WinResult<String> {
            BridgeContext3::result(context)?
        }

        pub fn begin_set_data_delay(
            &self,
            input: String,
            delay: Duration,
            callback: windows_core::Ref<IFabricAsyncOperationCallback>,
        ) -> crate::WinResult<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext3::make(callback);
            ctx.spawn(&self.rt, async move {
                inner.set_data_delay(input, delay, Some(token)).await
            })
        }

        pub fn end_set_data_delay(
            &self,
            context: windows_core::Ref<IFabricAsyncOperationContext>,
        ) -> crate::WinResult<()> {
            BridgeContext3::result(context)?
        }
    }

    /// This is a proxy to turn SF async Begin/End api
    /// to the rust trait.
    pub struct MyObjProxy<T: IMyObj> {
        com: MyObjBridge<T>,
    }

    impl<T: IMyObj> MyObjProxy<T> {
        pub fn new(rt: Handle, inner: T) -> Self {
            let bridge = MyObjBridge::new(rt, inner);
            Self { com: bridge }
        }
    }

    /// Converts option ref to windows ref for testing.
    /// They have the same ABI.
    /// Returned ref has the same lifetime as the opt.
    fn option_to_ref<T>(opt: Option<&T>) -> windows_core::Ref<T>
    where
        T: crate::Interface,
    {
        unsafe { core::mem::transmute_copy(opt.unwrap()) }
    }

    // The test trait implementation
    impl<T: IMyObj> IMyObj for MyObjProxy<T> {
        async fn get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool,
            token: Option<CancellationToken>,
        ) -> crate::WinResult<String> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| {
                    com1.begin_get_data_delay(delay, ignore_cancel, option_to_ref(callback))
                },
                move |context| com2.end_get_data_delay(option_to_ref(context)),
                token,
            )
            .await?
        }

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<CancellationToken>,
        ) -> crate::WinResult<()> {
            let com1 = &self.com;
            let com2 = self.com.clone();
            fabric_begin_end_proxy2(
                move |callback| com1.begin_set_data_delay(input, delay, option_to_ref(callback)),
                move |context| com2.end_set_data_delay(option_to_ref(context)),
                token,
            )
            .await?
        }
    }

    /// Constructs various test trait objects of different
    /// Bridge and Proxy nested wrapping and run cancellation tests
    /// for each of them.
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

    /// Given a test trait obj, run various cancellation tests on it.
    async fn test_cancel_interface(obj: &impl IMyObj, init_data: &str) {
        // get with no cancel
        {
            let token = CancellationToken::new();
            let out = obj
                .get_data_delay(Duration::ZERO, false, Some(token))
                .await
                .unwrap();
            assert_eq!(out, init_data);
        }
        // get with cancel
        {
            let token = CancellationToken::new();
            let fu = obj.get_data_delay(Duration::from_secs(5), false, Some(token.clone()));
            token.cancel();
            let err = fu.await.unwrap_err();
            assert_eq!(err, ErrorCode::E_ABORT.into());
        }
        // get with cancel but ignore cancel from inner impl.
        // Because the cancel is ignored by inner implementation, success will be returned.
        // This shows that the sender and receiver does not short circuit the future when token is cancelled,
        // the future result is always the result from the (SF) background task.
        {
            let token = CancellationToken::new();
            let fu = obj.get_data_delay(Duration::from_millis(3), true, Some(token.clone()));
            token.cancel();
            let out = fu.await.unwrap();
            assert_eq!(out, init_data);
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
            assert_eq!(err, ErrorCode::E_ABORT.into());
        }
        // because of cancel, data should not be changed.
        {
            // sleep past the delay time to observe the final state
            tokio::time::sleep(Duration::from_millis(20)).await;
            let out = obj
                .get_data_delay(Duration::ZERO, false, None)
                .await
                .unwrap();
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
            let out = obj
                .get_data_delay(Duration::ZERO, false, None)
                .await
                .unwrap();
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

    const TEST_DATA: &str = "data";
    /// Very simple benchmark to check the bridge layer performance.
    /// Adding a bridge layer should not introduce much perf degradation.
    #[tokio::test]
    async fn small_bench_test() {
        // Run get data function for IMyObj with different layers of wrapping.
        // All wrappings are run in parallel to reduce test run time.
        // plain object
        let j0 = tokio::spawn(async move {
            let obj0 = MyObj::new(TEST_DATA.to_string());
            small_bench(&obj0, TEST_DATA).await
        });
        // object with 1 layer of bridge proxy wrapping
        let j1 = tokio::spawn(async {
            let h = tokio::runtime::Handle::current();
            let obj1 = MyObjProxy::new(h.clone(), MyObj::new(TEST_DATA.to_string()));
            small_bench(&obj1, TEST_DATA).await
        });
        // object with 2 layers.
        let j2 = tokio::spawn(async {
            let h = tokio::runtime::Handle::current();
            let obj2 = MyObjProxy::new(
                h.clone(),
                MyObjProxy::new(h.clone(), MyObj::new(TEST_DATA.to_string())),
            );
            small_bench(&obj2, TEST_DATA).await
        });
        let count0 = j0.await.unwrap();
        let count1 = j1.await.unwrap();
        let count2 = j2.await.unwrap();
        println!("count0: {count0}, count1: {count1}, count2: {count2}");
        // Conservative check for 2 layer wrapping does not degrade performance by half.
        // Usually there is hardly any perf degradation by wrapping.
        assert!(count0 / count2 <= 2)
    }

    /// Run bench for 3 seconds and return the number of execution reached.
    /// It keeps running get_data api and returns how many times it got executed.
    async fn small_bench(obj: &impl IMyObj, expected_data: &str) -> usize {
        let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
        let join = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            tx.send(()).unwrap();
        });

        let mut count = 0;
        loop {
            tokio::select! {
                res = &mut rx =>{
                    res.unwrap();
                    break;
                }
                data = IMyObj::get_data_delay(obj,Duration::ZERO, false, None) =>{
                    assert_eq!(data.unwrap(), expected_data);
                    count += 1;
                    if rx.try_recv().is_ok(){
                        break;
                    }
                }
            }
        }
        join.await.unwrap();
        count
    }

    #[tokio::test]
    async fn test_user_code_panic() {
        let h = tokio::runtime::Handle::current();
        let expected_data1 = "mydata1";
        let inner = MyObj::new(expected_data1.to_string());
        let proxy = MyObjProxy::new(h.clone(), inner);
        {
            let out = IMyObj::get_data_delay(&proxy, Duration::ZERO, false, None)
                .await
                .expect("fail to get data");
            assert_eq!(out, expected_data1);
        }
        // enable panic for the user code
        // check the panic is converted to correct error code.
        proxy
            .com
            .inner
            .panic
            .store(true, std::sync::atomic::Ordering::Relaxed);
        {
            let out = IMyObj::get_data_delay(&proxy, Duration::ZERO, false, None)
                .await
                .expect_err("should error out");
            assert_eq!(out, ErrorCode::E_UNEXPECTED.into());
        }
    }
}
