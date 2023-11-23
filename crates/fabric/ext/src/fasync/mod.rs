// ------------------------------------------------------------
// Copyright 2022 Youyuan Wu
// Licensed under the MIT License (MIT). See License.txt in the repo root for
// license information.
// ------------------------------------------------------------

// this contains some experiments for async
#![allow(non_snake_case)]

use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use fabric_base::{
    FabricCommon::{
        FabricClient::{FabricCreateLocalClient, IFabricGetNodeListResult, IFabricQueryClient},
        IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
        IFabricAsyncOperationContext,
    },
    FABRIC_NODE_QUERY_DESCRIPTION,
};
use tokio::sync::oneshot::{error::RecvError, Receiver, Sender};
use windows::core::{implement, Interface};
//use windows_core::com_interface::ComInterface;
use windows::core::HSTRING;
use windows_core::ComInterface;

// fabric code begins here

// This is implement a call back the supports rust .await syntax
#[implement(IFabricAsyncOperationCallback)]
pub struct AwaitableCallback {
    tx: Cell<Option<Sender<()>>>,
}

impl AwaitableCallback {
    pub fn channel() -> (AwaitableToken, IFabricAsyncOperationCallback) {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let callback: AwaitableCallback = AwaitableCallback::new(tx);
        let token = AwaitableToken::new(rx);
        let i_callback: IFabricAsyncOperationCallback = callback.into();
        (token, i_callback)
    }

    fn new(tx: tokio::sync::oneshot::Sender<()>) -> AwaitableCallback {
        AwaitableCallback {
            tx: Cell::new(Some(tx)),
        }
    }
}

impl IFabricAsyncOperationCallback_Impl for AwaitableCallback {
    // notify the function has been invoked.
    fn Invoke(&self, _context: ::core::option::Option<&IFabricAsyncOperationContext>) {
        let tx = self.tx.take();
        let txx = tx.expect("tx is empty"); // This means invoke is called twice.
        txx.send(()).expect("fail to send");
    }
}

#[repr(C)]
pub struct AwaitableToken {
    rx: tokio::sync::oneshot::Receiver<()>,
}

impl AwaitableToken {
    fn new(rx: tokio::sync::oneshot::Receiver<()>) -> AwaitableToken {
        AwaitableToken { rx }
    }
}

impl Future for AwaitableToken {
    type Output = Result<(), RecvError>;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to receive the value from the sender
        <Receiver<()> as Future>::poll(Pin::new(&mut self.rx), _cx)
    }
}

macro_rules! beginmyclient {
    ($name: ident) => {
        paste::item! {
        pub struct $name {
            c_: SBox<fabric_base::FabricCommon::FabricClient::[<I $name>]>,
        }
        }

        // both are needed. But should be safe because COM ptr always lives on heap.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> $name {
                return $name {
                    c_: SBox::new(unsafe {
                        paste::item! {
                            fabric_base::FabricCommon::FabricClient::[<I $name>]::from_raw(
                            FabricCreateLocalClient(&fabric_base::FabricCommon::FabricClient::[<I $name>]::IID)
                                .expect("cannot get localclient"),
                        )
                        }
                    }),
                };
            }
        } // impl
    };
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
                let (token_inner,callback) = AwaitableCallback::channel();
                // make token accessible outside
                token = token_inner;

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
            token.await.expect("wait failed");

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
        SBox { b: Box::new(x) }
    }
}

beginmyclient!(FabricHealthClient);

impl FabricHealthClient {
    myasyncfunc!(
        get_cluster_health,
        GetClusterHealth,
        fabric_base::FABRIC_CLUSTER_HEALTH_POLICY,
        fabric_base::FabricCommon::FabricClient::IFabricClusterHealthResult,
    );
    // get node health does not work because it requires node id as additional argument
    myasyncfunc!(
        get_node_health,
        GetNodeHealth,
        fabric_base::FABRIC_CLUSTER_HEALTH_POLICY,
        fabric_base::FabricCommon::FabricClient::IFabricNodeHealthResult,
        HSTRING
    );
    // the u16 is likely wrong. Maybe need to write a url type and convert to const ptr.
    myasyncfunc!(
        get_application_health,
        GetApplicationHealth,
        fabric_base::FABRIC_APPLICATION_HEALTH_POLICY,
        fabric_base::FabricCommon::FabricClient::IFabricApplicationHealthResult,
        u16 // applicationName
    );
    myasyncfunc!(
        get_service_health,
        GetServiceHealth,
        fabric_base::FABRIC_APPLICATION_HEALTH_POLICY,
        fabric_base::FabricCommon::FabricClient::IFabricServiceHealthResult,
        u16 // serviceName
    );
}

pub struct FabricQueryClient {
    c_: SBox<IFabricQueryClient>,
}

// both are needed. But should be safe because COM ptr always lives on heap.
unsafe impl Send for FabricQueryClient {}
unsafe impl Sync for FabricQueryClient {}

impl Default for FabricQueryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricQueryClient {
    pub fn new() -> FabricQueryClient {
        FabricQueryClient {
            c_: SBox::new(unsafe {
                IFabricQueryClient::from_raw(
                    FabricCreateLocalClient(&IFabricQueryClient::IID)
                        .expect("cannot get localclient"),
                )
            }),
        }
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
        fabric_base::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        fabric_base::FabricCommon::FabricClient::IFabricGetApplicationTypeListResult,
    );

    myasyncfunc!(
        get_service_type_list,
        GetServiceTypeList,
        fabric_base::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
        fabric_base::FabricCommon::FabricClient::IFabricGetServiceTypeListResult,
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
            let (token_inner, callback) = AwaitableCallback::channel();
            // make token accessible outside
            token = token_inner;

            {
                ctx = SBox::new(unsafe {
                    self.c_.b.BeginGetNodeList(p.b.as_ref(), 1000, &callback)?
                });
            }
        }
        // await for async operation.
        token.await.expect("wait failed");
        unsafe { self.c_.b.EndGetNodeList(&(*ctx.b)) }
    }
}

#[cfg(test)]
mod tests {

    use fabric_base::{
        FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION, FABRIC_CLUSTER_HEALTH_POLICY,
        FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_RESULT_ITEM,
    };

    use crate::fasync::{FabricQueryClient, SBox};

    use super::FabricHealthClient;

    async fn get_node(id: i32) {
        println!("id {}: GetNodeCli", id);

        let c = FabricQueryClient::new();

        let querydescription = SBox::new(FABRIC_NODE_QUERY_DESCRIPTION::default());

        let result = c.get_node_list_example(querydescription).await;

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
            let _list = unsafe { app_types.get_ApplicationTypeList() };
            // assert_eq!(unsafe { (*list).Count }, 0);
        }

        // get health state
        let h = FabricHealthClient::new();
        {
            let q = SBox::new(FABRIC_CLUSTER_HEALTH_POLICY::default());
            let result = h.get_cluster_health(q).await;
            let health = result.expect("cannto get health");
            let health_ptr = unsafe { health.get_ClusterHealth() };
            let _state = unsafe { (*health_ptr).AggregatedHealthState };
            // assert_eq!(FABRIC_HEALTH_STATE_OK, state);
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
