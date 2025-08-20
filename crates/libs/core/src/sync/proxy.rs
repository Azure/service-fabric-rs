// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::sync::CancellationToken;
use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};

use super::{FabricReceiver, oneshot_channel};

// proxy impl

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
pub fn fabric_begin_end_proxy<BEGIN, END, T>(
    begin: BEGIN,
    end: END,
    token: Option<CancellationToken>,
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

#[cfg(test)]
mod test {
    use std::{
        cell::Cell,
        sync::{Arc, Mutex, atomic::AtomicBool},
        time::Duration,
    };

    use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
    use tokio::{runtime::Handle, select};
    use tokio_util::sync::CancellationToken;

    use crate::{
        error::ErrorCode, runtime::executor::Executor, sync::bridge_context::BridgeContext,
    };

    use super::fabric_begin_end_proxy;

    /// Temporary test executor for running tasks
    /// TODO: remove this. Or move the sync tests to the util crate.
    #[derive(Clone)]
    struct TestExecutor {
        rt: Handle,
    }

    impl TestExecutor {
        pub fn new(rt: Handle) -> Self {
            Self { rt }
        }
    }

    impl Executor for TestExecutor {
        fn spawn<F>(&self, future: F)
        where
            F: Future + Send + 'static,
            F::Output: Send,
        {
            self.rt.spawn(future);
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
        rt: TestExecutor,
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
                rt: TestExecutor::new(rt),
            }
        }

        pub fn begin_get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool,
            callback: windows_core::Ref<IFabricAsyncOperationCallback>,
        ) -> crate::WinResult<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext::make(callback);
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
            BridgeContext::result(context)?
        }

        pub fn begin_set_data_delay(
            &self,
            input: String,
            delay: Duration,
            callback: windows_core::Ref<IFabricAsyncOperationCallback>,
        ) -> crate::WinResult<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext::make(callback);
            ctx.spawn(&self.rt, async move {
                inner.set_data_delay(input, delay, Some(token)).await
            })
        }

        pub fn end_set_data_delay(
            &self,
            context: windows_core::Ref<IFabricAsyncOperationContext>,
        ) -> crate::WinResult<()> {
            BridgeContext::result(context)?
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
    fn option_to_ref<T>(opt: Option<&T>) -> windows_core::Ref<'_, T>
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
            fabric_begin_end_proxy(
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
            fabric_begin_end_proxy(
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
