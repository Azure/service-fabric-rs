# Reflection Sample

A stateful Service Fabric application built with Rust using the `mssf-core` and `mssf-util` crates.

## Overview

The Reflection app (`ReflectionApp`) demonstrates how to build a persistent stateful reliable service
using high-level Rust traits (`IStatefulServiceFactory`, `IStatefulServiceReplica`) instead of
calling the raw `IFabric*` COM interfaces directly.

The service registers as `ReflectionAppService` and:
- Obtains its listen address from the `KvReplicatorEndpoint` endpoint resource.
- Reports custom load metrics (`MyLoad`) periodically so that the Service Fabric cluster
  resource manager can balance replicas.
- Supports primary / secondary / auxiliary replica roles with persistent state.

## Layout

| Path | Description |
|------|-------------|
| `src/main.rs` | Entry point – creates the tokio runtime, registers the service factory. |
| `src/statefulstore.rs` | `Factory` and `Replica` implementations for the stateful service. |
| `src/echo.rs` | Load-reporting loop that runs on the primary replica. |
| `src/test.rs` | Integration tests: partition queries, replica failover, service CRUD, repartition. |
| `src/test2.rs` | Integration tests: resolve notifications, auxiliary replicas, mock driver. |
| `manifests/` | Service Fabric application and service manifests. |

## Deploying

```powershell
# Build
cmake --build build --config Debug

# Deploy to a local one-box cluster
.\scripts\reflection_ctl.ps1 -Action Add

# Resolve the service
.\scripts\reflection_ctl.ps1 -Action Resolve

# Remove
.\scripts\reflection_ctl.ps1 -Action Remove
```