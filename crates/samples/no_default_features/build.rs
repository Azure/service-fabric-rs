// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::path::Path;

fn main() {
    if cfg!(windows) {
        // Add link dir for fabric libs on windows
        let dir = String::from(r#"build\_deps\fabric_metadata-src\importlibs"#);
        println!("cargo:rustc-link-search={}", Path::new(&dir).display());
    }
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV");
    if target_env.as_deref() == Ok("msvc") {
        // Make linking warnings errors on msvc
        println!("cargo:rustc-link-arg=/WX");
    }
}
