# Reflection App: Test-Controlled Replica Behavior via gRPC

## Status

**Implemented.** See [crates/samples/reflection/src/control.rs](../../crates/samples/reflection/src/control.rs),
[grpc_control.rs](../../crates/samples/reflection/src/grpc_control.rs),
[proto/control.proto](../../crates/samples/reflection/proto/control.proto),
[proto/initdata.proto](../../crates/samples/reflection/proto/initdata.proto),
the e2e test in [tests/control_e2e.rs](../../crates/samples/reflection/tests/control_e2e.rs),
and the operator CLI at [reflection_ctl/main.rs](../../crates/samples/reflection/reflection_ctl/main.rs).

## Motivation

The reflection sample is used for integration tests against a real
Service Fabric cluster. Without control, lifecycle methods (`open`,
`change_role`, `close`, `abort`) run as fast as SF drives them, which
makes it hard to test slow opens, failed change-role operations,
hangs during close, abort-during-close races, and similar
interleavings.

This design lets a test driver gate every lifecycle method via a
gRPC service that the reflection sample hosts on every node. The
test reaches in, says "wait for `open` on this replica → return
this decision", and SF observes the replica's behaviour as if the
service were genuinely slow / faulty.

## Goals

1. Per-`(partition_id, replica_id)` control of when `open` /
   `change_role` / `close` / `abort` return and what they return.
2. Mode is chosen at service-create time via SF's
   `InitializationData`. A service created without setting initdata
   uses the production code path with no behaviour change.
3. Multiple replicas controllable independently and concurrently.
4. Production path adds one async call returning
   `Decision::Proceed` and one `Arc` clone per lifecycle method —
   negligible for the sample's scale.

## Non-Goals

- Replication faults inside the replicator (use SF fault-injection
  APIs instead).
- Persisting test commands across replica restarts.
- Auth on the gRPC channel — the channel is bound to the cluster
  network; see §Security.
- Cross-machine clusters. The transport scheme is designed for
  local onebox only.
- Driving SF itself (e.g., forcing failovers).

## Architecture

```
                    SF runtime                Test driver
                       │                          │
                       │ create_replica(initdata) │
                       ▼                          │
                ┌─────────────┐                   │
   Factory ───► │  Replica    │                   │
                │  + ctrl     │◄──────── ReplicaControl gRPC ────────┐
                └─────────────┘                   │                   │
                       │                          │                   │
                       │ open / change_role /     │                   │
                       │ close / abort            │                   │
                       ▼                          │                   │
                await_approval(gate)              │ WaitForApproval/  │
                       │                          │ Approve / List    │
                       │                          │ Pending / Detach  │
                       └──────────────────────────┴───────────────────┘
                       (parks at gate_lock + oneshot)
```

Each `Replica` holds an `Arc<dyn ReplicaController>`:

| Initdata (`ReplicaInitData`) | Controller         | Behaviour                                          |
|---|---|---|
| empty / decode failure / `control = false` | `NoopController` | `await_approval` returns `Decision::Proceed` inline; not registered |
| `control = true`                            | `GrpcController` | Lifecycle parks at `gate_lock` until `Approve` arrives over gRPC |

## Components

### 1. `ReplicaController` trait
([src/control.rs](../../crates/samples/reflection/src/control.rs))

```rust
#[async_trait]
pub trait ReplicaController: Send + Sync + Debug {
    async fn await_approval(&self, gate: Approval) -> Decision;
    fn is_controllable(&self) -> bool { false }
    fn as_any(&self) -> &dyn std::any::Any;
}

pub enum Approval { Open, ChangeRole(ReplicaRole), Close, Abort }
pub enum Decision { Proceed, Fail(mssf_core::Error) }
```

`as_any` lets the gRPC handler downcast to `GrpcController` for the
inspection methods (`peek_pending`, `wait_for_approval`, `approve`,
`detach`) that aren't on the trait.

