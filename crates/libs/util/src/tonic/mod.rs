// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Service Fabric tonic connector with auto-failover.
//!
//! See `docs/design/TonicConnectorDesign.md` for the design.
//!
//! The public surface is intentionally flat — internal
//! sub-modules are an organizational detail. Use the re-exports
//! below.

mod channel;
mod connector;
mod middleware;
mod naming;

pub use self::channel::{SwapChannel, TargetChannel, TargetChannelBuilder};
pub use self::connector::{TargetConnector, TargetConnectorBuilder};
pub use self::middleware::ResolveStatusMiddleware;
pub use self::naming::{
    BoxError, DialTarget, FabricTargetResolver, FabricTargetResolverBuilder, SelectError,
    TargetResolver, TargetSelector,
};
