// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Pluggable interpretation of the opaque SF endpoint address.
//!
//! A Service Fabric endpoint address is a **service-defined string**. It may be
//! a plain `host:port`, a URL, a JSON envelope, or anything else the service
//! author chose to publish. The mapping layer therefore cannot infer it, and
//! interpretation is a required, caller-supplied concern.

use std::sync::Arc;

use crate::endpoint::HostPort;

/// Interprets a raw SF endpoint address string into a connectable target.
///
/// Takes the raw address **string**, not a `&ResolvedServicePartition`, on
/// purpose: the SF change-notification payload carries a list of endpoints and
/// **no** `ResolvedServicePartition`, so a selector shaped around an RSP could
/// not serve the notification path at all. Splitting role selection (owned by
/// this crate — always the stateful primary) from address interpretation
/// (owned by the caller) is what lets both the resolve path and the
/// notification path share one interpreter.
pub type AddressInterpreter =
    Arc<dyn Fn(&str) -> Result<HostPort, AddressError> + Send + Sync + 'static>;

/// Failure to interpret an SF endpoint address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    /// The address could not be parsed in the expected format.
    Unparseable(String),
    /// The address parsed but carried no host.
    MissingHost(String),
    /// The address parsed but carried no usable port.
    MissingPort(String),
}

impl std::fmt::Display for AddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressError::Unparseable(a) => write!(f, "unparseable endpoint address: {a:?}"),
            AddressError::MissingHost(a) => write!(f, "endpoint address has no host: {a:?}"),
            AddressError::MissingPort(a) => write!(f, "endpoint address has no port: {a:?}"),
        }
    }
}

impl std::error::Error for AddressError {}

/// An interpreter for services that publish a bare `host:port` address.
///
/// Provided for convenience and as a worked example. Note that this is **not**
/// what every service does — the reflection sample in this repository publishes
/// a URL carrying query parameters, and needs its own interpreter.
pub fn host_port_interpreter() -> AddressInterpreter {
    Arc::new(|raw: &str| {
        let raw = raw.trim();
        let (host, port) = raw
            .rsplit_once(':')
            .ok_or_else(|| AddressError::Unparseable(raw.to_string()))?;
        if host.is_empty() {
            return Err(AddressError::MissingHost(raw.to_string()));
        }
        let port: u16 = port
            .parse()
            .map_err(|_| AddressError::MissingPort(raw.to_string()))?;
        if port == 0 {
            return Err(AddressError::MissingPort(raw.to_string()));
        }
        Ok(HostPort::new(host, port))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_host_and_port() {
        let f = host_port_interpreter();
        assert_eq!(f("localhost:20001").unwrap(), HostPort::new("localhost", 20001));
        assert_eq!(f("10.0.0.4:1").unwrap(), HostPort::new("10.0.0.4", 1));
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let f = host_port_interpreter();
        assert_eq!(f("  host:9  ").unwrap(), HostPort::new("host", 9));
    }

    #[test]
    fn rejects_missing_colon() {
        let f = host_port_interpreter();
        assert_eq!(
            f("localhost"),
            Err(AddressError::Unparseable("localhost".into()))
        );
    }

    #[test]
    fn rejects_empty_host() {
        let f = host_port_interpreter();
        assert_eq!(f(":20001"), Err(AddressError::MissingHost(":20001".into())));
    }

    #[test]
    fn rejects_non_numeric_or_zero_port() {
        let f = host_port_interpreter();
        assert_eq!(
            f("host:abc"),
            Err(AddressError::MissingPort("host:abc".into()))
        );
        assert_eq!(f("host:0"), Err(AddressError::MissingPort("host:0".into())));
        assert_eq!(
            f("host:99999"),
            Err(AddressError::MissingPort("host:99999".into())),
            "out-of-range ports must be rejected, not truncated"
        );
    }

    /// A URL-shaped address is exactly the case a bare host:port interpreter
    /// must not silently mis-handle; callers publishing URLs supply their own.
    #[test]
    fn rejects_url_shaped_address() {
        let f = host_port_interpreter();
        assert!(f("http://host:1234/path?partition=abc").is_err());
    }
}
