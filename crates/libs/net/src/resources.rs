// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Construction of the xDS resource graph.
//!
//! Everything here is a **pure function** of `(XdsMapping, EndpointSnapshot)`:
//! no IO, no global state, no Service Fabric types.
//!
//! The published chain is the minimum a gRPC xDS client accepts:
//! LDS (with an *inline* `RouteConfiguration`) → CDS → EDS. RDS is not used.
//!
//! Several fixed values below are not stylistic — they avoid client-side
//! rejections (NACKs):
//!
//! | Fixed value | Why |
//! |---|---|
//! | `lb_policy = ROUND_ROBIN` | anything other than `ROUND_ROBIN`/`LEAST_REQUEST` is NACK'd |
//! | `domains = ["*"]` | virtual-host matching does **not** strip `:port` from the authority |
//! | numeric `PortValue` | a named port is NACK'd |
//! | CDS name == EDS `cluster_name` | the client falls back to the cluster name for EDS lookup |

use envoy_types::pb::envoy::config::cluster::v3::{Cluster, cluster};
use envoy_types::pb::envoy::config::core::v3::{Address, SocketAddress, address, socket_address};
use envoy_types::pb::envoy::config::endpoint::v3::{
    ClusterLoadAssignment, Endpoint, LbEndpoint, LocalityLbEndpoints, lb_endpoint,
};
use envoy_types::pb::envoy::config::listener::v3::{ApiListener, Listener};
use envoy_types::pb::envoy::config::route::v3::{
    Route, RouteAction, RouteConfiguration, RouteMatch, VirtualHost, route, route_action,
    route_match,
};
use envoy_types::pb::envoy::extensions::filters::network::http_connection_manager::v3::{
    HttpConnectionManager, http_connection_manager,
};
use prost::Message;

use crate::config::XdsMapping;
use crate::endpoint::{EndpointSnapshot, HostPort};

/// Resource type URL for `Listener`.
pub const LISTENER_TYPE_URL: &str = "type.googleapis.com/envoy.config.listener.v3.Listener";
/// Resource type URL for `Cluster`.
pub const CLUSTER_TYPE_URL: &str = "type.googleapis.com/envoy.config.cluster.v3.Cluster";
/// Resource type URL for `ClusterLoadAssignment`.
pub const ENDPOINT_TYPE_URL: &str =
    "type.googleapis.com/envoy.config.endpoint.v3.ClusterLoadAssignment";
/// Type URL of the `HttpConnectionManager` packed inside the `ApiListener`.
pub const HCM_TYPE_URL: &str = "type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager";

