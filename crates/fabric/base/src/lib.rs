#![doc(html_no_source)]

extern crate windows;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::derivable_impls,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::extra_unused_lifetimes,
    clippy::useless_transmute
)]
pub mod Microsoft;

#[cfg(feature = "ServiceFabric")]
pub use Microsoft::ServiceFabric::*;

pub use fabric_metadata;

// In linux force to pull in pal lib for linking
#[cfg(target_os = "linux")]
extern crate fabric_rust_pal;
