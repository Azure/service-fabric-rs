// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::WString;
use mssf_core::runtime::CodePackageActivationContext;
use mssf_util::tokio::TokioExecutor;
use samples_reflection::SELF_RECONFIGURING_SERVICE_TYPE_NAME;
use samples_reflection::SERVICE_TYPE_NAME;
use samples_reflection::grpc;
use samples_reflection::grpc::ReplicaRegistry;
use samples_reflection::grpc_control::{control_port_for_node, replica_control_server};
use samples_reflection::self_reconfiguring::SelfReconfiguringFactory;
use samples_reflection::statefulstore::Factory;
use tokio_util::sync::CancellationToken;
use tracing::info;

mod log_fmt {
    //! Custom tracing-subscriber event formatter that prepends a
    //! constant prefix (e.g. `pid=<n> node=<name>`) to every log
    //! line. Runs at the subscriber layer, so it fires for every
    //! event regardless of which thread, task, or span produced it
    //! — unlike a root span, which relies on thread-local
    //! propagation and only shows up if the formatter walks the
    //! span context.

    use tracing_subscriber::fmt::{FormatEvent, FormatFields, format::Writer};

    pub struct HostFields<F> {
        pub prefix: String,
        pub inner: F,
    }

    impl<S, N, F> FormatEvent<S, N> for HostFields<F>
    where
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
        N: for<'a> FormatFields<'a> + 'static,
        F: FormatEvent<S, N>,
    {
        fn format_event(
            &self,
            ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
            mut writer: Writer<'_>,
            event: &tracing::Event<'_>,
        ) -> std::fmt::Result {
            write!(writer, "{} ", self.prefix)?;
            self.inner.format_event(ctx, writer, event)
        }
    }
}

fn main() -> mssf_core::Result<()> {
    // Fetch host identity *before* tracing init so every log line
    // (including the very first) carries pid + SF node name.
    let pid = std::process::id();
    let node_ctx = mssf_core::runtime::node_context::NodeContext::get_sync()
        .expect("failed to get NodeContext");
    let node_name = node_ctx.node_name.to_string();

    tracing_subscriber::fmt()
        .event_format(log_fmt::HostFields {
            prefix: format!("pid={pid} node={node_name}"),
            inner: tracing_subscriber::fmt::format::Format::default(),
        })
        .init();
    info!("main start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = TokioExecutor::new(rt.handle().clone());
    let runtime = mssf_core::runtime::Runtime::create(e.clone()).unwrap();
    let actctx = CodePackageActivationContext::create().unwrap();
    let endpoint = actctx
        .get_endpoint_resource(&WString::from("KvReplicatorEndpoint"))
        .unwrap();
    let hostname = get_hostname().expect("cannot get hostname");

    // Bind the gRPC server on a fixed port derived from the running
    // node's name. See docs/design/ReflectionReplicaTestControl.md §7.
    // Both the demo Greeter and the test-only ReplicaControl services
    // share this socket. Bind on 0.0.0.0 so a test driver in a
    // sibling container can reach it via the onebox container's IP
    // (Linux devcontainer setup) and so a same-host test driver can
    // reach it via 127.0.0.1 (Windows onebox).
    let exclusive_activation = std::env::var("Fabric_ServicePackageActivationId")
        .is_ok_and(|activation_id| !activation_id.is_empty());
    let grpc_port = if exclusive_activation {
        0
    } else {
        control_port_for_node(&node_name)
    };
    let grpc_bind_addr: std::net::SocketAddr = ([0, 0, 0, 0], grpc_port).into();
    let std_listener = std::net::TcpListener::bind(grpc_bind_addr).unwrap_or_else(|error| {
        panic!("failed to bind gRPC listener on {grpc_bind_addr} (node {node_name}): {error}")
    });
    std_listener
        .set_nonblocking(true)
        .expect("failed to set non-blocking");
    let grpc_local_addr = std_listener.local_addr().expect("failed to get local addr");
    info!(
        "gRPC server listening on {grpc_local_addr} (node {node_name}, exclusive={exclusive_activation})"
    );

    // Shared state between gRPC and Service Fabric
    let registry = ReplicaRegistry::new();

    // Start the gRPC server (Greeter + ReplicaControl)
    let token = CancellationToken::new();
    let grpc_token = token.clone();
    let grpc_registry = registry.clone();
    let grpc_handle = rt.spawn(async move {
        let tokio_listener = tokio::net::TcpListener::from_std(std_listener)
            .expect("failed to convert to tokio listener");
        let incoming = tonic::transport::server::TcpIncoming::from(tokio_listener);
        tonic::transport::Server::builder()
            .add_service(grpc::greeter_server(grpc_registry.clone()))
            .add_service(replica_control_server(grpc_registry))
            .serve_with_incoming_shutdown(incoming, async move {
                grpc_token.cancelled().await;
            })
            .await
            .expect("gRPC server failed");
    });

    let factory = Box::new(Factory::create(
        endpoint.port,
        hostname,
        e.clone(),
        grpc_port,
        registry.clone(),
    ));
    runtime
        .register_stateful_service_factory(&WString::from(SERVICE_TYPE_NAME), factory)
        .unwrap();
    runtime
        .register_self_reconfiguring_service_factory(
            &WString::from(SELF_RECONFIGURING_SERVICE_TYPE_NAME),
            Box::new(SelfReconfiguringFactory::new(
                e.clone(),
                registry,
                get_hostname()?.to_string(),
                grpc_local_addr.port(),
            )),
        )
        .unwrap();

    e.block_until_ctrlc();
    // If we get past block_until_ctrlc, SF (or an operator) sent
    // SIGTERM/Ctrl+C and the process is exiting cleanly. A
    // SIGKILL — which is what `delete_service2(force=true)` and
    // other hard terminations produce — would NOT reach here.
    info!("graceful shutdown signal received; draining gRPC server");
    token.cancel();
    rt.block_on(grpc_handle).expect("gRPC task panicked");
    info!("main exit");
    Ok(())
}

fn get_hostname() -> mssf_core::Result<WString> {
    let node_ctx = mssf_core::runtime::node_context::NodeContext::get_sync()?;
    Ok(node_ctx.ip_address_or_fqdn)
}
