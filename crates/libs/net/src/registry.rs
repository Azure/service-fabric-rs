// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! The set of services one ADS server publishes.
//!
//! xDS is designed for a single control plane serving many resources: clients
//! subscribe by resource name, so one ADS server can back an arbitrary number
//! of Service Fabric services. This module holds that set and the index used
//! to answer a subscription.
//!
//! # Why the index is keyed by `(type_url, name)`
//!
//! Every entry contributes two names: a Listener named after
//! [`XdsMapping::xds_name`] and a Cluster named after
//! [`XdsMapping::cluster_name`] (which EDS reuses). A map keyed by the bare
//! string would conflate the Listener of a mapping named `x-primary` with the
//! Cluster derived from a *different* mapping named `x`. Including the type URL
//! in the key makes that impossible.
//!
//! Consequently the only uniqueness rule the builder has to enforce is that
//! `xds_name` is unique across entries: cluster names are derived from it by
//! appending a constant suffix, which is injective, so distinct xDS names can
//! never yield colliding cluster names.

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::XdsMapping;
use crate::endpoint::EndpointSource;
use crate::error::Error;
use crate::resources::{CLUSTER_TYPE_URL, ENDPOINT_TYPE_URL, LISTENER_TYPE_URL};

/// One mapped service: its xDS naming and the source of its endpoint state.
#[derive(Clone)]
pub struct RegisteredService {
    mapping: Arc<XdsMapping>,
    source: Arc<dyn EndpointSource>,
}

impl RegisteredService {
    /// The service's xDS naming configuration.
    pub fn mapping(&self) -> &XdsMapping {
        &self.mapping
    }

    /// The service's endpoint source.
    pub fn source(&self) -> &Arc<dyn EndpointSource> {
        &self.source
    }
}

impl std::fmt::Debug for RegisteredService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredService")
            .field("mapping", &self.mapping)
            .finish_non_exhaustive()
    }
}

/// An immutable set of services served by one [`crate::ads::AdsService`].
///
/// Build one with [`ServiceRegistry::builder`], or use
/// [`ServiceRegistry::single`] for the one-service case.
#[derive(Clone, Debug)]
pub struct ServiceRegistry {
    entries: Vec<RegisteredService>,
    /// `(type_url, resource_name)` → index into `entries`.
    index: HashMap<(&'static str, String), usize>,
}

impl ServiceRegistry {
    /// Start building a registry.
    pub fn builder() -> ServiceRegistryBuilder {
        ServiceRegistryBuilder::default()
    }

    /// A registry holding exactly one service.
    ///
    /// Cannot fail: a single entry has nothing to collide with, and the mapping
    /// was already validated when it was constructed.
    pub fn single(mapping: XdsMapping, source: Arc<dyn EndpointSource>) -> Self {
        let mut index = HashMap::new();
        insert_names(&mut index, &mapping, 0);
        Self {
            entries: vec![RegisteredService {
                mapping: Arc::new(mapping),
                source,
            }],
            index,
        }
    }

    /// The registered services, in registration order.
    pub fn entries(&self) -> &[RegisteredService] {
        &self.entries
    }

    /// How many services are registered. Always at least one.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Always `false`: a registry cannot be built empty.
    ///
    /// Present because clippy asks for it alongside [`ServiceRegistry::len`],
    /// and because callers reading `len()` should not have to know the
    /// invariant to satisfy the lint.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Which entry owns `name` for the given resource type, if any.
    pub fn index_of(&self, type_url: &str, name: &str) -> Option<usize> {
        // The key borrows a `&'static str` for the type URL, so look up by the
        // canonical constant rather than the caller's (possibly unknown) string.
        let type_url = canonical_type_url(type_url)?;
        self.index.get(&(type_url, name.to_string())).copied()
    }
}

/// Accumulates services, rejecting name collisions as they are added.
#[derive(Default, Debug)]
pub struct ServiceRegistryBuilder {
    entries: Vec<RegisteredService>,
    index: HashMap<(&'static str, String), usize>,
}

impl ServiceRegistryBuilder {
    /// Register one service.
    ///
    /// Fails if `mapping`'s xDS name is already registered. Validating here
    /// rather than at serve time keeps a configuration mistake cheap to
    /// diagnose: a duplicate would otherwise silently shadow a resource on a
    /// live stream, which looks like a routing bug rather than a config bug.
    pub fn add(
        mut self,
        mapping: XdsMapping,
        source: Arc<dyn EndpointSource>,
    ) -> Result<Self, Error> {
        let position = self.entries.len();
        for (type_url, name) in names_of(&mapping) {
            if self.index.contains_key(&(type_url, name.clone())) {
                return Err(Error::Config(format!(
                    "duplicate xds resource name {name:?} for type {type_url}"
                )));
            }
        }
        insert_names(&mut self.index, &mapping, position);
        self.entries.push(RegisteredService {
            mapping: Arc::new(mapping),
            source,
        });
        Ok(self)
    }

