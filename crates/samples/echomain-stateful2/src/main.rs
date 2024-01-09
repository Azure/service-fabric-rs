use fabric_rs::runtime::{
        executor::{DefaultExecutor, Executor},
        ActivationContext,
    };
use log::info;
use windows_core::HSTRING;

use crate::kvstore::Factory;

mod kvstore;

fn main() -> windows::core::Result<()> {
    env_logger::init();
    info!("main start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());
    let runtime = fabric_rs::runtime::Runtime::create(e.clone()).unwrap();
    let actctx = ActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&HSTRING::from("KvReplicatorEndpoint"))
        .unwrap();

    let factory = Factory::create(endpoint.Port, e.clone());
    runtime
        .register_stateful_service_factory(&HSTRING::from("StatefulEchoAppService"), factory)
        .unwrap();

    e.run_until_ctrl_c();
    Ok(())
}
