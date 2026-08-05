// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Configuration for the single mapped service.

use crate::address::AddressInterpreter;
use crate::error::Error;

/// Maps one Service Fabric service onto one xDS resource name.
///
/// The prototype serves exactly one mapping per ADS server instance.
#[derive(Clone)]
pub struct XdsMapping {
    /// The xDS resource (Listener) name. A client targets `xds:///<name>`.
    xds_name: String,
    /// The Service Fabric service URI, e.g. `fabric:/App/Service`.
    service_uri: String,
    /// Interprets the service's opaque endpoint address.
    interpreter: AddressInterpreter,
}

impl std::fmt::Debug for XdsMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdsMapping")
            .field("xds_name", &self.xds_name)
            .field("service_uri", &self.service_uri)
            .field("cluster_name", &self.cluster_name())
            .finish_non_exhaustive()
    }
}

impl XdsMapping {
    /// Create a mapping, validating the names.
    pub fn new(
        xds_name: impl Into<String>,
        service_uri: impl Into<String>,
        interpreter: AddressInterpreter,
    ) -> Result<Self, Error> {
        let xds_name = xds_name.into();
        let service_uri = service_uri.into();
        if xds_name.trim().is_empty() {
            return Err(Error::Config("xds resource name must not be empty".into()));
        }
        if service_uri.trim().is_empty() {
            return Err(Error::Config("service uri must not be empty".into()));
        }
        Ok(Self {
            xds_name,
            service_uri,
            interpreter,
        })
    }

    /// The Listener / route-target resource name.
    pub fn xds_name(&self) -> &str {
        &self.xds_name
    }

    /// The Service Fabric service URI.
    pub fn service_uri(&self) -> &str {
        &self.service_uri
    }

    /// The address interpreter supplied by the caller.
    pub fn interpreter(&self) -> &AddressInterpreter {
        &self.interpreter
    }

    /// The CDS/EDS cluster name, derived deterministically from the xDS name.
    ///
    /// CDS and EDS must agree on this string: the xDS client falls back to the
    /// cluster name when a cluster carries no explicit EDS service name.
    pub fn cluster_name(&self) -> String {
        format!("{}-primary", self.xds_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::host_port_interpreter;

    #[test]
    fn derives_cluster_name_deterministically() {
        let m = XdsMapping::new("reflection", "fabric:/App/Svc", host_port_interpreter()).unwrap();
        assert_eq!(m.cluster_name(), "reflection-primary");
        assert_eq!(m.cluster_name(), m.cluster_name());
    }

    #[test]
    fn rejects_empty_names() {
        assert!(XdsMapping::new("", "fabric:/A/B", host_port_interpreter()).is_err());
        assert!(XdsMapping::new("  ", "fabric:/A/B", host_port_interpreter()).is_err());
        assert!(XdsMapping::new("n", "", host_port_interpreter()).is_err());
    }

    #[test]
    fn accessors_round_trip() {
        let m = XdsMapping::new("n", "fabric:/A/B", host_port_interpreter()).unwrap();
        assert_eq!(m.xds_name(), "n");
        assert_eq!(m.service_uri(), "fabric:/A/B");
    }
}
