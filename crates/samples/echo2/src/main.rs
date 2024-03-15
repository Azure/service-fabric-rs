// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use log::info;
use mssf_core::runtime::{
    executor::{DefaultExecutor, Executor},
    Runtime,
};
use windows_core::HSTRING;

mod echo;

fn main() -> windows::core::Result<()> {
    env_logger::init();

    info!("echomain start");

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
