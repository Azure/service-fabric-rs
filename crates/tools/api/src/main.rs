// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::{
    fs::{self, File},
    io::{self, BufRead, Write},
};

use windows_bindgen::{bindgen, Result};

fn main() -> Result<()> {
    let winmd = "./build/_deps/fabric_metadata-src/.windows/winmd/Microsoft.ServiceFabric.winmd";
    // create output dir if not exist
    fs::create_dir_all("crates/libs/com/src/ServiceFabric/").unwrap();
    // Generate FabricTypes
    {
        let out_file = "crates/libs/com/src/ServiceFabric/FabricTypes.rs";
        let log = bindgen([
            "--in",
            winmd,
            "--out",
            out_file,
            "--filter",
            "Microsoft.ServiceFabric.FabricTypes", // include fabric types
            "--config",
            "implement",
        ])?;
        println!("{}", log);
        // TODO: need to modify the generated files.
        let mut lines = read_file_as_lines(out_file);
        remove_namespace(&mut lines, "pub mod ServiceFabric ");
        remove_namespace(&mut lines, "pub mod FabricTypes ");
        write_content(out_file, lines);
    }
    // Generate FabricCommon
    {
        let out_file = "crates/libs/com/src/ServiceFabric/FabricCommon.rs";
        let log = bindgen([
            "--in",
            winmd,
            "--out",
            out_file,
            // include types
            "--filter",
            "Microsoft.ServiceFabric.FabricCommon",
            "--config",
            "implement",
        ])?;
        println!("{}", log);
        let mut lines = read_file_as_lines(out_file);
        remove_namespace(&mut lines, "pub mod ServiceFabric");
        remove_namespace(&mut lines, "pub mod FabricCommon");
        write_content(out_file, lines);
    }
    // Generate FabricRuntime
    {
        let out_file = "crates/libs/com/src/ServiceFabric/FabricRuntime.rs";
        let log = bindgen([
            "--in",
            winmd,
            "--out",
            out_file,
            "--filter",
            "Microsoft.ServiceFabric.FabricRuntime", // include fabric types
            "--config",
            "implement",
        ])?;
        println!("{}", log);
        let mut lines = read_file_as_lines(out_file);
        remove_namespace(&mut lines, "pub mod ServiceFabric");
        remove_namespace(&mut lines, "pub mod FabricRuntime");
        write_content(out_file, lines);
    }

    // Generate FabricClient
    {
        let out_file = "crates/libs/com/src/ServiceFabric/FabricClient.rs";
        let log = bindgen([
            "--in",
            winmd,
            "--out",
            out_file,
            "--filter",
            "Microsoft.ServiceFabric.FabricClient", // include fabric types
            "--config",
            "implement",
        ])?;
        println!("{}", log);
        let mut lines = read_file_as_lines(out_file);
        remove_namespace(&mut lines, "pub mod ServiceFabric");
        remove_namespace(&mut lines, "pub mod FabricClient");
        write_content(out_file, lines);
    }

    // Generate FabricTransport
    {
        let out_file = "crates/libs/com/src/ServiceFabric/FabricTransport.rs";
        let log = bindgen([
            "--in",
            winmd,
            "--out",
            out_file,
            "--filter",
            "Microsoft.ServiceFabric.FabricTransport", // include fabric types
            "--config",
            "implement",
        ])?;
        println!("{}", log);
        let mut lines = read_file_as_lines(out_file);
        remove_namespace(&mut lines, "pub mod ServiceFabric");
        remove_namespace(&mut lines, "pub mod FabricTransport");
        write_content(out_file, lines);
    }
    Ok(())
}

fn read_file_as_lines(path: &str) -> Vec<String> {
    let r = File::open(path).unwrap();
    let reader = io::BufReader::new(r);
    // process each line and skip the lines targeted
    reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>()
}

fn remove_namespace(lines: &mut Vec<String>, skip_str: &str) {
    lines.retain(|line| {
        if line.contains(skip_str) {
            return false;
        }
        true
    });
    lines.pop();
}

fn write_content(path: &str, lines: Vec<String>) {
    File::create(path)
        .unwrap()
        .write_all(lines.join("\n").as_bytes())
        .unwrap();
}
