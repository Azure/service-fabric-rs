// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Compile guard for the examples in `README.md`.
//!
//! These functions are never executed — the point is that they *type-check*, so
//! the README cannot drift away from the real API without CI noticing. Keep
//! them in sync with the README, and prefer changing the API over contorting
//! the example: writing this file is what surfaced that `ServerHandle::shutdown`
//! originally returned a `Box<dyn Error + Send + Sync>`, which does not convert
//! via `?` into the `Box<dyn Error>` most callers return.

use std::sync::Arc;
use std::time::Duration;

use mssf_core::WString;
use mssf_net::endpoint::EndpointSource;
use mssf_net::{
    AddressError, AddressInterpreter, AdsService, EndpointSnapshot, FabricEndpointSource,
    FabricNaming, HostPort, ScriptedEndpointSource, ServiceRegistry, XdsMapping,
    host_port_interpreter,
};

/// README — "Hosting the mapping".
#[allow(dead_code)]
async fn readme_host_example() -> Result<(), Box<dyn std::error::Error>> {
    let mapping = XdsMapping::for_service_uri("fabric:/MyApp/MyService", host_port_interpreter())?;

    let source = FabricEndpointSource::new(
        &mapping,
        vec![WString::from("localhost:19000")],
        Duration::from_secs(10),
    )
    .await?;

    let server = AdsService::new(mapping, source.clone())
        .serve_on_ephemeral_loopback()
        .await?;

    println!("point GRPC_XDS_BOOTSTRAP at {}", server.addr());

    server.shutdown().await?;
    source.shutdown().await;
    Ok(())
}

/// README — "Serving several services from one control plane".
#[allow(dead_code)]
async fn readme_multi_service_example(
    orders_mapping: XdsMapping,
    inventory_mapping: XdsMapping,
) -> Result<(), Box<dyn std::error::Error>> {
    let naming = FabricNaming::new(vec![WString::from("localhost:19000")])?;

    let orders = naming
        .source_for(&orders_mapping, Duration::from_secs(10))
        .await?;
    let inventory = naming
        .source_for(&inventory_mapping, Duration::from_secs(10))
        .await?;

    let registry = ServiceRegistry::builder()
        .add(orders_mapping, orders.clone())?
        .add(inventory_mapping, inventory.clone())?
        .build()?;

    let server = AdsService::from_registry(registry)
        .serve_on_ephemeral_loopback()
        .await?;

    server.shutdown().await?;
    orders.shutdown().await;
    inventory.shutdown().await;
    naming.shutdown().await;
    Ok(())
}

/// README — "Endpoint addresses need an interpreter".
#[allow(dead_code)]
fn readme_interpreter_example() -> AddressInterpreter {
    Arc::new(|raw: &str| {
        let (host, port) = raw
            .rsplit_once(':')
            .ok_or_else(|| AddressError::Unparseable(raw.to_string()))?;
        let port: u16 = port
            .parse()
            .map_err(|_| AddressError::MissingPort(raw.to_string()))?;
        Ok(HostPort::new(host, port))
    })
}

/// README — "Testing without a cluster".
#[allow(dead_code)]
async fn readme_scripted_example(mapping: XdsMapping) -> Result<(), Box<dyn std::error::Error>> {
    let (source, handle) =
        ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new("127.0.0.1", 50051)));
    let server = AdsService::new(mapping, source)
        .serve_on_ephemeral_loopback()
        .await?;

    // Simulate a failover; a connected client follows it.
    handle.set(EndpointSnapshot::Primary(HostPort::new("127.0.0.1", 50052)));

    server.shutdown().await?;
    Ok(())
}

/// The value of this file is that the above compiles; there is nothing to run.
#[test]
fn readme_examples_typecheck() {}
