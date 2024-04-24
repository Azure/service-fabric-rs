// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use log::info;
use mssf_core::{
    conf::{Config, FabricConfigSource},
    runtime::{
        executor::{DefaultExecutor, Executor},
        ActivationContext, Runtime,
    },
};
use windows_core::HSTRING;

use crate::config::MySettings;

mod config;
mod echo;

fn main() -> windows::core::Result<()> {
    env_logger::init();

    info!("echomain start");
    let actctx = ActivationContext::create().unwrap();
    validate_configs(&actctx);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());

    let runtime = Runtime::create(e.clone()).unwrap();
    let factory = echo::Factory::default();
    runtime
        .register_stateless_service_factory(&HSTRING::from("EchoAppService2"), factory)
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
    settings
        .sections
        .iter()
        .enumerate()
        .for_each(|(_, section)| {
            info!("Section: {}", section.name);
            section
                .parameters
                .iter()
                .enumerate()
                .for_each(|(_, p)| info!("Param: {:?}", p))
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
