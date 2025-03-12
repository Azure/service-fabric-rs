use mssf_core::WString;
use mssf_core::{
    debug::wait_for_debugger,
    runtime::{
        executor::{DefaultExecutor, Executor},
        CodePackageActivationContext,
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
    let actctx = CodePackageActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&WString::from("KvReplicatorEndpoint"))
        .unwrap();

    let factory = Factory::create(endpoint.port, e.clone());
    runtime
        .register_stateful_service_factory(&WString::from("KvStoreService"), factory)
        .unwrap();

    e.block_on(async {
        tokio::signal::ctrl_c().await.expect("fail to get ctrl-c");
    });
    Ok(())
}
