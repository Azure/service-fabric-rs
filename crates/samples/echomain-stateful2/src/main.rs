// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::statefulstore::Factory;
use mssf_com::{
    FabricCommon::IFabricAsyncOperationCallback,
    FabricRuntime::{FabricBeginGetNodeContext, FabricEndGetNodeContext, IFabricNodeContextResult},
};
use mssf_core::{
    runtime::{
        executor::{DefaultExecutor, Executor},
        ActivationContext,
    },
    sync::wait::WaitableCallback,
};
use tracing::info;
use windows::core::Interface;
use windows_core::HSTRING;

mod statefulstore;
// Disable test for Linux ci for now due to SF app problem
#[cfg(target_os = "windows")]
#[cfg(test)]
mod test;

fn main() -> windows::core::Result<()> {
    tracing_subscriber::fmt().init();
    info!("main start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());
    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let actctx = ActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&HSTRING::from("KvReplicatorEndpoint"))
        .unwrap();
    let hostname = get_hostname();

    let factory = Factory::create(endpoint.port, hostname, e.clone());
    runtime
        .register_stateful_service_factory(&HSTRING::from("StatefulEchoAppService"), factory)
        .unwrap();

    e.run_until_ctrl_c();
    Ok(())
}

fn get_hostname() -> HSTRING {
    let (token, callback) = WaitableCallback::channel();

    let callback_arg = callback
        .cast::<IFabricAsyncOperationCallback>()
        .expect("castfailed");
    let ctx = unsafe { FabricBeginGetNodeContext(1000, &callback_arg).expect("getctx failed") };

    token.wait();

    let result_raw = unsafe { FabricEndGetNodeContext(&ctx).expect("end failed") };
    let result = unsafe { IFabricNodeContextResult::from_raw(result_raw) };
    let node_ctx = unsafe { result.get_NodeContext() };
    let hostname_raw = unsafe { (*node_ctx).IPAddressOrFQDN };
    let ret = HSTRING::from_wide(unsafe { hostname_raw.as_wide() }).expect("hstring");
    info!("got hostname: {:?}", ret);
    ret
}