/// Build the Listener, with its `RouteConfiguration` inlined.
///
/// Inlining the route config is what removes the need for RDS entirely.
pub fn build_listener(mapping: &XdsMapping) -> Listener {
    let route_config = RouteConfiguration {
        name: mapping.xds_name().to_string(),
        virtual_hosts: vec![VirtualHost {
            name: mapping.xds_name().to_string(),
            // Must be "*": the client does not strip `:port` from the
            // authority, so a literal host match would miss `xds:///name:port`.
            domains: vec!["*".to_string()],
            routes: vec![Route {
                r#match: Some(RouteMatch {
                    path_specifier: Some(route_match::PathSpecifier::Prefix("/".to_string())),
                    ..Default::default()
                }),
                action: Some(route::Action::Route(RouteAction {
                    cluster_specifier: Some(route_action::ClusterSpecifier::Cluster(
                        mapping.cluster_name(),
                    )),
                    ..Default::default()
                })),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    let hcm = HttpConnectionManager {
        stat_prefix: mapping.xds_name().to_string(),
        route_specifier: Some(http_connection_manager::RouteSpecifier::RouteConfig(
            route_config,
        )),
        ..Default::default()
    };

    Listener {
        name: mapping.xds_name().to_string(),
        api_listener: Some(ApiListener {
            api_listener: Some(prost_types_any(HCM_TYPE_URL, hcm.encode_to_vec())),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Build the EDS Cluster.
pub fn build_cluster(mapping: &XdsMapping) -> Cluster {
    Cluster {
        name: mapping.cluster_name(),
        cluster_discovery_type: Some(cluster::ClusterDiscoveryType::Type(
            cluster::DiscoveryType::Eds as i32,
        )),
        // Only ROUND_ROBIN and LEAST_REQUEST are accepted. With a single
        // primary endpoint, round-robin degenerates to "always the primary".
        lb_policy: cluster::LbPolicy::RoundRobin as i32,
        ..Default::default()
    }
}

/// Build the `ClusterLoadAssignment` for the current endpoint state.
///
/// [`EndpointSnapshot::NoPrimary`] and [`EndpointSnapshot::NotFound`] both
/// produce a **valid but empty** assignment — an endpoint is never fabricated.
/// (`NotFound` additionally causes the Listener to be withheld; see the ADS
/// layer.)
pub fn build_endpoints(mapping: &XdsMapping, snapshot: &EndpointSnapshot) -> ClusterLoadAssignment {
    let lb_endpoints = match snapshot {
        EndpointSnapshot::Primary(hp) => vec![lb_endpoint(hp)],
        EndpointSnapshot::NoPrimary | EndpointSnapshot::NotFound => vec![],
    };

    ClusterLoadAssignment {
        cluster_name: mapping.cluster_name(),
        endpoints: if lb_endpoints.is_empty() {
            vec![]
        } else {
            vec![LocalityLbEndpoints {
                lb_endpoints,
                ..Default::default()
            }]
        },
        ..Default::default()
    }
}

fn lb_endpoint(hp: &HostPort) -> LbEndpoint {
    LbEndpoint {
        host_identifier: Some(lb_endpoint::HostIdentifier::Endpoint(Endpoint {
            address: Some(Address {
                address: Some(address::Address::SocketAddress(SocketAddress {
                    address: hp.host.clone(),
                    // Must be numeric; a named port is rejected.
                    port_specifier: Some(socket_address::PortSpecifier::PortValue(hp.port as u32)),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })),
        ..Default::default()
    }
}

fn prost_types_any(type_url: &str, value: Vec<u8>) -> envoy_types::pb::google::protobuf::Any {
    envoy_types::pb::google::protobuf::Any {
        type_url: type_url.to_string(),
        value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::host_port_interpreter;

    fn mapping() -> XdsMapping {
        XdsMapping::new("reflection", "fabric:/App/Svc", host_port_interpreter()).unwrap()
    }

    /// The route config must be inlined; if it were RDS the client would need
    /// a separate RouteConfiguration resource we never serve.
    #[test]
    fn listener_inlines_the_route_configuration() {
        let l = build_listener(&mapping());
        let any = l.api_listener.unwrap().api_listener.unwrap();
        assert_eq!(any.type_url, HCM_TYPE_URL);

        let hcm = HttpConnectionManager::decode(any.value.as_slice()).unwrap();
        match hcm.route_specifier.unwrap() {
            http_connection_manager::RouteSpecifier::RouteConfig(rc) => {
                assert_eq!(rc.virtual_hosts.len(), 1);
            }
            other => panic!("expected an inline RouteConfig, got {other:?}"),
        }
    }

    /// NACK trap: the authority retains `:port`, so only "*" reliably matches.
    #[test]
    fn virtual_host_domain_is_universal() {
        let l = build_listener(&mapping());
        let any = l.api_listener.unwrap().api_listener.unwrap();
        let hcm = HttpConnectionManager::decode(any.value.as_slice()).unwrap();
        let http_connection_manager::RouteSpecifier::RouteConfig(rc) =
            hcm.route_specifier.unwrap()
        else {
            panic!("expected inline route config");
        };
        assert_eq!(rc.virtual_hosts[0].domains, vec!["*".to_string()]);
    }

    #[test]
    fn route_matches_prefix_slash_and_targets_the_cluster() {
        let l = build_listener(&mapping());
        let any = l.api_listener.unwrap().api_listener.unwrap();
        let hcm = HttpConnectionManager::decode(any.value.as_slice()).unwrap();
        let http_connection_manager::RouteSpecifier::RouteConfig(rc) =
            hcm.route_specifier.unwrap()
        else {
            panic!("expected inline route config");
        };
        let route = &rc.virtual_hosts[0].routes[0];
        assert!(matches!(
            route.r#match.as_ref().unwrap().path_specifier,
            Some(route_match::PathSpecifier::Prefix(ref p)) if p == "/"
        ));
        assert!(matches!(
            route.action.as_ref().unwrap(),
            route::Action::Route(a) if matches!(
                a.cluster_specifier.as_ref().unwrap(),
                route_action::ClusterSpecifier::Cluster(c) if c == "reflection-primary"
            )
        ));
    }

    /// NACK trap: any lb_policy other than ROUND_ROBIN/LEAST_REQUEST is rejected.
    #[test]
    fn cluster_uses_round_robin_and_eds() {
        let c = build_cluster(&mapping());
        assert_eq!(c.lb_policy, cluster::LbPolicy::RoundRobin as i32);
        assert_eq!(c.lb_policy, 0, "ROUND_ROBIN must be the proto default");
        assert!(matches!(
            c.cluster_discovery_type,
            Some(cluster::ClusterDiscoveryType::Type(t)) if t == cluster::DiscoveryType::Eds as i32
        ));
    }

    /// CDS and EDS must agree, since the client falls back to the cluster name.
    #[test]
    fn cluster_and_endpoint_names_agree() {
        let m = mapping();
        let c = build_cluster(&m);
        let e = build_endpoints(&m, &EndpointSnapshot::NoPrimary);
        assert_eq!(c.name, e.cluster_name);
        assert_eq!(c.name, "reflection-primary");
    }

    /// NACK trap: the port must be numeric.
    #[test]
    fn primary_endpoint_uses_numeric_port_value() {
        let cla = build_endpoints(
            &mapping(),
            &EndpointSnapshot::Primary(HostPort::new("10.0.0.4", 20001)),
        );
        let lb = &cla.endpoints[0].lb_endpoints[0];
        let lb_endpoint::HostIdentifier::Endpoint(ep) = lb.host_identifier.as_ref().unwrap() else {
            panic!("expected an Endpoint host identifier");
        };
        let address::Address::SocketAddress(sa) =
            ep.address.as_ref().unwrap().address.as_ref().unwrap()
        else {
            panic!("expected a SocketAddress");
        };
        assert_eq!(sa.address, "10.0.0.4");
        assert_eq!(
            sa.port_specifier,
            Some(socket_address::PortSpecifier::PortValue(20001))
        );
    }

    /// The transient no-primary window must still be a *valid* resource, so the
    /// client sees "no endpoints" rather than a malformed/missing cluster.
    #[test]
    fn no_primary_yields_a_valid_but_empty_assignment() {
        let cla = build_endpoints(&mapping(), &EndpointSnapshot::NoPrimary);
        assert_eq!(cla.cluster_name, "reflection-primary");
        assert!(cla.endpoints.is_empty());
        // Round-trips as a valid protobuf.
        let bytes = cla.encode_to_vec();
        let back = ClusterLoadAssignment::decode(bytes.as_slice()).unwrap();
        assert_eq!(back.cluster_name, "reflection-primary");
    }

    #[test]
    fn not_found_never_fabricates_an_endpoint() {
        let cla = build_endpoints(&mapping(), &EndpointSnapshot::NotFound);
        assert!(cla.endpoints.is_empty());
    }

    #[test]
    fn construction_is_deterministic() {
        let m = mapping();
        let snap = EndpointSnapshot::Primary(HostPort::new("h", 7));
        assert_eq!(
            build_listener(&m).encode_to_vec(),
            build_listener(&m).encode_to_vec()
        );
        assert_eq!(
            build_endpoints(&m, &snap).encode_to_vec(),
            build_endpoints(&m, &snap).encode_to_vec()
        );
    }
}
