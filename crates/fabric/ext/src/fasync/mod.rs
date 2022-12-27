// this contains some experiments for async

#![allow(improper_ctypes_definitions)] // for AwaitableToken. TODO: remove this
#![allow(non_snake_case)]

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use service_fabric_rs::{
    FabricCommon::{
        FabricClient::{FabricCreateLocalClient, IFabricGetNodeListResult, IFabricQueryClient},
        IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
        IFabricAsyncOperationCallback_Vtbl, IFabricAsyncOperationContext,
    },
    FABRIC_NODE_QUERY_DESCRIPTION,
};
use windows::core::{implement, Interface, Vtable};

use windows::core::HSTRING;

/// Shared state between the future and the waiting thread
#[derive(Debug)]
#[allow(improper_ctypes_definitions)]
pub struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

// fabric code begins here

#[windows::core::interface("a9445a72-838b-4ed3-8073-bb6423198241")]
pub unsafe trait IFabricAwaitableCallback: IFabricAsyncOperationCallback {
    // This has warning
    pub unsafe fn get_token(&self) -> AwaitableToken;
}

// This is implement a call back the supports rust .await syntax
#[derive(Debug)]
#[implement(IFabricAsyncOperationCallback, IFabricAwaitableCallback)]
pub struct AwaitableCallback {
    shared_state: Arc<Mutex<SharedState>>,
}

impl AwaitableCallback {
    pub fn new() -> AwaitableCallback {
        return AwaitableCallback {
            shared_state: Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None,
            })),
        };
    }
}

impl IFabricAsyncOperationCallback_Impl for AwaitableCallback {
    // notify the function has been invoked.
    fn Invoke(&self, _context: &core::option::Option<IFabricAsyncOperationContext>) {
        let mut shared_state = self.shared_state.lock().unwrap();
        // Signal that the timer has completed and wake up the last
        // task on which the future was polled, if one exists.
        shared_state.completed = true;
        if let Some(waker) = shared_state.waker.take() {
            waker.wake()
        }
    }
}

impl IFabricAwaitableCallback_Impl for AwaitableCallback {
    unsafe fn get_token(&self) -> AwaitableToken {
        return AwaitableToken::new(self.shared_state.clone());
    }
}

#[repr(C)]
pub struct AwaitableToken {
    shared_state: Arc<Mutex<SharedState>>,
}

impl AwaitableToken {
    pub fn new(state: Arc<Mutex<SharedState>>) -> AwaitableToken {
        return AwaitableToken {
            shared_state: state,
        };
    }
}

impl Future for AwaitableToken {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

macro_rules! beginmyclient {
    ($name: ident) => {
        paste::item! {
        pub struct $name {
            c_: SBox<service_fabric_rs::FabricCommon::FabricClient::[<I $name>]>,
        }
        }

        // both are needed. But should be safe because COM ptr always lives on heap.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            pub fn new() -> $name {
                return $name {
                    c_: SBox::new(unsafe {
                        paste::item! {
                            service_fabric_rs::FabricCommon::FabricClient::[<I $name>]::from_raw(
                            FabricCreateLocalClient(&service_fabric_rs::FabricCommon::FabricClient::[<I $name>]::IID)
                                .expect("cannot get localclient"),
                        )
                        }
                    }),
                };
            }
        } // impl
    }
}

// macros for impl async fn
macro_rules! myasyncfunc {
    ($fn_name: ident, $inner_name: ident, $param: ty, $res : ty, $( $param_opt:ty ),*) => {
    paste::item! {
        pub async fn $fn_name(&self, p: SBox<$param>
        // optional params
        $(
            , [<$param_opt _name>]: SBox<$param_opt>
        )*

        ) -> ::windows::core::Result<$res> {
            let ctx: SBox<IFabricAsyncOperationContext>;
            let token: AwaitableToken;

            {
                let callback: IFabricAwaitableCallback = AwaitableCallback::new().into();

                token = unsafe { callback.get_token() };

                {
                    let callback_arg: IFabricAsyncOperationCallback =
                        callback.cast().expect("castfailed");

                    paste::item! {
                    ctx = SBox::new(unsafe {
                        self.c_
                            .b
                        .[<Begin $inner_name>](
                            $(
                                [<$param_opt _name>].b.as_ref(),
                            )*
                            p.b.as_ref(), 1000, &callback_arg)?
                    });
                    }
                }
            }

            // await for async operation.
            token.await;

            paste::item! {
            unsafe { self.c_.b.[<End $inner_name>](&(*ctx.b)) }
            }
        }
    }
    };
}

// Send Box. Wrap a type and implement send.
// c pointers are not send in rust, so this forces it.
pub struct SBox<T> {
    pub b: Box<T>,
}

// We know that T is send. This requires programmer's check of the internals.
unsafe impl<T> Send for SBox<T> {}

impl<T> SBox<T> {
    pub fn new(x: T) -> SBox<T> {
        return SBox { b: Box::new(x) };
    }
}

beginmyclient!(FabricHealthClient);

