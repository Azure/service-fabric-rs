// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use futures::future::BoxFuture;

use super::selector::DialTarget;

/// Common boxed error alias used throughout the `tonic` module.
/// Matches what `tower::Service` impls (including hyper) use, so
/// it propagates without wrapping.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// SF naming abstraction used by [`super::super::TargetConnector`].
/// Each call returns the concrete [`DialTarget`] the connector
/// should dial next.
///
/// The trait is intentionally minimal: one method, no arguments,
/// no associated types. The implementation owns everything between
/// "what does SF currently say about this partition?" and "where
/// do I actually open a TCP connection?":
/// - which Fabric URI to look up,
/// - which partition key,
/// - the `previousResult` cache and always-complain rule,
/// - the per-resolve timeout / cancellation policy,
/// - the role-pick + address-parse selector that turns an RSP
///   into a `DialTarget`.
///
/// Returning `DialTarget` (not `Arc<View>`) keeps the connector
/// non-generic. Custom resolvers that want different selection
/// logic implement the trait directly.
///
/// **v1 has no cancellation token on `resolve()`.** Per-call
/// cancellation works implicitly via future-drop.
pub trait TargetResolver: Send + Sync + 'static {
    fn resolve(&self) -> BoxFuture<'_, Result<DialTarget, BoxError>>;
}
