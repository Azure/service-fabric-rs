// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
use futures::future::BoxFuture;
use mssf_core::client::FabricClient;
use mssf_core::client::svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition};
use mssf_core::types::Uri as FabricUri;

use crate::resolve::ServicePartitionResolver;
use crate::retry::OperationRetryer;

use super::resolver::{BoxError, TargetResolver};
use super::selector::{DialTarget, SelectError, TargetSelector};

/// Production [`TargetResolver`] for Service Fabric.
///
/// Wraps the existing [`ServicePartitionResolver`], applies the
/// always-complain rule against a cached
/// COM-backed `ResolvedServicePartition`, and runs the
/// user-supplied selector against that RSP to produce a
/// `DialTarget`.
pub struct FabricTargetResolver {
    inner: ServicePartitionResolver,
    uri: FabricUri,
    key: PartitionKeyType,
    timeout: Option<Duration>,
    selector: TargetSelector,
    /// `previousResult` for the next SF call. `None` until first
    /// successful resolve.
    cached: ArcSwapOption<ResolvedServicePartition>,
}

impl TargetResolver for FabricTargetResolver {
    fn resolve(&self) -> BoxFuture<'_, Result<DialTarget, BoxError>> {
        Box::pin(async move {
            let prev = self.cached.load_full();
            let new_rsp = self
                .inner
                .resolve(&self.uri, &self.key, prev.as_deref(), self.timeout, None)
                .await
                .map_err(|e| Box::new(e) as BoxError)?;
            // Same-version reply: keep cached Arc identity (avoids
            // pointer churn under steady-state always-complain).
            let rsp = match prev.as_deref() {
                Some(p) => match p.compare_version(&new_rsp) {
                    Ok(0) => prev.unwrap(),
                    Ok(_) => {
                        let arc = Arc::new(new_rsp);
                        self.cached.store(Some(arc.clone()));
                        arc
                    }
                    // compare_version returns Err only when the two
                    // RSPs refer to different services / partitions.
                    // Treat as "newer view," replace cache.
                    Err(_) => {
                        let arc = Arc::new(new_rsp);
                        self.cached.store(Some(arc.clone()));
                        arc
                    }
                },
                None => {
                    let arc = Arc::new(new_rsp);
                    self.cached.store(Some(arc.clone()));
                    arc
                }
            };
            // Run the user's role-pick + address-parse closure.
            (self.selector)(&rsp).map_err(|e| match e {
                SelectError::NoMatch => "no matching endpoint".into(),
                SelectError::Fatal(b) => b,
            })
        })
    }
}

/// Builder for [`FabricTargetResolver`].
pub struct FabricTargetResolverBuilder {
    fc: FabricClient,
    uri: Option<FabricUri>,
    key: PartitionKeyType,
    timeout: Option<Duration>,
    retryer: Option<OperationRetryer>,
    selector: Option<TargetSelector>,
}

impl FabricTargetResolverBuilder {
    pub fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            uri: None,
            key: PartitionKeyType::None,
            timeout: None,
            retryer: None,
            selector: None,
        }
    }

    /// Required. The Fabric URI (`fabric:/App/Service`) this
    /// resolver will look up. Concrete type is
    /// `mssf_core::types::Uri`, **not** `http::Uri`.
    pub fn service_uri(mut self, uri: impl Into<FabricUri>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Defaults to `PartitionKeyType::None` (Singleton partitions).
    pub fn partition_key(mut self, key: PartitionKeyType) -> Self {
        self.key = key;
        self
    }

    /// Per-resolve total deadline. `None` means no extra deadline
    /// (rely on `OperationRetryer` policy + caller cancellation).
    pub fn resolve_timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }

    /// Retry / backoff policy applied inside
    /// [`ServicePartitionResolver::resolve`] for transient failures.
    pub fn retryer(mut self, r: OperationRetryer) -> Self {
        self.retryer = Some(r);
        self
    }

    /// **Required.** Role-pick + address-parse closure run inside
    /// `resolve()` against the just-confirmed RSP.
    pub fn target_selector<F>(mut self, f: F) -> Self
    where
        F: Fn(&ResolvedServicePartition) -> Result<DialTarget, SelectError> + Send + Sync + 'static,
    {
        self.selector = Some(Arc::new(f));
        self
    }

    /// Panics if `service_uri` or `target_selector` was not set.
    /// Returns an `Arc<FabricTargetResolver>`. Coerces implicitly
    /// to `Arc<dyn TargetResolver>` at the
    /// [`super::super::TargetConnectorBuilder::resolver`] /
    /// [`super::super::TargetChannelBuilder::resolver`] call
    /// site.
    pub fn build(self) -> Arc<FabricTargetResolver> {
        let uri = self
            .uri
            .expect("FabricTargetResolverBuilder::service_uri is required");
        let selector = self
            .selector
            .expect("FabricTargetResolverBuilder::target_selector is required");
        let retryer = self
            .retryer
            .unwrap_or_else(|| OperationRetryer::builder().build());
        Arc::new(FabricTargetResolver {
            inner: ServicePartitionResolver::new(self.fc, retryer),
            uri,
            key: self.key,
            timeout: self.timeout,
            selector,
            cached: ArcSwapOption::empty(),
        })
    }
}
