// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Trailer-aware middleware: inspects gRPC response trailers and
//! triggers a non-blocking rebuild on the configured header.
//!
//! Lives **above** `SwapChannel` in the tower stack. The dedup
//! state machine is documented in
//! `docs/design/TonicConnectorDesign.md` ("Rebuild dedup").

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::future::BoxFuture;
use http_body::Frame;
use tonic::body::Body;
use tower::Service;

/// Inspects gRPC response trailers; on the configured trailer
/// header triggers a non-blocking rebuild of the inner Channel
/// via the supplied rebuild handle.
///
/// Stateful for dedup: tracks the last trailer value observed
/// and only triggers `rebuild()` when the value differs (with
/// two special signals — absent trailer resets state, empty
/// value always rebuilds without affecting state).
///
/// **Concurrency.** The dedup decision
/// (load → classify → store) is serialized under a
/// `std::sync::Mutex`, so concurrent in-flight RPCs that
/// complete with the **same** trailer value collapse to one
/// `rebuild()` call. Distinct values still produce one rebuild
/// each. The mutex is held only across the decision; the
/// rebuild closure runs outside the lock so back-to-back
/// `connect_with_connector_lazy` calls don't serialize.
#[derive(Clone)]
pub struct ResolveStatusMiddleware<S> {
    inner: S,
    rebuild: Arc<dyn Fn() + Send + Sync>,
    header_name: http::HeaderName,
    last_seen: Arc<Mutex<Option<String>>>,
}

impl<S> ResolveStatusMiddleware<S> {
    /// Construct from any rebuild trigger. Useful for tests with
    /// a mock and for users who want to plug a different
    /// invalidation mechanism behind the same trailer-detection
    /// logic.
    pub fn new<F>(inner: S, header_name: http::HeaderName, rebuild: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            inner,
            rebuild: Arc::new(rebuild),
            header_name,
            last_seen: Arc::new(Mutex::new(None)),
        }
    }
}

/// Decision applied by the dedup state machine after observing a
/// response. Extracted as an enum to make the middleware unit
/// tests trivial.
#[derive(Debug, PartialEq, Eq)]
enum DedupAction {
    /// No-op; trailer absent and `last_seen` already `None`, or
    /// trailer matched `last_seen`.
    None,
    /// Trailer present (non-empty) with a value differing from
    /// `last_seen`; store it and rebuild.
    StoreAndRebuild(String),
    /// Trailer present with empty value; rebuild without
    /// touching `last_seen`.
    RebuildKeepLast,
    /// No trailer on the response; reset `last_seen` to `None`,
    /// no rebuild.
    Reset,
}

/// Apply the dedup rule documented in
/// `TonicConnectorDesign.md#rebuild-dedup`.
fn classify(observed: Option<&str>, last_seen: Option<&str>) -> DedupAction {
    match observed {
        None => {
            if last_seen.is_none() {
                DedupAction::None
            } else {
                DedupAction::Reset
            }
        }
        Some("") => DedupAction::RebuildKeepLast,
        Some(v) => match last_seen {
            Some(prev) if prev == v => DedupAction::None,
            _ => DedupAction::StoreAndRebuild(v.to_string()),
        },
    }
}

impl<S> Service<http::Request<Body>> for ResolveStatusMiddleware<S>
where
    S: Service<
            http::Request<Body>,
            Response = http::Response<Body>,
            Error = tonic::transport::Error,
        > + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = http::Response<Body>;
    type Error = tonic::transport::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        // Use the readied service clone; re-arm `self.inner` with
        // a fresh clone for next call (standard tower idiom).
        let mut inner = self.inner.clone();
        std::mem::swap(&mut inner, &mut self.inner);
        let header = self.header_name.clone();
        let rebuild = self.rebuild.clone();
        let last_seen = self.last_seen.clone();
        Box::pin(async move {
            let resp = inner.call(req).await?;
            let (parts, body) = resp.into_parts();
            let observer = TrailerObserver::new(body, header, last_seen, rebuild);
            let wrapped = Body::new(observer);
            Ok(http::Response::from_parts(parts, wrapped))
        })
    }
}

