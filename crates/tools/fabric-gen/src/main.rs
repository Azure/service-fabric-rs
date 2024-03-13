// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

mod gen;

use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::Once,
};

use gen::{code::Gen, Parser};
use std::io::Write;
use windows_metadata::{File, Reader, TypeName};

static FILE_PATH: &str =
    r#"build/_deps/fabric_metadata-src/.windows/winmd/Microsoft.ServiceFabric.winmd"#;
static INIT: Once = Once::new();
static mut SF_FILE: Vec<File> = vec![];
static mut SF_READER: Option<Reader> = None;

// init the global reader
pub fn initialize() {
    INIT.call_once(|| {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        println!("test work dir: {}", d.display());
        let root_dir = d.parent().unwrap().parent().unwrap().parent().unwrap();
        println!("root dir: {}", root_dir.display());
        let abs_path_buf = root_dir.join(FILE_PATH);
        // initialization code here
        let bytes = std::fs::read(abs_path_buf.as_path()).expect("File not found");

        let file = File::new(bytes).expect("Invalid metadata");

        let binding = vec![file];
        //let reader = Reader::new(&binding);
        unsafe { SF_FILE = binding };
        unsafe { SF_READER = Some(Reader::new(&SF_FILE)) };
    });
}

// retrieves the global reader
pub fn get_reader() -> &'static Reader<'static> {
    initialize();
    unsafe { SF_READER.as_ref().unwrap() }
}

fn gen_class(ns: &str, name: &str, file_path: &Path, exclude: &Vec<&str>) {
    let reader = get_reader();
    let p = Parser::new(reader);
    let mut itf = p.get_interface_layout_recursive(&TypeName {
        namespace: ns,
        name,
    });
    itf.exclude_funcs(exclude);

    let gen = Gen::new(itf.to_async());
    let token = gen.gen();

    let path = file_path;
    let mut output = std::fs::File::create(path).expect("Cannot create file");
    write!(output, "{}", token).expect("write failed");

    Command::new("rustfmt")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .expect("fmt failed");
}

fn main() {
    let gen_folder = Path::new("crates/fabric/rs/src/client");

    // property client
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricPropertyManagementClient2",
        &gen_folder.join("property.rs"),
        &vec![
            "PutPropertyBinary",   // has array not yet handled
            "SubmitPropertyBatch", // multi return
        ],
    );
    // IFabricServiceManagementClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricServiceManagementClient6",
        &gen_folder.join("svc.rs"),
        &vec!["CreateServiceFromTemplate"], // array
    );
    //IFabricServiceGroupManagementClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricServiceGroupManagementClient4",
        &gen_folder.join("svcgp.rs"),
        &vec!["CreateServiceGroupFromTemplate"], // array
    );

    //IFabricApplicationManagementClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricApplicationManagementClient10",
        &gen_folder.join("app.rs"),
        &vec![], // array
    );

    // IFabricClusterManagementClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricClusterManagementClient10",
        &gen_folder.join("mgmt.rs"),
        &vec![],
    );

    // IFabricHealthClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricHealthClient4",
        &gen_folder.join("health.rs"),
        &vec![],
    );

    //IFabricQueryClient
    gen_class(
        "Microsoft.ServiceFabric.FabricCommon.FabricClient",
        "IFabricQueryClient10",
        &gen_folder.join("query.rs"),
        &vec![],
    );
}
