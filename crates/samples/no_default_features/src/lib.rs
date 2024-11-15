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
use std::borrow::Borrow;

use mssf_core::{client::FabricClientBuilder, runtime::CodePackageActivationContext};
#[no_mangle]
fn test_fn() {
    // Make sure we link something
    //
    let my_ctx = CodePackageActivationContext::create();
    my_ctx.unwrap();

    // One might wish to use such a callback to e.g. trigger custom handling of configuration changes
    // This doesn't require the config feature to be enabled
    let _client = FabricClientBuilder::new()
    .with_on_configuration_package_change(|c|
        {
            let change_type = c.change_type;
            let changed_package_name = c.config_package.as_ref().map(|x |x.get_description().name.to_string_lossy());
            let changed_package_str = changed_package_name.borrow().as_deref().unwrap_or("Unknown package name");
            println!("Received config package change of type {change_type:?} to package {changed_package_str}");
            Ok(())
        }
    )
    .build();
}
