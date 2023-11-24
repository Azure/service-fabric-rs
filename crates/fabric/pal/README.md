# pal (Platform Abstraction Layer)
This crate provides a single shared library for linux environment.
It contains bare minimum WIN32 API substitute on linux to make windows-rs crate COM support working.
Service Fabric Rust app and test all requires this pal shared library to work.
So make sure the rust executable can load this lib, i.e. this lib is in `LD_LIBRARY_PATH`.