use fabric_rs::{debug::wait_for_debugger, runtime::ActivationContext};
use log::info;
use tokio::sync::mpsc::channel;
use windows_core::HSTRING;

use crate::kvstore::Factory;

mod kvstore;

fn has_debug_arg() -> bool {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == "-WaitForDebugger" {
            return true;
        }
    }
    false
}

fn main() -> windows::core::Result<()> {
    env_logger::init();
    info!("main start");
    if has_debug_arg() {
        wait_for_debugger();
    }

    // set ctrc event
    let (tx, mut rx) = channel(1);
    let handler = move || {
        tx.blocking_send(())
            .expect("Could not send signal on channel.")
    };
    ctrlc::set_handler(handler).expect("Error setting Ctrl-C handler");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let runtime = fabric_rs::runtime::Runtime::create(rt.handle().clone()).unwrap();

    let actctx = ActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&HSTRING::from("KvReplicatorEndpoint"))
        .unwrap();

    let factory = Box::new(Factory::create(endpoint.Port, rt.handle().clone()));
    runtime
        .register_stateful_service_factory(&HSTRING::from("KvStoreService"), factory)
        .unwrap();

    // wait for ctrl-c signal.
    rt.block_on(async move {
        info!("Waiting for Ctrl-C...");
        rx.recv().await.expect("Could not receive from channel.");
        info!("Got it! Exiting...");
    });
    Ok(())
}
