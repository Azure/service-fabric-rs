// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// echo server impl using tokio

use std::io::Error;

use mssf_core::runtime::stateful_proxy::StatefulServicePartition;
use mssf_core::types::LoadMetric;
use mssf_core::WString;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub fn get_addr(port: u32, hostname: WString) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

/// Report load for the app via SF partition api periodically
pub async fn report_load_loop(partition: StatefulServicePartition, token: CancellationToken) {
    let mut value = 0;
    let metric_name = WString::from("MyLoad");
    loop {
        // Default load is 0 set in the manifest.
        // Make report value changing betwen 2 or 1.
        value %= 2; // make 1 or 0
        value += 1; // make 2 or 1

        let metrics = vec![LoadMetric::new(metric_name.clone(), value)];
        if let Err(e) = partition.report_load(&metrics) {
            error!("report_load failed with {}", e)
        } else {
            info!("report_load: name: {}, value: {}", metric_name, value);
        }

        select! {
            _ = token.cancelled() => {
                // The token was cancelled
                info!("report_load_loop cancelled. Stop reporting load");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {}
        }
    }
}

// main loop entrypoint for the app logic
pub async fn start_echo(
    token: CancellationToken,
    port: u32,
    hostname: WString,
    partition: StatefulServicePartition,
) -> Result<(), Error> {
    let addr = get_addr(port, hostname);
    info!("start_echo: Listening on: {}", addr);
    // launch report load loop and listner separately
    let h2 = tokio::spawn(async move { report_load_loop(partition, token).await });

    if let Err(e) = h2.await {
        error!("report_load_loop task failed {e}");
    }
    Ok(())
}
