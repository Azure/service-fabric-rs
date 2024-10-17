//! If we have an optional feature, we must be able to compile a library or executable without it.
//!
//! This simple unmangled function requires no asynchronous calls (which occurs in the real world when integrating new Rust code into existing apps written in C++ or C# that already have lifecycle management logic)
//!
//! It also doesn't use the config abstraction (not all apps will want to use config-rs)
//!
//! Further, its build.rs provides the path to the import libraries on windows, so it does not need mssf-metadata
//!
//! Therefore, none of the crate's 3 default-enabled features is required
//!
//! This sample demonstrates it is possible to use the library with default-features = false and ensures that that scenario remains compiling as PRs go into the repository.
//!
use mssf_core::runtime::CodePackageActivationContext;
#[no_mangle]
fn test_fn() {
    // Make sure we link something
    //
    let my_ctx = CodePackageActivationContext::create();
    my_ctx.unwrap();
}