/// Body wrapper that delegates `poll_frame` and inspects the
/// final trailer frame. Apply the dedup rule before forwarding
/// the frame downstream so the caller observes trailers
/// unchanged.
struct TrailerObserver {
    inner: Body,
    header: http::HeaderName,
    last_seen: Arc<Mutex<Option<String>>>,
    rebuild: Arc<dyn Fn() + Send + Sync>,
    /// Set once we observe a trailer frame; if the body ends
    /// without one we apply the "no trailer" branch.
    saw_trailers: bool,
    /// Set once we've applied the dedup decision (either via a
    /// trailer frame or via the end-of-stream reset). Prevents
    /// double-firing within a single response body.
    fired: bool,
}

impl TrailerObserver {
    fn new(
        inner: Body,
        header: http::HeaderName,
        last_seen: Arc<Mutex<Option<String>>>,
        rebuild: Arc<dyn Fn() + Send + Sync>,
    ) -> Self {
        Self {
            inner,
            header,
            last_seen,
            rebuild,
            saw_trailers: false,
            fired: false,
        }
    }

    fn fire(&mut self, observed: Option<&str>) {
        if self.fired {
            return;
        }
        self.fired = true;

        // Hold the lock across load + classify + store so
        // concurrent observers of the same value collapse to a
        // single decision. Drop the guard *before* invoking the
        // rebuild closure: rebuild does its own lazy work
        // (connect_with_connector_lazy + ArcSwap::store) and
        // doesn't need to be serialized; we only need the
        // decision step itself to be atomic.
        let should_rebuild = {
            let mut guard = self.last_seen.lock().expect("middleware mutex poisoned");
            let prev_str: Option<&str> = guard.as_deref();
            let action = classify(observed, prev_str);
            match action {
                DedupAction::None => false,
                DedupAction::Reset => {
                    *guard = None;
                    false
                }
                DedupAction::RebuildKeepLast => true,
                DedupAction::StoreAndRebuild(v) => {
                    *guard = Some(v);
                    true
                }
            }
        };
        if should_rebuild {
            (self.rebuild)();
        }
    }
}

impl http_body::Body for TrailerObserver {
    type Data = Bytes;
    type Error = tonic::Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        let polled = Pin::new(&mut this.inner).poll_frame(cx);
        match &polled {
            // Trailers frame: extract the configured header value
            // (if any) and run the dedup decision before
            // forwarding the frame to the caller.
            Poll::Ready(Some(Ok(frame))) if frame.is_trailers() => {
                if let Some(map) = frame.trailers_ref() {
                    let observed = map.get(&this.header).and_then(|v| v.to_str().ok());
                    this.saw_trailers = true;
                    this.fire(observed);
                }
            }
            // End-of-stream without ever seeing a trailers frame:
            // apply the "no trailer" branch of the dedup rule.
            Poll::Ready(None) if !this.saw_trailers => {
                this.fire(None);
            }
            _ => {}
        }
        polled
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_none_to_value_rebuilds() {
        assert_eq!(
            classify(Some("not-primary"), None),
            DedupAction::StoreAndRebuild("not-primary".to_string()),
        );
    }

    #[test]
    fn dedup_same_value_is_noop() {
        assert_eq!(classify(Some("v"), Some("v")), DedupAction::None);
    }

    #[test]
    fn dedup_different_value_rebuilds() {
        assert_eq!(
            classify(Some("w"), Some("v")),
            DedupAction::StoreAndRebuild("w".to_string()),
        );
    }

    #[test]
    fn dedup_no_trailer_resets_when_seen() {
        assert_eq!(classify(None, Some("v")), DedupAction::Reset);
    }

    #[test]
    fn dedup_no_trailer_steady_state_is_noop() {
        assert_eq!(classify(None, None), DedupAction::None);
    }

    #[test]
    fn dedup_empty_always_rebuilds_without_state_change() {
        assert_eq!(classify(Some(""), None), DedupAction::RebuildKeepLast);
        assert_eq!(classify(Some(""), Some("v")), DedupAction::RebuildKeepLast);
    }
}
