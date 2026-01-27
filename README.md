# Service Fabric Rust SDK
![ci](https://github.com/Azure/service-fabric-rs/actions/workflows/build.yaml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://raw.githubusercontent.com/Azure/service-fabric-rs/main/LICENSE)

Build Service Fabric Reliable Services in Rust.

Service Fabric is open sourced on github: [service-fabric](https://github.com/microsoft/service-fabric).

SF SDK idl `11.1` version is available on github here: [service-fabric-metadata](https://github.com/Azure/service-fabric-metadata).

Rust SDK currently supports SF `11.1` COM apis, but only `6.4` version apis are used (this is subject to change), which is the minimum SF version supported by this SDK.

This lib is in alpha state, and apis are subjected to change.

## Getting Started - Windows
* Install Service Fabric Runtime for Windows. See [Prepare your development environment on Windows](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-get-started)
* Install [Visual Studio](https://visualstudio.microsoft.com/) with "Desktop development with C++" to include MSVC toolchain

## Getting Started - Ubuntu Dev Container
* Clone the repo on linux host machine.
* Follow [quick-start-open-an-existing-folder-in-a-container](https://code.visualstudio.com/docs/devcontainers/containers#_quick-start-open-an-existing-folder-in-a-container)
to open the project in devcontainer. The devcontainer setting is [devcontainer.json](./.devcontainer/devcontainer.json).

DevContainer brings up 2 containers: `onebox` container has a SF onebox running, and `repo` container has the repo code, and inside repo container, the SF onebox container ports are forwarded to localhost, so u can use sfctl in repo container as if onebox runs inside the same container.

## Getting Started - Ubuntu
<em>Note: Service Fabric currently only supports Ubuntu 22.04 LTS.</em>

* Install Service Fabric Runtime for Linux (Ubuntu). See [Prepare your development environment on Linux](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-get-started-linux?tabs=sdksetupubuntu%2Clocalclusteroneboxcontainer)
* Following the documentation, proceed to **Manual installation** and skip **Step 6. Add Azul JDK Key**..

## Getting Started - Ubuntu on WSL
The setup is similar to the regular Ubuntu setup, but with some twicks to avoid Service Fabric installer to search for Windows mount paths, which can slow down installation.

Remove `/mnt/c` paths to speed up installation by adding the following to `/etc/wsl.conf` and restart WSL to apply the changes:
```sh
# Remove windows path
[interop]
appendWindowsPath = false

# Do not mount windows drive
[automount]
enabled = false
```

Now, proceed to the regular Ubuntu setup: [Getting Started - Ubuntu](#Getting-Started---Ubuntu)

Edit the `/etc/wsl.conf` to the following to re-enable automount and restart WSL to apply the changes:
```sh
# Remove windows path
[interop]
appendWindowsPath = false

# Mount windows drive
[automount]
enabled = true
```

## Quick Build
Build Rust libraries only:
```sh 
cargo build
```

Alternatively, build all Rust libraries and samples:
```sh
cmake . -B build
cmake --build build
```

## License
Microsoft MIT license