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
            let had_cache = prev.is_some();
            let new_rsp = self
                .inner
                .resolve(&self.uri, &self.key, prev.as_deref(), self.timeout, None)
                .await
                .map_err(|e| {
                    tracing::warn!(
                        uri = %self.uri,
                        had_cache,
                        error = ?e,
                        "FabricTargetResolver: SF resolve_service_partition failed",
                    );
                    Box::new(e) as BoxError
                })?;
            // Reconcile the new reply against the cache. The
            // cache only advances when SF returns a strictly
            // *newer* RSP — never moves backward to an older
            // version — so a stale or out-of-order reply doesn't
            // poison subsequent dials.
            //
            // `ResolvedServicePartition: PartialOrd` per
            // `svc_mgmt_client.rs`: `a > b` ⇔ `a` is newer;
            // `partial_cmp == None` ⇔ different service /
            // partition (treat as a hard cache reset).
            let (rsp, cache_outcome) = match prev.as_deref() {
                Some(p) => match p.partial_cmp(&new_rsp) {
                    Some(std::cmp::Ordering::Less) => {
                        // prev < new_rsp → new_rsp is newer → advance.
                        let arc = Arc::new(new_rsp);
                        self.cached.store(Some(arc.clone()));
                        (arc, "advanced")
                    }
                    Some(_) => {
                        // Equal or prev > new_rsp: keep cached
                        // Arc identity, drop new_rsp.
                        (prev.unwrap(), "kept")
                    }
                    None => {
                        // Different service / partition: hard
                        // reset.
                        let arc = Arc::new(new_rsp);
                        self.cached.store(Some(arc.clone()));
                        (arc, "hard-reset")
                    }
                },
                None => {
                    let arc = Arc::new(new_rsp);
                    self.cached.store(Some(arc.clone()));
                    (arc, "first")
                }
            };
            // Run the user's role-pick + address-parse closure.
            match (self.selector)(&rsp) {
                Ok(target) => {
                    tracing::info!(
                        uri = %self.uri,
                        had_cache,
                        cache = cache_outcome,
                        host = %target.host,
                        port = target.port,
                        "FabricTargetResolver: resolved dial target",
                    );
                    Ok(target)
                }
                Err(SelectError::NoMatch) => {
                    tracing::warn!(
                        uri = %self.uri,
                        had_cache,
                        cache = cache_outcome,
                        endpoint_count = rsp.endpoints.len(),
                        "FabricTargetResolver: selector found no matching endpoint",
                    );
                    Err("no matching endpoint".into())
                }
                Err(SelectError::Fatal(b)) => {
                    tracing::warn!(
                        uri = %self.uri,
                        had_cache,
                        cache = cache_outcome,
                        error = %b,
                        "FabricTargetResolver: selector returned fatal error",
                    );
                    Err(b)
                }
            }
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
