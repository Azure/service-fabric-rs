// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generated code is consumed by tests/tonic_server_trailers.rs
    // via `tonic::include_proto!("testsvc")`.
    tonic_prost_build::compile_protos("proto/testsvc.proto")?;
    Ok(())
}
