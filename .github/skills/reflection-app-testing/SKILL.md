---
name: reflection-app-testing
description: 'Run, gate, inspect, and clean up reflection-sample integration tests against the local onebox (Linux devcontainer or Windows host). Use when the user asks to "run the reflection e2e test", "approve a stuck gate", "list pending replica gates", "detach a controller", "clean up test apps", "redeploy the reflection sample after a code change", or after touching crates/samples/reflection/src/control.rs / grpc_control.rs / proto/control.proto / proto/initdata.proto. Wraps the reflection_ctl operator CLI, the e2e tests in tests/control_e2e.rs and tests/fail_change_role.rs, prepare_test_apps.sh / remove_test_apps.sh (Linux), and the *_ctl.ps1 scripts (Windows).'
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
  - `crates/samples/reflection/tests/fail_change_role.rs`
  - `crates/samples/reflection/src/test_cluster.rs`

## When NOT to Use

- Pure unit tests for `control.rs` / `grpc_control.rs` — `cargo test
  -p samples_reflection --lib -- control:: grpc_control::` is enough
  and doesn't need a cluster.
- Cluster bring-up or non-reflection samples — use
  [`onebox-deploy-test`](../onebox-deploy-test/SKILL.md).

## Cluster host defaulting

The test harness ([src/test_cluster.rs](../../../crates/samples/reflection/src/test_cluster.rs))
and the `reflection_ctl` CLI use a `cfg`-gated default for the
cluster hostname:

- **Windows** → `localhost` (SF onebox runs on the same host; the
  reflection server binds `0.0.0.0` so `localhost` reaches every node).
- **Unix** → `onebox` (sibling-container DNS in the devcontainer
  setup).

Override at runtime with `REFLECTION_CLUSTER_HOST=<host>` (env var)
or `--host <host>` for `reflection_ctl`. Do **not** set the env var
on a normal Windows or Linux-devcontainer run — the cfg default
already matches.

## Preconditions

### Linux devcontainer

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

### Windows host

In PowerShell (`$env:OS -eq 'Windows_NT'`):

```powershell
Get-Service FabricHostSvc           # must be Running (StartOnebox.ps1 starts it)
Test-Path .\target\debug\reflection_ctl.exe   # operator CLI; built by cmake or `cargo build -p samples_reflection --bin reflection_ctl`
```

For cluster bring-up + initial provisioning of `EchoApp` and
`ReflectionApp`, see the [`onebox-deploy-test`](../onebox-deploy-test/SKILL.md)
skill's **Procedure (Windows)** section. This skill assumes the
cluster is already up and `fabric:/ReflectionApp` is deployed.

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

**Linux**:

```bash
[ -f build/CMakeCache.txt ] || cmake . -DCMAKE_BUILD_TYPE=Debug -B build
cmake --build build --config Debug
```

**Windows** (PowerShell, repo root):

```powershell
if (-not (Test-Path build\CMakeCache.txt)) {
    cmake . -DCMAKE_BUILD_TYPE=Debug -B build
}
cmake --build build --config Debug
```

After success: `build/sf_apps/samples_reflection/` (Linux) or
`build\sf_apps\samples_reflection\` (Windows) should exist with
fresh `ApplicationManifest.xml` and `ServicePackage/`.

If you only changed the operator CLI, build just that:

```bash
cargo build -p samples_reflection --bin reflection_ctl
```

### Step 2 — Clean cluster state

Required when the *deployed* `samples_reflection.exe` is stale and
SF won't reactivate it without a clean reprovision, or when a prior
test crash left `ApprovalE2e_*` / `FailCrE2e_*` services parked.

**Linux** — [scripts/remove_test_apps.sh](../../../scripts/remove_test_apps.sh)
is idempotent and tolerates "doesn't exist" at every step:

```bash
bash ./scripts/remove_test_apps.sh
```

**Windows** — no equivalent script. Do the same sequence by hand
(only run the parts that apply):

```powershell
# 1. Release any parked gates so SF can finish closing replicas.
.\target\debug\reflection_ctl.exe detach --all

# 2. Force-remove any leftover per-test sub-services.
Get-ServiceFabricService -ApplicationName fabric:/ReflectionApp |
    Where-Object { $_.ServiceName.AbsolutePath -match '/(ApprovalE2e_|FailCrE2e_)' } |
    ForEach-Object { Remove-ServiceFabricService -ServiceName $_.ServiceName -Force }

# 3. Full app reset (mirrors what reflection_ctl.ps1 -Action Remove does).
.\scripts\reflection_ctl.ps1 -Action Remove
```

Then re-run `*_ctl.ps1 -Action Add` (Step 3 below).

### Step 3 — Provision

**Linux** — [scripts/prepare_test_apps.sh](../../../scripts/prepare_test_apps.sh):

```bash
bash ./scripts/prepare_test_apps.sh
```

Waits for cluster health, uploads + provisions + creates `EchoApp`
and `ReflectionApp`, then waits for both services to resolve.
Output ends with the resolved `ReflectionAppService` endpoint at
`http://172.18.0.2:28000+i/...` (ports 28000–28004). If you see
that line, the new binary is live.

