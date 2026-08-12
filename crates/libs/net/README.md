# mssf-net

Experimental Service Fabric naming → gRPC xDS (ADS) mapping.

Exposes an SF stateful service through a standard [xDS][xds] control plane, so a
**stock gRPC client** can reach the service's current primary replica — and keep
reaching it across failover — with **no Service Fabric code in the client**.

> **Experimental — no stable API guarantee.** Items may change or be removed in
> any release. This is the in-progress successor to the `mssf_util::tonic`
> "proxyless" client path tracked by
> [issue #300](https://github.com/Azure/service-fabric-rs/issues/300); that path
> remains in place and unmodified.

## The point: what a client writes

A caller targets `xds:///<name>` — where the name is just the SF service URI —
and gets failover-aware routing for free:

```rust
use tonic_xds::{XdsChannelBuilder, XdsChannelConfig, XdsUri};

let target = XdsUri::parse("xds:///fabric:/MyApp/MyService")?;
let channel = XdsChannelBuilder::new(XdsChannelConfig::new(target))
    .build_grpc_channel()?;

let mut client = GreeterClient::new(channel);
let reply = client.say_hello(HelloRequest { name: "world".into() }).await?;
```

Three things to notice:

- **No `FabricClient`, no `ServicePartitionResolver`, no endpoint parsing, no
  replica selection, no reconnect loop.** Compare the SF-aware equivalent in
  [`mssf_util::tonic`](../util/src/tonic).
- **The name a client writes is the service it wants.** `:` and `/` are fine in
  xDS resource names, so the SF URI doubles as the xDS name and there is no
  alias table to keep in sync. A short alias (`xds:///myservice`) is still
  available if you prefer one — see [Hosting the mapping](#hosting-the-mapping).
- **The client does not depend on `mssf-net`.** Its only new dependency is
  `tonic-xds`. All SF awareness lives in the host process described below.

And it is not Rust-specific: this is ordinary xDS, so Go, Java, C++ and Node
gRPC clients reach the same service with their own standard xDS channels — the
reason to prefer this over the Rust-only path.

The client finds the control plane through the standard gRPC bootstrap, so
nothing above is SF- or `mssf-net`-specific:

```bash
export GRPC_XDS_BOOTSTRAP=/etc/grpc/xds_bootstrap.json
```

```json
{
  "xds_servers": [
    {
      "server_uri": "http://127.0.0.1:18000",
      "channel_creds": [{ "type": "insecure" }],
      "server_features": ["xds_v3"]
    }
  ],
  "node": { "id": "my-node" }
}
```

## Hosting the mapping

Somewhere on the node, a process runs the SF→xDS mapping this crate provides:

```rust
use mssf_net::endpoint::EndpointSource;   // brings `shutdown` into scope
use mssf_net::{AdsService, FabricEndpointSource, XdsMapping, host_port_interpreter};

// The SF URI is also the xDS resource name, so clients target
// xds:///fabric:/MyApp/MyService.
let mapping = XdsMapping::for_service_uri(
    "fabric:/MyApp/MyService",
    host_port_interpreter(),
)?;

// Prefer a short alias instead? Name the two independently:
//   XdsMapping::new("myservice", "fabric:/MyApp/MyService", interpreter)?
// ... and clients target xds:///myservice.

// Owns its own FabricClient and registers its own notification filter.
let source = FabricEndpointSource::new(
    &mapping,
    vec![WString::from("localhost:19000")],
    Duration::from_secs(10),
)
.await?;

let server = AdsService::new(mapping, source.clone())
    .serve_on_ephemeral_loopback()
    .await?;

println!("point GRPC_XDS_BOOTSTRAP at {}", server.addr());

// ... serve ...

server.shutdown().await?;   // ends open ADS streams, then awaits the task
source.shutdown().await;    // unregisters the filter, releases the FabricClient
```

To bind a fixed port, use `serve_with_listener(listener)`. To tie the server's
lifetime to an existing scope — an SF replica's `close(cancellation_token)`, for
instance — construct it with
`AdsService::with_cancellation(mapping, source, token)`.

### Endpoint addresses need an interpreter

An SF endpoint address is a **service-defined string**. It may be `host:port`, a
URL, a JSON envelope, or anything else the service author chose to publish, so
this crate cannot infer it. `host_port_interpreter()` handles the simple case;
anything else supplies its own:

```rust
let interpreter: AddressInterpreter = Arc::new(|raw: &str| {
    let url = MyServiceUrl::parse(raw).map_err(AddressError::Unparseable)?;
    Ok(HostPort::new(url.host, url.port))
});
```

The interpreter receives the raw address string rather than a
`ResolvedServicePartition`, because SF change notifications carry no
`ResolvedServicePartition` — see the design doc for why that matters.

### Testing without a cluster

`ScriptedEndpointSource` substitutes for `FabricEndpointSource`, so the whole
mapping and serving stack — including a real `tonic-xds` client — can be driven
with no Service Fabric runtime at all:

```rust
let (source, handle) = ScriptedEndpointSource::new(
    EndpointSnapshot::Primary(HostPort::new("127.0.0.1", 50051)),
);
let server = AdsService::new(mapping, source).serve_on_ephemeral_loopback().await?;

// Simulate a failover; a connected client follows it.
handle.set(EndpointSnapshot::Primary(HostPort::new("127.0.0.1", 50052)));
```

See `tests/scripted_ads.rs` for the full end-to-end version.

## Status and limitations

Deliberately a prototype:

- One mapped service per ADS server (serving N services means N servers).
- Stateful **singleton partition, primary only** — no partition-key routing and
  no secondaries.
- No TLS/mTLS, RDS, delta xDS, federation, or load reporting.
- No agent/relay deployment shape; the ADS server is hosted in-process.
- Depends on the pre-release `tonic-xds` crate.

Design and rationale: [`docs/design/XdsNamingDesign.md`](../../../docs/design/XdsNamingDesign.md).
Broader direction: [`docs/proposal/GrpcXds.md`](../../../docs/proposal/GrpcXds.md).

[xds]: https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
