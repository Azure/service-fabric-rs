// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;

use tonic::transport::Endpoint;

use crate::tonic::connector::TargetConnectorBuilder;
use crate::tonic::middleware::ResolveStatusMiddleware;
use crate::tonic::naming::TargetResolver;

use super::swap::SwapChannel;

/// Convenience composed channel: trailer middleware on top of a
/// hot-swappable `tonic::Channel`. Implements
/// `Service<Request<tonic::body::Body>>` so it can go straight into
/// generated tonic clients.
pub type TargetChannel = ResolveStatusMiddleware<SwapChannel>;

/// Builder for [`TargetChannel`].
///
/// The resolver embeds the target selector; the channel builder
/// has no selector setter of its own.
///
/// **TLS is not yet supported.** v1 ships plain TCP only via
/// [`TargetConnector`]. The TLS recipe sketched in the design
/// doc requires generalizing `SwapChannel` over its inner IO
/// type (today fixed at `TokioIo<TcpStream>`); see Future work
/// in `docs/design/TonicConnectorDesign.md`.
pub struct TargetChannelBuilder {
    resolver: Option<Arc<dyn TargetResolver>>,
    endpoint_template: Option<Endpoint>,
    trailer_header: Option<http::HeaderName>,
}

impl TargetChannelBuilder {
    pub fn new() -> Self {
        Self {
            resolver: None,
            endpoint_template: None,
            trailer_header: None,
        }
    }

    /// Required. Same trait object as
    /// [`TargetConnectorBuilder::resolver`].
    pub fn resolver(mut self, r: Arc<dyn TargetResolver>) -> Self {
        self.resolver = Some(r);
        self
    }

    /// **Required.** Trailer header name the middleware will
    /// inspect for rebuild signals. Pass `"mssf-status"` for the
    /// SF Rust SDK convention. Panics at `build()` time if not a
    /// valid HTTP header name.
    pub fn trailer_header(mut self, name: impl AsRef<str>) -> Self {
        let parsed = http::HeaderName::try_from(name.as_ref())
            .expect("TargetChannelBuilder::trailer_header: invalid header name");
        self.trailer_header = Some(parsed);
        self
    }

    /// Endpoint template applied to every generation of the inner
    /// `tonic::Channel`. The URI is a placeholder; the connector
    /// ignores it. Defaults to
    /// `Endpoint::from_static("http://fabric.invalid")`.
    pub fn endpoint_template(mut self, ep: Endpoint) -> Self {
        self.endpoint_template = Some(ep);
        self
    }

    /// Build a ready-to-use service. Sync; no IO until the first
    /// request.
    pub fn build(self) -> TargetChannel {
        let resolver = self
            .resolver
            .expect("TargetChannelBuilder::resolver is required");
        let trailer_header = self
            .trailer_header
            .expect("TargetChannelBuilder::trailer_header is required");
        let connector = TargetConnectorBuilder::new().resolver(resolver).build();
        let ep = self.endpoint_template.unwrap_or_else(default_endpoint);
        let swap = SwapChannel::new(ep, connector);
        ResolveStatusMiddleware::new(swap.clone(), trailer_header, move || swap.rebuild())
    }
}

impl Default for TargetChannelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn default_endpoint() -> Endpoint {
    Endpoint::from_static("http://fabric.invalid")
}
