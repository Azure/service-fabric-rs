---
name: reflection-app-testing
description: 'Run, gate, inspect, and clean up reflection-sample integration tests against the local Linux onebox. Use when the user asks to "run the reflection e2e test", "approve a stuck gate", "list pending replica gates", "detach a controller", "clean up test apps", "redeploy the reflection sample after a code change", or after touching crates/samples/reflection/src/control.rs / grpc_control.rs / proto/control.proto / proto/initdata.proto. Wraps the reflection_ctl operator CLI, the e2e test in tests/control_e2e.rs, prepare_test_apps.sh, and remove_test_apps.sh.'
---

# Reflection App Testing

End-to-end loop for the reflection sample's test-controlled replica
behavior — see [docs/design/ReflectionReplicaTestControl.md](../../../docs/design/ReflectionReplicaTestControl.md)
for the design.

For *generic* deploy-and-test of all SF samples (not specific to the
reflection control plane), use the
[`onebox-deploy-test`](../onebox-deploy-test/SKILL.md) skill instead.

## When to Use

Trigger phrases:
- "run the reflection e2e test"
- "approve / list / detach gates"
- "clean up reflection test state"
- "the test is stuck waiting on `Approve`"
- "redeploy the reflection sample"
- after editing any of:
  - `crates/samples/reflection/src/control.rs`
  - `crates/samples/reflection/src/grpc_control.rs`
  - `crates/samples/reflection/src/statefulstore.rs`
  - `crates/samples/reflection/proto/control.proto`
  - `crates/samples/reflection/proto/initdata.proto`
  - `crates/samples/reflection/reflection_ctl/main.rs`
  - `crates/samples/reflection/tests/control_e2e.rs`

## When NOT to Use

- Pure unit tests for `control.rs` / `grpc_control.rs` — `cargo test
  -p samples_reflection --lib -- control:: grpc_control::` is enough
  and doesn't need a cluster.
- Cluster bring-up or non-reflection samples — use
  [`onebox-deploy-test`](../onebox-deploy-test/SKILL.md).
- Windows onebox — this skill assumes the Linux devcontainer setup.

## Preconditions

Inside the devcontainer (`/etc/os-release` shows Ubuntu/AzureLinux):

```bash
command -v sfctl                 # must succeed
sfctl cluster select             # must connect to http://onebox:19080
cargo build -p samples_reflection --bin reflection_ctl  # operator CLI
```

If `sfctl cluster select` fails with `connection refused`, the
`onebox` sibling container hasn't started — wait 10 s and retry up
to 3 times. Cannot restart it from inside the devcontainer; ask the
user to do so from a host shell.

## Procedure

The *fast path* depends on what changed and what's currently running.
Pick the smallest applicable subset; you don't always need to redeploy.

### Quick decision tree

| Situation | Action |
|---|---|
| Cluster is fresh, nothing deployed | Step 1 → Step 3 → Step 4 |
| Already deployed; only edited test code or `reflection_ctl` | Step 4 only |
| Edited `src/control.rs` / `grpc_control.rs` / `statefulstore.rs` / proto / `main.rs` (production reflection-sample code) | Step 1 → Step 2 → Step 3 → Step 4 |
| Test failed, gates are parked, cluster is dirty | Step 5 (cleanup) → Step 1 if redeploying |
| Just need to inspect what's pending | Step 5a (`list`) only |

### Step 1 — Build (`cmake --build`)

`cmake` drives `cargo build` AND repackages the SF apps into
`build/sf_apps/` — *both* are required for SF to pick up new
binaries. `cargo build` alone is not enough.

```bash
[ -f build/CMakeCache.txt ] || cmake . -DCMAKE_BUILD_TYPE=Debug -B build
cmake --build build --config Debug
```

After success: `build/sf_apps/samples_reflection/` should exist with
fresh `ApplicationManifest.xml` and `ServicePackage/`.

If you only changed the operator CLI, build just that:

```bash
cargo build -p samples_reflection --bin reflection_ctl
```

### Step 2 — Clean cluster state ([scripts/remove_test_apps.sh](../../../scripts/remove_test_apps.sh))

Required when the *deployed* `samples_reflection.exe` is stale and
SF won't reactivate it without a clean reprovision. Idempotent:

```bash
bash ./scripts/remove_test_apps.sh
```

Sequence the script runs:
1. `reflection_ctl detach --all` (best-effort; release any parked
   gates so close can complete).
2. `reflection_ctl approve-all --yes` (fallback for pre-Detach
   binaries).
3. Delete every sub-service under each app (catches per-test
   `ApprovalE2e_*` services from prior runs).
4. Delete app instances (`fabric:/EchoApp`, `fabric:/ReflectionApp`).
5. Unprovision app types.
6. Delete uploaded packages from the image store.

Tolerates "doesn't exist" at every step. Always safe to run.

### Step 3 — Provision ([scripts/prepare_test_apps.sh](../../../scripts/prepare_test_apps.sh))

