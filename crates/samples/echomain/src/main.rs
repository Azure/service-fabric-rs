// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// This example app shows how to use SF safe API (mssf_core)
// to create a SF stateless application.

use std::sync::Arc;

use app::AppContext;
use mssf_core::WString;
use mssf_core::conf::{Config, FabricConfigSource};
use mssf_core::debug::wait_for_debugger;
use mssf_core::runtime::CodePackageActivationContext;
use mssf_core::runtime::config::ConfigurationPackage;
use mssf_core::runtime::node_context::NodeContext;
use mssf_core::runtime::package_change::PackageChangeEvent;
use mssf_core::types::{HealthInformation, HealthReportSendOption};
use mssf_util::tokio::TokioExecutor;
use tracing::{error, info};

use crate::config::MySettings;
mod app;
pub mod config;
pub mod echo;
mod service_factory;
mod service_instance;
// Disable test for Linux ci for now due to SF app problem
#[cfg(target_os = "windows")]
#[cfg(test)]
mod test;

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
    let actctx = CodePackageActivationContext::create().inspect_err(|e| {
        error!("Fail to create activation context: {e}");
    })?;

    // send an health report
    send_health_report(&actctx);

    validate_configs(&actctx);

    let code_info = actctx.get_code_package_info();
    info!("code package info: {:?}", code_info);

    // inspect and check the code package
    let names = actctx.get_code_package_names();
    assert_eq!(names.len(), 1);
    for name in names {
        info!("code package {name}: {:?}", actctx.get_code_package(&name));
        assert_eq!(name.to_string_lossy(), "Code");
    }

    // inspect and check the config package
    let config_package_names = actctx.get_config_package_names();
    assert_eq!(config_package_names.len(), 1);
    for name in config_package_names {
        info!("config package {name}");
        assert_eq!(name.to_string_lossy(), "Config");
    }

    // inspect and check the data package(s). Currently zero, which is completely valid
    let data_package_names = actctx.get_data_package_names();
    assert_eq!(data_package_names.len(), 0);

    // get listening port
    let endpoint = actctx
        .get_endpoint_resource(&WString::from("ServiceEndpoint1"))
        .unwrap();
    info!("Get ServiceEndpoint1: {:?}", endpoint);
    let port = endpoint.port;

    // get hostname
    let ctx = NodeContext::get_sync().unwrap();
    info!("NodeContext: {:?}", ctx);
    let hostname = ctx.ip_address_or_fqdn;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = TokioExecutor::new(rt.handle().clone());

    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let app_ctx = AppContext::new(port, hostname, e.clone());
    let factory = service_factory::ServiceFactory::new(Arc::new(app_ctx));
    runtime
        .register_stateless_service_factory(&WString::from("EchoAppService"), factory)
        .unwrap();

    e.block_until_ctrlc();
    Ok(())
}

// validates the configs in the config package have the right values.
fn validate_configs(actctx: &CodePackageActivationContext) {
    // loop and print all configs
    let config = actctx
        .get_configuration_package(&WString::from("Config"))
        .unwrap();
    let s = build_config(config);
    let val = s.get::<String>("my_config_section.my_string").unwrap();
    info!("entry: {}", val);
    // note that the config name lookup is case sensitive for struct fields.
    let settings = s.try_deserialize::<MySettings>().unwrap();
    info!("settings: {:?}", settings);
    let sect = settings.my_config_section;
    assert_eq!(sect.my_string, "Value1");
    assert!(sect.my_bool);
    assert_eq!(sect.my_int, 99);
    actctx.register_configuration_package_change_handler(|change| {
        let (some_package, change_type, validate_new) = match change
            {
                PackageChangeEvent::Addition { new_package } => (new_package, "Addition", true),
                PackageChangeEvent::Removal { previous_package } => (previous_package, "Removal", false),
                PackageChangeEvent::Modification { previous_package: _, new_package } => (new_package, "Modification", true),
            };
            let changed_package_name = some_package.get_description().name.to_string_lossy();
            let changed_package_str = &changed_package_name;
            info!("Received config package change of type {change_type:?} to package {changed_package_str}");
            if validate_new
            {
                // This is a bit hacky, but if there was a removal, not much point in validating the old package
                // In an application that actually uses its config settings, we'd probably put the result of this into a OnceLock<RwLock<Config>> 
                // or something more complicated, like a ArcSwap<Config> or similar
                build_config(some_package.clone());
            }
    }).unwrap();
}

fn build_config(config: ConfigurationPackage) -> Config {
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
            &WString::from("my_config_section"),
            &WString::from("my_string"),
        )
        .unwrap();
    assert_eq!(v.to_string_lossy(), "Value1");
    assert!(!encrypt);

    // TODO: add a overrideable parameter in the manifest / settings and log it here

    // Use the config framework
    let source = FabricConfigSource::new(config);

    Config::builder()
        .add_source(source)
        .build()
        .inspect_err(|e| info!("config build failed: {}", e))
        .unwrap()
}

/// Send health ok to SF to validate health reporting code
fn send_health_report(actctx: &CodePackageActivationContext) {
    let healthinfo = HealthInformation {
        source_id: WString::from("echoapp"),
        property: WString::from("echo-started"),
        time_to_live_seconds: 300,
        state: mssf_core::types::HealthState::Ok,
        description: WString::from("echo app started"),
        sequence_number: 1,
        remove_when_expired: true,
    };
    if let Err(e) = actctx.report_application_health(
        &healthinfo,
        Some(&HealthReportSendOption { immediate: true }),
    ) {
        error!("report application health failed: {:?}", e);
    }
}
