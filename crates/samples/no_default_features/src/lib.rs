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

use mssf_core::runtime::{package_change::PackageChangeEvent, CodePackageActivationContext};
#[no_mangle]
fn test_fn() {
    let my_ctx = CodePackageActivationContext::create().unwrap();

    // One might wish to use such a callback to e.g. trigger custom handling of configuration changes
    // This doesn't require the config feature to be enabled
    let _handler = my_ctx.register_configuration_package_change_handler( |c|
        {
            let (some_package, change_type) = match c
            {
                PackageChangeEvent::Addition { new_package } => (new_package, "Addition"),
                PackageChangeEvent::Removal { previous_package } => (previous_package, "Removal"),
                PackageChangeEvent::Modification { previous_package: _, new_package } => (new_package, "Modification"),
            };
            let changed_package_name = some_package.get_description().name.to_string_lossy();
            let changed_package_str = &changed_package_name;
            println!("Received config package change of type {change_type:?} to package {changed_package_str}");
        }
    ).unwrap();
}
