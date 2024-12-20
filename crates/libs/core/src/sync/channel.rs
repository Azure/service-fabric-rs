// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::oneshot::Receiver;

// Token that wraps oneshot receiver.
// The future recieve does not have error. This is designed for the use
// case where SF guarantees that sender will be called.
pub struct FabricReceiver<T> {
    rx: tokio::sync::oneshot::Receiver<T>,
}

impl<T> FabricReceiver<T> {
    pub(super) fn new(rx: tokio::sync::oneshot::Receiver<T>) -> FabricReceiver<T> {
        FabricReceiver { rx }
    }

    pub fn blocking_recv(self) -> T {
        // sender must send stuff so that there is not error.
        self.rx.blocking_recv().unwrap()
    }
}

// The future differs from tokio oneshot that it will not error when awaited.
impl<T> Future for FabricReceiver<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to receive the value from the sender
        let innner = <Receiver<T> as Future>::poll(Pin::new(&mut self.rx), _cx);
        match innner {
            Poll::Ready(x) => {
                // error only happens when sender is dropped without sending.
                // we ignore this error since in sf-rs use this will never happen.
                Poll::Ready(x.expect("sf sender closed without sending a value."))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct FabricSender<T> {
    tx: tokio::sync::oneshot::Sender<T>,
}

impl<T> FabricSender<T> {
    fn new(tx: tokio::sync::oneshot::Sender<T>) -> FabricSender<T> {
        FabricSender { tx }
    }

    pub fn send(self, data: T) {
        let e = self.tx.send(data);
        if e.is_err() {
            // In SF use case receiver should not be dropped by user.
            // If it acctually dropped by user, it is ok to ignore because user
            // does not want to want the value any more. But too bad SF has done
            // the work to get the value.
            debug_assert!(false, "receiver dropped.");
        }
    }
}

// Creates a fabric oneshot channel.
pub fn oneshot_channel<T>() -> (FabricSender<T>, FabricReceiver<T>) {
    let (tx, rx) = tokio::sync::oneshot::channel::<T>();
    (FabricSender::new(tx), FabricReceiver::new(rx))
}

// Send Box. Wrap a type and implement send.
// c pointers are not send in rust, so this forces it.
#[derive(Debug)]
pub struct SBox<T> {
    pub b: Box<T>,
}

// We know that T is send. This requires programmer's check of the internals.
unsafe impl<T> Send for SBox<T> {}

impl<T> SBox<T> {
    pub fn new(x: T) -> SBox<T> {
        SBox { b: Box::new(x) }
    }

    pub fn into_inner(self) -> T {
        *self.b
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_oneshot() {
        let (tx, rx) = super::oneshot_channel::<String>();
        tokio::spawn(async move {
            tx.send("hello".to_string());
        });
        let val = rx.await;
        assert_eq!("hello", val);
    }
}
