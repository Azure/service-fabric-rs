// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Service Fabric naming → gRPC xDS (ADS) mapping.
//!
//! Exposes one configured SF stateful singleton service through an Envoy v3
//! State-of-the-World [Aggregated Discovery Service][ads] so that a stock
//! xDS-capable gRPC client (e.g. [`tonic-xds`]) can reach the service's
//! current primary replica with **no SF-specific client code**.
//!
//! The crate publishes the minimum resource chain such a client accepts:
//! LDS (with an *inline* `RouteConfiguration`) → CDS → EDS. RDS is not used.
//!
//! # Layout
//!
//! - [`endpoint`] — SF-independent endpoint state and the subscription contract.
//! - [`address`] — pluggable interpretation of the opaque SF endpoint address.
//! - [`config`] — the one-service mapping configuration.
//!
//! # Experimental
//!
//! This crate is experimental and there is **no stable API guarantee**: items
//! may change or be removed in any release without a major version bump. It is
//! the in-progress successor to the `mssf_util::tonic` "proxyless" client path
//! tracked by <https://github.com/Azure/service-fabric-rs/issues/300>; that
//! path remains in place and unmodified.
//!
//! [ads]: https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
//! [`tonic-xds`]: https://crates.io/crates/tonic-xds

pub mod address;
pub mod ads;
pub mod config;
pub mod endpoint;
pub mod error;
pub mod fabric;
pub mod resources;

pub use crate::address::{AddressError, AddressInterpreter, host_port_interpreter};
pub use crate::ads::{AdsService, ServerHandle, ShutdownError, bootstrap_json};
pub use crate::config::XdsMapping;
pub use crate::endpoint::{
    EndpointSnapshot, EndpointSource, HostPort, ScriptedEndpointHandle, ScriptedEndpointSource,
};
pub use crate::error::Error;
pub use crate::fabric::{FabricEndpointSource, classify_resolve_error};
