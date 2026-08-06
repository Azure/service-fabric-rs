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
    /// Create a mapping whose xDS resource name **is** the SF service URI.
    ///
    /// This is usually what you want: clients then target
    /// `xds:///fabric:/App/Service`, so the name a caller writes is the service
    /// it wants and there is no alias to keep in sync. Verified end-to-end —
    /// `:` and `/` are fine in xDS resource names.
    ///
    /// Use [`XdsMapping::new`] when you deliberately want a short alias
    /// (`xds:///myservice`) decoupled from the SF URI.
    pub fn for_service_uri(
        service_uri: impl Into<String>,
        interpreter: AddressInterpreter,
    ) -> Result<Self, Error> {
        let service_uri = service_uri.into();
        Self::new(service_uri.clone(), service_uri, interpreter)
    }

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

    /// The SF URI doubles as the xDS name, so clients target
    /// `xds:///fabric:/App/Service` with no alias to maintain.
    #[test]
    fn for_service_uri_uses_the_uri_as_the_xds_name() {
        let m = XdsMapping::for_service_uri("fabric:/MyApp/MyService", host_port_interpreter())
            .unwrap();
        assert_eq!(m.xds_name(), "fabric:/MyApp/MyService");
        assert_eq!(m.service_uri(), "fabric:/MyApp/MyService");
        assert_eq!(m.cluster_name(), "fabric:/MyApp/MyService-primary");
    }

    #[test]
    fn for_service_uri_rejects_an_empty_uri() {
        assert!(XdsMapping::for_service_uri("", host_port_interpreter()).is_err());
    }
}
