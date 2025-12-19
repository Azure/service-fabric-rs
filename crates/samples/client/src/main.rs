// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    WString,
    client::{ClaimsRetrievalMetadata, FabricClient, GatewayInformationResult},
    types::{FabricClaimsCredentials, Uri},
};

// Sample client that connects to a Service Fabric cluster with AAD enabled.
// This is similar to how SF pwsh client connects to such cluster.
// Run with: cargo run --bin samples_client -- <host:port>
// This is tested against with a AAD enabled cluster.
// TODO: expand this sample to parse args and do some actual requests.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    // get endpoint from argv
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2, "Usage: client <host:port>");
    let env_endpoint = WString::from(&args[1]);

    // split host and port
    let endpoint_str = env_endpoint.to_string_lossy();
    let mut parts = endpoint_str.split(':');
    let host_str = parts.next().unwrap();
    assert_eq!(parts.next().unwrap(), "19000");
    tracing::info!("Using host_str: {}", host_str);

    // channel for client connection notification
    let (cc_tx, mut cc_rx) = tokio::sync::mpsc::channel::<GatewayInformationResult>(1);
    // channel for message retrieval notification
    let (claims_tx, mut claims_rx) = tokio::sync::mpsc::channel::<ClaimsRetrievalMetadata>(1);
    let fc = FabricClient::builder()
        .with_connection_strings(vec![env_endpoint])
        .with_on_client_connect(move |gw| {
            tracing::info!("Client connected: {:?}", gw);
            cc_tx.blocking_send(gw.clone()).expect("cannot send");
            Ok(())
        })
        .with_on_client_disconnect(move |_| {
            // This is not invoked in this test. FabricClient does not invoke this on drop.
            panic!("client disconnected");
        })
        .with_on_claims_retrieval(move |meta| {
            claims_tx
                .blocking_send(meta.clone())
                .expect("cannot send claims retrieval metadata");
            // For test purpose, return empty claims.
            // Empty claims will trigger default handler to run.
            // See: https://github.com/microsoft/service-fabric/blob/36f7531df0fd990f8af1792ae2cd5cf811521ab3/src/prod/src/client/ClientConnectionManager.cpp#L933
            // Ok(WString::from("_Invalid_"))
            Ok(WString::from(""))
        })
        .with_credentials(mssf_core::types::FabricSecurityCredentials::Claims(
            FabricClaimsCredentials {
                ServerCommonNames: vec![WString::from(host_str)],
                ..Default::default()
            },
        ))
        .with_client_role(mssf_core::types::ClientRole::Unknown)
        .build()
        .unwrap();

    let timeout = Duration::from_secs(5);
    // dummy request to trigger connection
    let err = fc
        .get_property_manager()
        .name_exists(&Uri::from("fabric:/DummyApp/DummySvc"), timeout, None)
        .await
        .inspect_err(|e| tracing::info!("expected first error: {e}"));

    if err.is_ok() {
        tracing::info!("Cluster is not secured with AAD. Exiting.");
        return Ok(());
    }

    // try wait for claims retrieval notification for some timeout
    let meta = tokio::time::timeout(Duration::from_secs(10), claims_rx.recv())
        .await
        .inspect_err(|e| {
            tracing::error!("Timeout waiting for claims retrieval: {e}");
        })?
        .expect("Sender should not be dropped");
    tracing::info!("Claims retrieval metadata received: {:?}", meta);
    // Connection event notification should be received since we already sent a request.
    let gw = cc_rx.try_recv().expect("notification not present");
    assert!(!gw.node_name.is_empty());
    Ok(())
}
