// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! mssf-pal acts like the windows_core (and windows) crate for windows-bindgen code to compile on Linux.
//! It re-exports cross-platform windows-core support and provides the remaining Service Fabric
//! compatibility types and Win32 error codes.
//!
//! To use mssf-pal for windows-bindgen generated code, alias mssf-pal crate as the windows-core and windows crate,
//! so that the generated code can resolve windows_core crate content via mssf-pal.

// Expose the windows-core types needed by generated bindings.
pub mod imp {
    pub use windows_core::imp::*;
}
pub use windows_core::imp::{CopyType, Type, TypeKind};
#[doc(hidden)]
pub use windows_core::w as __windows_core_w;
pub use windows_core::{
    AsImpl, BOOL, ComObject, ComObjectInner, ComObjectInterface, Compose, ComposeBase,
    DYNAMIC_CAST_IID, Error, GUID, HRESULT, IInspectable, IInspectable_Vtbl, IUnknown,
    IUnknown_Vtbl, IUnknownImpl, Interface, InterfaceRef, OutParam, OutRef, PCSTR, Param,
    ParamValue, Ref, Result, RuntimeName, RuntimeType, StaticComObject, implement,
};

/// `link!` macro for windows-bindgen generated free functions.
///
/// The Service Fabric winmd does not carry the owning dll name, so bindgen emits
/// `link!("" ...)` with an empty library name. mssf resolves these entry points
/// dynamically via `libloading` (see `mssf-core`), so we never rely on static
/// linkage. This expands to a plain `extern` declaration without a `#[link]`
/// attribute (mirroring `windows-link`'s non-windows arm), which compiles on all
/// platforms and links only when the symbol is actually referenced.
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $name:ident($($params:tt)*) $(-> $ret:ty)?) => (
        unsafe extern $abi {
            pub fn $name($($params)*) $(-> $ret)?;
        }
        #[allow(non_camel_case_types)]
        pub type $name = unsafe extern $abi fn($($params)*) $(-> $ret)?;
    )
}

/// A literal UTF-16 wide string with a trailing null terminator.
#[macro_export]
macro_rules! w {
    ($s:literal) => {
        $crate::PCWSTR::from_raw($crate::__windows_core_w!($s).as_ptr())
    };
}

// provide other implemenations missing for linux
// extern crate self as windows_core;
// This is used on windows as well.
mod strings;
pub use strings::*;

// pal definition for windows types
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

/// Provides windows crate Win32 mod contents needed to build windows-bindgen
/// generated code on linux, and some minimal common windows definitions.
pub mod Win32;
