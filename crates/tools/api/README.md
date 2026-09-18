# tool_api

This crate generates `mssf-com` from the Service Fabric winmd embedded in the
published `mssf-metadata` crate. Set `MSSF_WINMD_PATH` to generate from a local
metadata artifact instead:

```pwsh
$env:MSSF_WINMD_PATH = 'C:\path\to\Windows.ServiceFabric.winmd'
cargo run -p tools_api
```