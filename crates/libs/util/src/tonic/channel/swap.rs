// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;
use std::task::{Context, Poll};

use arc_swap::ArcSwap;
use futures::future::BoxFuture;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tonic::body::Body;
use tonic::transport::{Channel, Endpoint};
use tower::util::BoxCloneSyncService;
use tower::{Service, ServiceExt as _};

use crate::tonic::connector::TargetConnector;
use crate::tonic::naming::BoxError;

/// Type-erased connector slot stored inside [`SwapChannel`].
type ErasedConnector = BoxCloneSyncService<http::Uri, TokioIo<TcpStream>, BoxError>;

/// Wraps a [`Channel`] behind an `ArcSwap` so the inner Channel
/// can be replaced atomically without invalidating the user-facing
/// handle.
///
/// `SwapChannel` is **type-erased over the inner connector**: it
/// stores a `BoxCloneSyncService<Uri, TokioIo<TcpStream>, BoxError>`
/// internally. The cost is one `Box::pin` per dial — negligible
/// against TCP connect.
///
/// **The IO type is fixed at `TokioIo<TcpStream>`**, so any
/// connector other than [`TargetConnector`] must also produce
/// `TokioIo<TcpStream>` (i.e. plain TCP). TLS-wrapped connectors
/// (which return `TokioIo<TlsStream<...>>`) do not fit. Generalizing
/// the IO bound to support TLS is Future Work — see
/// `docs/design/TonicConnectorDesign.md`.
///
/// Readiness is driven inside the response future rather than
/// across separate `poll_ready` / `call` invocations, so cloning
/// `SwapChannel` (e.g. inside layered tower stacks) doesn't risk
/// leaking a stale readied snapshot from before a `rebuild()`.
#[derive(Clone)]
pub struct SwapChannel {
    inner: Arc<Inner>,
}

struct Inner {
    /// Current Channel; replaced atomically by `rebuild()`.
    channel: ArcSwap<Channel>,
    /// Type-erased connector; cloned on every rebuild.
    connector: ErasedConnector,
    /// Endpoint template (timeouts, keepalive, HTTP/2 windows,
    /// ...). The URI inside is a placeholder; the connector
    /// ignores it.
    endpoint_template: Endpoint,
}

impl SwapChannel {
    /// Build a `SwapChannel` from a plain [`TargetConnector`].
    /// Equivalent to
    /// `with_connector(template, BoxCloneSyncService::new(connector))`.
    /// The first inner Channel is built lazily via
    /// `Endpoint::connect_with_connector_lazy`, so this performs
    /// no IO.
    pub fn new(endpoint_template: Endpoint, connector: TargetConnector) -> Self {
        Self::with_connector(endpoint_template, connector)
    }

    /// Build a `SwapChannel` from any `Service<Uri>` that produces
    /// a plain `TokioIo<TcpStream>`. Useful for tests with a mock
    /// connector or for users who want to wrap [`TargetConnector`]
    /// in additional non-TLS tower middleware.
    ///
    /// **Does not support TLS** — see the type-level note on
    /// [`SwapChannel`].
    pub fn with_connector<S>(endpoint_template: Endpoint, connector: S) -> Self
    where
        S: Service<http::Uri, Response = TokioIo<TcpStream>, Error = BoxError>
            + Clone
            + Send
            + Sync
            + 'static,
        S::Future: Send + 'static,
    {
        let erased: ErasedConnector = BoxCloneSyncService::new(connector);
        let initial = endpoint_template.connect_with_connector_lazy(erased.clone());
        Self {
            inner: Arc::new(Inner {
                channel: ArcSwap::from_pointee(initial),
                connector: erased,
                endpoint_template,
            }),
        }
    }

    /// Trigger a rebuild of the inner Channel. Non-blocking; this
    /// schedules a `connect_with_connector_lazy(...)` (which itself
    /// does no IO) and atomically swaps the result in.
    ///
    /// Storm dedup happens **above** this layer in
    /// [`super::super::ResolveStatusMiddleware`].
    ///
    /// Does not affect in-flight requests: they hold clones of the
    /// previous Channel and run to completion on it.
    pub fn rebuild(&self) {
        let new_channel = self
            .inner
            .endpoint_template
            .connect_with_connector_lazy(self.inner.connector.clone());
        self.inner.channel.store(Arc::new(new_channel));
    }
}

impl Service<http::Request<Body>> for SwapChannel {
    type Response = http::Response<Body>;
    type Error = tonic::transport::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Always advertise ready. The actual `Channel` (which is
        // a buffered tonic service requiring `poll_ready` to be
        // paired with `call` on the same instance) is readied
        // inside the future returned by `call`. This avoids
        // cross-clone state leaks: tower wrappers above us may
        // clone `SwapChannel` between `poll_ready` and `call`,
        // and they may also outlive a `rebuild()` that swaps in
        // a different inner `Channel` generation.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        // Snapshot the current Channel; ready it; dispatch.
        // The returned future holds its own clone of that
        // Channel internally (tonic's future captures the
        // buffer sender), so a concurrent `rebuild()` does not
        // affect this in-flight call.
        let mut ch = (**self.inner.channel.load()).clone();
        Box::pin(async move {
            let svc = ch.ready().await?;
            svc.call(req).await
        })
    }
}
