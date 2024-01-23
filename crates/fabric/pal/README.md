# pal (Platform Abstraction Layer)
This crate fills windows api needed for windows-rs to work in linux environment.
It contains bare minimum WIN32 API substitute on linux to make windows-rs crate COM support working.
Service Fabric Rust app and test all requires this pal shared library to work.
This crate is forced to be linked with fabric_base crate.

Originally this crate is a shared lib, but it is a rlib now to avoid dynamic loading and packaging.