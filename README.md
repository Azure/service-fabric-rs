# service-fabric-rs

Service Fabric Rust Community SDK.

Build Service Fabric Reliable Services in Rust.

Service Fabric is open sourced on github: `https://github.com/microsoft/service-fabric`.
The latest open sourced version of SF is `6.4`.
This SDK only provides `6.4` functionalities. New functionalities in newer versions is are not accessible in this SDK.

The Fabric runtime and client are accessible from the dlls installed from service fabric runtime.
The c headers are generated from open sourced idls in repo [service-fabric](https://github.com/microsoft/service-fabric/tree/master/src/prod/src/idl/public)

This lib is developed for educational purposes, and not ready for production.
Linux is not supported.

## dependencies
* service-fabric-cpp and its dependencies:
    * Install service fabric runtime. See [get-started](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-get-started)
    * Visual Studio msvc tool chain.
* dotnet `winget install Microsoft.DotNet.SDK.6`
* ClangSharpPInvokeGenerator `dotnet tool install --global ClangSharpPInvokeGenerator --version 15.0.1`
* rust compiler

## Quick Start/Build
```
cmake . -B build
cmake --build build
cmake --build build --target generate_rust
```

## Build details
### Setup prerequisite non-rust build dependencies
* cmake configure `cmake . -B build`
    * This downloads idl files
* cmake build `cmake --build build`
    * Generates fabric import libs for fabric dlls from fabric dlls installed with fabric runtime. 
    * Generates winmd files
* Generate service fabric rust sdk code only (not part of cmake default build)
    * `cmake --build build --target generate_rust` or run `cargo run -p tools_api`

### Rust build
* Compile or run rust example
    * compile rust sdk `cargo build`
    * run sample client executable `cargo run -p samples_client`. source code is in `crates\samples\client\src\main.rs`
    * all other rust targets can be built similarly
### Other targets
* Generate winmd files only (part of build)
    * `cmake --build build --target generate_winmd` or run `dotnet build` in .metadata folder
* Build tcp echo service fabric singlton application.
    * `cmake --build build --target build_rust_sample_echomain`
* Test the echoapp in local cluster
    * Add app to cluster `.\scripts\echomain_ctl.ps1 -Action Add`
    * Run echo powershell to talk to the app `.\scripts\echomain_ctl.ps1 -Action Echo`
    * Remove app from cluster `.\scripts\echomain_ctl.ps1 -Action Remove`

