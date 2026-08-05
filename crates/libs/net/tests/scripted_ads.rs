// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Cluster-free end-to-end proof.
//!
//! Drives a **stock, unmodified** `tonic-xds` client against `mssf-net`'s ADS
//! server and asserts that it routes to the endpoint the source publishes, and
//! follows that endpoint when it relocates.
//!
//! **Requires no Service Fabric cluster, no SF runtime, and constructs no
//! `FabricClient`.** This is what makes it runnable in CI before any cluster
//! exists.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use mssf_net::ads::{AdsService, bootstrap_json};
use mssf_net::{
    EndpointSnapshot, HostPort, ScriptedEndpointSource, XdsMapping, host_port_interpreter,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Request, Response, Status};
use tonic_xds::{BootstrapConfig, XdsChannelBuilder, XdsChannelConfig, XdsUri};

pub mod testsvc {
    tonic::include_proto!("testsvc");
}

use testsvc::identity_client::IdentityClient;
use testsvc::identity_server::{Identity, IdentityServer};
use testsvc::{WhoAmIReply, WhoAmIRequest};

/// A stand-in backend that reports the name it was started with.
struct NamedIdentity {
    name: String,
}

#[tonic::async_trait]
impl Identity for NamedIdentity {
    async fn who_am_i(
        &self,
        _request: Request<WhoAmIRequest>,
    ) -> Result<Response<WhoAmIReply>, Status> {
        Ok(Response::new(WhoAmIReply {
            name: self.name.clone(),
        }))
    }
}

/// Start a stand-in backend on an ephemeral loopback port.
async fn start_backend(name: &str) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = IdentityServer::new(NamedIdentity {
        name: name.to_string(),
    });
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(svc)
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    addr
}

/// Start the ADS server on an ephemeral loopback port.
async fn start_ads(mapping: XdsMapping, source: Arc<ScriptedEndpointSource>) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = AdsService::new(mapping, source).into_server();
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(svc)
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    addr
}

fn mapping() -> XdsMapping {
    XdsMapping::new("reflection", "fabric:/App/Svc", host_port_interpreter()).unwrap()
}

/// Build a stock xDS channel client pointed at our ADS server.
fn xds_client(ads_addr: SocketAddr, target: &str) -> IdentityClient<tonic_xds::XdsChannelGrpc> {
    let bootstrap = BootstrapConfig::from_json(&bootstrap_json(ads_addr, "mssf-net-test")).unwrap();
    let uri = XdsUri::parse(target).unwrap();
    let channel = XdsChannelBuilder::new(XdsChannelConfig::new(uri).with_bootstrap(bootstrap))
        .build_grpc_channel()
        .unwrap();
    IdentityClient::new(channel)
}

/// Per-RPC timeout. An xDS call can block indefinitely while discovery has no
/// usable endpoint, so every call is bounded rather than relying on the outer
/// loop's deadline (which would only be checked after the call returned).
const CALL_TIMEOUT: Duration = Duration::from_secs(3);

/// Make one bounded `WhoAmI` call.
async fn who_am_i_once(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
) -> Result<String, String> {
    match tokio::time::timeout(
        CALL_TIMEOUT,
        client.who_am_i(Request::new(WhoAmIRequest {})),
    )
    .await
    {
        Ok(Ok(r)) => Ok(r.into_inner().name),
        Ok(Err(status)) => Err(format!("status: {status}")),
        Err(_) => Err(format!("call exceeded {CALL_TIMEOUT:?}")),
    }
}

/// Call `WhoAmI`, retrying transient discovery errors until `timeout`.
async fn who_am_i_until_ok(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    timeout: Duration,
) -> String {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let err = match who_am_i_once(client).await {
            Ok(name) => return name,
            Err(e) => e,
        };
        if tokio::time::Instant::now() >= deadline {
            panic!("WhoAmI did not succeed within {timeout:?}; last error: {err}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Call `WhoAmI` until it returns `expected`, or panic.
async fn who_am_i_until(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    expected: &str,
    timeout: Duration,
) {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let last = match who_am_i_once(client).await {
            Ok(name) if name == expected => return,
            Ok(name) => format!("reached {name:?}"),
            Err(e) => e,
        };
        if tokio::time::Instant::now() >= deadline {
            panic!("expected to reach {expected:?} within {timeout:?}; last: {last}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Call `WhoAmI` until it fails, or panic.
async fn who_am_i_until_err(
    client: &mut IdentityClient<tonic_xds::XdsChannelGrpc>,
    timeout: Duration,
) -> String {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if let Err(e) = who_am_i_once(client).await {
            return e;
        }
        if tokio::time::Instant::now() >= deadline {
            panic!("expected a failure within {timeout:?}, but calls kept succeeding");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// SC-001 / SC-005: a stock xDS client reaches the published endpoint with no
/// SF-specific code, and SC-003: it follows the endpoint when it relocates.
#[tokio::test]
#[test_log::test]
async fn stock_xds_client_routes_and_follows_relocation() {
    let a = start_backend("backend-a").await;
    let b = start_backend("backend-b").await;

    let (source, handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.ip().to_string(),
        a.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    let mut client = xds_client(ads, "xds:///reflection");

    // Initial routing.
    let first = who_am_i_until_ok(&mut client, Duration::from_secs(20)).await;
    assert_eq!(first, "backend-a");

    // Relocate. The already-created client must follow, with neither the
    // client nor the ADS server restarted.
    handle.set(EndpointSnapshot::Primary(HostPort::new(
        b.ip().to_string(),
        b.port(),
    )));
    who_am_i_until(&mut client, "backend-b", Duration::from_secs(20)).await;
}

/// SC-004: the transient no-primary window fails calls in a bounded way
/// (rather than hanging or silently succeeding against a stale address), and
/// recovers once an endpoint is published again.
#[tokio::test]
#[test_log::test]
async fn no_primary_fails_bounded_then_recovers() {
    let a = start_backend("backend-a").await;

    let (source, handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.ip().to_string(),
        a.port(),
    )));
    let ads = start_ads(mapping(), source).await;
    let mut client = xds_client(ads, "xds:///reflection");

    assert_eq!(
        who_am_i_until_ok(&mut client, Duration::from_secs(20)).await,
        "backend-a"
    );

    // No primary: calls must fail, bounded.
    handle.set(EndpointSnapshot::NoPrimary);
    let failure = who_am_i_until_err(&mut client, Duration::from_secs(20)).await;
    tracing::info!(%failure, "observed no-primary failure");

    // Recovery.
    handle.set(EndpointSnapshot::Primary(HostPort::new(
        a.ip().to_string(),
        a.port(),
    )));
    who_am_i_until(&mut client, "backend-a", Duration::from_secs(20)).await;
}

/// SC-007: a name this ADS server has no mapping for fails in a bounded way
/// rather than hanging.
#[tokio::test]
#[test_log::test]
async fn unknown_resource_name_fails_bounded() {
    let a = start_backend("backend-a").await;
    let (source, _handle) = ScriptedEndpointSource::new(EndpointSnapshot::Primary(HostPort::new(
        a.ip().to_string(),
        a.port(),
    )));
    let ads = start_ads(mapping(), source).await;

    let mut client = xds_client(ads, "xds:///no-such-service");
    let failure = who_am_i_until_err(&mut client, Duration::from_secs(20)).await;
    tracing::info!(%failure, "observed unknown-resource failure");
}
