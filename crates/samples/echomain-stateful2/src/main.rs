// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::statefulstore::Factory;
use mssf_core::WString;
use mssf_core::runtime::CodePackageActivationContext;
use mssf_util::tokio::TokioExecutor;
use tracing::info;

mod echo;
mod statefulstore;

#[cfg(test)]
mod test;

#[cfg(test)]
mod test2;

fn main() -> mssf_core::Result<()> {
    tracing_subscriber::fmt().init();
    info!("main start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = TokioExecutor::new(rt.handle().clone());
    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let actctx = CodePackageActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&WString::from("KvReplicatorEndpoint"))
        .unwrap();
    let hostname = get_hostname().expect("cannot get hostname");

    let factory = Factory::create(endpoint.port, hostname, e.clone());
    runtime
        .register_stateful_service_factory(&WString::from("StatefulEchoAppService"), factory)
        .unwrap();

    e.block_until_ctrlc();
    Ok(())
}

fn get_hostname() -> mssf_core::Result<WString> {
    let node_ctx = mssf_core::runtime::node_context::NodeContext::get_sync()?;
    Ok(node_ctx.ip_address_or_fqdn)
}
