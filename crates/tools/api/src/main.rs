// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::{env, path::PathBuf};

use windows_bindgen::Bindgen;

fn main() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .expect("tools_api must be located under crates/tools")
        .to_path_buf();
    env::set_current_dir(&workspace)
        .unwrap_or_else(|e| panic!("change directory to {}: {e}", workspace.display()));

    let mut bindgen = Bindgen::new();
    if let Some(winmd) = env::var_os("MSSF_WINMD_PATH").map(PathBuf::from) {
        assert!(
            winmd.is_file(),
            "Service Fabric metadata override not found at {}",
            winmd.display()
        );
        bindgen.input(winmd);
    } else {
        bindgen.input_bytes(mssf_metadata::METADATA);
    }
    bindgen
        .input_default()
        .output("crates/libs/com/")
        .package()
        .filters([
            "Windows.ServiceFabric.FabricTypes",
            "Windows.ServiceFabric.FabricCommon",
            "Windows.ServiceFabric.FabricRuntime",
            "Windows.ServiceFabric.FabricClient",
        ])
        .write();
}
