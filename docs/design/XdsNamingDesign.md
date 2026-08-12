# Service Fabric Naming → gRPC xDS — Design

Status: Prototype. See [`mssf-net`](../../crates/libs/net).

Owners: mssf-rs maintainers

## Background

A Service Fabric (SF) stateful service runs as a replica set: one primary and
one or more secondaries / auxiliaries. Traffic is normally routed to the
primary, and the primary can move between nodes at any time (failover,
rebalancing, upgrade).

Today a Rust caller reaches such a service through
[`mssf_util::tonic`](../../crates/libs/util/src/tonic) — resolve the service,
select the primary, interpret its address, and drive a channel that rebuilds on
failover. That path works, but it is SF-aware client code and it exists **only
for Rust**. Callers in Go, Java, C++ or Node have no equivalent.

Standard gRPC clients in those languages already know how to discover endpoints
and follow them, through [xDS][xds]. They need only a control plane that speaks
xDS and understands SF naming. This crate is that control plane, at prototype
scale.

It is the in-progress successor to `mssf_util::tonic`, which is already marked
experimental and slated for replacement by the sf-xds proposal
([issue #300](https://github.com/Azure/service-fabric-rs/issues/300)). **This
work does not modify or remove that path**; retirement is future work,
contingent on the deferred capabilities below.

See also the broader proposal: [`docs/proposal/GrpcXds.md`](../proposal/GrpcXds.md).

## Goals

1. Let a caller reach an SF service through a standard, off-the-shelf
   xDS-capable gRPC client with **no SF-specific code in the caller**.
2. Keep the discovered endpoint current as the authoritative replica relocates,
   without the caller restarting or re-resolving.
3. Prove both of the above end-to-end: once with no cluster at all (fast, runs
   in CI) and once against a real onebox with a genuine failover.
4. Stay minimal. This is a prototype; ship the smallest thing that proves the
   concept.

## Non-Goals

- Dynamic registration: the set of mapped services is fixed at build time.
- A shared SF-facing tier: each mapped service still owns its own
  `FabricClient` and notification filter.
- Multiple partitions, partition keys, stateless services, secondaries, or
  read-from-secondary routing.
- RDS, delta/incremental xDS, federation (`xdstp://`), or LRS/ORCA load
  reporting.
- A deployable per-node agent or two-tier (central discovery + relay) topology.
- TLS/mTLS, authN/authZ, traffic policy, health checking, endpoint weighting.
- Holding stale last-known-good endpoints through a no-primary window.
- Inferring arbitrary SF endpoint address formats.

## Where this lives

`crates/libs/net`, package `mssf-net`. `publish = false` — it depends on a
pre-release xDS crate.

Note it depends on `mssf-util` **without** that crate's `tonic` feature. The
`resolve` / `retry` helpers sit behind the default `tokio` feature, while the
incumbent `mssf_util::tonic` module is a separate non-default feature. The pin
therefore structurally guarantees the successor does not link the incumbent.

### File layout

| File | Responsibility |
|---|---|
| `endpoint.rs` | `HostPort`, `EndpointSnapshot`, the `EndpointSource` trait, `ScriptedEndpointSource` |
| `address.rs` | `AddressInterpreter` — pluggable interpretation of the opaque SF address |
| `config.rs` | `XdsMapping` — one SF service ↔ one xDS resource name |
| `registry.rs` | `ServiceRegistry` — the set of services one ADS server publishes |
| `resources.rs` | Pure LDS / CDS / EDS construction |
| `ads.rs` | The SotW ADS service, and `bootstrap_json` |
| `fabric.rs` | `FabricEndpointSource` — the SF-backed source |

## Architecture

```
 SF Naming ──notify──► FabricEndpointSource ──watch──► AdsService ──ADS──► stock xDS client
                          (fabric.rs)                    (ads.rs)                │
                                                        serves a                │ direct gRPC
                                                   ServiceRegistry              ▼
                                                     (registry.rs)      primary replica
```

Only *configuration* flows through this crate. Client RPCs go **directly** to
the replica; there is no data-path hop.

### Why an `EndpointSource` trait, not a `FabricClient` parameter

This is the seam that makes the crux test possible. With the SF dependency
behind a trait, the entire mapping and serving stack — resource construction,
ADS streaming, version/nonce bookkeeping, push-on-change — is exercised end to
end against a real `tonic-xds` client with **no cluster, no SF runtime, and no
`FabricClient`**. Had the ADS server taken a `FabricClient` directly, the only
way to test any of it would have been a live cluster.

### Why LDS-with-inline-`RouteConfiguration`, not RDS

The xDS client accepts a Listener whose `HttpConnectionManager` carries either
an RDS reference or an inline `RouteConfiguration`. Inlining removes an entire
resource type (and round trip) from the protocol we must implement, with no
loss of function at this scope.

The minimum accepted chain is therefore **LDS → CDS → EDS**.

### Why State-of-the-World, not delta

Delta xDS is `unimplemented` in the target client. SotW is sufficient, and at
this scale the "send everything" semantics cost little — though see
[Serving several services](#serving-several-services) for the one place where
they are genuinely dangerous.

### Why a `watch` channel

[`tokio::sync::watch`][watch] is latest-value-wins, which gives the required
"coalesce to the newest state" behavior *structurally* rather than through
bespoke debouncing. Two further properties matter:

- `send_replace` is **non-blocking and non-async**, so it is safe to call from
  the synchronous SF COM callback thread.
- It carries an initial value, so a newly connected ADS stream needs no
  separate "fetch current state" call.

## Resource mapping

| SF concept | xDS resource / field |
|---|---|
| Mapped service | `Listener` named `<xds_name>`, targeted by `xds:///<xds_name>` |
| — | inline `RouteConfiguration`, one virtual host, prefix `/` route |
| Mapped service | `Cluster` named `<xds_name>-primary`, type EDS |
| `StatefulPrimary` endpoint | one `LbEndpoint` in `ClusterLoadAssignment` |
| No primary right now | **empty but valid** `ClusterLoadAssignment` |
| Service does not exist | `Listener` withheld from the LDS response |

`<xds_name>` may be the SF service URI verbatim — `xds:///fabric:/App/Service`,
which is the shape the proposal calls for. `:` and `/` are legal in xDS resource
names, and the target string is retained unmodified by the client, so the name a
caller targets can simply *be* the service it wants, with no alias table to keep
in sync. `XdsMapping::for_service_uri` builds that form;
`XdsMapping::new` keeps them independent when a short alias is preferred. Note
this is also why the virtual host uses `domains: ["*"]`: the whole target string
becomes the authority, so a literal domain match would have to reproduce it
exactly.

### Fixed values, and the NACK traps they avoid

These are not stylistic. Each one prevents a client-side rejection:

| Fixed value | Why |
|---|---|
| `lb_policy = ROUND_ROBIN` | anything but `ROUND_ROBIN` / `LEAST_REQUEST` is NACK'd. With one endpoint it degenerates to "always the primary". |
| `domains = ["*"]` | virtual-host matching does **not** strip `:port` from the authority, so a literal host match would miss `xds:///name:port`. |
| numeric `PortValue` | a named port is NACK'd. |
| CDS name == EDS `cluster_name` | the client falls back to the cluster name when a cluster carries no explicit EDS service name. |

Each of these has a dedicated unit test in `resources.rs`, so a regression is
caught without needing a live client to reject it.

## Serving several services

One ADS server publishes any number of services. Clients subscribe by resource
name, so this is what xDS is designed for; `ServiceRegistry` holds the set and
`AdsService::from_registry` serves it. The single-service constructors remain as
one-entry sugar.

### The index is keyed by `(type_url, name)`, not by name

Each entry contributes two names — a `Listener` named `<xds_name>` and a
`Cluster` named `<xds_name>-primary`, which EDS reuses. Deriving the cluster
name by appending a constant suffix is injective, so distinct xDS names can
never produce colliding cluster names; unique `xds_name` is therefore the only
rule the builder has to enforce, and it does so at registration time, where a
configuration mistake is cheap to diagnose.

What a bare-name index *would* get wrong is the cross-type case: a mapping named
`x-primary` has a Listener whose name equals the Cluster of a different mapping
named `x`. Both are legitimate and must resolve to different entries. Including
the type URL in the key makes conflating them impossible rather than merely
unlikely.

### LDS and CDS carry the whole subscribed set; EDS carries only what changed

This is the sharpest correctness constraint in the multi-service design.
Listeners and Clusters are `ALL_RESOURCES_REQUIRED_IN_SOTW`: a response that
omits a resource the client is subscribed to is not a no-op, it is a
**deletion**. So any LDS or CDS response must enumerate every subscribed entry —
including when the response was triggered by a change to just one of them.
Pushing only the changed service's Listener would silently break every *other*
service on that stream.

EDS has no such rule; it is tracked per-resource. An endpoint change therefore
pushes exactly one `ClusterLoadAssignment`, for the cluster that actually moved.

`tests/scripted_ads.rs` guards this directly: it relocates one service's
endpoint and then asserts a *second* service's client still routes.

### Fan-in across sources

Each entry has its own `watch::Receiver`, and a stream awaits the first change
across all of them. `watch::Receiver::changed` is cancel-safe, so the fan-in set
is rebuilt on each loop iteration rather than held across them — which would
otherwise require a self-referential future. The resolved snapshot is returned
by the fan-in future itself, releasing the mutable borrow before the handler
runs.

The registry is non-empty by construction, which is load-bearing: the underlying
`select_all` panics on an empty set.

### What this does not consolidate

Each `FabricEndpointSource` still owns its own `FabricClient` and its own
notification filter. Multi-service serving consolidates *ports and streams*, not
SF-side naming load; collapsing N registrations into one cluster-wide
subscription is the two-tier discovery work in the proposal.

The registry is also immutable once built — there is no dynamic
registration while serving.

## Endpoint state: three values, not two

```rust
pub enum EndpointSnapshot {
    Primary(HostPort),
    NoPrimary,   // transient
    NotFound,    // permanent
}
```

The distinction is load-bearing. `NoPrimary` publishes an empty assignment and
the client reports "no ready endpoints"; `NotFound` withholds the Listener, and
because Listeners are *all-resources-required* in SotW, that omission is a
**resource deletion** the client treats as permanent.

[`ServicePartitionResolver`](../../crates/libs/util/src/resolve.rs) cannot make
this distinction: it collapses an empty endpoint list into
`FABRIC_E_SERVICE_OFFLINE` and retries. So `fabric.rs` classifies explicitly,
in a pure function:

```rust
pub fn classify_resolve_error(err: &mssf_core::Error) -> EndpointSnapshot
```

Only explicitly-known "does not exist" codes yield `NotFound`. Everything else —
offline, timeout, communication failures, unrecognized codes, non-Fabric
HRESULTs — falls back to the transient `NoPrimary`. The asymmetry is deliberate:
mis-classifying a transient failure as permanent withdraws the route and is hard
to recover from, whereas the reverse merely delays recovery.

Note the retryer surfaces `FABRIC_E_TIMEOUT` on exhaustion, so that — not
`FABRIC_E_SERVICE_OFFLINE` — is the code that usually arrives. Both are
transient; both are unit-tested.

## Endpoint addresses are opaque

An SF endpoint address is a **service-defined string**. It may be `host:port`, a
URL, a JSON envelope, or anything else. The reflection sample publishes a URL
carrying query parameters. The crate therefore cannot infer it:

```rust
pub type AddressInterpreter =
    Arc<dyn Fn(&str) -> Result<HostPort, AddressError> + Send + Sync>;
```

The interpreter takes the raw **string**, not a `&ResolvedServicePartition`.
That is forced by the notification path: an SF notification carries a list of
endpoints and **no** RSP, so an RSP-shaped selector could not be applied there at
all. Splitting role selection (crate-owned — always the primary) from address
interpretation (caller-owned) lets the resolve path and the notification path
share one interpreter.

`host_port_interpreter()` ships as a worked example; the reflection test
supplies its own built on `ReflectionUrl::parse`.

## Notification path

Registration is two-part and order matters:

1. The callback is installed **while building** the `FabricClient` — it cannot
   be attached afterwards.
2. The `PrimaryOnly` filter for the exact service URI is registered **before**
   the seeding resolve, so a change occurring during startup is not missed. The
   seed is then applied only if no notification has already produced a state, so
   a newer notification is never clobbered by an older resolve.

### COM-thread constraints

The callback is synchronous and runs on an SF COM thread. It must not await,
block, or do heavy work. It therefore does exactly one thing: a non-blocking
`send_replace` of the raw notification. A Tokio task performs address
interpretation and publishes the resulting snapshot.

## Lifecycle and issue #184

Dropping a `FabricClient` promptly after use can trigger an invalid memory
access ([issue #184](https://github.com/Azure/service-fabric-rs/issues/184)).
The source owns a client, so `EndpointSource` carries an explicit async
teardown:

```rust
async fn shutdown(self: Arc<Self>);
```

It takes `Arc<Self>` because the ADS server holds its own clone, which means it
cannot move fields out of `self` — releasable handles are held behind
`Mutex<Option<..>>` and `take()`n. Ordering is: unregister the filter (async, so
it cannot happen in `Drop`), release the client, then delay.

A test that also builds its own admin `FabricClient` must additionally apply
`fabric_client_drop_hack` to that handle; `shutdown` covers only the source's.

## Server lifecycle and shutdown

Shutdown is driven by a single [`tokio_util::sync::CancellationToken`], matching
this repository's existing cancellation convention (`BoxedCancelToken` in
`mssf-core`, wrapped by `mssf_util::tokio::TokioCancelToken`, which converts in
both directions).

`AdsService::serve_on_ephemeral_loopback` / `serve_with_listener` return a
`ServerHandle` rather than a bare address. The handle owns the serving task:

- `shutdown()` cancels the token and **awaits** the task, so a serving error
  surfaces instead of being swallowed by a detached `tokio::spawn`.
- `Drop` cancels and aborts, so a server can never outlive the scope that
  started it. The type is `#[must_use]`.
- `cancellation_token()` exposes the token for callers who want to stop the
  server from elsewhere.

One token covers two jobs, which is why a single primitive is the right fit.
An ADS stream is long-lived by design, and `tonic`'s graceful shutdown waits for
open connections to drain — so a server with a connected xDS client would **hang
forever** if only the accept loop were stopped. Cancellation both ends the open
streams (each selects on `cancelled()`) and serves as the graceful-shutdown
signal, so connections drain and the task completes.

`AdsService::with_cancellation(mapping, source, token)` accepts a caller-supplied
token, so the server's lifetime can be tied to an existing scope — for example
an SF service's `close(cancellation_token)`, so the ADS server stops when the
replica does. If you mount the service on your own `tonic` server, take
`AdsService::cancellation_token()` **before** `into_server()` and cancel it as
part of your shutdown signal — otherwise you will reproduce that hang.

`tests/scripted_ads.rs` covers both paths: shutting down with a live client
still attached, and stopping via a caller-supplied token.

The stream's `select!` is `biased;` with cancellation first. Unbiased `select!`
picks a random ready branch, so under steady traffic shutdown latency would be
random and a stream could keep emitting responses after cancellation. Biasing
makes it deterministic — once cancelled, the stream ends at the next poll and
does no further work — which matters because shutdown blocks on these streams
draining. Note this is about determinism, not liveness: an unbiased select still
wins the race with probability 1, so no black-box test distinguishes the two.

## Testing

| Test | Location | Needs a cluster? |
|---|---|---|
| unit tests | `crates/libs/net/src/*.rs` | no |
| `scripted_ads.rs` | `crates/libs/net/tests/` | **no** |
| `xds_failover.rs` | `crates/samples/reflection/tests/` | yes |

Separation is by crate location, matching existing repository convention — no
`#[ignore]` or feature flag needed.

In CI, the cluster-backed jobs (`build` on Windows, `build-devcontainer`) run
`cargo test --all` after provisioning a onebox, which already covers this
crate. The one added step is in `build-azl3`, which has no Service Fabric
installed and otherwise runs no tests at all: it is both the only test execution
in that job and the strongest available proof that the mapping needs no SF
runtime. Deliberately *not* duplicated into the Windows job — that runner has SF
installed, so running there before the cluster starts would only show "no
cluster", and `cargo test --all` already executes these tests.

`scripted_ads.rs` is the crux. It starts two stand-in backends and the ADS
server on ephemeral loopback ports, points a **stock, unmodified** `tonic-xds`
channel at it, and asserts:

- calls reach the published endpoint;
- after the source publishes a different endpoint, the **already-created**
  client reaches the new one, with neither client nor server restarted;
- a `NoPrimary` window fails calls in bounded time, and recovers;
- an unknown resource name fails in bounded time rather than hanging.

Every RPC is wrapped in a per-call timeout. An xDS call blocks indefinitely
while discovery has no usable endpoint, so a retry loop that checked its
deadline only *after* the call returned would hang forever — this bit us during
development.

### Serving-replica isolation in the live test

`GetReplicas` returns a list built from a **process-wide** registry covering
every partition hosted in the answering process, and several reflection test
services can share one activated process. The live test therefore filters the
response to its own partition id and asserts exactly one entry. An unfiltered
"find any primary" could match a different service's primary and pass even if
xDS had routed to a secondary.

## Manual verification procedure

1. Start the cluster (elevated): `.\onebox\windows\StartOnebox.ps1 -Auto`
2. Wait for it: `.\scripts\check_cluster_online.ps1`
3. Deploy the app: `.\scripts\reflection_ctl.ps1 -Action Add`
4. Run: `cargo test -p samples_reflection --test xds_failover -- --nocapture`

The test creates and deletes its own service, so re-runs are idempotent.

## Known limitations and deferred work

- **One `FabricClient` and one notification filter per registered service.**
  Multi-service serving consolidated ports and ADS streams, not SF-side naming
  load — which is the concern the proposal's two-tier design addresses, and the
  one that matters at cluster scale.
- **The registry is fixed at build time.** Services cannot be added or removed
  while the server runs. Lifting this means making the registry observable and
  re-pushing LDS/CDS to every open stream on a change, since both are
  `ALL_RESOURCES_REQUIRED_IN_SOTW`.
- **A NACK is logged and then forgotten.** `stream_aggregated_resources` traces
  the `error_detail` and does not advance state, which is correct, but a
  persistently rejected resource is never re-sent and raises no metric or health
  signal. That makes it the hardest failure to diagnose in the field: the client
  silently has no route while the server looks healthy. A real deployment wants
  a counter and, on repeated NACKs for the same resource, a surfaced error.
- **`protoc` is required to build this crate**, not just to test it: `build.rs`
  compiles the test-only `proto/testsvc.proto` on every build. Acceptable here
  because CI already installs `protoc` for the other crates, but a consumer
  building only the library pays for a test fixture. Moving the stand-in service
  behind a cargo feature (or into a separate test crate) would remove it.
- Singleton partition only; no partition-key routing.
- Primary only; secondaries are never exposed.
- The xDS client crate is pre-release (`0.1.0-alpha.2`); its error *text* is not
  a stable contract, so tests assert on failure/success and bounded timing
  rather than exact messages.
- No agent/relay deployment shape — the ADS server is hosted in-process by the
  caller.
- The empty-endpoint and unknown-resource conditions may surface under the same
  coarse gRPC status code; they are distinguishable only by detail.

[xds]: https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
[watch]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
