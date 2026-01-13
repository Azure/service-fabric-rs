// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::fs::{self};

use windows_bindgen::bindgen;

fn main() {
    let winmd = "./build/fabric_metadata-src/.windows/winmd/Microsoft.ServiceFabric.winmd";
    // create output dir if not exist
    fs::create_dir_all("crates/libs/com/src/Microsoft/ServiceFabric/").unwrap();

    {
        let out_file = "crates/libs/com/";

        let args = vec![
            "--in",
            winmd,
            "--in",
            "default",
            "--out",
            out_file,
            "--package",
            "--no-allow",
            "--reference",
            "windows,skip-root,Windows",
            "--filter",
        ];
        // Note: winmd currently does not contain C free standing functions,
        // if they are added in the future, we may need to add more filters here.
        let filter_types = vec!["Microsoft.ServiceFabric.FabricTypes"];
        let filter_common = vec![
            "Microsoft.ServiceFabric.FabricCommon", // include fabric types
        ];

        let filter_runtime = vec![
            "Microsoft.ServiceFabric.FabricRuntime", // include fabric types
        ];

        let filter_client = vec![
            "Microsoft.ServiceFabric.FabricClient", // include fabric types
        ];

        bindgen(
            args.into_iter()
                .chain(filter_types)
                .chain(filter_common)
                .chain(filter_runtime)
                .chain(filter_client),
        )
        .unwrap();
    }
}
