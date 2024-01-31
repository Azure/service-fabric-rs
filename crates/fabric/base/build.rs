// use std::{env, path::{Path, PathBuf}};

use std::path::Path;

fn main() {
    if cfg!(unix) {
        // Add link dir for fabric libs on linux.
        let dir = String::from("/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code/");
        println!("cargo:rustc-link-search={}", Path::new(&dir).display());

        // On linux, for windows-rs to work we need have a pal shared lib.
        // No need to have search dir since it is in the target dir
        // let outdir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        // println!("cargo:rustc-link-search={}", outdir.as_path().display());
        // println!("cargo:rustc-link-lib=static=fabric_rust_pal");
    }
}
