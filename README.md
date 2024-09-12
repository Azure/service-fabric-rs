# Service Fabric Rust SDK
![ci](https://github.com/Azure/service-fabric-rs/actions/workflows/build.yaml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://raw.githubusercontent.com/Azure/service-fabric-rs/main/LICENSE)

Build Service Fabric Reliable Services in Rust.

Service Fabric is open sourced on github: `https://github.com/microsoft/service-fabric`.
The latest open sourced version of SF is `6.4`.
This SDK only provides `6.4` functionalities. New functionalities in newer versions is are not accessible in this SDK.

The FabricRuntime and FabricClient are accessible from the DLLs installed from Service Fabric Runtime. The C header files are generated from open-sourced IDLs in repo [service-fabric](https://github.com/microsoft/service-fabric/tree/master/src/prod/src/idl/public)

This lib is in alpha state, and apis are subjected to change.

## Getting Started - Windows
* Install Service Fabric Runtime for Windows. See [Prepare your development environment on Windows](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-get-started)
* Install [Visual Studio](https://visualstudio.microsoft.com/) with "Desktop development with C++" to include MSVC toolchain

## Getting Started - Ubuntu
<em>Note: Service Fabric currently only supports Ubuntu 18.04 LTS and Ubuntu 20.04 LTS.</em>

* Install Service Fabric Runtime for Linux (Ubuntu). See [Prepare your development environment on Linux](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-get-started-linux?tabs=sdksetupubuntu%2Clocalclusteroneboxcontainer)
* Following the documentation, proceed to **Manual installation** and skip **Step 6. Add Azul JDK Key**..

## Getting Started - Ubuntu on WSL
The setup is similar to the regular Ubuntu setup, but with some twicks to avoid Service Fabric installer to search for Windows mount paths, which can slow down installation.

Remove `/mnt/c` paths to speed up installation by adding the following to `/etc/wsl.conf` and restart WSL to apply the changes:
```sh
# This removes windows path
[interop]
appendWindowsPath = false
# This do not mount windows drive
[automount]
enabled = false
```

Now, proceed to the regular Ubuntu setup: [Getting Started - Ubuntu](#Getting-Started---Ubuntu)

Edit the `/etc/wsl.conf` to the following and restart WSL to apply the changes:
```sh
# This removes windows path
# [interop]
# appendWindowsPath = false
# This do not mount windows drive
[automount]
enabled = false
```

## Quick Build
Build all Rust libraries and examples
```sh
cmake . -B build
cmake --build build
```

## License
Microsoft MIT license