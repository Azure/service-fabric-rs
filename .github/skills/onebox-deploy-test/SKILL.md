---
name: onebox-deploy-test
description: 'Build the Rust SF samples, deploy them to the local Linux onebox cluster, and run the integration test suite. Use when the user asks to "run the integration tests", "deploy to onebox and test", "redeploy and re-run", "verify on a real cluster", "bring up onebox", or after non-trivial changes to crates/samples/* or crates/libs/* that need end-to-end validation. Covers cmake build, sfctl provisioning, cluster health waits, and `cargo test --all -- --nocapture`.'
---

# Onebox Deploy & Test (Linux)

End-to-end loop for validating changes against a real Service Fabric onebox
cluster running in the devcontainer. Mirrors what CI does in
[`.github/workflows/build.yaml`](../../workflows/build.yaml) under the
`build-devcontainer` job (`u22` and `azl3` matrix entries).

## When to Use

- The user wants to validate a change end-to-end on a real cluster, not just `cargo test --lib`.
- Trigger phrases: "deploy and test", "run the integration tests", "redeploy onebox", "bring up the cluster and run", "full e2e", "what CI runs".
- Changes touched any of: `crates/samples/echomain/`, `crates/samples/echomain-stateful/`, `crates/samples/reflection/`, `crates/samples/kvstore/`, `crates/libs/com/`, `crates/libs/core/`, `crates/libs/util/`, `crates/libs/pal/`, the `proto/` directories, `manifests/`, or [build.rs](../../../crates/samples/reflection/build.rs).
- A previous test run failed and you want to redeploy clean.

## When NOT to Use

- Pure unit-test runs (`cargo test --lib` is enough).
- Doc-only changes.
- Refactors that don't touch SF-facing code paths.
- The user is on Windows — this skill assumes the Linux devcontainer setup where the `onebox` and `repo-u22` containers run side by side. For Windows, defer to `.\onebox\windows\StartOnebox.ps1` and `.\scripts\reflection_ctl.ps1`.

## Preconditions

Run a quick check before doing anything else; if any fails, stop and report:

1. **Inside the devcontainer.** Confirm with `cat /etc/os-release | head -2` (expect `Ubuntu` or `Azure Linux`) and `command -v sfctl` (must succeed). The host machine doesn't have SF or `sfctl` installed.
2. **Onebox sibling container reachable.** `sfctl cluster select` should connect to the default `http://onebox:19080` (the sibling container's hostname). If it errors with `connection refused`, the onebox container hasn't finished starting — wait 10 s and retry up to 3 times.
3. **Working tree compiles.** `cargo check -p samples_reflection -p samples_echomain -p samples_echomain_stateful` — fail fast if there's a build error before invoking the slow cmake path.

## Procedure

Run the steps below sequentially. Stop on the first failure unless the step is marked `(best-effort)`. All commands are run from the repo root (`/home/vscode/repo`).

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

## What CI Does Differently

The CI job ([build-devcontainer](../../workflows/build.yaml)) runs the whole skill in one shot via `devcontainers/ci@v0.3` and adds:

- Disk-space prep (`/usr/local/lib/android` etc. removal).
- Core-dump capture under `/tmp/artifacts/coredumps/`.
- Artifact upload of crashed binaries.

For local iteration these are unnecessary. If the user is debugging a crash that only repros in CI, point them at the artifact upload step rather than re-running this skill locally.

## Common Pitfalls

- **`cmake --build` skips a sample after editing only `Cargo.toml`.** Cargo notices the change but cmake's package step caches based on file timestamps. Force re-package with `cmake --build build --config Debug --target build_rust_sample_reflection` (or `force_clean` then full rebuild).
- **`sfctl: command not found`.** The host doesn't have it; you're outside the devcontainer. Start it via VS Code's "Reopen in Container" or rerun in the right shell.
- **`http://onebox:19080` unreachable from the repo container.** The onebox container may have crashed. The devcontainer can't reach the host's docker daemon, so ask the user to check container status from a host shell (`docker ps`) and restart it there if needed — there is no auto-restart.
- **Tests pass locally but fail in CI.** CI runs both `u22` and `azl3` devcontainers; check whether the failure is platform-specific by switching `.devcontainer/<u22|azl3>/devcontainer.json` and reopening the container.
