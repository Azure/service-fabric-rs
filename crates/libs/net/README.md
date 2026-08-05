# mssf-net

Experimental Service Fabric naming → gRPC xDS (ADS) mapping.

Exposes one configured SF stateful singleton service through an Envoy v3
State-of-the-World Aggregated Discovery Service, so a stock xDS-capable gRPC
client can reach the service's current primary replica with no SF-specific
client code.

**Experimental — no stable API guarantee.** This crate is the in-progress
successor to the `mssf_util::tonic` "proxyless" client path tracked by
[issue #300](https://github.com/Azure/service-fabric-rs/issues/300). That path
remains in place and unmodified.

See `docs/design/XdsNamingDesign.md` for the design.