### 2. `NoopController`
Stateless. `await_approval` returns `Decision::Proceed` immediately;
never registered with `ReplicaRegistry`. This is the only path that
runs when initdata is empty — preserves current behaviour for every
existing test.

### 3. `GrpcController`

State:

```rust
pub struct GrpcController {
    gate_lock: tokio::sync::Mutex<()>,            // serializes await_approval
    pending:   std::sync::Mutex<Option<Pending>>, // observation slot
    notify:    tokio::sync::Notify,               // wakes WaitForApproval
    detached:  AtomicBool,                        // proceed-forever flag
}

struct Pending {
    gate_id: Uuid,                                // fresh per gate
    gate:    Approval,
    sender:  oneshot::Sender<Decision>,
}
```

`await_approval(gate)`:

1. If `detached`, return `Decision::Proceed` immediately.
2. Acquire `gate_lock` (held across the receiver `await`). A
   concurrent call from another lifecycle method blocks here until
   the current one completes — this is what enforces single
   occupancy of `pending`.
3. Re-check `detached` (in case `Detach` fired while we were queued).
4. Mint a fresh `gate_id` (UUID v4), create a `oneshot`, publish
   `Pending { gate_id, gate, sender }` under the std mutex, drop
   the std mutex, call `notify.notify_waiters()`.
5. `await` the receiver. On `Drop` of the controller the receiver
   resolves with `Err`, mapped to `Decision::Proceed`.
6. Clear `pending`, release `gate_lock`.

`wait_for_approval(expected)` (gRPC `WaitForApproval` handler) does
NOT touch `gate_lock`. It loops:

```rust
loop {
    let notified = self.notify.notified();   // register interest first
    tokio::pin!(notified);
    if let Some(ev) = self.peek_matching(expected) {
        return ev;                            // never consumes pending
    }
    notified.await;
}
```

`approve(gate_id, decision)` consumes `pending` only when
`pending.gate_id == gate_id`, then sends on the stored oneshot.
Mismatched id → `FailedPrecondition`.

`detach()` flips the atomic and releases any pending oneshot with
`Decision::Proceed`. Operator-only escape hatch — tests should walk
gates explicitly so they're actually testing approval semantics.

### 4. Initdata pipeline
([src/control.rs](../../crates/samples/reflection/src/control.rs),
proto [proto/initdata.proto](../../crates/samples/reflection/proto/initdata.proto))

Wire format (forward-additive; do not change existing field numbers
or types):

```proto
message ReplicaInitData {
  bool control = 1;
  // future tunables go here
}
```

Decoding and policy are split so each is independently testable:

```rust
// Wire format only; empty bytes / decode error → default message.
pub fn decode_init_data(bytes: &[u8]) -> ReplicaInitData;

// Pure mapping; no I/O, no logging.
pub enum ControlMode { NoControl, Control }
impl ControlMode {
    pub fn from_init_data(msg: &ReplicaInitData) -> Self;
}

pub fn make_controller(mode: ControlMode) -> Arc<dyn ReplicaController>;
```

`Factory::create_replica` composes these and registers the
controller with the registry only when `is_controllable()` is true.

### 5. `ReplicaRegistry` extension
([src/grpc.rs](../../crates/samples/reflection/src/grpc.rs))

Existing entry struct gains an optional controller handle. Lookup
by `(partition_id, replica_id)` returns `None` for nocontrol
replicas, which the gRPC handler maps to `NotFound`.

```rust
pub struct ReplicaEntry {
    pub partition_id: GUID,
    pub replica_id:   i64,
    pub role:         ReplicaRole,
    pub controller:   Option<Arc<dyn ReplicaController>>,  // NEW
}
```

### 6. `Replica::abort` sync→async bridge
([src/statefulstore.rs](../../crates/samples/reflection/src/statefulstore.rs))

`abort` is a sync trait method but `await_approval` is async. The
`Replica` already owns a `TokioExecutor`; bridge with
`block_on_any`:

```rust
fn abort(&self) {
    let controller = self.controller.clone();
    self.exec.block_on_any(async move {
        let _ = controller.await_approval(Approval::Abort).await;
    });
    // ... existing abort logic ...
}
```

