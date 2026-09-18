// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::{env, fs, path::PathBuf};

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

    let old_namespace = workspace.join("crates/libs/com/src/Microsoft");
    if old_namespace.exists() {
        fs::remove_dir_all(&old_namespace)
            .unwrap_or_else(|e| panic!("remove {}: {e}", old_namespace.display()));
    }

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

    // The new windows-bindgen projects scoped (newtype) enums with their variants
    // as *associated* constants (`impl FABRIC_X { pub const FABRIC_X_Y: Self = Self(n); }`),
    // referenced as `FABRIC_X::FABRIC_X_Y`. The old dotnet toolchain instead emitted
    // free module-level constants (`pub const FABRIC_X_Y: FABRIC_X = ...;`), which is
    // how mssf-core references them. Emit those free-const aliases here so the ergonomic
    // `FabricTypes::FABRIC_X_Y` spelling keeps working alongside the newtype.
    for ns in [
        "FabricTypes",
        "FabricCommon",
        "FabricClient",
        "FabricRuntime",
    ] {
        let path = format!("crates/libs/com/src/Windows/ServiceFabric/{ns}/mod.rs");
        emit_enum_const_aliases(&path);
    }
}

/// Appends free module-level constant aliases for every scoped-enum associated
/// constant in `path` (see call site for rationale).
fn emit_enum_const_aliases(path: &str) {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));

    let mut current_enum: Option<&str> = None;
    let mut aliases = String::new();
    for line in text.lines() {
        // Top-level `impl NAME {` (a plain identifier — excludes `impl Trait for T`,
        // generic impls, and `impl Default for ...`). Only scoped enums carry
        // `pub const X: Self = Self(..)` members, so interface/struct impls yield none.
        if let Some(name) = line
            .strip_prefix("impl ")
            .and_then(|s| s.strip_suffix(" {"))
            && !name.is_empty()
            && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            current_enum = Some(name);
            continue;
        }
        if line == "}" {
            current_enum = None;
            continue;
        }
        if let Some(en) = current_enum {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("pub const ") {
                // Enum variants are `pub const NAME: Self = Self(..)`. rustfmt may
                // wrap long declarations anywhere (even between `NAME:` and `Self`),
                // so take the identifier up to the first `:`. Skip `pub const fn`
                // helpers (flag enums).
                if !rest.starts_with("fn ")
                    && let Some((name, _)) = rest.split_once(':')
                {
                    let name = name.trim();
                    if !name.is_empty()
                        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    {
                        aliases.push_str(&format!("pub const {name}: {en} = {en}::{name};\n"));
                    }
                }
            }
        }
    }

    if aliases.is_empty() {
        return;
    }

    let mut out = text;
    out.push_str(
        "\n// Free module-level aliases for scoped-enum variants (see tools_api). This\n\
         // restores the old dotnet-metadata spelling `FabricTypes::FABRIC_X_Y` in\n\
         // addition to the newtype's associated const `FABRIC_X::FABRIC_X_Y`.\n",
    );
    out.push_str(&aliases);
    fs::write(path, out).unwrap_or_else(|e| panic!("write {path}: {e}"));
}
