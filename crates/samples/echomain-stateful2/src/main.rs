use fabric_base::FabricCommon::{IFabricAsyncOperationCallback, FabricRuntime::{FabricBeginGetNodeContext, FabricEndGetNodeContext, IFabricNodeContextResult, IFabricCodePackageActivationContext}};
use fabric_rs::{runtime::{
        executor::{DefaultExecutor, Executor},
        ActivationContext,
    }, WaitableCallback};
use log::info;
use windows_core::{HSTRING, w};
use windows_core::ComInterface;
use crate::statefulstore::Factory;
use windows::core::Interface;

mod statefulstore;

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
    let hostname = get_hostname();

    let factory = Factory::create(endpoint.Port, hostname, e.clone());
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

