// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::WString;
use mssf_core::runtime::Runtime;
use mssf_util::tokio::TokioExecutor;
use tracing::info;

mod factory;
mod instance;

use factory::SelfReconfigFactory;

/// Service type name, matching the `<SelfReconfiguringServiceType>` declared in
/// the service manifest.
const SERVICE_TYPE_NAME: &str = "DummySelfReconfigServiceType";

fn main() -> mssf_core::Result<()> {
    tracing_subscriber::fmt().init();
    info!("dummy-self-reconfig host start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let executor = TokioExecutor::new(rt.handle().clone());
    let runtime = Runtime::create(executor.clone())?;

    runtime.register_self_reconfiguring_service_factory(
        &WString::from(SERVICE_TYPE_NAME),
        Box::new(SelfReconfigFactory),
    )?;
    info!("registered self-reconfiguring service factory for {SERVICE_TYPE_NAME}");

    // Keep the host process alive until Ctrl+C / SIGTERM.
    executor.block_until_ctrlc();
    info!("dummy-self-reconfig host exiting");
    Ok(())
}
