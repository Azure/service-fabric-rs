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

    let rt = tokio::runtime::Runtime::new().unwrap();

    let runtime = Runtime::create(rt.handle().clone()).unwrap();
    let factory = Box::<echo::Factory>::default();
    runtime
        .register_stateless_service_factory(&HSTRING::from("EchoAppService2"), factory)
        .unwrap();

    // wait for ctrl-c signal.
    rt.block_on(async move {
        info!("Waiting for Ctrl-C...");
        rx.recv().await.expect("Could not receive from channel.");
        info!("Got it! Exiting...");
    });
    Ok(())
}
