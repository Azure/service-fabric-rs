# Service Fabric Tonic Connector — Design

Status: Implemented (v1). See [`mssf_util::tonic`](../../crates/libs/util/src/tonic).

Owners: mssf-rs maintainers

## Background

A Service Fabric (SF) stateful service runs as a replica set: one
primary and one or more secondaries / auxiliaries. Read/write
traffic is normally routed to the primary, and the primary can move
between nodes at any time (failover, rebalancing, upgrade). After a
move the gRPC server endpoint a client previously connected to may
stop accepting new requests *or* may keep accepting them as a
secondary that no longer satisfies the request.

Today, callers route gRPC through plain
[`tonic::transport::Channel`](https://docs.rs/tonic/0.14/tonic/transport/struct.Channel.html)
and fall back to manually re-resolving via
[`ServicePartitionResolver::resolve`](../../crates/libs/util/src/resolve.rs#L42)
in a polling loop on failure (see
[`crates/samples/reflection/src/test.rs`](../../crates/samples/reflection/src/test.rs)).

This module ships a reusable building block: a tonic-compatible
channel that automatically re-resolves the current SF endpoint,
reconnects after failover, and reacts to a server-signalled
"reconnect for next time" trailer — driven by SF's complaint-based
resolve API.

## Goals

1. Ship a tonic integration so a user can compose a failover-aware
   `tonic::Channel` from a `FabricClient` and a small selector
   closure, then hand it to a generated tonic client.
2. Reuse the existing
   [`ServicePartitionResolver`](../../crates/libs/util/src/resolve.rs#L21)
   so refreshes go through the standard retry / cancellation path.
   v1 uses **complaint-based resolve only** — see
   [Resolve strategy](#resolve-strategy).
3. Keep the public API surface small and feature-gated. tonic /
   hyper / tower do not become required deps of `mssf-util`.
4. Support arbitrary user-defined endpoint selection (role,
   partition key, sticky-by-replica-id, round-robin across
   secondaries, etc.). No fixed
   [`ServiceEndpointRole`](../../crates/libs/core/src/client/svc_mgmt_client.rs#L425)
   filter is baked in.
5. Compose cleanly with TLS — deferred. v1 ships plain TCP
   only. The TLS extension requires generalizing
   [`SwapChannel`](#public-surface) over its inner IO type;
   today the type is fixed at `TokioIo<TcpStream>`. See
   [TLS (deferred)](#tls-deferred) and Future Work.

## Non-Goals

- A full replacement for tonic's load balancing / service discovery.
- gRPC-level retry semantics (idempotency, deadlines, hedging).
  **The client stack does not auto-retry**, on any signal.
- **Killing in-flight requests from the client side.** Once
  dispatched, a request's lifecycle belongs to the server.
- Cross-partition routing.
- **TLS in v1.** Plain TCP only. See
  [TLS (deferred)](#tls-deferred).

## Where this lives

All code ships inside `mssf-util` under
[`mssf_util::tonic`](../../crates/libs/util/src/tonic), gated by the
`tonic` cargo feature. No new crate.

`Cargo.toml` adds:

```toml
[features]
tonic = [
    "tokio",
    "dep:tonic", "dep:tower", "dep:hyper",
    "dep:hyper-util", "dep:http", "dep:http-body",
    "dep:arc-swap", "dep:futures", "dep:bytes",
]
```

**TLS is not yet supported.** v1's `SwapChannel` fixes the
connector IO type at `TokioIo<TcpStream>`, which is incompatible
with TLS connectors that wrap the stream as
`TokioIo<TlsStream<...>>`. Adding TLS requires generalizing the
IO bound — see [TLS (deferred)](#tls-deferred) and Future work.

### File layout

```
crates/libs/util/src/tonic/
├── mod.rs                          flat `pub use` re-exports
├── naming/                         naming layer (transport-agnostic)
│   ├── resolver.rs                 TargetResolver trait + BoxError
│   ├── selector.rs                 TargetSelector + DialTarget + SelectError
│   └── default.rs                  FabricTargetResolver(+Builder)
├── connector/                      Service<Uri> connector
│   └── service.rs                  TargetConnector(+Builder)
├── channel/                        channel composition
│   ├── swap.rs                     SwapChannel
│   └── builder.rs                  TargetChannel + TargetChannelBuilder
└── middleware.rs                   ResolveStatusMiddleware + dedup state machine
```

The naming layer (`naming/`) is technically transport-agnostic and
could live unconditionally in `mssf_util::naming`. v1 keeps it nested
under `tonic` because `TargetConnector` is its only consumer; hoisting
it later is a `mv` plus internal `use`-path tweak that doesn't break
the public `mssf_util::tonic::*` paths.

## Architecture

Four composable layers, each at the level where it can naturally
see the signals it needs to act on:

```
+---------------------------------------------------+
|              user gRPC client                      |
|             (e.g. GreeterClient)                   |
+--------------------------+------------------------+
                           v
+---------------------------------------------------+
|        ResolveStatusMiddleware<SwapChannel>        |
|  - inspects gRPC trailers via http_body::Frame     |
|  - on trailer header (default `mssf-status`):      |
|      swap_channel.rebuild()  (non-blocking)        |
|  - propagates response unchanged (no retry, no kill)|
|  - Stateful dedup: see `Rebuild dedup`              |
+--------------------------+------------------------+
                           v
+---------------------------------------------------+
|                   SwapChannel                      |
|  - holds ArcSwap<tonic::Channel>                   |
|  - holds BoxCloneSyncService<Uri, ...> connector   |
|  - rebuild(): connect_with_connector_lazy + store  |
|  - in-flight requests stay on old Channel          |
+--------------------------+------------------------+
                           v
+---------------------------------------------------+
|   tonic::Channel (lazy; rebuilt per generation)    |
|  - hyper Client + connection pool                  |
+--------------------------+------------------------+
                           v
+---------------------------------------------------+
|                  TargetConnector                   |
|  - Service<Uri>: ask resolver, TCP dial            |
|  - holds Arc<dyn TargetResolver>                   |
+--------------------------+------------------------+
                           v
+---------------------------------------------------+
|       TargetResolver (e.g. FabricTargetResolver)   |
|  - owns previousResult cache, selector, timeout    |
|  - resolve(): complaint resolve + run selector     |
|              -> DialTarget                         |
+---------------------------------------------------+
```

### Why four layers, not one

Each failure mode is naturally observable at exactly one layer.

| Failure mode | Observable at | Action |
|---|---|---|
| TCP RST / `GOAWAY` / hyper IO error | hyper pool eviction | next dial → `TargetConnector` re-resolves; failing call surfaces to caller's outer retry |
| Trailer on response (success or error) | `Frame::trailers()` above the channel | `ResolveStatusMiddleware` calls `swap_channel.rebuild()`; response delivered unchanged |
| Plain `Code::Unavailable` (no trailer) | gRPC response | propagated unchanged; **no rebuild** (could be a downstream flake, not a role change) |

Cross-layer coupling is exactly one `Arc<SwapChannel>` clone held by
the middleware so it can call `rebuild()`. The connector knows
nothing about channels; the channel knows nothing about middleware.

### Why each layer is the way it is

- **Middleware above the channel.** Hyper does not expose gRPC
  trailers to the connector; trailers live on `http_body::Frame`,
  visible only above `tonic::Channel`. `ResolveStatusMiddleware` is
  the only layer that can observe "the call completed at HTTP/2 but
  the application told us we're talking to the wrong replica."

  **Layer-ordering constraint:** the middleware MUST sit directly
  above `SwapChannel`, with no body-transforming layers between
  them — anything that consumes the response body before trailers
  are read will hide them.
- **`SwapChannel` as its own layer.** The middleware sees the
  trailer; the connector does the next dial. But hyper will keep
  multiplexing new requests onto the existing alive HTTP/2
  connection until something tears it down. With the no-kill
  contract (server completes requests; client never yanks
  in-flight) nothing tears it down on its own. The only way to make
  hyper redial is to drop the Channel its pool lives inside.
  `SwapChannel::rebuild()` does exactly that, building a new
  Channel via `Endpoint::connect_with_connector_lazy` and storing
  it via `ArcSwap`. In-flight requests keep their own Channel
  clones and run to completion.

(A planned third design point — "connector lives below the TLS
wrapper" — is deferred along with TLS itself; see
[TLS (deferred)](#tls-deferred).)

### Why not `tower::reconnect::Reconnect`?

`Reconnect` rebuilds when `inner.call(req)` returns `Err`. Our
most important case is a trailer riding on a **successful**
response, where `Reconnect` never trips. There is also no way to
express "rebuild for next time without disrupting in-flight" — only
"errored ⇒ rebuild." `SwapChannel::rebuild()` is decoupled from
the response status: the middleware calls it after seeing the
trailer regardless of Ok/Err, and the response goes back to the
caller untouched.

`Reconnect` does still have a legitimate role for **bootstrap
fallback** — wrapping the whole `TargetChannel` so the *initial*
build can self-heal if SF naming is offline at startup. Listed in
[Future work](#future-work).

### Properties of the composition

- In-flight requests are never killed. `mssf-status` is a
  forward-looking hint about the *next* request.
- No background task. Refresh is synchronous in
  `TargetConnector::call` when hyper asks for a new connection;
  rebuild is synchronous (lazy allocation) in
  `SwapChannel::rebuild()`.
- Storm-safe dedup. The middleware tracks the last trailer value
  and only triggers `rebuild()` when the value differs (with a
  reset on no-trailer responses and an empty-value escape hatch).
  See [Rebuild dedup](#rebuild-dedup).
- The user-facing handle (`TargetChannel`) is stable across
  rebuilds; the inner `tonic::Channel` is what gets swapped.
- Zero datapath cost. No per-poll atomic checks, no `KillableIo`
  wrapper. The IO path is plain `TokioIo<TcpStream>`.

## Public surface

All types are re-exported flat from
[`mssf_util::tonic`](../../crates/libs/util/src/tonic/mod.rs):

| Type | Source | Role |
|---|---|---|
| [`TargetResolver`](../../crates/libs/util/src/tonic/naming/resolver.rs) | trait | "what should I dial next?" |
| `BoxError` | type alias | `Box<dyn Error + Send + Sync + 'static>` |
| [`DialTarget`](../../crates/libs/util/src/tonic/naming/selector.rs) | struct | `host: String, port: u16` |
| `TargetSelector` | type alias | `Arc<dyn Fn(&ResolvedServicePartition) -> Result<DialTarget, SelectError> + Send + Sync>` |
| `SelectError` | enum | `NoMatch \| Fatal(BoxError)` |
| [`FabricTargetResolver`](../../crates/libs/util/src/tonic/naming/default.rs) | struct | SF-naming impl of `TargetResolver` |
| `FabricTargetResolverBuilder` | struct | Builder for above |
| [`TargetConnector`](../../crates/libs/util/src/tonic/connector/service.rs) | struct | `Service<http::Uri>` doing resolve + TCP dial |
| `TargetConnectorBuilder` | struct | Builder for above |
| [`SwapChannel`](../../crates/libs/util/src/tonic/channel/swap.rs) | struct | `ArcSwap<tonic::Channel>` + rebuild |
| [`TargetChannel`](../../crates/libs/util/src/tonic/channel/builder.rs) | type alias | `ResolveStatusMiddleware<SwapChannel>` |
| `TargetChannelBuilder` | struct | Sugar that composes everything |
| [`ResolveStatusMiddleware<S>`](../../crates/libs/util/src/tonic/middleware.rs) | struct | trailer-aware `Service` middleware |

Signatures, `where`-bounds, and rustdoc live next to the code. This
doc only spells out behavior the impl can't express on its own.

## Usage

### Convenience builder

```rust
use mssf_util::tonic::{
    DialTarget, FabricTargetResolverBuilder, SelectError, TargetChannelBuilder,
};
use mssf_core::client::svc_mgmt_client::{
    PartitionKeyType, ResolvedServicePartition, ServiceEndpointRole,
};

let resolver = FabricTargetResolverBuilder::new(fabric_client)
    .service_uri("fabric:/MyApp/MyService")
    .partition_key(PartitionKeyType::None)
    .target_selector(|rsp: &ResolvedServicePartition| {
        let ep = rsp.endpoints.iter()
            .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
            .ok_or(SelectError::NoMatch)?;
        let url = url::Url::parse(&ep.address.to_string())
            .map_err(|e| SelectError::Fatal(e.into()))?;
        Ok(DialTarget {
            host: url.host_str()
                .ok_or_else(|| SelectError::Fatal("missing host".into()))?
                .to_string(),
            port: url.port()
                .ok_or_else(|| SelectError::Fatal("missing port".into()))?,
        })
    })
    .build();

let channel = TargetChannelBuilder::new()
    .resolver(resolver)
    .trailer_header("mssf-status")  // SDK convention; required setter
    .build();

let mut client = GreeterClient::new(channel);
```

### Manual composition

When the convenience builder isn't enough (custom layer
ordering, custom endpoint template):

```rust
let connector = TargetConnectorBuilder::new().resolver(resolver).build();
let endpoint = tonic::transport::Endpoint::from_static("http://fabric.invalid")
    .keep_alive_while_idle(true);
let swap = SwapChannel::new(endpoint, connector);
let channel = ResolveStatusMiddleware::new(
    swap.clone(),
    http::HeaderName::from_static("mssf-status"),
    move || swap.rebuild(),
);
```

### Two URIs: SF Fabric URI vs. hyper placeholder URI

| URI | Source | Format | Used for |
|---|---|---|---|
| **SF Fabric URI** | `FabricTargetResolverBuilder::service_uri("fabric:/...")` | `fabric:/App/Service` (`mssf_core::types::Uri`, **not** `http::Uri`) | Passed to `ServicePartitionResolver::resolve(name, ...)`. The connector never sees it. |
| **Placeholder URI** | `Endpoint::from_static("http://fabric.invalid")` (defaulted by `TargetChannelBuilder`) | `http(s)://...` (must parse as `http::Uri`) | Hyper connection-pool key; passed to `TargetConnector::call(uri)` which **ignores it** |

`fabric.invalid` uses the [reserved `.invalid`
TLD](https://datatracker.ietf.org/doc/html/rfc2606#section-2) so a
misconfigured layer that accidentally resolves the placeholder
authority fails loudly with NXDOMAIN rather than escaping to the
public internet.

### Writing a selector

There is no bundled `targets` module. SF endpoint addresses are
user-defined strings (no canonical encoding the SDK can parse on
the user's behalf), and role selection is a one-line closure.
Each app writes its own
`Fn(&ResolvedServicePartition) -> Result<DialTarget, SelectError>`.

The selector receives the **whole** `ResolvedServicePartition`
(not just `&[ResolvedServiceEndpoint]`) so it can also see
`service_name`, `service_partition_kind`, and `partition_key_type`
— enabling sticky-by-key selectors and selectors that parse extra
info encoded in the address.

`SelectError::NoMatch` is the soft failure case; the resolver
surfaces it as a `BoxError` and the caller's outer retry loop
decides what to do. `SelectError::Fatal` is non-retryable.

## Server contract

Failover detection has two cases. The v1 stack handles both, at
different layers.

### Case 1 — connection lost (transport-level)

Symptoms: TCP RST / `ECONNREFUSED` / `ECONNRESET`, or HTTP/2
`GOAWAY`. Hyper detects, evicts from the pool, and the next
request misses → hyper invokes `TargetConnector` →
`resolver.resolve()` returns the current `DialTarget` → fresh
TCP connection.

The middleware does **not** retry the failing call; it surfaces
to the caller's outer retry, which knows the call's idempotency.

This case occurs **only when the old primary's listener
disappears** (process crash, role-handler closes the gRPC server,
node loses connectivity). In normal stateful failover, the replica
process keeps running — Case 2 is the common path.

### Case 2 — server is alive but no longer the primary

Normal SF failover for stateful services with persistent replicas:

- The replica process keeps running through primary→secondary.
- Its gRPC listener stays open across the role change.
- Existing TCP / HTTP/2 connections stay alive.
- The server **completes** the in-flight request normally —
  success if the request was satisfiable as a secondary, or an
  error otherwise — and attaches the trailer (default
  `mssf-status`) to either to signal "the connection you used is
  no longer the right one for next time."

Without the middleware + `SwapChannel` pair, hyper would happily
keep multiplexing new requests onto the same alive HTTP/2
connection forever. With them, the middleware sees the trailer →
calls `rebuild()` → next request hits a fresh Channel with an
empty pool → connector dials the new primary.

### Required server behavior

A SF gRPC service that wants graceful client recovery MUST:

1. Check its current
   [`ServicePartitionAccessStatus`](../../crates/libs/core/src/runtime/stateful_types.rs)
   (or equivalent role gate) on every request that requires the
   chosen role.
2. **Complete the request normally** — success if it can be
   satisfied (e.g. read on a secondary if allowed), or an error
   (typically `Code::Unavailable`) if it cannot.
3. Attach the trailer (default `mssf-status`) to the response
   (success or error) when the role state is no longer correct
   for this client's intended target.

The server does **not** need to close the connection or send
`GOAWAY` on role change. Client-side invalidation is the
middleware's job.

### Trailer wire format

The trailer header name is **configured at the middleware** (passed
to `TargetChannelBuilder::trailer_header` or
`ResolveStatusMiddleware::new`); the SDK convention is
`mssf-status`. Values are ASCII opaque strings; the middleware
does string equality only.

The recommended SDK vocabulary:

- `not-primary` — this replica is no longer the primary.
- `not-readable` — reads are not granted in the current state.
- `reconfiguration-pending` — partition is mid-reconfiguration.

Any non-empty value is accepted; values are forward-compatible
(unknown values are treated like documented ones). Servers wanting
**per-event** distinction during multi-step failovers can append a
monotonic suffix (e.g. `not-primary:42`); the dedup will then
treat each as a distinct event.

Three signals the middleware distinguishes:

| Signal | Meaning |
|---|---|
| trailer absent | "I served you and I'm still the right one" — resets dedup |
| trailer present, value V | "switch for next time" — rebuild if V differs from last seen |
| trailer present, **empty value** | dumb-server escape hatch: always rebuild, don't store |

If the trailer block contains multiple entries under the
configured header, the first one wins. Server / client must agree
on the header name; mismatch silently disables rebuild (no error,
no log — the steady-state case is *supposed* to be silent).

### Streaming RPCs

The trailer arrives whenever the server **ends the stream**, not
before — a direct consequence of the no-kill contract. If the
server wants clients to switch mid-stream, it ends the stream with
the trailer attached (typically with `Status::unavailable`). A
server that keeps a stream open across a role change is declaring
"this stream is still mine to serve." The client honors that.

## Rebuild dedup

The dedup state machine has one piece of state — `last_seen:
Mutex<Option<String>>` — and four transitions. Implemented in
[`middleware.rs::classify`](../../crates/libs/util/src/tonic/middleware.rs)
and exhaustively unit-tested.

```
state                       observed                         action
----------------------      -------------------------        --------------------------
last_seen = None            (no trailer)                     no-op
last_seen = None            mssf-status: V (non-empty)       last_seen = Some(V); rebuild
last_seen = Some(V)         mssf-status: V (same value)      no-op
last_seen = Some(V)         mssf-status: W (different)       last_seen = Some(W); rebuild
last_seen = Some(_)         (no trailer)                     last_seen = None; no rebuild

independent (any state):    mssf-status: (empty value)       rebuild; last_seen UNCHANGED
```

**Concurrency.** The load → classify → store sequence runs under
a `std::sync::Mutex`, so concurrent in-flight RPCs that complete
with the **same** trailer value collapse to one `rebuild()`
call. Distinct values still produce one rebuild each. The mutex
is released **before** invoking the rebuild closure so back-to-back
`connect_with_connector_lazy` calls don't serialize. The critical
section is a string comparison + an `Option` write (microseconds);
contention is bounded by concurrent trailer arrivals.

Why each transition matters:

- **Different value rebuilds.** A new failover event is a new event.
- **Same value is a no-op.** A server that conservatively attaches
  `reconfiguration-pending` to *every* response during a reconfig
  window (tens of seconds) produces one rebuild for the whole
  window, not N.
- **No trailer resets.** Without the reset, the middleware would be
  stuck at `Some(V)` forever and miss the *next* event if it used
  the same string V. With the reset, a successful no-trailer
  response is a positive "still the right one" re-arming signal.
- **Empty value rebuilds without storing.** The dumb-server
  contract: a server too simple to track state attaches an empty
  trailer to any response that should rebuild; every empty trailer
  rebuilds unconditionally. We deliberately don't store
  `Some("")` — that would defeat the contract by deduping
  subsequent empty trailers.

### Out-of-order trailers

Multiple in-flight RPCs may complete in any order. A stale trailer
arriving after a newer one will trigger one redundant rebuild
(the new Channel converged on the right target already, but we
rebuild it again then immediately re-converge on the same target).
Cost is bounded by in-flight count; harmless to correctness.
Servers concerned about this can include a monotonic token so the
dedup distinguishes stale from fresh.

## Caller-side retry

**The v1 stack does not auto-retry on any signal.** Every
response, success or error, is delivered to the caller unchanged.
The middleware's only effect is to fire `swap_channel.rebuild()`
for *future* requests when it sees the trailer.

This is deliberate: we cannot tell at the middleware layer whether
a mid-stream RST means the request reached a handler, and silently
retrying a non-idempotent call that already mutated state is
exactly the bug `tower::retry` plus caller-owned idempotency rules
are designed to avoid. Idempotency is the caller's policy.

For all error cases the caller needs an outer retry loop:

1. A `tower::retry::Retry` layer wrapping `TargetChannel`,
   configured with the caller's idempotency rules.
2. The existing
   [`OperationRetryer`](../../crates/libs/util/src/retry.rs)
   wrapped around individual gRPC calls.
3. Application-level retry at the call site for non-idempotent
   operations.

## Resolve strategy

v1 uses **complaint-based resolve only**. Per the .NET docs for
[`ResolveServicePartitionAsync`](https://learn.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.servicemanagementclient.resolveservicepartitionasync):

> When called **with** `previousResult` … the system will try to
> return a more up-to-date `ResolvedServicePartition` than
> `previousResult` in the most efficient way possible.

`FabricTargetResolver` always passes the cached RSP back as
`previousResult`. SF either confirms (same-version reply) or
returns something fresher. The connector never decides whether to
refresh; it just always asks. Eliminates an entire class of
"did we remember to invalidate the cache?" bugs.

**Why not notifications in v1.** Filter lifecycle (`Drop` can't
`await`), the FabricClient cleanup race
([issue #184](https://github.com/Azure/service-fabric-rs/issues/184)),
missed/out-of-order notifications, and a small steady-state
latency win on a *failover-recovery* path don't justify the
complexity. See [Future work](#future-work) for the opt-in mode.

## TLS (deferred)

v1 ships **plain TCP only**. The `TargetChannelBuilder` has no
`with_tls(...)` setter and `SwapChannel` fixes its connector slot
at `BoxCloneSyncService<Uri, TokioIo<TcpStream>, BoxError>`,
which is incompatible with any TLS wrapper that returns
`TokioIo<TlsStream<...>>`.

This was a deliberate scope cut once an earlier `with_tls` API
was found to be unusable: the bound matched only plain TCP, so
no real TLS connector — `tonic_tls::*::TlsConnector<...>`,
rustls, openssl, native-tls, schannel — actually fit. Shipping
an API whose type bounds reject every realistic implementation
is worse than not shipping the API.

### What enabling TLS will require

1. **Generalize `SwapChannel`'s IO bound.** Replace the fixed
   `TokioIo<TcpStream>` with whatever
   [`Endpoint::connect_with_connector_lazy`](https://docs.rs/tonic/0.14/tonic/transport/struct.Endpoint.html#method.connect_with_connector_lazy)
   actually requires (`hyper::rt::Read + hyper::rt::Write +
   Send + Unpin + 'static`). Two implementation shapes:
   - **Erase further:** store the IO behind a
     `Box<dyn AsyncRead + AsyncWrite + ...>` so `SwapChannel`
     stays non-generic. Adds a vtable hop per byte; probably
     fine.
   - **Re-generic:** add a type parameter `<S, IO>` to
     `SwapChannel`. Loses the type-erasure benefit; users have
     to thread a connector type parameter through their
     wiring.
2. **Add a TLS composition seam.** Either ship a
   `tonic-tls` cargo sub-feature with a
   [`tonic_tls::Transport`](https://github.com/youyuanwu/tonic-tls)
   impl on `TargetConnector` (so `TlsConnector::new(target,
   ssl, sni)` slots straight in), or document the manual
   composition pattern with the generalized `SwapChannel`.
3. **Decide the SNI policy.** tonic-tls takes SNI as an
   explicit parameter; SF endpoint addresses are arbitrary
   user-defined strings, so we can't auto-derive SNI from the
   resolved endpoint. v1's selector returns only host+port;
   passing SNI through requires either widening `DialTarget`
   (breaking change for existing selectors) or making SNI a
   builder-time configuration on the TLS layer.

None of the above is hard — the trailer + rebuild + dedup
plumbing is already TLS-agnostic. It just hasn't been done yet,
and was scoped out so v1 ships with an honest API surface.

When `SwapChannel::rebuild()` eventually runs over a
TLS-wrapped connector, it composes naturally: new `tonic::Channel`
→ empty pool → next request triggers `tls_conn.call(uri)` →
`target_conn.call(uri)` (fresh TCP via SF resolve) → TLS
handshake on the new TCP stream → HTTP/2 connection. Old TLS
connections close along with the rest of the old hyper pool.

## Refresh path

The connector is invoked by hyper (via `tonic::Channel`'s
`MakeConnection` hook) whenever the pool needs a new connection.
**No background task**, **no channel cache** (hyper's pool serves
that role), **no in-process single-flight mutex** (hyper's pool
checkout already serializes `MakeConnection` calls per pool key).

Two paths in:

1. **`SwapChannel::rebuild()` (trailer-driven, Case 2).**
   Middleware → rebuild → new `tonic::Channel` with empty pool →
   next request pool-misses → connector → resolver returns the
   post-failover `DialTarget`.
2. **Hyper pool miss on the existing Channel (transport-error,
   Case 1).** Old connection died. Hyper evicts it; next request
   misses; same connector path runs.

In both, the connector's behavior is identical: always pass the
cached RSP back to SF as `previousResult`.

### Why always resolve?

A simpler alternative to tracking "is the cache stale?" is to
always pass the cache back and let SF's complaint protocol confirm
or supersede it. Trade-offs:

- For single-primary selectors, this does the same network work
  as a heuristic-based design — every connector invocation issues
  one resolve.
- For non-deterministic selectors (round-robin, custom shard
  routing), it costs one extra resolve per dial that *would* have
  been served from a stable cached RSP. The resolve is cheap
  (same-version reply) and dials are per-connection, not
  per-request.
- Eliminates an entire class of correctness bugs.

### Mental model: Fabric URI ≈ DNS name

| | DNS | Service Fabric |
|---|---|---|
| Stable name | hostname | Fabric URI (`fabric:/App/Service`) |
| Resolves to | A/AAAA records | `ResolvedServicePartition` |
| Resolution mechanism | UDP/TCP query | FabricClient COM call |
| Client cache | OS resolver, TTL-bound | FabricClient cache, version-tracked |
| Stale-result feedback | none — TTL only | `previousResult` complaint |
| Re-resolve on transport failure | **no** (hyper's IP-pinning sharp edge) | **yes** (`TargetConnector`) |
| Re-resolve on application-level signal | **no** | **yes** (`SwapChannel::rebuild()` on trailer) |

The connector is the "pluggable resolver" piece that hyper
deliberately doesn't ship for DNS, plus an application-level
invalidation path that SF naming exposes via the trailer.

## Lifecycle & cleanup

- All public types are `Clone`. Clones share `Arc<Inner>`.
- Sync construction; no IO until first request.
- Drop of last `Arc<Inner>` releases everything. No background task,
  no notification filter, no `Drop` dance. Pooled HTTP/2
  connections are owned by hyper inside whatever generation of
  `tonic::Channel` is alive; they close when the last in-flight
  response on each generation drops.
- Graceful shutdown: drop the channel handle. In-flight requests
  hold their own clones of whatever generation of inner
  `tonic::Channel` they were dispatched on; they run to completion
  if the tokio runtime is still alive. We deliberately do not add
  an explicit `shutdown()` API; the no-kill contract makes
  "drop and let outstanding finish" the natural pattern.

## Concurrency

- Per-RPC cancellation: tonic's built-in cancellation propagation
  works as-is.
- Connector-call cancellation falls out of future-drop. The only
  shared per-call state is the resolver's
  `ArcSwapOption<ResolvedServicePartition>`, which is only
  *written* on a successful resolve.
- `SwapChannel::rebuild()` is non-blocking and always produces a
  new Channel. Storm dedup is the middleware's job; concurrent
  same-value trailer arrivals serialize through the middleware's
  `Mutex<Option<String>>` (see [Rebuild dedup](#rebuild-dedup))
  and collapse to a single `rebuild()` call.
- Concurrent failing requests on one Channel are deduplicated by
  hyper's pool checkout (one `MakeConnection` per pool key).
- `TargetResolver::resolve()` takes no arguments in v1. The
  builder-configured `resolve_timeout` plus the resolver's
  internal retryer bound the call. A future revision can add
  `&CancellationToken` additively.

## Implementation deviations from earlier drafts

A few small differences between this doc and what shipped:

- **Type erasure uses `BoxCloneSyncService`, not `BoxCloneService`.**
  `SwapChannel` is shared via `Arc<Inner>` between the user-facing
  service and the rebuild closure captured by the middleware, so
  the inner connector slot must be `Sync`. `TargetConnector`
  satisfies this; any user-supplied connector must too.
- **`ResolveStatusMiddleware::new` is the only constructor.** The
  earlier draft showed `layer` / `layer_with_rebuild`
  factory-style `tower::Layer` helpers; the impl ships only
  `new(inner, header_name, rebuild)`, which is enough for both the
  convenience builder and manual composition. A `Layer` impl is
  trivial to add if/when a user needs one.
- **Readiness driven inside the response future.** `SwapChannel`'s
  `poll_ready` always returns `Ready(Ok(()))`; the
  per-`tonic::Channel` readiness is awaited inside the future
  returned by `call`. Necessary because tonic's `Channel` is
  internally buffered (`poll_ready` and `call` must hit the same
  instance); also avoids leaking a stale readied snapshot via
  `#[derive(Clone)]` when a layer above us clones `SwapChannel`
  between `poll_ready` and `call` — and across a `rebuild()`.
- **`tonic-tls` adapter is deferred.** See
  [Composition with tonic-tls](#composition-with-tonic-tls). The
  `tonic-tls` cargo sub-feature isn't shipped yet; users wire TLS
  via `SwapChannel::with_connector`.
- **The integration test against a live SF cluster is deferred.**
  See [Testing](#testing).

## Testing

### Unit / integration tests (shipped)

- 6 dedup state-machine tests in
  [`middleware.rs`](../../crates/libs/util/src/tonic/middleware.rs).
- 7 end-to-end middleware tests in
  [`tonic_middleware.rs`](../../crates/libs/util/tests/tonic_middleware.rs)
  using a scripted inner `Service` + scripted bodies with trailers:
  trailer→rebuild, same-value dedup, distinct-value rebuilds,
  no-trailer reset, empty-value escape hatch, empty-value twice,
  unrelated-header ignored.
- 3 e2e failover tests in
  [`tests/mssf-tests/tests/tonic_failover.rs`](../../tests/mssf-tests/tests/tonic_failover.rs)
  spinning up two real HTTP/2 servers on ephemeral ports with
  graceful shutdown:
  - `failover_via_trailer_and_resolver_flip` — A trailers always,
    flip resolver to B between calls; assert routing landed
    correctly and resolver call counts are exactly 2.
  - `no_trailer_no_rebuild_pool_reused` — steady-state HTTP/2 pool
    reuse, exactly 1 resolver call across 3 requests.
  - `server_becomes_stable_after_one_trailer` — single server
    starts unstable then quiesces; exercises the `Reset`
    transition + same-target rebuild + steady state.

### Deferred — live-cluster failover sample

Originally planned as a stateful `ReflectionApp` with `MyGreeter`
honoring the trailer contract, plus `move_primary` /
`restart_replica` orchestration. Deferred — the e2e mock-server
suite covers the channel layers, the dedup state machine, and the
failover orchestration deterministically. The live-cluster test
is most useful for validating
[`FabricTargetResolver`](../../crates/libs/util/src/tonic/naming/default.rs)'s
always-complain bookkeeping and address-parse selectors against
real SF naming, neither of which is exercised by the mock tests.

When it lands, the test plan is:

1. Stateful service with target/min replicas (3,3,0); `MyGreeter`
   gates on `ServicePartitionAccessStatus` and attaches
   `mssf-status: not-primary` on writes from non-primaries.
2. Build a `TargetChannel` for the service URI with a primary-only
   selector.
3. Clean failover (Case 2): `move_primary` so the former primary
   stays up as a secondary. First call returns trailer; second
   call lands on new primary without test-level retry.
4. Process-restart failover (Case 1): `restart_replica`; next call
   succeeds without test-level retry.
5. In-flight survival under failover.
6. TLS composition repeat once the `tonic_tls::Transport` adapter
   ships.

## Open questions

Most of the original 14 are resolved or moved to Future Work.
Genuinely open:

- **Exposed types.** We expose `ResolvedServicePartition` (and
  transitively `ResolvedServiceEndpoint`) directly to the
  user-supplied selector. Alternative: a smaller `EndpointInfo`
  struct insulating users from internal type churn. v1 prioritizes
  not losing information. Custom `TargetResolver` impls don't see
  RSP, so this is strictly a `FabricTargetResolver` concern.
- **Generic transport.** `TargetConnector` doesn't depend on tonic
  at the type level — it's a hyper-compatible `Service<Uri>`.
  Could be advertised as usable with any `hyper::Client`. v1
  documents the tonic recipe; generic-HTTP usage is supported but
  not the headline.

## Future work

- **TLS support.** See [TLS (deferred)](#tls-deferred). Requires
  generalizing `SwapChannel`'s IO bound, plus either a
  `tonic_tls::Transport` adapter on `TargetConnector` (behind a
  `tonic-tls` cargo sub-feature) or a documented manual recipe.
  An earlier `TargetChannelBuilder::with_tls` API and the
  matching `SwapChannel::with_connector` TLS framing were
  removed before the v1 commit because their type bounds (fixed
  at `TokioIo<TcpStream>`) rejected every realistic TLS
  connector.
- **Live-cluster failover sample / test.** See
  [Testing](#testing).
- **Trailer-aware caller retry recipe.** A worked
  `tower::retry::Policy` that inspects the trailer for callers
  who want to retry on `not-primary` even when the gRPC status
  isn't `Unavailable`. Requires body-plumbing in the policy to
  read trailers before deciding; v1 sidesteps this by guaranteeing
  the next request goes through a fresh Channel.
- **Notification-backed resolve mode.** Opt-in mode that registers
  a `ServiceNotificationFilterDescription` so steady-state
  resolves hit the FabricClient cache. Requires designing for
  filter lifecycle on `Drop`, the FabricClient cleanup race, and
  fall-back to complaint resolve when notifications go quiet.
- **Bootstrap fallback via `tower::reconnect::Reconnect`.** Today
  the *first* dial happens on the first user request; if SF naming
  is offline at startup, that first request fails. Wrapping
  `TargetChannel` in `Reconnect<TargetChannelMaker>` would let it
  self-heal on subsequent requests.
- **`Channel::balance_channel`** for fan-out to stateless /
  secondary replicas.
- **Per-call deadline propagation** from the gRPC call into the
  resolver call. Requires plumbing the deadline through hyper's
  `Service<Uri>` invocation, which has no standard mechanism.
- **Metrics hooks** (`tracing` spans for resolve / dial / rebuild
  / trailer detection).
- **Hoist the naming layer.** When a second client lands (e.g. a
  non-tonic transport over SF), move `naming/` to
  `mssf_util::naming` and have `mssf_util::tonic` re-export.
  Internal refactor; `mssf_util::tonic::*` paths stay stable.

## References

- [`crates/libs/core/src/client/svc_mgmt_client.rs`](../../crates/libs/core/src/client/svc_mgmt_client.rs) — `resolve_service_partition`, partition key types, `ResolvedServicePartition`.
- [`crates/libs/util/src/resolve.rs`](../../crates/libs/util/src/resolve.rs) — `ServicePartitionResolver`.
- [`crates/samples/reflection/src/test.rs`](../../crates/samples/reflection/src/test.rs) — current tonic usage and manual failover handling.
- [`crates/samples/reflection/src/test2.rs`](../../crates/samples/reflection/src/test2.rs) — `resolve_until_change(.., complain=true)` (prior art).
- [`ResolveServicePartitionAsync` remarks](https://learn.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.servicemanagementclient.resolveservicepartitionasync) — complaint protocol semantics.
