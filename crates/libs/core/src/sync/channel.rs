// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use mssf_com::FabricCommon::IFabricAsyncOperationContext;

use crate::{
    ErrorCode,
    runtime::executor::{CancelToken, EventFuture},
};

pub use futures_channel::oneshot::{self, Receiver, Sender};

// Token that wraps oneshot receiver.
// The future recieve does not have error. This is designed for the use
// case where SF guarantees that sender will be called.
pub struct FabricReceiver<T> {
    rx: Receiver<T>,
    token: Option<Arc<dyn CancelToken + 'static>>,
    // event for cancelling
    cancel_event: Option<Pin<Box<dyn EventFuture + 'static>>>,
    // saved ctx from SF Begin COM api for cancalling.
    ctx: Option<IFabricAsyncOperationContext>,
}

impl<T> FabricReceiver<T> {
    fn new(rx: Receiver<T>, token: Option<Arc<dyn CancelToken + 'static>>) -> FabricReceiver<T> {
        FabricReceiver {
            rx,
            cancel_event: token.as_ref().map(|t| t.wait()),
            token,
            ctx: None,
        }
    }

    // This does not handle cancel. It is commented out because it is not used.
    // pub(crate) fn blocking_recv(self) -> crate::Result<T> {
    //     if let Some(t) = self.token {
    //         if t.is_cancelled() {
    //             return Err(ErrorCode::E_ABORT.into());
    //         }
    //     }
    //     // sender must send stuff so that there is not error.
    //     Ok(self.rx.blocking_recv().unwrap())
    // }

    // Set the SF ctx to hook up cancellation.
    pub(crate) fn set_ctx(&mut self, ctx: IFabricAsyncOperationContext) {
        let prev = self.ctx.replace(ctx);
        assert!(prev.is_none());
    }

    // Cancels the inner SF operation if exists, and reset the ctx.
    fn cancel_inner_ctx(&mut self) -> crate::WinResult<()> {
        if let Some(ctx) = &self.ctx {
            if let Err(e) = unsafe { ctx.Cancel() } {
                // fail to cancel inner operation.
                return Err(e);
            } else {
                // clear the sf ctx to avoid cancel twice.
                self.ctx.take();
            }
        } else {
            // The inner ctx can be empty after we already cancelled the inner ctx.
            // This can happen because we cancel during polling, and polling can
            // happen many times.
        }
        Ok(())
    }
}

// Returns error if cancelled.
// If there is an inner SF ctx, cancellation signal will
// trigger cancellation of the ctx.
impl<T> Future for FabricReceiver<T> {
    // The error code should be OperationCanceled, unless cancellation
    // of SF ctx returns other errors.
    // (TODO: observe other error code from SF, maybe some code should be ignored).
    type Output = crate::WinResult<T>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the receiver first, if ready then return the output,
        // else poll the cancellation token, if cancelled propergate the cancel to SF ctx,
        // and return pending. SF task should continue finish execute in the background,
        // and finish with error code OperationCancelled
        // and send the error code from FabricSender.
        //
        // There can be the case that cancellation wakes the waker, but receiver
        // then got the result. The next poll will return received output rather
        // than cancelled error.

        let this = self.get_mut();