    /// Finish, rejecting an empty registry.
    ///
    /// An ADS server with nothing to serve would answer every subscription with
    /// an empty response, which a client reports as a resource-not-found — a
    /// confusing way to learn that nothing was registered.
    pub fn build(self) -> Result<ServiceRegistry, Error> {
        if self.entries.is_empty() {
            return Err(Error::Config(
                "an ads registry must contain at least one service".into(),
            ));
        }
        Ok(ServiceRegistry {
            entries: self.entries,
            index: self.index,
        })
    }
}

/// The `(type_url, name)` pairs one mapping answers to.
///
/// EDS is looked up by the *cluster* name, because a cluster without an
/// explicit EDS service name falls back to its own name.
fn names_of(mapping: &XdsMapping) -> [(&'static str, String); 3] {
    [
        (LISTENER_TYPE_URL, mapping.xds_name().to_string()),
        (CLUSTER_TYPE_URL, mapping.cluster_name()),
        (ENDPOINT_TYPE_URL, mapping.cluster_name()),
    ]
}

fn insert_names(
    index: &mut HashMap<(&'static str, String), usize>,
    mapping: &XdsMapping,
    position: usize,
) {
    for key in names_of(mapping) {
        index.insert(key, position);
    }
}

/// Map a request's type URL onto the crate's `'static` constant.
fn canonical_type_url(type_url: &str) -> Option<&'static str> {
    match type_url {
        LISTENER_TYPE_URL => Some(LISTENER_TYPE_URL),
        CLUSTER_TYPE_URL => Some(CLUSTER_TYPE_URL),
        ENDPOINT_TYPE_URL => Some(ENDPOINT_TYPE_URL),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::host_port_interpreter;
    use crate::endpoint::{EndpointSnapshot, ScriptedEndpointSource};

    fn source() -> Arc<dyn EndpointSource> {
        ScriptedEndpointSource::new(EndpointSnapshot::NoPrimary).0
    }

    fn mapping(name: &str) -> XdsMapping {
        XdsMapping::new(name, "fabric:/App/Svc", host_port_interpreter()).unwrap()
    }

    #[test]
    fn registers_several_services() {
        let registry = ServiceRegistry::builder()
            .add(mapping("a"), source())
            .unwrap()
            .add(mapping("b"), source())
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());
        assert_eq!(registry.index_of(LISTENER_TYPE_URL, "a"), Some(0));
        assert_eq!(registry.index_of(LISTENER_TYPE_URL, "b"), Some(1));
        assert_eq!(registry.index_of(CLUSTER_TYPE_URL, "a-primary"), Some(0));
        assert_eq!(registry.index_of(ENDPOINT_TYPE_URL, "b-primary"), Some(1));
        assert_eq!(registry.entries()[0].mapping().xds_name(), "a");
    }

    #[test]
    fn rejects_a_duplicate_xds_name() {
        let err = ServiceRegistry::builder()
            .add(mapping("a"), source())
            .unwrap()
            .add(mapping("a"), source())
            .unwrap_err();
        assert!(err.to_string().contains("duplicate"), "{err}");
    }

    #[test]
    fn rejects_an_empty_registry() {
        assert!(ServiceRegistry::builder().build().is_err());
    }

    /// The reason the index is keyed by `(type_url, name)`: a Listener and a
    /// Cluster may legitimately share a string while belonging to different
    /// services. A bare-name map would collapse them.
    #[test]
    fn a_listener_and_a_cluster_may_share_a_name() {
        let registry = ServiceRegistry::builder()
            .add(mapping("x"), source())
            .unwrap()
            // Listener "x-primary" collides textually with entry 0's cluster.
            .add(mapping("x-primary"), source())
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(registry.index_of(CLUSTER_TYPE_URL, "x-primary"), Some(0));
        assert_eq!(registry.index_of(LISTENER_TYPE_URL, "x-primary"), Some(1));
        assert_eq!(
            registry.index_of(CLUSTER_TYPE_URL, "x-primary-primary"),
            Some(1)
        );
    }

    #[test]
    fn unknown_names_and_types_are_not_found() {
        let registry = ServiceRegistry::single(mapping("a"), source());
        assert_eq!(registry.index_of(LISTENER_TYPE_URL, "nope"), None);
        assert_eq!(
            registry.index_of("type.googleapis.com/what.Ever", "a"),
            None
        );
    }

    #[test]
    fn single_holds_exactly_one_entry() {
        let registry = ServiceRegistry::single(mapping("a"), source());
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.index_of(LISTENER_TYPE_URL, "a"), Some(0));
    }
}
