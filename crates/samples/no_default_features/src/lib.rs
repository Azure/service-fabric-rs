//! The known use case for no-default-features is as part of existing apps that already have lifecycle management
//! in another language
//! So building a sample app is somewhat more involved
//! For now, we just check that we can compile mssf-core and mssf-com without any features except for the bundled import libs.

use mssf_core::runtime::ActivationContext;
#[no_mangle]
fn test_fn() {
    // Make sure we link something
    //
    let my_ctx = ActivationContext::create();
    my_ctx.unwrap();
}
