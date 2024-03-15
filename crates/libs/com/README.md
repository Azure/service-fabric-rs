# fabric_base

* This crate is generated from ServiceFabric winmd file by windows-bindgen.
  * [ServiceFabric.winmd](https://github.com/youyuanwu/fabric-metadata/tree/main/.windows/winmd)
  * [windows-bindgen](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

This is the lowest layer for Service Fabric Rust. Higher layer APIs are built on top of this.
User should prefer using fabric_rs crate safe rust API when possible.