The decision is intentionally discarded — `IStatefulServiceReplica::abort`
returns `()` and cannot fail. Under `NoopController` this resolves
inline. Under `GrpcController`, if a previous lifecycle method
(typically `close`) is still parked, this `block_on_any` queues at
`gate_lock` until the prior gate is approved or the controller is
dropped.

### 7. gRPC service
([proto/control.proto](../../crates/samples/reflection/proto/control.proto),
[src/grpc_control.rs](../../crates/samples/reflection/src/grpc_control.rs))

Served on the same `tonic::transport::Server` as the existing
`Greeter` service.

| RPC               | Purpose |
|---|---|
| `WaitForApproval` | Block until replica reaches a gate (or timeout). Server-cap 30 s default / 5 min max. |
| `Approve`         | Release the matching pending gate with `Proceed` or `Fail(message)`. |
| `ListPending`     | Snapshot of in-flight gates; optional partition / replica filter. |
| `Detach`          | Operator escape hatch — flip a controller into proceed-forever mode. |
| `DetachAll`       | Same, every controllable replica on this server. |

Filters use a `oneof` rather than a sentinel value so any `int64`
(including 0) is an exact match:

```proto
message ListPendingRequest {
  string partition_id = 1;        // empty = all partitions
  oneof replica_filter {
    int64 specific_replica_id = 2;  // unset = all replicas
  }
}
```

### 8. Test transport: fixed port + per-node offset

Onebox runs every SF node as a process sharing one host. SF
allocates each node a slice of `<ApplicationEndpoints>` (Linux
`22001–27000`, Windows `30001–35000`). To avoid colliding with
SF's allocator and the OS ephemeral ranges, `ReplicaControl` binds
to a fixed port `28000 + node_index(Fabric_NodeName)` on `0.0.0.0`.

| Platform | Manifest | Node names      | Index |
|---|---|---|---|
| Windows  | [onebox/windows/ClusterManifestTemplate.json](../../onebox/windows/ClusterManifestTemplate.json) | `_Node_0..4`     | strip leading `_Node_`, parse |
| Linux    | [.devcontainer/onebox/ClusterManifest.SingleMachineFSS.xml](../../.devcontainer/onebox/ClusterManifest.SingleMachineFSS.xml) | `N0010..N0050`   | strip leading `N`, divide by 10, subtract 1 |

`grpc_control::node_index` is `cfg(target_os = ...)`-gated so each
arm only knows about the manifest format that produces it. Both
arms produce `0..=4`, so per-node ports are `28000..=28004`
regardless of platform. Test driver dials all candidate ports and
treats `Connection refused` as "no replica here" (handles partial
placement). The test crate consumes the same constant + parser so
nothing diverges between client and server.

### 9. RPC error model

| RPC | Status                              | When                                                                 |
|---|---|---|
| any                  | `NotFound`                | Target replica not registered (or already removed).                 |
| `WaitForApproval`    | `DeadlineExceeded`        | Server timeout elapsed; replica stays parked; client may reissue.   |
| `WaitForApproval`    | `InvalidArgument`         | Missing/bad `partition_id`, or unknown `expected` enum value.       |
| `Approve`            | `FailedPrecondition`      | Slot empty, OR `gate_id` mismatch (distinguished in message text).  |
| `Approve`            | `InvalidArgument`         | `fail_message` set on an `APPROVAL_ABORT` gate (SF's `abort` can't fail). |
| `ListPending`        | `InvalidArgument`         | `specific_replica_id` set with empty `partition_id`.                |
| `Detach`             | `NotFound`                | Replica not registered.                                              |

## Default Behaviour Without a Test Client

Replicas of services created without `InitializationData`, or with
`control = false`, use `NoopController`. `await_approval` returns
`Decision::Proceed` inline and the replica is never registered with
the gRPC server. Existing tests in
[src/test.rs](../../crates/samples/reflection/src/test.rs) and
[src/test2.rs](../../crates/samples/reflection/src/test2.rs) (none
of which set initdata) see no behaviour change.

