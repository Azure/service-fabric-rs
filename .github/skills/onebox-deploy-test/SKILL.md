---
name: onebox-deploy-test
description: 'Build the Rust SF samples, deploy them to a local onebox cluster (Linux devcontainer or Windows host), and run the integration test suite. Use when the user asks to "run the integration tests", "deploy to onebox and test", "redeploy and re-run", "verify on a real cluster", "bring up onebox", or after non-trivial changes to crates/samples/* or crates/libs/* that need end-to-end validation. Covers cmake build, sfctl/PowerShell provisioning, cluster health waits, and `cargo test --all -- --nocapture`.'
---

# Onebox Deploy & Test

End-to-end loop for validating changes against a real Service Fabric onebox
cluster. Mirrors what CI does in
[`.github/workflows/build.yaml`](../../workflows/build.yaml):

- **Linux devcontainer** path → `build-devcontainer` job (`u22` and `azl3` matrix entries).
- **Windows host** path → `build` job (`runs-on: windows-latest`).

Pick the section that matches the user's environment. Detect with:

- Linux/devcontainer if `cat /etc/os-release` shows Ubuntu/Azure Linux and `command -v sfctl` succeeds → use [Procedure (Linux)](#procedure-linux).
- Windows if `$env:OS -eq 'Windows_NT'` and `Test-Path 'C:\Program Files\Microsoft Service Fabric'` → use [Procedure (Windows)](#procedure-windows).

## When to Use

- The user wants to validate a change end-to-end on a real cluster, not just `cargo test --lib`.
- Trigger phrases: "deploy and test", "run the integration tests", "redeploy onebox", "bring up the cluster and run", "full e2e", "what CI runs".
- Changes touched any of: `crates/samples/echomain/`, `crates/samples/echomain-stateful/`, `crates/samples/reflection/`, `crates/samples/kvstore/`, `crates/libs/com/`, `crates/libs/core/`, `crates/libs/util/`, `crates/libs/pal/`, the `proto/` directories, `manifests/`, or [build.rs](../../../crates/samples/reflection/build.rs).
- A previous test run failed and you want to redeploy clean.

## When NOT to Use

- Pure unit-test runs (`cargo test --lib` is enough).
- Doc-only changes.
- Refactors that don't touch SF-facing code paths.

## Procedure (Linux)

Devcontainer setup where the `onebox` and `repo-u22`/`repo-azl3` containers run
side by side. All commands are run from the repo root (`/home/vscode/repo`).
Stop on the first failure unless the step is marked `(best-effort)`.

### Preconditions (Linux)

Run a quick check before doing anything else; if any fails, stop and report:

1. **Inside the devcontainer.** Confirm with `cat /etc/os-release | head -2` (expect `Ubuntu` or `Azure Linux`) and `command -v sfctl` (must succeed). The host machine doesn't have SF or `sfctl` installed.
2. **Onebox sibling container reachable.** `sfctl cluster select` should connect to the default `http://onebox:19080` (the sibling container's hostname). If it errors with `connection refused`, the onebox container hasn't finished starting — wait 10 s and retry up to 3 times.
3. **Working tree compiles.** `cargo check -p samples_reflection -p samples_echomain -p samples_echomain_stateful` — fail fast if there's a build error before invoking the slow cmake path.

### Step 1 — Build everything via cmake

`cmake` drives `cargo build` *and* packages every sample into `build/sf_apps/<app-name>/` (this is what `sfctl application upload` consumes).

```bash
# First-time only: configure the build directory.
[ -f build/CMakeCache.txt ] || cmake . -DCMAKE_BUILD_TYPE=Debug -B build

# Always: rebuild rust binaries + repackage SF apps.
cmake --build build --config Debug
```

Expected outputs after success:
- `build/sf_apps/samples_reflection/` (with `ApplicationManifest.xml`, `ServicePackage/`, etc.)
- `build/sf_apps/samples_echomain/`
- `build/sf_apps/samples_echomain_stateful/`
- `build/sf_apps/kvstore/` *(Windows only — gated by `if(WIN32)` in the root CMakeLists)*

If any are missing, inspect the cmake log around the failing target — the per-sample `CMakeLists.txt` files (e.g. [crates/samples/reflection/CMakeLists.txt](../../../crates/samples/reflection/CMakeLists.txt)) define what gets packaged.

### Step 2 — Wait for the cluster to be healthy and provision apps

The repo's [scripts/prepare_test_apps.sh](../../../scripts/prepare_test_apps.sh) does the entire wait-and-deploy loop with retries. Use it directly rather than re-implementing the logic:

```bash
bash ./scripts/prepare_test_apps.sh
```

This script:
1. Waits up to ~150 s for `sfctl cluster select` to succeed.
2. Waits for the cluster's aggregated health state to be `Ok` or `Warning`.
3. Sleeps 5 s for the ImageStore to stabilise.
4. Uploads + provisions + creates `EchoApp` (`fabric:/EchoApp`) and `ReflectionApp` (`fabric:/ReflectionApp`) with retries.
5. Waits for both `EchoAppService` and `ReflectionAppService` to become resolvable.

If the script fails:
- `application upload` failures usually mean the onebox container is still warming up or the ImageStore isn't ready — retry the script once.
- `application create` failures with `FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS` mean a previous run left state behind. Run [Step 4 — Cleanup](#step-4--cleanup-best-effort) first, then retry.
- `service resolve` failures usually mean a placement constraint can't be satisfied — check the SF Explorer at `http://localhost:19080` (port-forwarded by docker-compose).

### Step 3 — Run tests

The integration tests use `FabricClient::with_connection_strings(["localhost:19000"])` to reach the cluster. Inside the devcontainer this resolves to the `onebox` sibling container via the container runtime's DNS; the existing tests rely on this and don't need any extra wiring.

```bash
cargo test --all -- --nocapture
```

Two recipes for narrower runs:
- Unit tests only (no cluster needed): `cargo test --all --lib -- --nocapture`
- One sample's integration tests: `cargo test -p samples_reflection -- --nocapture`
- A specific test name: `cargo test -p samples_reflection test_partition_info -- --nocapture --exact`

If a test fails with `FABRIC_E_APPLICATION_NOT_FOUND` you skipped Step 2; if it fails with `FabricError { code: -2147017735 }` (timeout) the cluster is unhealthy — check `sfctl cluster health` and re-run the prepare step.

### Step 4 — Cleanup (best-effort)

Run after a successful test pass to leave the cluster clean for the next iteration. Errors here are non-fatal.

```bash
# Per-app deletion. Skip silently if the app isn't registered.
sfctl application delete --application-id ReflectionApp 2>/dev/null || true
sfctl application unprovision --application-type-name ReflectionApp --application-type-version 0.0.1 2>/dev/null || true
sfctl application delete --application-id EchoApp 2>/dev/null || true
sfctl application unprovision --application-type-name EchoApp --application-type-version 0.0.1 2>/dev/null || true
```

If state is irretrievably stuck, the onebox container needs to be
restarted from the host — the devcontainer cannot reach the host's
docker daemon. Ask the user to restart it from outside the
devcontainer (e.g., "reopen folder in container" from VS Code, which
rebuilds the containers; or `docker restart <onebox-container-name>`
from a host shell). Then re-run from Step 2.

## Procedure (Windows)

Windows host setup where Service Fabric runtime + SDK are installed
locally and apps are provisioned via the `ServiceFabric` PowerShell
module (not `sfctl`). All commands are run from the repo root in a
**PowerShell** shell. Stop on the first failure unless the step is
marked `(best-effort)`.

The CI equivalent is the `build` job in
[`.github/workflows/build.yaml`](../../workflows/build.yaml)
(`runs-on: windows-latest`).

### Preconditions (Windows)

1. **Working tree compiles.** `cargo check --all-targets` — fail
   fast before the slow cmake path. CI also runs `cargo fmt --all -- --check`
   and `cargo clippy -- -D warnings`; mirror them locally if you
   want a CI-clean state.
2. **`protoc` is on `PATH`.** Required by the reflection sample's
   build script. CI installs it with `taiki-e/install-action@protoc`;
   locally use `winget install protobuf` or any equivalent.
3. **`cmake` is on `PATH`** (`cmake --version`). If a Visual
   Studio install ships cmake (e.g. VS 18 ships
   `C:\Program Files\Microsoft Visual Studio\18\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe`),
   prepend that directory to `$env:PATH` once per shell instead of
   installing a second copy.

> Skip the CI-only `Remove conflict dll paths` step (deletes
> `C:\Program Files\MySQL\...`) and the `check_sf_installed.ps1` /
> `check_cluster_online.ps1` smoke checks — they exist to harden
> the GitHub-hosted runner image and are not needed locally.

> **Per-shell setup, not per-command.** `Import-Module
> ServiceFabric` and `Connect-ServiceFabricCluster` only need to
> run **once** per PowerShell session — the module stays loaded
> and the cluster connection is cached on the runspace. The
> `*_ctl.ps1` scripts each call `Connect-ServiceFabricCluster`
> internally, so for ad-hoc inspection (`Get-ServiceFabricApplication`,
> etc.) just reuse the existing connection. Do **not** repeat
> `Import-Module` or `Connect-ServiceFabricCluster` between
> commands in the same shell.

### Step 1 — Build everything via cmake

Same as Linux — `cmake` drives `cargo build` *and* packages every
sample into `build\sf_apps\<app-name>\`.

```powershell
# First-time only: configure the build directory.
if (-not (Test-Path build\CMakeCache.txt)) {
    cmake . -DCMAKE_BUILD_TYPE=Debug -B build
}

# Always: rebuild rust binaries + repackage SF apps.
cmake --build build --config Debug
```

Expected outputs after success:
- `build\sf_apps\samples_reflection\`
- `build\sf_apps\samples_echomain\`
- `build\sf_apps\samples_echomain_stateful\`
- `build\sf_apps\kvstore\` *(Windows only — gated by `if(WIN32)` in the root CMakeLists)*

### Step 2 — Bring up the local 5-node cluster

Use [onebox/windows/StartOnebox.ps1](../../../onebox/windows/StartOnebox.ps1).
The script auto-elevates to Administrator via UAC. `-Auto` skips the
"existing cluster will be removed" prompt.

```powershell
Powershell.exe -File .\onebox\windows\StartOnebox.ps1 -Auto
```

After success: SF Explorer at `http://localhost:19080/Explorer`.
The script already waits for the cluster to be reachable and the
naming service to be ready, so a separate readiness wait is not
required for local runs.

### Step 3 — Provision apps

The Linux `prepare_test_apps.sh` does not apply on Windows. Use the
per-sample PowerShell controllers, which mirror what CI runs:

```powershell
# Lightweight smoke test for echomain (adds, resolves, echoes, removes).
.\tests\echo_script_test.ps1

# Long-lived deployments needed by `cargo test --all`.
.\scripts\reflection_ctl.ps1       -Action Add
.\scripts\echomain_ctl.ps1         -Action Add
# Optional — only needed if your tests touch these apps:
.\scripts\echomain_stateful_ctl.ps1 -Action Add
.\scripts\kvstore_ctl.ps1           -Action Add
```

Each `*_ctl.ps1 -Action Add` does:
`Connect-ServiceFabricCluster` → `Test-ServiceFabricApplicationPackage`
→ `Copy-ServiceFabricApplicationPackage` → `Register-ServiceFabricApplicationType`
→ `New-ServiceFabricApplication`. Image-store path is `MyApplicationV1`
(or `MyKvStoreApplicationV1` for kvstore), so back-to-back `Add`s of
two different apps will collide — `Remove` the previous one first or
they will overwrite each other's image-store entry.

If `New-ServiceFabricApplication` fails with
`FABRIC_E_APPLICATION_TYPE_ALREADY_EXISTS`, run the matching
`-Action Remove` then retry.

### Step 4 — Run tests

```powershell
cargo test --all -- --nocapture
```

Same narrowing recipes as Linux:
- Unit tests only: `cargo test --all --lib -- --nocapture`
- One sample: `cargo test -p samples_reflection -- --nocapture`
- Specific test: `cargo test -p samples_reflection test_partition_info -- --nocapture --exact`

Tests connect to `localhost:19000` via `FabricClient`.

### Step 5 — Cleanup (best-effort)

Each controller's `-Action Remove` reverses its `Add`:

```powershell
.\scripts\reflection_ctl.ps1       -Action Remove
.\scripts\echomain_ctl.ps1         -Action Remove
.\scripts\echomain_stateful_ctl.ps1 -Action Remove   # if added
.\scripts\kvstore_ctl.ps1           -Action Remove   # if added
```

To stop the cluster while preserving data, or to fully tear it down:

```powershell
Powershell.exe -File .\onebox\windows\StopOnebox.ps1            # stops FabricHostSvc only
Powershell.exe -File .\onebox\windows\StopOnebox.ps1 -CleanData -Auto   # full removal
```

A stopped cluster can be restarted with `Start-Service FabricHostSvc`
or by re-running `StartOnebox.ps1`.

## What CI Does Differently

### `build-devcontainer` job (Linux)

Runs Steps 1–4 in one shot via `devcontainers/ci@v0.3` and adds:

- Disk-space prep (`/usr/local/lib/android` etc. removal).
- Core-dump capture under `/tmp/artifacts/coredumps/`.
- Artifact upload of crashed binaries.

### `build` job (Windows)

Runs the matrix `(rust-min, rust-max)` × `windows-latest` and adds:

- `cargo fmt --all -- --check` and `cargo clippy -- -D warnings` gates before the build.
- `Remove conflict dll paths` — deletes `C:\Program Files\MySQL\MySQL Server 8.0\bin` because its `libprotobuf.dll` is incompatible with Service Fabric and prevents the runtime from starting on the GitHub-hosted runner image. Not needed on a clean local box.
- `check_sf_installed.ps1` and `check_cluster_online.ps1` smoke checks — also CI-image hardening.

For local iteration both extra-step blocks are unnecessary. If the
user is debugging a crash that only repros in CI, point them at the
artifact upload step rather than re-running this skill locally.

## Common Pitfalls

- **`cmake --build` skips a sample after editing only `Cargo.toml`.** Cargo notices the change but cmake's package step caches based on file timestamps. Force re-package with `cmake --build build --config Debug --target build_rust_sample_reflection` (or `force_clean` then full rebuild).
- **(Linux) `sfctl: command not found`.** The host doesn't have it; you're outside the devcontainer. Start it via VS Code's "Reopen in Container" or rerun in the right shell.
- **(Linux) `http://onebox:19080` unreachable from the repo container.** The onebox container may have crashed. The devcontainer can't reach the host's docker daemon, so ask the user to check container status from a host shell (`docker ps`) and restart it there if needed — there is no auto-restart.
- **(Windows) `Connect-ServiceFabricCluster` fails immediately after `StartOnebox.ps1`.** Re-run the connect; the SDK script already waits, but a freshly-restarted cluster occasionally needs a few extra seconds for the naming service.
- **(Windows) `New-ServiceFabricApplication` errors with `image store path already exists`.** Two `*_ctl.ps1` scripts share the `MyApplicationV1` image-store path. Run `-Action Remove` for the previous app before adding the next, or accept that only one of them will be live.
- **(Windows) `*_ctl.ps1` must be run from the repo root.** The scripts use the relative path `build\sf_apps\<sample>` — running from elsewhere fails `Test-ServiceFabricApplicationPackage`.
- **Tests pass locally but fail in CI.** CI runs `u22`, `azl3`, *and* `windows-latest`; check whether the failure is platform-specific by reproducing in the matching environment.
