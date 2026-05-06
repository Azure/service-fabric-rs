// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Reflection sample as a library so the binary (`samples_reflection`) can
//! be paired with integration tests under `tests/`.
//!
//! The bin target ([`main.rs`](../bin/main.rs.html) — runs as the SF
//! service host) imports from this crate; integration tests under
//! [`tests/`](../../tests/) drive the deployed cluster via the public
//! API exposed here (notably the `grpc_control` module's gRPC client
//! types and the `transport`-style constants for endpoint discovery).

pub mod control;
pub mod echo;
pub mod grpc;
pub mod grpc_control;
pub mod lifecycle;
pub mod statefulstore;
pub mod test_admin;
pub mod test_cluster;

pub use statefulstore::Factory;

/// Service-type name registered by the reflection sample. Must match the
/// value declared in `manifests/ServiceManifest.xml`.
pub const SERVICE_TYPE_NAME: &str = "ReflectionAppService";