A test that wants control opts in by encoding a `ReplicaInitData`:

```rust
use prost::Message;
use samples_reflection::control::ReplicaInitData;

let initdata = ReplicaInitData { control: true }.encode_to_vec();
let desc = StatefulServiceDescription::new(/* ... */)
    .with_initialization_data(initdata)
    /* ...rest of builder chain... */;
```

**Assumption.** SF persists `InitializationData` durably with the
service registration and re-passes the same bytes to every
`create_replica` call across failovers, application upgrades, and
cluster restarts. If this guarantee is violated, controlled
services could silently downgrade to `NoControl` (test hangs) or
production services could silently upgrade to `Control` (every
lifecycle gates on a never-arriving `Approve`). The integration
test in [tests/control_e2e.rs](../../crates/samples/reflection/tests/control_e2e.rs)
exercises a full create→delete cycle and would catch a regression
in the pipe; a failover sub-test is a recommended future addition.

## Replica Lifetime as Seen by the Test Client

The server has no "replica gone" event; tests learn that a replica
is gone via `NotFound`. Normal close-then-remove sequence:

1. SF calls `Replica::close` → `await_approval(Close)` parks.
2. Test `WaitForApproval` returns `ApprovalEvent { kind: APPROVAL_CLOSE, gate_id }`.
3. Test `Approve(gate_id, proceed)` → oneshot fires, `await_approval` returns,
   `Replica::close` runs `ReplicaRegistry::remove`.
4. SF eventually drops the `Replica`. `GrpcController::Drop` runs;
   `pending` is already empty so it's a no-op.

| Test does this… | Server returns… | Meaning |
|---|---|---|
| `WaitForApproval` after step 4 | `NotFound` | Terminal — stop polling. |
| `Approve(gate_id, _)` for an old `gate_id` | `NotFound` (entry gone) or `FailedPrecondition` (entry present but slot empty / id mismatch) | Don't retry. |
| In-flight `WaitForApproval` blocked at removal | `NotFound` next loop iteration | Same. |
| `ListPending` after step 4 | Replica absent | Same. |

Notes:

- **Drop releases pending with `Proceed`, not `Fail`.** A forgotten
  test must not make SF think close failed. Trade-off: a test
  cannot distinguish "I approved" from "controller dropped before I
  approved" purely from SF's perspective — observe whether its own
  `Approve` returned `Ok` or `NotFound`.
- **A new replica with the same `replica_id` may appear later.** It
  registers fresh `gate_id`s, so the UUID check still protects
  against misrouted `Approve` from the previous incarnation.
- **`abort`-then-remove follows the same shape** with
  `Approval::Abort` in step 1.

## Concurrency and Failure Modes

- **Single occupancy of `pending` is enforced, not assumed.**
  `gate_lock` serializes all `await_approval` calls per replica;
  an `abort` arriving while `close` is parked queues at the lock
  until the test approves close, then the abort gate becomes
  pending. SF's serialization is not relied on.
- **Stale-approve race is closed by per-gate UUIDs.** Without the
  id check, a sequence like `Open(A) → test sees A → controller
  drops A → fresh Open(B) → test's stale Approve lands on B` would
  misroute the decision. `Approve(gate_id != pending.gate_id)` is
  rejected, and the test re-issues `WaitForApproval` to learn the
  current gate.
- **`await_approval` vs. `WaitForApproval` ordering race.** Two
  valid orderings:
  - *Test-first:* handler parks on `Notify`; `await_approval`
    populates `pending` and notifies; handler wakes and returns.
  - *SF-first:* `await_approval` populates `pending` and notifies
    while no handler is parked (notification dropped); next
    `WaitForApproval` finds the populated slot on its first peek.
  Both work because the handler registers `notified` *before*
  inspecting state.
- **`Approve` arrives before any `WaitForApproval`** →
  `FailedPrecondition` (slot empty). v1 doesn't pre-stage decisions.
