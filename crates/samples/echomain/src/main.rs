// ------------------------------------------------------------
// Copyright 2022 Youyuan Wu
// Licensed under the MIT License (MIT). See License.txt in the repo root for
// license information.
// ------------------------------------------------------------

use fabric_base::FabricCommon::FabricRuntime::{
    FabricBeginGetNodeContext, FabricCreateRuntime, FabricEndGetNodeContext,
    FabricGetActivationContext, IFabricCodePackageActivationContext, IFabricNodeContextResult,
    IFabricRuntime,
};
use fabric_base::FabricCommon::IFabricAsyncOperationCallback;
use fabric_ext::WaitableCallback;
use log::info;
use std::sync::mpsc::channel;
use windows::core::w;
use windows::core::{Interface, HSTRING};
use windows_core::ComInterface;
pub mod app;

fn main() -> windows::core::Result<()> {
    env_logger::init();
    // set ctrc event
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    info!("echomain start");
    // hack to wait for debugger
    // std::thread::sleep(std::time::Duration::from_secs(90));
    // info!("sleep ended");

    let rawruntime =
        unsafe { FabricCreateRuntime(&IFabricRuntime::IID).expect("cannot create runtime") };
    let runtime = unsafe { IFabricRuntime::from_raw(rawruntime) };

    let raw_activation_ctx = unsafe {
        FabricGetActivationContext(&IFabricCodePackageActivationContext::IID)
            .expect("Cannot get activation ctx")
    };

    let activation_ctx =
        unsafe { IFabricCodePackageActivationContext::from_raw(raw_activation_ctx) };

    run_app(&runtime, &activation_ctx);

    info!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    info!("Got it! Exiting...");
    Ok(())
}

fn run_app(runtime: &IFabricRuntime, activation_ctx: &IFabricCodePackageActivationContext) {
    let port = get_port(activation_ctx);
    let hostname = get_hostname();
    app::run(runtime, port, hostname);
}

fn get_port(activation_ctx: &IFabricCodePackageActivationContext) -> u32 {
    info!("trying to get port");
    let endpoint_name = w!("ServiceEndpoint1");
    let endpoint = unsafe {
        activation_ctx
            .GetServiceEndpointResource(endpoint_name)
            .expect("cannot get endpoint")
    };
    unsafe { (*endpoint).Port }
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
