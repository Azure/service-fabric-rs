// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// This example app shows how to use SF com API (unsafe)
// to create a SF stateless application.

use log::info;
use mssf_com::FabricCommon::FabricRuntime::IFabricRuntime;
use mssf_core::runtime::create_com_runtime;
use mssf_core::runtime::node_context::NodeContext;
use mssf_core::runtime::ActivationContext;
use std::sync::mpsc::channel;
use std::time::Duration;
use windows::core::HSTRING;
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

    let runtime = create_com_runtime().expect("cannot create runtime");

    let activation_ctx = ActivationContext::create().expect("Cannot get activation ctx");

    run_app(&runtime, &activation_ctx);

    info!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    info!("Got it! Exiting...");
    Ok(())
}

fn run_app(runtime: &IFabricRuntime, activation_ctx: &ActivationContext) {
    let port = get_port(activation_ctx);
    let hostname = get_hostname();
    app::run(runtime, port, hostname);
}

fn get_port(activation_ctx: &ActivationContext) -> u32 {
    info!("trying to get port");
    let endpoint = activation_ctx
        .get_endpoint_resource(&HSTRING::from("ServiceEndpoint1"))
        .unwrap();
    info!("Endpoint: {:?}", endpoint);
    endpoint.Port
}

fn get_hostname() -> HSTRING {
    let ctx = NodeContext::get_sync(Duration::from_secs(1)).unwrap();
    info!("NodeContext: {:?}", ctx);
    ctx.ip_address_or_fqdn
}
