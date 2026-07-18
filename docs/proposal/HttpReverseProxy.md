# HTTP Reverse Proxy for Service Fabric — Proposal

Status: Proposal / experiment. Nothing shipped.

Date: 2026-07-17

Owners: mssf maintainers

Relates to:
[gRPC xDS over Service Fabric Naming](./GrpcXds.md) (the "base xDS
proposal"; like this doc, `Status: Proposal / experiment. Nothing
shipped.`) and
[gRPC xDS over Service Fabric on Azure SLB](./GrpcXdsSlb.md) (the
"SLB proposal"). This doc **generalizes the SLB proposal's former
Scenario B** — originally a gRPC-only gateway — into a generic
**SF-naming-aware HTTP reverse proxy** that covers HTTP/1.1,
HTTP/2, and gRPC (which is HTTP/2). It is architecturally a
**reverse proxy**, not an xDS design — see
[Is this xDS?](#is-this-xds).

## Why pursue this?

The xDS scenarios cover most callers that can reach service
endpoints directly:

- **In-VNet / peered** clients get direct-to-replica xDS
  (base proposal + SLB proposal Scenario A).
- **Public, any-replica** clients dial the VIP directly
  (SLB proposal Scenario C).
- **Public, primary-targeted** clients dial per-instance NAT ports
  directly (SLB proposal Scenario C2) **when** the service has a
  fixed port and the NAT budget is acceptable.

Two gaps remain, and both point at a reverse proxy:

1. **Public / VIP-only, primary-targeted, can't meet C2.** A client
   that can reach only the VIP, needs partition/primary-aware
   routing, and cannot satisfy Scenario C2's fixed-port +
   per-instance-NAT constraints (many services, dynamic ports,
   large cluster). A single L4 VIP cannot steer it to "the node
   holding partition K's primary" — something inside the cluster
   must decide.
2. **Plain HTTP services, not just gRPC.** SF hosts REST/HTTP
   services too. A single L7 entry point that resolves SF naming,
   selects the partition/primary, and proxies **any** HTTP verb —
   plus gRPC as a first-class HTTP/2 case — is more broadly useful
   than a gRPC-only gateway and matches what SF's built-in reverse
   proxy and SF-YARP already do for HTTP (but without their
   limitations; see [SF-YARP](#can-sf-yarp-be-used-directly)).

This proposal defines `mssf-http-proxy`: an SF-naming-aware,
partition/primary-aware HTTP reverse proxy that terminates external
HTTP/HTTP2/gRPC behind the SLB and forwards each request to the
correct replica.

## Is this xDS?

**No — and that is why it lives in its own doc.** The defining
property of xDS is that the *client's own* data plane receives the
endpoint set and connects **directly** to the chosen backend; the
control plane is never on the data path. That holds for SLB
Scenarios A / C / C2. It does **not** hold here:

| | xDS scenarios (A/C/C2) | This proposal (proxy) |
|---|---|---|
| Client runs xDS? | yes | **no** — plain HTTP/gRPC to the VIP |
| Who connects to the replica? | the client, directly | **the proxy**, then re-originates |
| Control/proxy on the data path? | no | **yes** — the proxy is the data path |
| Pattern | xDS | **L7 reverse proxy** |

The client here has no endpoint set, no LB policy, no xDS resolver
— it opens a normal HTTP or gRPC connection to a fixed VIP and (for
partitioned services) sets the `mssf-partition-key` header. All
routing happens **server-side** in the proxy. xDS appears only
*inside* the proxy, as one way it ingests topology (below) — and
even that is optional. This is the textbook definition of a reverse
proxy, the same slot SF's built-in reverse proxy and
[SF-YARP](https://github.com/microsoft/service-fabric-yarp) occupy.

Treat this as the **escape hatch / general ingress** for callers
the direct xDS paths cannot serve — not as "xDS for public
clients".

## Protocol scope

The proxy is **transport-generic at the routing layer** (it routes
on host / path / headers) but must be **protocol-correct at the
transport layer**. Concretely:

- **HTTP/1.1** — standard request/response proxying.
- **HTTP/2 (h2, h2c)** — required, including for gRPC.
- **gRPC** — gRPC *is* HTTP/2 with `content-type: application/grpc`,
  **trailers** (`grpc-status` / `grpc-message` arrive in HTTP/2
  *trailing* headers), and **long-lived streams**. To carry gRPC
  correctly the proxy **must**:
  - speak **HTTP/2 end-to-end**, including to the backend (never
    downgrade to HTTP/1.1 upstream);
  - **forward trailers** (drop them and every RPC looks like it
    returned no status);
  - **stream** without buffering whole request/response bodies.

  These are hard requirements, not niceties: a stock HTTP/1.1 proxy
  (or one that terminates to HTTP/1.1, strips trailers, or buffers
  bodies) **silently breaks gRPC**. This is why nginx needs
  `grpc_pass` rather than plain `proxy_pass`. Building on
  **tonic / hyper / tower** gives HTTP/2 + trailers + streaming
  natively, so gRPC is covered without special-casing routing.

**Out of scope for v1** (possible future work): **gRPC-Web**
translation (browser clients over HTTP/1.1), **HTTP/JSON ↔ gRPC
transcoding** (needs proto descriptors), and **WebSocket**
upgrades. None are required to proxy HTTP or native gRPC.

## Goals

1. Give **public / VIP-only** clients partition/primary-aware
   access to SF services — HTTP and gRPC alike — when SLB
   Scenarios A/C/C2 do not apply.
2. Reuse the base proposal's **partition-key** and **replica-role**
   semantics (`mssf-partition-key` / `mssf-partition-role`) and its
   **failover model** so routing behavior matches the in-cluster
   xDS path; only the *place* the decision is made differs.
3. Keep the external client trivial: normal HTTP/gRPC to the VIP,
   plus the `mssf-partition-key` header for partitioned services
   and the optional `mssf-partition-role` header for read routing.
   No SF-specific client code.
4. Improve on SF's built-in reverse proxy and SF-YARP where they
   fall short: **key/role-based** partition and replica selection
   (`mssf-partition-key` / `mssf-partition-role`), **Linux**
   support, and **config/stack unification** with the xDS agent
   (one xDS snapshot and runtime across the in-cluster and proxy
   paths). HTTP/2 + gRPC proxying itself is table stakes — SF-YARP
   already does it (see [SF-YARP](#can-sf-yarp-be-used-directly)).

## Non-Goals

- A general-purpose API gateway (rate limiting, quotas, developer
  portal, etc.). This is an SF-naming-aware L7 data plane.
- Replacing the xDS scenarios. Direct-to-replica (A/C/C2) is
  preferred whenever the client can reach endpoints; the proxy is
  the fallback and adds a hop.
- Protocol translation (gRPC-Web, JSON transcoding) in v1.
- Reimplementing SF naming or the SF→xDS mapping — inherited from
  the base proposal.

## Design

### The proxy

`mssf-http-proxy` is a stateless `-1` service deployed on a node
type exposed by the SLB. It:

1. **Ingests topology** (see [Config ingest](#config-ingest)) —
   which services exist, their partitions, and each partition's
   current primary/secondary endpoints.
2. **Terminates external HTTP/HTTP2/gRPC** on stable ports
   (`VIP:80` / `VIP:443`). The SLB spreads external connections
   across proxy instances by 5-tuple; any instance can serve any
   request because all share the same topology.
3. **Selects the service** from the request (Host header and/or URL
   path prefix — e.g. `/<App>/<Service>/...`, mirroring SF's
   built-in reverse proxy addressing — or an explicit mapping).
4. **Selects the partition** by reading `mssf-partition-key`,
   using the base proposal's
   [partition-key semantics](./GrpcXds.md#partition-key-semantics)
   (decimal `i64` for Int64Range, exact name for Named). Singleton
   services skip this step.
5. **Selects the replica role** by reading `mssf-partition-role`,
   reusing the base proposal's
   [replica-role semantics](./GrpcXds.md#partition-key-semantics):
   `primary` (default) / `secondary` / `any`. The header is a
   normal HTTP header for HTTP callers and request metadata for
   gRPC callers, so it maps to the proxy unchanged. **Absent or
   unmatched ⇒ primary** (fail-*safe*, write-safe), matching the
   in-cluster xDS path. It then forwards the request to a replica
   of that role, preserving method, headers, body, and (for gRPC)
   trailers and streaming.
6. **Follows failover** — when the primary moves, the topology
   update repoints the proxy to the new endpoint; the proxy then
   **stops routing new work to the old-primary pool and establishes
   a new pool to the new primary** (a topology repoint alone does not
   tear down established upstream connections). The proxy is
   **passive toward in-flight streams**: it drains the old-primary
   pool for **new** streams only and does **not** proactively reset
   or cancel established upstream streams. The backend app (the
   replica that is no longer primary) is responsible for ending its
   own stream; the proxy just propagates whatever the backend does
   back to the client (see [Failover and retries](#failover-and-retries)).
   Retry behavior is bounded by idempotency, not automatic.

```mermaid
flowchart LR
    Ext["external client<br/>(HTTP / gRPC)"]
    SLB["Azure SLB<br/>VIP:80/443 (L4)"]
    subgraph Cluster
        P1["mssf-http-proxy<br/>(stateless -1)"]
        P2["mssf-http-proxy"]
        Disc["mssf-xds-discovery<br/>(control tier, snapshot)"]
        Svc["target replica<br/>(partition primary)"]
    end
    Ext -->|"HTTP/gRPC + mssf-partition-key"| SLB
    SLB -->|"5-tuple spread"| P1
    SLB --> P2
    Disc -.->|"topology (xDS snapshot)"| P1
    Disc -.->|"topology (xDS snapshot)"| P2
    P1 -->|"in-cluster hop to primary"| Svc
```

**Tradeoff, stated honestly:** this is **not** direct-to-replica.
It adds one in-cluster hop and a proxy tier, and gives up the "no
data-path component" property the base proposal is proud of. It is
the price of serving a client that can reach only a single L4 VIP,
and of offering one generic HTTP entry point.

### Failover and retries

Goal 3 keeps the external client free of **SF/xDS-specific
failover logic** — no xDS resolution, no partition/replica
awareness, no SF-specific client code. It does **not** relieve
the client of the *ordinary* retry and reconnection any HTTP/gRPC
client must already handle: a `503` / `Code::Unavailable`, or a
closed stream, is still the client's to retry or re-establish,
exactly as with any upstream. What the proxy absorbs is only
**SF-topology failover** — repointing to the new primary — and,
within that, it may transparently retry only *known-idempotent*
requests. It cannot delegate SF failover to "the caller's own
idempotency rules" the way the base proposal does, but it also
does not take over the caller-owned retry model — that stays
consistent with the sibling proposal. This constrains what the
proxy may safely do when a primary moves mid-request:

- **Idempotent requests** (safe HTTP methods, or requests the
  operator explicitly marks idempotent) may be **transparently
  retried** against the new primary once the topology update
  lands.
- **Non-idempotent requests** (unary gRPC, `POST`/`PATCH` and
  similar) **must not** be auto-retried by the proxy — a replay
  could double-apply a write. On failover mid-request these
  surface a `Code::Unavailable` / `503` to the client, which
  owns the decision to retry.
- **Long-lived gRPC streams and streaming bodies cannot be
  transparently replayed.** A primary move is not a graceful drain
  of a *still-valid* backend (which stops **new** streams but lets
  in-flight ones finish); the old primary is no longer the primary,
  so new work is repointed to the new primary. The proxy does **not**
  proactively reset or cancel the affected streams. Instead, the
  **backend app** owns termination: when the replica detects it is no
  longer primary it closes the stream / ends the RPC (typically
  surfacing `NotPrimary` / an error), or completes the in-flight
  request if it still can. The proxy simply propagates that back to
  the client (`Code::Unavailable` / `503` as applicable); the client
  then re-establishes the stream against the new primary. A long-lived
  stream pinned to the old primary therefore ends when the backend
  closes it, not via a proxy-initiated reset, and there is no
  proxy-side buffering that would make a stream survive a backend
  change.

So the proxy's default policy is: retry only known-idempotent
requests, once the pool has been rebuilt to the new primary;
surface an error for everything else. Status-aware retry/hedging
over the failover window is Phase P3 work (see [Phasing](#phasing)),
not an MVP guarantee.

### Config ingest

The proxy needs SF topology; two sources, an implementation detail
that does not change its reverse-proxy nature:

- **Reuse the xDS control-tier snapshot (preferred).** If the
  [two-tier discovery](./GrpcXds.md#two-tier-discovery-sf-yarp-inspired)
  control tier is deployed, the proxy subscribes to the same
  LDS/RDS/CDS/EDS snapshot every other consumer uses — one source
  of truth, one config model, no extra SF-naming load.
- **Read SF naming directly.** The proxy owns a `FabricClient` and
  resolves/subscribes itself (what SF-YARP's
  `FabricDiscovery.Service` does). Simpler to deploy standalone but
  duplicates the naming subscription and the SF→xDS logic.

Prefer reuse when the control tier already exists; standalone
direct-naming is a valid MVP.

### Routing model

- **Service:** Host header and/or path prefix, or explicit config.
- **Partition:** `mssf-partition-key` header — a normal HTTP header
  for HTTP callers, request metadata for gRPC callers — matched to
  the owning partition per the base proposal's semantics. Malformed
  or missing keys fail the same way (hard error, never a silent
  fallback to an arbitrary partition).
- **Replica role:** `mssf-partition-role` header — a normal HTTP
  header for HTTP callers, request metadata for gRPC callers —
  selects `primary` (default) / `secondary` (read-from-secondary)
  / `any` (all replicas), reusing the base proposal's
  [replica-role semantics](./GrpcXds.md#partition-key-semantics).
  **Absent or unmatched ⇒ primary** (fail-*safe*, write-safe), so
  read routing is opt-in and identical to the in-cluster xDS path.

## Can SF-YARP be used directly?

Now that this is a generic HTTP proxy, [SF-YARP](https://github.com/microsoft/service-fabric-yarp)
is the closest prior art — it is exactly an SF-naming-fed YARP
(HTTP) reverse proxy. So the question is sharper than before.

**What SF-YARP already does:** a YARP-based L7 HTTP proxy with
HTTP/2 (so it can proxy gRPC), SF discovery + endpoint resolution,
and `StatefulReplicaSelectionMode` = `PrimaryOnly` / `All` /
`SecondaryOnly`. For **HTTP or gRPC** to **stateless, singleton, or
primary-only** services, SF-YARP may already be sufficient with no
new code.

**Where SF-YARP falls short:**

- **No key-based partition selection.** SF-YARP "does not handle
  the partitioning key" — the caller must pass the **partition GUID
  as a query parameter**. This proposal routes on
  `mssf-partition-key` (i64 / name matching the SF scheme), so the
  caller supplies the natural key, not a resolved GUID. Decisive
  functional gap.
- **Windows-only.** "YarpProxy app is only supported on Windows."
  mssf targets Linux and Windows.
- **Separate stack / config model.** SF-YARP is .NET/YARP with its
  own `FabricDiscovery.Service` and service-manifest `Yarp.*`
  labels. A tonic proxy reusing the base proposal's xDS snapshot
  keeps one source of truth and one runtime across in-cluster and
  proxy paths.
- **Maturity.** The SF-YARP repo is effectively dormant (~2022,
  .NET 5/6-era).

**Recommendation.** Use SF-YARP as a valid **stopgap** when *all*
of these hold: Windows cluster, callers that can pass an explicit
partition GUID (or stateless/primary-only services), and no desire
to unify on the xDS snapshot. Build `mssf-http-proxy` when
**key-based** partition routing, **Linux**, or **config/stack
unification** matters. Extending SF-YARP with a key→partition
plugin is possible but stays Windows/.NET and closes neither gap.

## Azure SLB integration

The proxy is normally the **public ingress**, so it sits behind the
SLB. Required LB shape (illustrative ports):

| Purpose | Frontend | Backend | Probe | Notes |
|---|---|---|---|---|
| HTTP ingress | `VIP:80` | proxy HTTP port | TCP/HTTP | plaintext / h2c |
| HTTPS ingress | `VIP:443` | proxy HTTPS port | TCP/HTTPS | TLS termination at the proxy |
| gRPC ingress | `VIP:443` (h2) | proxy HTTPS port | TCP/HTTPS | **gRPC rides the HTTPS/443 listener** (ALPN `h2`); no separate frontend port. This is the "gRPC ingress rule" the [SLB proposal](./GrpcXdsSlb.md#azure-slb-configuration-required) points at. |

Plus the Standard-SLB essentials from the SLB proposal apply:
outbound rule (443), NSG opening the ingress ports inbound, and —
because HTTP/2 and gRPC streams are long-lived — **keepalive tuned
below the SLB idle timeout** (~4–5 min) so streams are not silently
dropped. See
[SLB proposal — Azure SLB configuration](./GrpcXdsSlb.md#azure-slb-configuration-required).

## Security

A public VIP is internet-facing, so the proxy is the security
boundary:

- **TLS termination.** The proxy terminates client TLS at the VIP
  and re-originates to the replica with mTLS inside the cluster
  (cluster certs). For gRPC, upstream must remain HTTP/2. **Backend
  assumption:** this presumes the target replica accepts
  cluster-cert mTLS on its listener. SF application services
  commonly present their **own** service/endpoint certificate, or
  listen plaintext; when a backend does not speak cluster-cert
  mTLS, the proxy must be configured **per service** with the
  expected upstream scheme (its service cert / CA to validate, or
  explicit plaintext for trusted in-cluster networks) rather than
  assuming every replica is a cluster-cert mTLS peer.
- **AuthN/AuthZ.** The proxy is the natural place to enforce client
  identity (mTLS client certs, tokens/JWT) and per-service
  authorization before it will proxy.
- **DoS / abuse surface.** A public, L4-fronted L7 proxy has a
  real abuse surface (connection floods, slowloris/half-open,
  HTTP/2 stream-concurrency exhaustion). The proxy enforces basic
  **connection, stream-concurrency, and header/idle timeouts** to
  bound this; broader volumetric protection is **delegated to the
  upstream layer** (SLB/NSG, or a WAF/DDoS front). Application-level
  rate limiting and quotas remain a Non-Goal (see [Non-Goals](#non-goals)).
- **NSG posture.** Expose only the ingress ports (and control-plane
  ports if the proxy ingests xDS over the network); everything else
  closed. Prefer per-node-type subnets/NSGs so blast radius is
  bounded.

## Phasing

- **Phase P1 — MVP.** `mssf-http-proxy` ingesting the control-tier
  snapshot (or direct SF naming), selecting service + primary;
  **HTTP/1.1 and plain HTTP/2** first, stateless / singleton
  services. Until P2 adds `mssf-partition-role` selection, the proxy
  routes to the partition **primary by default** (role selection
  deferred), so the default-primary / write-safety contract holds
  from this first public phase. Because this phase already accepts
  public traffic, the
  **baseline security boundary lands here, not later**: TLS
  termination at `VIP:443` with backend transport policy
  (cluster-cert mTLS or the per-service upstream scheme), client
  authN/authZ where the deployment requires it, and the
  connection / stream-concurrency / header-idle limits from
  [Security](#security). Plain HTTP at `VIP:80` is for
  **private or test-only** deployments; public exposure uses `:443`.
- **Phase P2 — gRPC + partitioning.** End-to-end HTTP/2 with
  **trailer forwarding and streaming** (native gRPC), plus
  `mssf-partition-key` / `mssf-partition-role` selection (RDS
  `Range`/`Exact`) and primary+secondary read routes. The P1
  security controls remain in force.
- **Phase P3 — hardening.** Mandatory end-to-end mTLS (beyond the
  P1 baseline), richer authZ policy, outlier ejection, metrics, and
  status-aware retry/hedging over failover windows.

Only build this when the client population actually includes
VIP-only callers (or plain-HTTP services) the direct xDS paths
cannot serve.

## Open questions

- **Service addressing convention.** Host-based
  (`svc.app.example.com`), path-based (`/<App>/<Service>/...` like
  SF's built-in reverse proxy), or configurable? Path-based matches
  existing SF muscle memory; host-based is cleaner for per-service
  TLS/authz.
- **Config-ingest choice.** Reuse control-tier snapshot vs. direct
  SF naming — pick per deployment, or support both behind one
  interface?
- **Placement.** Dedicated node type behind its own LB, or
  co-resident with the discovery control tier? Isolation vs.
  footprint.
- **gRPC value-adds.** Should the proxy read `grpc-status` trailers
  for status-aware retry/hedging during failover, or stay
  status-agnostic? Reading trailers is more useful but couples the
  proxy to gRPC semantics.
- **Per-instance public IPs as an alternative.** Secondary node
  types can get per-instance public IPs; a public client could then
  go direct-to-replica (an xDS path) and skip the proxy. Fragile
  (public IP per node, NSG surface, cost, primary node types
  excluded) — niche, but it moves such callers back onto the xDS
  scenarios.

## Future work

- **gRPC-Web** translation for browser clients.
- **HTTP/JSON ↔ gRPC transcoding** (proto-descriptor driven).
- **ORCA / LRS** load-aware balancing across replicas.
- **Private Link ingress** as a VIP alternative for partner
  clients.

## References

- [gRPC xDS over Service Fabric Naming](./GrpcXds.md) — base proposal: SF→xDS mapping, partition-key semantics, two-tier discovery, failover.
- [gRPC xDS over Service Fabric on Azure SLB](./GrpcXdsSlb.md) — the xDS scenarios (A/C/C2) this proposal is the fallback for; Azure SLB configuration details.
- [microsoft/service-fabric-yarp](https://github.com/microsoft/service-fabric-yarp) — prior art: an SF-naming-fed HTTP reverse proxy (no partition-key routing; Windows-only).
- [Networking patterns for Azure Service Fabric](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-patterns-networking) — SLB, mgmt ports, outbound requirement.
