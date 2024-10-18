// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// echo server impl using tokio

use std::io::Error;

use mssf_core::runtime::stateful::StatefulServicePartition;
use mssf_core::types::LoadMetric;
use mssf_core::HSTRING;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

async fn echo_loop(listener: TcpListener, token: CancellationToken) -> Result<(), Error> {
    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = select! {
            _ = token.cancelled() => {
                // The token was cancelled
                info!("echo_loop cancelled.");
                break Ok(());
            }
            socket = listener.accept() =>{ socket? }
        };

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}

/// Report load for the app via SF partition api periodically
pub async fn report_load_loop(partition: StatefulServicePartition, token: CancellationToken) {
    let mut value = 0;
    let metric_name = HSTRING::from("MyLoad");
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
    hostname: HSTRING,
    partition: StatefulServicePartition,
) -> Result<(), Error> {
    let addr = get_addr(port, hostname);

    let listener = TcpListener::bind(&addr).await?;
    let token2 = token.clone();
    info!("start_echo: Listening on: {}", addr);
    // launch report load loop and listner separately
    let h = tokio::spawn(async move { echo_loop(listener, token2).await });
    let h2 = tokio::spawn(async move { report_load_loop(partition, token).await });

    match h.await {
        Ok(e) => {
            if let Err(e2) = e {
                error!("echo_loop task failed {e2}");
            }
        }
        Err(e) => {
            error!("echo_loop task join failed {e}");
        }
    }

    if let Err(e) = h2.await {
        error!("report_load_loop task failed {e}");
    }
    Ok(())
}
