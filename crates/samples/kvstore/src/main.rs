use mssf_core::HSTRING;
use mssf_core::{
    debug::wait_for_debugger,
    runtime::{
        executor::{DefaultExecutor, Executor},
        ActivationContext,
    },
};
use tracing::info;

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

fn main() -> mssf_core::Result<()> {
    tracing_subscriber::fmt().init();
    info!("main start");
    if has_debug_arg() {
        wait_for_debugger();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());
    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let actctx = ActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&HSTRING::from("KvReplicatorEndpoint"))
        .unwrap();

    let factory = Factory::create(endpoint.port, e.clone());
    runtime
        .register_stateful_service_factory(&HSTRING::from("KvStoreService"), factory)
        .unwrap();

    e.run_until_ctrl_c();
    Ok(())
}