**Windows** — use the per-app PowerShell controllers (no combined
script):

```powershell
.\scripts\reflection_ctl.ps1 -Action Add
# Optional, only if your tests touch echomain too:
.\scripts\echomain_ctl.ps1 -Action Add
```

Verify with `Get-ServiceFabricApplication` — status should be
`Ready` / `Ok`. Replicas advertise `http://<machine>:28000+i/...`,
but the test harness uses `localhost` on Windows and the server
binds `0.0.0.0`, so this works without configuration.

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

Expected runtime: ~2–4 seconds. If it hangs at "waiting for OPEN
gate", the deployed binary is stale (skipped Step 1 + 3) — see
Common Pitfalls.

#### 4b¹. The fail-change-role e2e test ([tests/fail_change_role.rs](../../../crates/samples/reflection/tests/fail_change_role.rs))

```bash
cargo test -p samples_reflection --test fail_change_role -- --nocapture
```

Verifies the failure-recovery path: OPEN → CHANGE_ROLE(fail) →
ABORT(approve to recover) → OPEN (retry) → CHANGE_ROLE(approve) →
teardown. Expected runtime: ~20–25 s (SF reactivation between
attempts dominates).

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
Binary path: `./target/debug/reflection_ctl` (Linux) or
`.\target\debug\reflection_ctl.exe` (Windows). The `--host` default
is cfg-gated (see [Cluster host defaulting](#cluster-host-defaulting));
add `--host <hostname>` to override.

#### 5a. Inspect

```bash
reflection_ctl ping     # which nodes are reachable
reflection_ctl list     # what gates are pending
reflection_ctl list --partition <GUID>  # filter
```

`ping` shows ports `28000..=28004`. Some "unreachable: transport
error" lines are normal — the reflection sample only runs on nodes
where SF placed it.

#### 5b. Approve a stuck gate

```bash
reflection_ctl approve \
  --partition 92A2B906-B26C-774F-8FCA-E04CCD254142 \
  --replica   134218820574294250
# Or to fail it instead:
reflection_ctl approve --partition X --replica Y --fail-message "boom"
```

Auto-discovers the `gate_id` via `ListPending`. If no gate is
pending for that replica → exit code 1.

#### 5c. Bulk unblock

```bash
reflection_ctl approve-all --yes   # release every pending gate
reflection_ctl detach   --all      # proceed-forever for every controller
```

`detach --all` is the bigger hammer — once detached, a controller
auto-proceeds *every future* gate too, not just the current one.
Use it when you want SF to resume normal teardown without further
test interaction.

`approve-all` only releases what's currently parked; new gates
arriving after will park again.

#### 5d. Detach a single replica

```bash
reflection_ctl detach \
  --partition 92A2B906-... \
  --replica   134218820574294250
```

## Common Pitfalls

- **Deployed binary is stale.** If e2e hangs at OPEN or `Detach`
  returns `Unimplemented`, you skipped Step 1 + Step 3 after
  changing reflection-sample code. Run them in order.
- **(Linux) `--force-remove` syntax.** sfctl 11.x requires
  `--force-remove true`, not bare `--force-remove`.
  `remove_test_apps.sh` already handles this; ad-hoc `sfctl service
  delete` calls don't.
- **Test panics leave gates parked.** Run
  `reflection_ctl list` to see them, then `approve-all --yes` or
  `detach --all` to clear before the next run. On Linux,
  `remove_test_apps.sh` does this automatically; on Windows, follow
  the manual sequence in Step 2.
- **Stuck `Active`/Error sub-service after a panic (Windows).**
  `Remove-ServiceFabricService -ServiceName ... -Force` is the
  PowerShell equivalent of the Linux script's `--force-remove true`.
- **`cmake --build` skips a sample after editing only `Cargo.toml`.**
  Force re-package with `cmake --build build --config Debug --target
  build_rust_sample_reflection` or `force_clean` then full rebuild.
- **(Linux) `onebox` hostname doesn't resolve from a fresh shell.**
  Inside the devcontainer, `getent hosts onebox` should return
  `172.18.0.2`. If not, the sibling container isn't running.
- **Don't set `REFLECTION_CLUSTER_HOST` casually.** The cfg-gated
  default already picks `localhost` (Windows) or `onebox` (Unix).
  Only override for non-standard topologies.

## What CI Does

- **Linux** — [`.github/workflows/build.yaml`](../../workflows/build.yaml)'s
  `build-devcontainer` job runs steps 1 + 3 + 4c via
  `devcontainers/ci@v0.3`.
- **Windows** — the `build` job (`runs-on: windows-latest`) runs
  the cmake build, `StartOnebox.ps1 -Auto`, the per-sample
  `*_ctl.ps1 -Action Add` scripts, and `cargo test --all -- --nocapture`.

Since both e2e tests are default-on, they are included automatically
on both platforms — no extra CI changes needed.
