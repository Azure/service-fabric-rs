use tokio::sync::mpsc::channel;

use fabric_rs::runtime::Runtime;
use log::info;
use windows_core::HSTRING;

mod echo;

fn main() -> windows::core::Result<()> {
    env_logger::init();
    // set ctrc event
    let (tx, mut rx) = channel(1);
    let handler = move || {
        tx.blocking_send(())
            .expect("Could not send signal on channel.")
    };
    ctrlc::set_handler(handler).expect("Error setting Ctrl-C handler");

    info!("echomain start");
    // hack to wait for debugger
    // std::thread::sleep(std::time::Duration::from_secs(90));
    // info!("sleep ended");

    let rt = tokio::runtime::Runtime::new().unwrap();

    let runtime = Runtime::create(rt.handle().clone()).unwrap();
    let factory = Box::<echo::Factory>::default();
    runtime
        .register_stateless_service_factory(&HSTRING::from("EchoAppService2"), factory)
        .unwrap();

    // let activation_ctx = get_com_activation_context().expect("Cannot get activation ctx");

    //run_app(&runtime, &activation_ctx);

    // wait for ctrl-c signal.
    rt.block_on(async move {
        info!("Waiting for Ctrl-C...");
        rx.recv().await.expect("Could not receive from channel.");
        info!("Got it! Exiting...");
    });
    Ok(())
}
