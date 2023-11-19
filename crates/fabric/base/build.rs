use std::path::Path;

fn main() {
    if cfg!(unix) {
        // Add link dir for fabric libs on linux.
        let dir = String::from("/opt/microsoft/servicefabric/bin/Fabric/Fabric.Code/");
        println!("cargo:rustc-link-search={}", Path::new(&dir).display());

        // On linux, for windows-rs to work we need have a pal shared lib.
        // No need to have search dir since it is in the target dir
        println!("cargo:rustc-link-lib=dylib=fabric_rust_pal");
    }
}