```bash
bash ./scripts/prepare_test_apps.sh
```

Waits for cluster health, uploads + provisions + creates `EchoApp`
and `ReflectionApp`, then waits for both services to resolve.
Output ends with the resolved `ReflectionAppService` endpoint at
`http://172.18.0.2:28000+i/...` (ports 28000–28004). If you see
that line, the new binary is live.

### Step 4 — Run tests

#### 4a. Unit tests (no cluster needed)

```bash
cargo test -p samples_reflection --lib
```

Covers everything in `control.rs` (gate_lock serialization,
stale-approve UUID protection, drop semantics, detach behaviour)
and `grpc_control.rs` (timeout clamping, node_index parser).

#### 4b. The e2e approval test ([tests/control_e2e.rs](../../../crates/samples/reflection/tests/control_e2e.rs))

```bash
cargo test -p samples_reflection --test control_e2e -- --nocapture
```

Runs by default like the other reflection tests — it requires the
cluster to be up and `fabric:/ReflectionApp` provisioned. The test:
1. Dials all `28000..=28004` via `Cluster::ensure()` (lazy re-dial).
2. Creates a fresh `ApprovalE2e_<uuid>` service with
   `ReplicaInitData { control: true }` initdata.
3. Walks `Open → ChangeRole(Primary)` gates via
   `WaitForApproval` + `Approve(proceed)`.
4. Spawns `delete_service` in the background.
5. Drains teardown gates (`ChangeRole(None)` + `Close`).
6. Verifies the registry no longer contains the replica.

Expected runtime: ~2 seconds. If it hangs at "waiting for OPEN
gate", the deployed binary is stale (skipped Step 1 + 3) — see
Common Pitfalls.

#### 4c. Run all reflection sample tests

```bash
cargo test -p samples_reflection
```

Runs the unit tests, the existing `test.rs` / `test2.rs`
integration tests, *and* the e2e approval test in one shot — they
all require the cluster to be up.

### Step 5 — Operate the cluster ([reflection_ctl/main.rs](../../../crates/samples/reflection/reflection_ctl/main.rs))

The operator CLI is built by Step 1 (`cmake --build`) or
explicitly via `cargo build -p samples_reflection --bin reflection_ctl`.

#### 5a. Inspect

```bash
./target/debug/reflection_ctl ping     # which nodes are reachable
./target/debug/reflection_ctl list     # what gates are pending
./target/debug/reflection_ctl list --partition <GUID>  # filter
```

`ping` shows ports `28000..=28004`. Some "unreachable: transport
error" lines are normal — the reflection sample only runs on nodes
where SF placed it.

#### 5b. Approve a stuck gate

```bash
./target/debug/reflection_ctl approve \
  --partition 92A2B906-B26C-774F-8FCA-E04CCD254142 \
  --replica   134218820574294250
# Or to fail it instead:
./target/debug/reflection_ctl approve --partition X --replica Y --fail-message "boom"
```

Auto-discovers the `gate_id` via `ListPending`. If no gate is
pending for that replica → exit code 1.

#### 5c. Bulk unblock

```bash
./target/debug/reflection_ctl approve-all --yes   # release every pending gate
./target/debug/reflection_ctl detach   --all      # proceed-forever for every controller
```

`detach --all` is the bigger hammer — once detached, a controller
auto-proceeds *every future* gate too, not just the current one.
Use it when you want SF to resume normal teardown without further
test interaction.

`approve-all` only releases what's currently parked; new gates
arriving after will park again.

#### 5d. Detach a single replica

```bash
./target/debug/reflection_ctl detach \
  --partition 92A2B906-... \
  --replica   134218820574294250
```

## Common Pitfalls

- **Deployed binary is stale.** If e2e hangs at OPEN or `Detach`
  returns `Unimplemented`, you skipped Step 1 + Step 3 after
  changing reflection-sample code. Run them in order.
- **`--force-remove` syntax.** sfctl 11.x requires
  `--force-remove true`, not bare `--force-remove`.
  `remove_test_apps.sh` already handles this; ad-hoc `sfctl service
  delete` calls don't.
- **Test panics leave gates parked.** Run
  `./target/debug/reflection_ctl list` to see them, then
  `approve-all --yes` or `detach --all` to clear before the next
  run. `remove_test_apps.sh` does this automatically.
- **`cmake --build` skips a sample after editing only `Cargo.toml`.**
  Force re-package with `cmake --build build --config Debug --target
  build_rust_sample_reflection` or `force_clean` then full rebuild.
- **`onebox` hostname doesn't resolve from a fresh shell.** Inside
  the devcontainer, `getent hosts onebox` should return `172.18.0.2`.
  If not, the sibling container isn't running.

## What CI Does

[`.github/workflows/build.yaml`](../../workflows/build.yaml)'s
`build-devcontainer` job runs steps 1 + 3 + 4c via
`devcontainers/ci@v0.3`. Since the e2e test is now default-on, it
is included automatically — no additional CI changes needed.
