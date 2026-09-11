# tool_api

This crate generates `mssf-com` from the Service Fabric winmd fetched by CMake.
Set `MSSF_WINMD_PATH` to generate from another metadata artifact:

```pwsh
$env:MSSF_WINMD_PATH = 'C:\path\to\Windows.ServiceFabric.winmd'
cargo run -p tools_api
```