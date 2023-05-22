use std::path::Path;

fn main() {
    // add link dir for fabric support libs. This is propagated to downstream targets
    let dir = String::from("build/_deps/fabric_metadata-build/src");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).display()
    );
}
