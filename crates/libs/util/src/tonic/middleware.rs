// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Status-aware middleware: inspects the configured header on
//! gRPC responses and triggers a non-blocking rebuild when it
//! appears. The header is inspected on **both** the initial
//! response `HeaderMap` *and* any trailers frame in the body,
//! because tonic's wire placement of user metadata differs by
//! call shape:
//!
//! | Call shape                         | Where the header lands       |
//! |------------------------------------|------------------------------|
//! | Unary `Ok` + `metadata_mut()`      | Initial HEADERS frame        |
//! | Unary `Err(Status::with_metadata)` | Trailers-only HEADERS frame  |
//! | Streaming `Ok` + initial metadata  | Initial HEADERS frame        |
//! | Streaming yielding `Err(Status..)` | Real HTTP/2 trailers frame   |
//!
//! Both wire locations surface to a Tower service as either
//! `response.headers()` (the first three rows) or as a
//! body-level `Frame::trailers()` (the last row). We classify
//! whichever fires first and short-circuit the other; at most
//! one rebuild signal is consumed per response.
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

/// Inspects gRPC response headers and trailers; on the
/// configured header (whichever surface it arrives on)
/// triggers a non-blocking rebuild of the inner Channel via the
/// supplied rebuild handle.
///
/// Stateful for dedup: tracks the last header value observed
/// and only triggers `rebuild()` when the value differs (with
/// two special signals — absent header resets state, empty
/// value always rebuilds without affecting state).
///
/// **Concurrency.** The dedup decision
/// (load → classify → store) is serialized under a
/// `std::sync::Mutex`, so concurrent in-flight RPCs that
/// complete with the **same** header value collapse to one
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
    /// No-op; header absent and `last_seen` already `None`, or
    /// header matched `last_seen`.
    None,
    /// Header present (non-empty) with a value differing from
    /// `last_seen`; store it and rebuild.
    StoreAndRebuild(String),
    /// Header present with empty value; rebuild without
    /// touching `last_seen`.
    RebuildKeepLast,
    /// No header on the response; reset `last_seen` to `None`,
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

/// Single-shot dedup-and-rebuild. Loads `last_seen`, classifies
/// `observed`, updates state, and (outside the lock) invokes
/// `rebuild` if the decision called for it. Shared by the
/// initial-headers path (in `call()`) and the trailers/EOS path
/// (in `TrailerObserver::fire`).
///
/// Emits a single `tracing::info!` per actual rebuild, carrying
/// the value that triggered it, the previous `last_seen` value
/// (so a prod operator can tell first-ever-signal apart from
/// failover-after-failover), and the dedup decision variant.
/// Skipped decisions (no-op, reset) are not logged at info
/// level to keep the channel quiet in steady state.
fn apply_dedup(
    observed: Option<&str>,
    last_seen: &Mutex<Option<String>>,
    rebuild: &(dyn Fn() + Send + Sync),
) {
    // Capture `prev` and the chosen action inside the lock so
    // the trace below has accurate context; the `rebuild()`
    // closure itself runs outside the lock (it does its own
    // lazy reconnect work and shouldn't be serialized).
    let (action, prev, should_rebuild) = {
        let mut guard = last_seen.lock().expect("middleware mutex poisoned");
        let prev: Option<String> = guard.clone();
        let action = classify(observed, prev.as_deref());
        let should_rebuild = match &action {
            DedupAction::None => false,
            DedupAction::Reset => {
                *guard = None;
                false
            }
            DedupAction::RebuildKeepLast => true,
            DedupAction::StoreAndRebuild(v) => {
                *guard = Some(v.clone());
                true
            }
        };
        (action, prev, should_rebuild)
    };
    if should_rebuild {
        tracing::info!(
            observed = observed.unwrap_or("<absent>"),
            previous = prev.as_deref().unwrap_or("<none>"),
            decision = ?action,
            "ResolveStatusMiddleware firing channel rebuild",
        );
        rebuild();
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
            // Inspect the initial response HeaderMap first. This
            // covers (a) unary Ok with `Response::metadata_mut()`
            // metadata (rides on the initial HEADERS frame) and
            // (b) unary Err via `Status::with_metadata`, which
            // tonic emits as a gRPC "Trailers-Only" response —
            // a single HEADERS frame with END_STREAM carrying
            // `grpc-status` + user metadata, no DATA, no
            // separate trailers frame. Either way the metadata
            // surfaces on `parts.headers`.
            //
            // If we fire here, the body needs no observation:
            // we've already consumed this response's at-most-one
            // rebuild signal, so we skip wrapping it in
            // `TrailerObserver` and let the original body flow
            // through unchanged. That avoids one `poll_frame`
            // indirection per body frame on the streaming hot
            // path. Otherwise we wrap the body and let the
            // observer watch for a trailers frame (streaming-Err
            // path) or end-of-stream (no-signal reset).
            let wrapped = match parts.headers.get(&header).and_then(|v| v.to_str().ok()) {
                Some(v) => {
                    apply_dedup(Some(v), &last_seen, &*rebuild);
                    body
                }
                None => Body::new(TrailerObserver::new(body, header, last_seen, rebuild)),
            };
            Ok(http::Response::from_parts(parts, wrapped))
        })
    }
}

/// Body wrapper that delegates `poll_frame` and inspects any
/// trailer frame the body produces. Apply the dedup rule before
/// forwarding the frame downstream so the caller observes
/// trailers unchanged.
///
/// Only installed when the initial response headers did **not**
/// carry the configured header — if they did, `call()` fires
/// immediately and returns the original body unwrapped (at most
/// one rebuild signal per response).
struct TrailerObserver {
    inner: Body,
    header: http::HeaderName,
    last_seen: Arc<Mutex<Option<String>>>,
    rebuild: Arc<dyn Fn() + Send + Sync>,
    /// Set once we observe a trailer frame; if the body ends
    /// without one we apply the "no trailer" branch.
    saw_trailers: bool,
    /// Set once we've applied the dedup decision (via a trailer
    /// frame or the end-of-stream reset). Prevents double-firing
    /// within a single response body.
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
        apply_dedup(observed, &self.last_seen, &*self.rebuild);
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
