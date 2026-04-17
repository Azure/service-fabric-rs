// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::grpc::ReplicaRegistry;
use crate::statefulstore::Factory;
use mssf_core::WString;
use mssf_core::runtime::CodePackageActivationContext;
use mssf_util::tokio::TokioExecutor;
use tokio_util::sync::CancellationToken;
use tracing::info;

mod echo;
mod grpc;
mod statefulstore;

#[cfg(test)]
mod test;

#[cfg(test)]
mod test2;

const SERVICE_TYPE_NAME: &str = "ReflectionAppService";

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

    // Bind gRPC listener on localhost port 0 to let OS assign a port
    let grpc_bind_addr: std::net::SocketAddr = ([127, 0, 0, 1], 0).into();
    let std_listener =
        std::net::TcpListener::bind(grpc_bind_addr).expect("failed to bind gRPC listener");
    std_listener
        .set_nonblocking(true)
        .expect("failed to set non-blocking");
    let grpc_local_addr = std_listener.local_addr().expect("failed to get local addr");
    info!("gRPC server listening on {}", grpc_local_addr);

    // Shared state between gRPC and Service Fabric
    let registry = ReplicaRegistry::new();

    // Start the gRPC hello world server
    let token = CancellationToken::new();
    let grpc_token = token.clone();
    let grpc_registry = registry.clone();
    let grpc_handle = rt.spawn(async move {
        let tokio_listener = tokio::net::TcpListener::from_std(std_listener)
            .expect("failed to convert to tokio listener");
        let incoming = tonic::transport::server::TcpIncoming::from(tokio_listener);
        tonic::transport::Server::builder()
            .add_service(grpc::greeter_server(grpc_registry))
            .serve_with_incoming_shutdown(incoming, async move {
                grpc_token.cancelled().await;
            })
            .await
            .expect("gRPC server failed");
    });
    info!("gRPC server listening on {}", grpc_local_addr);

    let factory = Box::new(Factory::create(
        endpoint.port,
        hostname,
        e.clone(),
        grpc_local_addr,
        registry,
    ));
    runtime
        .register_stateful_service_factory(&WString::from(SERVICE_TYPE_NAME), factory)
        .unwrap();

    e.block_until_ctrlc();
    token.cancel();
    rt.block_on(grpc_handle).expect("gRPC task panicked");
    Ok(())
}

fn get_hostname() -> mssf_core::Result<WString> {
    let node_ctx = mssf_core::runtime::node_context::NodeContext::get_sync()?;
    Ok(node_ctx.ip_address_or_fqdn)
}