impl FabricHealthClient {
    myasyncfunc!(
        get_cluster_health,
        GetClusterHealth,
        service_fabric_rs::FABRIC_CLUSTER_HEALTH_POLICY,
        service_fabric_rs::FabricCommon::FabricClient::IFabricClusterHealthResult,
    );
    // get node health does not work because it requires node id as additional argument
    myasyncfunc!(
        get_node_health,
        GetNodeHealth,
        service_fabric_rs::FABRIC_CLUSTER_HEALTH_POLICY,
        service_fabric_rs::FabricCommon::FabricClient::IFabricNodeHealthResult,
        HSTRING
    );
    // the u16 is likely wrong. Maybe need to write a url type and convert to const ptr.
    myasyncfunc!(
        get_application_health,
        GetApplicationHealth,
        service_fabric_rs::FABRIC_APPLICATION_HEALTH_POLICY,
        service_fabric_rs::FabricCommon::FabricClient::IFabricApplicationHealthResult,
        u16 // applicationName
    );
    myasyncfunc!(
        get_service_health,
        GetServiceHealth,
        service_fabric_rs::FABRIC_APPLICATION_HEALTH_POLICY,
        service_fabric_rs::FabricCommon::FabricClient::IFabricServiceHealthResult,
        u16 // serviceName
    );
}

pub struct FabricQueryClient {
    c_: SBox<IFabricQueryClient>,
}

// both are needed. But should be safe because COM ptr always lives on heap.
unsafe impl Send for FabricQueryClient {}
unsafe impl Sync for FabricQueryClient {}

impl FabricQueryClient {
    pub fn new() -> FabricQueryClient {
        return FabricQueryClient {
            c_: SBox::new(unsafe {
                IFabricQueryClient::from_raw(
                    FabricCreateLocalClient(&IFabricQueryClient::IID)
                        .expect("cannot get localclient"),
                )
            }),
        };
    }

    myasyncfunc!(
        get_node_list,
        GetNodeList,
        FABRIC_NODE_QUERY_DESCRIPTION,
        IFabricGetNodeListResult,
    );

    myasyncfunc!(
        get_application_type_list,
        GetApplicationTypeList,
        service_fabric_rs::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        service_fabric_rs::FabricCommon::FabricClient::IFabricGetApplicationTypeListResult,
    );

    myasyncfunc!(
        get_service_type_list,
        GetServiceTypeList,
        service_fabric_rs::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
        service_fabric_rs::FabricCommon::FabricClient::IFabricGetServiceTypeListResult,
    );
    // example of not using macro.
    // param is SBox because it crosses await boundary.
    pub async fn get_node_list_example(
        &self,
        p: SBox<FABRIC_NODE_QUERY_DESCRIPTION>,
    ) -> ::windows::core::Result<IFabricGetNodeListResult> {
        let ctx: SBox<IFabricAsyncOperationContext>;
        let token: AwaitableToken;

        {
            let callback: IFabricAwaitableCallback = AwaitableCallback::new().into();

            token = unsafe { callback.get_token() };

            {
                let callback_arg: IFabricAsyncOperationCallback =
                    callback.cast().expect("castfailed");

                ctx = SBox::new(unsafe {
                    self.c_
                        .b
                        .BeginGetNodeList(p.b.as_ref(), 1000, &callback_arg)?
                });
            }
        }

        // await for async operation.
        token.await;

        unsafe { self.c_.b.EndGetNodeList(&(*ctx.b)) }
    }
}

#[cfg(test)]
mod tests {

    use service_fabric_rs::{
        FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION, FABRIC_CLUSTER_HEALTH_POLICY,
        FABRIC_HEALTH_STATE_OK, FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_RESULT_ITEM,
    };

    use crate::fasync::{FabricQueryClient, SBox};

    use super::FabricHealthClient;

    async fn get_node(id: i32) {
        println!("id {}: GetNodeCli", id);

        let c = FabricQueryClient::new();

        let querydescription = SBox::new(FABRIC_NODE_QUERY_DESCRIPTION::default());

        let result = c.get_node_list(querydescription).await;

        assert!(!result.is_err());

        let result_node_list = result.expect("endcall_failed");

        let nodes = unsafe { result_node_list.get_NodeList() };
        let node_count = unsafe { (*nodes).Count };
        let node_list = unsafe { (*nodes).Items };

        println!("id {}: node_count {}", id, node_count);

        if !node_list.is_null() {
            let node: FABRIC_NODE_QUERY_RESULT_ITEM = unsafe { *node_list };
            println!("id {}: node info: name: {}", id, unsafe {
                node.NodeName.display()
            });
        }
    }

    async fn get_stuff() {
        // do get applicationtype
        let c = FabricQueryClient::new();
        {
            let query_description = SBox::new(FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION::default());
            let result = c.get_application_type_list(query_description).await;
            let app_types = result.expect("cannot get types");
            let list = unsafe { app_types.get_ApplicationTypeList() };
            assert_eq!(unsafe { (*list).Count }, 0);
        }

        // get health state
        let h = FabricHealthClient::new();
        {
            let q = SBox::new(FABRIC_CLUSTER_HEALTH_POLICY::default());
            let result = h.get_cluster_health(q).await;
            let health = result.expect("cannto get health");
            let health_ptr = unsafe { health.get_ClusterHealth() };
            let state = unsafe { (*health_ptr).AggregatedHealthState };
            assert_eq!(FABRIC_HEALTH_STATE_OK, state);
        }
    }

    // requires local cluster to be running.
    #[test]
    fn fabricclient_test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let handle = tokio::spawn(async {
                get_node(1).await;
            });

            let handle2 = tokio::spawn(async {
                get_node(2).await;
            });

            handle.await.expect("handle wait");
            handle2.await.expect("handle2 wait");

            get_stuff().await;
        })
    }
}
