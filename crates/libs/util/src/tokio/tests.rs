// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
mod proxy_test {
    use std::{
        cell::Cell,
        sync::{Arc, Mutex, atomic::AtomicBool},
        time::Duration,
    };

    use mssf_com::FabricCommon::{IFabricAsyncOperationCallback, IFabricAsyncOperationContext};
    use tokio::{runtime::Handle, select};

    use mssf_core::{
        ErrorCode,
        runtime::executor::BoxedCancelToken,
        sync::{BridgeContext, fabric_begin_end_proxy},
    };
    use tokio_util::sync::CancellationToken;

    use crate::tokio::{TokioCancelToken, TokioExecutor};

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
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<String>;

        async fn set_data_delay(
            &self,
            input: String,
            delay: Duration,
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<()>;
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
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<String> {
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
                    // Register a callback on the incoming boxed token that cancels
                    // a local TokioCancelToken. This exercises on_cancel propagation
                    // through the bridge/proxy layers.
                    let local = CancellationToken::new();
                    let local_clone = local.clone();
                    t.on_cancel(Box::new(move || local_clone.cancel()));
                    select! {
                        _ = local.cancelled() => {
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
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<()> {
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
                        _ = t.wait() => {
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
        rt: TokioExecutor,
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
                rt: TokioExecutor::new(rt),
            }
        }

        pub fn begin_get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool,
            callback: mssf_core::Ref<IFabricAsyncOperationCallback>,
        ) -> mssf_core::WinResult<IFabricAsyncOperationContext> {
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
            context: mssf_core::Ref<IFabricAsyncOperationContext>,
        ) -> mssf_core::WinResult<String> {
            BridgeContext::result(context)?
        }

        pub fn begin_set_data_delay(
            &self,
            input: String,
            delay: Duration,
            callback: mssf_core::Ref<IFabricAsyncOperationCallback>,
        ) -> mssf_core::WinResult<IFabricAsyncOperationContext> {
            let inner = self.inner.clone();
            let (ctx, token) = BridgeContext::make(callback);
            ctx.spawn(&self.rt, async move {
                inner.set_data_delay(input, delay, Some(token)).await
            })
        }

        pub fn end_set_data_delay(
            &self,
            context: mssf_core::Ref<IFabricAsyncOperationContext>,
        ) -> mssf_core::WinResult<()> {
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
    fn option_to_ref<T>(opt: Option<&T>) -> mssf_core::Ref<'_, T>
    where
        T: mssf_core::Interface,
    {
        unsafe { core::mem::transmute_copy(opt.unwrap()) }
    }

    // The test trait implementation
    impl<T: IMyObj> IMyObj for MyObjProxy<T> {
        async fn get_data_delay(
            &self,
            delay: Duration,
            ignore_cancel: bool,
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<String> {
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
            token: Option<BoxedCancelToken>,
        ) -> mssf_core::WinResult<()> {
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
            let token = TokioCancelToken::new_boxed();
            let out = obj
                .get_data_delay(Duration::ZERO, false, Some(token))
                .await
                .unwrap();
            assert_eq!(out, init_data);
        }
        // get with cancel
        {
            let token = TokioCancelToken::new_boxed();
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
            let token = TokioCancelToken::new_boxed();
            let fu = obj.get_data_delay(Duration::from_millis(3), true, Some(token.clone()));
            token.cancel();
            let out = fu.await.unwrap();
            assert_eq!(out, init_data);
        }
        // set with cancel
        {
            let token = TokioCancelToken::new_boxed();
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

mod cancel_token_tests {
    use mssf_core::runtime::executor::CancelToken;
    use mssf_core::sync::SimpleCancelToken;

    use crate::tokio::TokioCancelToken;

    // --- cancel ---

    fn test_cancel<T: CancelToken + Clone + Default>() {
        let token = T::default();
        assert!(!token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn simple_cancel() {
        test_cancel::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_cancel() {
        test_cancel::<TokioCancelToken>();
    }

    // --- cancel_async_wait ---

    async fn test_cancel_async_wait<T: CancelToken + Clone + Default>() {
        let token = T::default();
        let token_clone = token.clone();
        let h = tokio::spawn(async move {
            token_clone.wait().await;
        });
        token.cancel();
        assert!(token.is_cancelled());
        h.await.unwrap();
    }

    #[tokio::test]
    async fn simple_cancel_async_wait() {
        test_cancel_async_wait::<SimpleCancelToken>().await;
    }

    #[tokio::test]
    async fn tokio_cancel_async_wait() {
        test_cancel_async_wait::<TokioCancelToken>().await;
    }

    // --- cancel_multi_waiters ---

    async fn test_cancel_multi_waiters<T: CancelToken + Clone + Default>() {
        let token = T::default();
        let mut join_set = tokio::task::JoinSet::new();

        for _ in 0..10 {
            let t = token.clone();
            join_set.spawn(async move {
                t.wait().await;
            });
            tokio::task::yield_now().await;
        }
        token.cancel();
        assert!(token.is_cancelled());
        join_set.join_all().await;
    }

    #[tokio::test]
    async fn simple_cancel_multi_waiters() {
        test_cancel_multi_waiters::<SimpleCancelToken>().await;
    }

    #[tokio::test]
    async fn tokio_cancel_multi_waiters() {
        test_cancel_multi_waiters::<TokioCancelToken>().await;
    }

    // --- on_cancel_propagates ---

    fn test_on_cancel_propagates<T: CancelToken + Clone + Default>() {
        let parent = T::default();
        let child = T::default();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        assert!(!parent.is_cancelled());
        assert!(!child_clone.is_cancelled());

        parent.cancel();
        assert!(parent.is_cancelled());
        assert!(child_clone.is_cancelled());
    }

    #[test]
    fn simple_on_cancel_propagates() {
        test_on_cancel_propagates::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_on_cancel_propagates() {
        test_on_cancel_propagates::<TokioCancelToken>();
    }

    // --- on_cancel_already_cancelled ---

    fn test_on_cancel_already_cancelled<T: CancelToken + Clone + Default>() {
        let parent = T::default();
        parent.cancel();

        let child = T::default();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        assert!(child_clone.is_cancelled());
    }

    #[test]
    fn simple_on_cancel_already_cancelled() {
        test_on_cancel_already_cancelled::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_on_cancel_already_cancelled() {
        test_on_cancel_already_cancelled::<TokioCancelToken>();
    }

    // --- on_cancel_independent_cancel ---

    fn test_on_cancel_independent_cancel<T: CancelToken + Clone + Default>() {
        let parent = T::default();
        let child = T::default();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        child_clone.cancel();
        assert!(child_clone.is_cancelled());
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn simple_on_cancel_independent_cancel() {
        test_on_cancel_independent_cancel::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_on_cancel_independent_cancel() {
        test_on_cancel_independent_cancel::<TokioCancelToken>();
    }

    // --- on_cancel_async_wait ---

    async fn test_on_cancel_async_wait<T: CancelToken + Clone + Default>() {
        let parent = T::default();
        let child = T::default();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        let waiter = child_clone.clone();
        let h = tokio::spawn(async move {
            waiter.wait().await;
        });

        parent.cancel();
        h.await.unwrap();
        assert!(child_clone.is_cancelled());
    }

    #[tokio::test]
    async fn simple_on_cancel_async_wait() {
        test_on_cancel_async_wait::<SimpleCancelToken>().await;
    }

    #[tokio::test]
    async fn tokio_on_cancel_async_wait() {
        test_on_cancel_async_wait::<TokioCancelToken>().await;
    }

    // --- Cross-impl tests ---

    #[test]
    fn cross_impl_simple_parent_tokio_child() {
        let parent = SimpleCancelToken::new();
        let child = TokioCancelToken::new();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        parent.cancel();
        assert!(parent.is_cancelled());
        assert!(child_clone.is_cancelled());
    }

    #[test]
    fn cross_impl_tokio_parent_simple_child() {
        let parent = TokioCancelToken::new();
        let child = SimpleCancelToken::new();
        let child_clone = child.clone();
        parent.on_cancel(Box::new(move || child.cancel()));

        parent.cancel();
        assert!(parent.is_cancelled());
        assert!(child_clone.is_cancelled());
    }

    // --- cancel and on_cancel race stress test ---

    /// Stress test: one thread calls cancel() while another thread
    /// concurrently registers a callback. The callback must be invoked
    /// regardless of ordering.
    fn test_on_cancel_race<T: CancelToken + Clone + Default + 'static>() {
        use std::sync::{
            Arc, Barrier,
            atomic::{AtomicBool, Ordering},
        };

        const ITERATIONS: usize = 100;

        for _ in 0..ITERATIONS {
            let parent = T::default();
            let barrier = Arc::new(Barrier::new(2));

            let called = Arc::new(AtomicBool::new(false));
            let called_clone = called.clone();

            let parent_clone = parent.clone();
            let barrier_clone = barrier.clone();
            let handle = std::thread::spawn(move || {
                barrier_clone.wait();
                parent_clone.on_cancel(Box::new(move || {
                    called_clone.store(true, Ordering::Release);
                }));
            });

            // The main thread cancels the parent concurrently.
            barrier.wait();
            parent.cancel();

            handle.join().unwrap();

            // Invariant: the callback must have been invoked.
            assert!(
                called.load(Ordering::Acquire),
                "callback was not invoked after race"
            );
        }
    }

    #[test]
    fn simple_on_cancel_race() {
        test_on_cancel_race::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_on_cancel_race() {
        test_on_cancel_race::<TokioCancelToken>();
    }

    // --- self-cancel in callback (should not deadlock) ---

    fn test_self_cancel_in_callback<T: CancelToken + Clone + Default>() {
        let token = T::default();
        let token_clone = token.clone();
        token.on_cancel(Box::new(move || {
            // This is a no-op but tests that we don't deadlock
            token_clone.cancel();
        }));

        // cancel must not deadlock
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn simple_self_cancel_in_callback() {
        test_self_cancel_in_callback::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_self_cancel_in_callback() {
        test_self_cancel_in_callback::<TokioCancelToken>();
    }

    // --- circular cancel (A→B→A, should not deadlock) ---

    fn test_circular_cancel<T: CancelToken + Clone + Default>() {
        let a = T::default();
        let b = T::default();
        let b_clone = b.clone();
        let a_clone = a.clone();
        a.on_cancel(Box::new(move || b_clone.cancel()));
        b.on_cancel(Box::new(move || a_clone.cancel()));

        // cancelling a cascades to b, which tries to cancel a again — must not deadlock
        a.cancel();
        assert!(a.is_cancelled());
        assert!(b.is_cancelled());
    }

    #[test]
    fn simple_circular_cancel() {
        test_circular_cancel::<SimpleCancelToken>();
    }

    #[test]
    fn tokio_circular_cancel() {
        test_circular_cancel::<TokioCancelToken>();
    }

    // --- double on_cancel panics (debug builds only) ---

    fn test_on_cancel_twice_panics<T: CancelToken + Clone + Default>() {
        let parent = T::default();
        parent.on_cancel(Box::new(|| {}));
        parent.on_cancel(Box::new(|| {})); // should panic in debug builds
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "a callback has already been registered")
    )]
    fn simple_on_cancel_twice_panics() {
        test_on_cancel_twice_panics::<SimpleCancelToken>();
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "a callback has already been registered")
    )]
    fn tokio_on_cancel_twice_panics() {
        test_on_cancel_twice_panics::<TokioCancelToken>();
    }
}
