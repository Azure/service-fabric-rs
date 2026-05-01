// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tower::Service;

use crate::tonic::naming::{BoxError, TargetResolver};

/// Hyper-compatible connector. Implements `tower::Service<http::Uri>`
/// returning a connected IO. Suitable for
/// [`tonic::transport::Endpoint::connect_with_connector_lazy`].
///
/// The connector is **pure** transport: ask the resolver for a
/// `DialTarget`, open a TCP connection. It has no knowledge of
/// channels, retries, or trailers. Channel-level invalidation
/// lives one layer up, in `SwapChannel`.
#[derive(Clone)]
pub struct TargetConnector {
    inner: Arc<Inner>,
}

struct Inner {
    resolver: Arc<dyn TargetResolver>,
}

impl TargetConnector {
    /// Construct directly from a resolver, bypassing the builder.
    pub fn new(resolver: Arc<dyn TargetResolver>) -> Self {
        Self {
            inner: Arc::new(Inner { resolver }),
        }
    }
}

impl Service<http::Uri> for TargetConnector {
    /// Concrete IO type returned to hyper. Plain TCP wrapped in
    /// hyper-util's `TokioIo` adapter for the hyper IO traits.
    /// For TLS, the user composes a TLS connector on top via
    /// [`super::super::SwapChannel::with_connector`].
    type Response = TokioIo<TcpStream>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _placeholder_uri: http::Uri) -> Self::Future {
        // The placeholder URI is hyper's pool key, NOT our SF Fabric
        // URI. We ignore it.
        let inner = self.inner.clone();
        Box::pin(async move {
            let target = inner.resolver.resolve().await?;
            let stream = TcpStream::connect((target.host.as_str(), target.port))
                .await
                .map_err(|e| Box::new(e) as BoxError)?;
            Ok(TokioIo::new(stream))
        })
    }
}

/// Builder for [`TargetConnector`].
pub struct TargetConnectorBuilder {
    resolver: Option<Arc<dyn TargetResolver>>,
}

impl TargetConnectorBuilder {
    pub fn new() -> Self {
        Self { resolver: None }
    }

    /// Required. The SF naming abstraction. Use
    /// [`super::super::FabricTargetResolverBuilder`] for the
    /// production impl, or any custom [`TargetResolver`].
    pub fn resolver(mut self, r: Arc<dyn TargetResolver>) -> Self {
        self.resolver = Some(r);
        self
    }

    /// Panics if `resolver` was not set. Sync; no IO until first
    /// dial.
    pub fn build(self) -> TargetConnector {
        let resolver = self
            .resolver
            .expect("TargetConnectorBuilder::resolver is required");
        TargetConnector::new(resolver)
    }
}

impl Default for TargetConnectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