        // Try to receive the value from the sender
        let inner = <Receiver<T> as Future>::poll(Pin::new(&mut this.rx), cx);
        match (inner, this.token.as_ref()) {
            (Poll::Ready(Ok(data)), _) => Poll::Ready(Ok(data)),
            (Poll::Ready(Err(_)), Some(t)) => {
                if t.is_cancelled() {
                    // clear the token since we only propergate the signal once.
                    this.token.take();
                    this.cancel_event.take();
                    // cancel the SF ctx and clear it.
                    if let Err(e) = this.cancel_inner_ctx() {
                        Poll::Ready(Err(e))
                    } else {
                        Poll::Ready(Err(ErrorCode::E_ABORT.into()))
                    }
                } else {
                    panic!("sender dropped without sending")
                }
            }
            (Poll::Ready(Err(_)), None) => {
                panic!("sender dropped without sending")
            }
            (Poll::Pending, Some(_)) => {
                // If the action is canceled we can safely stop and return canceled error.
                // this is cancel safe so we can poll it once and discard
                let event = this
                    .cancel_event
                    .as_mut()
                    .expect("cancel event should be set");
                let inner = std::pin::pin!(event).poll(cx);
                match inner {
                    Poll::Ready(_) => {
                        // clear the token since we only propergate the signal once.
                        this.cancel_event.take();
                        this.cancel_event.take();
                        // operation cancelled. Propergate to inner sf ctx.
                        if let Err(e) = this.cancel_inner_ctx() {
                            Poll::Ready(Err(e))
                        } else {
                            // The cancellation is propergated to sf task,
                            // the receiver from now on should wait for the
                            // final result from the sf task. (as we have cleared the token)
                            // Most likely the task finishes with OperationCancelled error code.
                            Poll::Pending
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            (Poll::Pending, None) => Poll::Pending,
        }
    }
}

pub struct FabricSender<T> {
    tx: Sender<T>,
    token: Option<Arc<dyn CancelToken + 'static>>,
}

impl<T> FabricSender<T> {
    fn new(tx: Sender<T>, token: Option<Arc<dyn CancelToken + 'static>>) -> FabricSender<T> {
        FabricSender { tx, token }
    }

    pub fn send(self, data: T) {
        let e = self.tx.send(data);
        if e.is_err() {
            // In SF use case receiver should not be dropped by user.
            // If it acctually dropped by user, it is ok to ignore because user
            // does not want to want the value any more. But too bad SF has done
            // the work to get the value.

            // receiver should never be dropped if operation is not cancelled.
            if let Some(t) = self.token {
                debug_assert!(
                    t.is_cancelled(),
                    "task should be cancelled when receiver dropped."
                );
            }
        }
    }
}

/// Creates a fabric oneshot channel.
/// Operation can be cancelled by cancelling the token.
pub fn oneshot_channel<T, C: CancelToken>(
    token: Option<C>,
) -> (FabricSender<T>, FabricReceiver<T>) {
    let arc_token = token.map(|t| Arc::new(t) as Arc<dyn CancelToken + 'static>);
    let (tx, rx) = oneshot::channel::<T>();
    (
        FabricSender::new(tx, arc_token.clone()),
        FabricReceiver::new(rx, arc_token),
    )
}

#[cfg(test)]
mod test {

    use crate::{
        ErrorCode,
        sync::{SimpleCancelToken, oneshot_channel},
    };

    /// Test various cancellation cases for the channel used
    /// to send data in proxy layer.
    #[tokio::test]
    async fn test_channel() {
        // success send
        {
            let (tx, rx) = oneshot_channel::<bool, _>(Some(SimpleCancelToken::new()));
            tx.send(true);
            assert!(rx.await.unwrap());
        }
        // receiver cancelled after send, still received the result.
        {
            let token = SimpleCancelToken::new();
            let (tx, rx) = oneshot_channel::<bool, _>(Some(token.clone()));
            tx.send(true);
            token.cancel();
            assert!(rx.await.unwrap());
        }
        // receiver cancelled before send, still received the result.
        {
            let token = SimpleCancelToken::new();
            let (tx, rx) = oneshot_channel::<bool, _>(Some(token.clone()));
            token.cancel();
            tx.send(true);
            assert!(rx.await.unwrap(),);
        }
        // receiver cancelled and droped, send is no op
        {
            let token = SimpleCancelToken::new();
            let (tx, rx) = oneshot_channel::<bool, _>(Some(token.clone()));
            token.cancel();
            std::mem::drop(rx);
            tx.send(true);
        }
        // receiver cancelled and sender dropped. receiver get error
        {
            let token = SimpleCancelToken::new();
            let (tx, rx) = oneshot_channel::<bool, _>(Some(token.clone()));
            token.cancel();
            std::mem::drop(tx);
            assert_eq!(rx.await.unwrap_err(), ErrorCode::E_ABORT.into());
        }
    }
}
