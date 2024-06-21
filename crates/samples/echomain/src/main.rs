// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// This example app shows how to use SF safe API (mssf_core)
// to create a SF stateless application.

use mssf_core::conf::{Config, FabricConfigSource};
use mssf_core::debug::wait_for_debugger;
use mssf_core::runtime::executor::{DefaultExecutor, Executor};
use mssf_core::runtime::node_context::NodeContext;
use mssf_core::runtime::ActivationContext;
use mssf_core::HSTRING;
use std::time::Duration;
use tracing::{error, info};

use crate::config::MySettings;
pub mod app;
pub mod config;
pub mod echo;

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
    info!("echomain start");
    if has_debug_arg() {
        wait_for_debugger();
    }
    let actctx = ActivationContext::create().inspect_err(|e| {
        error!("Fail to create activation context: {e}");
    })?;
    validate_configs(&actctx);

    // get listening port
    let endpoint = actctx
        .get_endpoint_resource(&HSTRING::from("ServiceEndpoint1"))
        .unwrap();
    info!("Get ServiceEndpoint1: {:?}", endpoint);
    let port = endpoint.Port;

    // get hostname
    let ctx = NodeContext::get_sync(Duration::from_secs(1)).unwrap();
    info!("NodeContext: {:?}", ctx);
    let hostname = ctx.ip_address_or_fqdn;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());

    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let factory = app::Factory::new(port, hostname, rt.handle().clone());
    runtime
        .register_stateless_service_factory(&HSTRING::from("EchoAppService"), factory)
        .unwrap();

    e.run_until_ctrl_c();
    Ok(())
}

// validates the configs in the config package have the right values.
fn validate_configs(actctx: &ActivationContext) {
    // loop and print all configs
    let config = actctx
        .get_configuration_package(&HSTRING::from("Config"))
        .unwrap();
    let settings = config.get_settings();
    settings.sections.iter().for_each(|section| {
        info!("Section: {}", section.name);
        section
            .parameters
            .iter()
            .for_each(|p| info!("Param: {:?}", p))
    });

    // get the required config
    let (v, encrypt) = config
        .get_value(
            &HSTRING::from("my_config_section"),
            &HSTRING::from("my_string"),
        )
        .unwrap();
    assert_eq!(v, "Value1");
    assert!(!encrypt);

    // Use the config framework
    let source = FabricConfigSource::new(config);
    let s = Config::builder()
        .add_source(source)
        .build()
        .inspect_err(|e| info!("config build failed: {}", e))
        .unwrap();
    let val = s.get::<String>("my_config_section.my_string").unwrap();
    info!("entry: {}", val);
    // note that the config name lookup is case sensitive for struct fields.
    let settings = s.try_deserialize::<MySettings>().unwrap();
    info!("settings: {:?}", settings);
    let sect = settings.my_config_section;
    assert_eq!(sect.my_string, "Value1");
    assert!(sect.my_bool);
    assert_eq!(sect.my_int, 99);
}
