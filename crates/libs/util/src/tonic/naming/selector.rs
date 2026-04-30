// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;

use mssf_core::client::svc_mgmt_client::ResolvedServicePartition;

use super::resolver::BoxError;

/// User-supplied function that picks one connectable target from a
/// `ResolvedServicePartition`. Combines two concerns: choosing
/// *which* replica to dial (by role / partition key / round-robin
/// / ...) and parsing the chosen replica's `address` (which is a
/// user-defined SF endpoint string — not necessarily a URL) into
/// a host:port pair.
///
/// `TargetSelector` is the value type accepted by
/// [`super::default::FabricTargetResolverBuilder::target_selector`].
/// Custom [`super::resolver::TargetResolver`] impls don't have to
/// use this type — they embed whatever closure shape they like
/// internally and just return a `DialTarget` from `resolve()`.
pub type TargetSelector =
    Arc<dyn Fn(&ResolvedServicePartition) -> Result<DialTarget, SelectError> + Send + Sync>;

/// Returns a concrete dial target. `host` is what we pass to DNS
/// or parse as an `IpAddr`; `port` is the TCP port. v1 supports
/// TCP destinations only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialTarget {
    pub host: String,
    pub port: u16,
}

/// Error returned by a [`TargetSelector`] closure.
#[derive(Debug)]
pub enum SelectError {
    /// No endpoint in the current partition matches. The dial
    /// fails with this surfaced as a `BoxError`. The caller's
    /// outer retry loop is responsible for waiting and retrying.
    NoMatch,
    /// Hard error — covers both "selector explicitly rejected"
    /// and "address couldn't be parsed." Won't be fixed by
    /// re-resolving.
    Fatal(BoxError),
}

impl std::fmt::Display for SelectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectError::NoMatch => write!(f, "no matching endpoint"),
            SelectError::Fatal(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SelectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SelectError::NoMatch => None,
            SelectError::Fatal(e) => Some(&**e),
        }
    }
}
