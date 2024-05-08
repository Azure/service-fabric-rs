// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// echo server impl using tokio

use std::io::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot::Receiver;
use tracing::info;
use windows::core::HSTRING;

pub fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

async fn echo_loop(listener: TcpListener) -> Result<(), Error> {
    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

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

#[tokio::main()]
pub async fn start_echo(rx: Receiver<()>, port: u32, hostname: HSTRING) -> Result<(), Error> {
    let addr = get_addr(port, hostname);

    let listener = TcpListener::bind(&addr).await?;
    info!("start_echo: Listening on: {}", addr);

    // The accept loop runs until an error is encountered or rx receives a value.
    tokio::select! {
        _ = async {
            echo_loop(listener).await?;
            Ok::<_,Error>(())
        } => {}
        _ = rx => {
            info!("start_echo: signal oneshot recieved. Terminate server.")
        }
    }
    info!("start_echo: thread exiting");
    Ok(())
}