- **`Approve` after the replica is gone** → `NotFound`.
- **Replica removed mid-wait.** `Drop` releases pending with
  `Proceed`; in-flight `WaitForApproval` returns `NotFound` on
  next iteration.
- **Sync→async bridge from outside a tokio context.**
  `TokioExecutor::block_on_any` uses `block_in_place` on a worker
  and `Handle::block_on` otherwise. The multi-threaded runtime is
  enforced by `TokioExecutor::new` in
  [crates/libs/util/src/tokio/mod.rs](../../crates/libs/util/src/tokio/mod.rs).
- **Lock order.** `gate_lock` (tokio) outer; `pending` (std) inner,
  held only briefly to publish/clear `Pending`, never across an
  `await`. No inversion possible.
- **Multiple `WaitForApproval` clients per replica.** All receive
  the same event; only the first to issue `Approve(gate_id, _)`
  succeeds; the rest see `FailedPrecondition`. Intentional — tests
  sharing a replica must coordinate.

## Operator Tooling

[reflection_ctl/main.rs](../../crates/samples/reflection/reflection_ctl/main.rs)
is a thin clap CLI for manual cluster operations:

```text
reflection_ctl ping                      # which nodes are reachable
reflection_ctl list [--partition X]      # what's pending where
reflection_ctl approve --partition X --replica Y [--fail-message M]
reflection_ctl approve-all [--yes]       # bulk-approve everything
reflection_ctl detach --partition X --replica Y
reflection_ctl detach --all              # proceed-forever everywhere
```

`Detach` is for unsticking the cluster after a test panics with
parked gates. Tests themselves should walk gates explicitly.

## Security

The gRPC server binds `0.0.0.0:28000+node_index` with no auth.
Acceptable for onebox — the cluster network is the security
boundary (loopback on Windows, a private docker network on Linux
devcontainer). For non-test environments, gate the
`ReplicaControl` server behind a `--enable-test-control` CLI flag
or a Cargo feature.

## Test Patterns Enabled

All assume `ReplicaInitData { control: true }` initdata.

| Scenario | Sequence |
|---|---|
| Slow open                       | `WaitForApproval(OPEN)` → sleep → `Approve(proceed)`. |
| Open failure                    | `WaitForApproval(OPEN)` → `Approve(fail_message="boom")`. SF retries / reports fault. |
| ChangeRole hang then succeed    | After open: `WaitForApproval(CHANGE_ROLE)` → hold → `Approve(proceed)`. |
| Approve abort                   | After whichever method ran last: `WaitForApproval(ABORT)` → `Approve(proceed)`. Decision payload is ignored. |
| Hang abort to inspect state     | While `close` is parked, SF eventually issues `abort`; `Replica::abort`'s `block_on_any` blocks at `gate_lock` until the test approves close. Then the abort gate becomes pending. |
| Close-then-remove               | `WaitForApproval(CLOSE)` → `Approve(proceed)`. Subsequent calls return `NotFound`. |
| Failover ordering across replicas | Two `WaitForApproval` calls in parallel; release the new primary's `CHANGE_ROLE` only after the old primary's `CLOSE` is observed. |
| Mixed-mode coexistence          | One service with `control=false` and one with `control=true` in the same cluster; verify the `control=false` service is unaffected. |

## Open Questions

- **Server-streaming `WaitForApproval`** would let a single test
  observe every gate for a replica without polling. Start unary;
  revisit if tests get awkward.
- **`abort` observations after the replica is gone.** Currently
  not reportable. Option: per-partition ring buffer of completed
  `ApprovalEvent`s in the registry.
- **`ReplicaInitData` extension fields.** Likely next:
  per-service tag for filtering, and `repeated PreloadedDecision`
  so a test can stage decisions before SF starts driving the
  replica (avoids a `WaitForApproval` round-trip on the hot path).
- **Failover sub-test for the SF-persists-initdata assumption** —
  recommended future addition to the e2e suite.
