// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Stand-in service used only by the cluster-free integration tests.
    tonic_prost_build::compile_protos("proto/testsvc.proto")?;
    Ok(())
}
