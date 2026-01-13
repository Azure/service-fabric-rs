// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

fn main() {
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV");
    if target_env.as_deref() == Ok("msvc") {
        // Make linking warnings errors on msvc
        println!("cargo:rustc-link-arg=/WX");
    }
}
