// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! mssf-pal acts like the windows_core (and windows) crate for the windows-bindgen code to compile on linux.
//! It reexposes windows_core com supports, and some Win32 error codes.
//! windows_core does not support string types on linux, so we provide an minimal implementation here.
//!
//! To use mssf-pal for windows-bindgen generated code, alias mssf-pal crate as the windows-core and windows crate,
//! so that the generated code can resolve windows_core crate content via mssf-pal.

// expose minimal windows_core types except string types for mssf to work on linux.
pub mod imp {
    pub use windows_core::imp::*;
}
pub use windows_core::imp::{CopyType, Type, TypeKind};
pub use windows_core::{
    AsImpl, BOOL, ComObject, ComObjectInner, ComObjectInterface, Compose, ComposeBase,
    DYNAMIC_CAST_IID, Error, GUID, HRESULT, IInspectable, IInspectable_Vtbl, IUnknown,
    IUnknown_Vtbl, IUnknownImpl, Interface, InterfaceRef, OutParam, OutRef, Param, ParamValue, Ref,
    Result, RuntimeName, RuntimeType, StaticComObject, implement,
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

/// A literal UTF-16 wide string with a trailing null terminator, returning the
/// mssf-pal [`PCWSTR`]. Mirrors `windows-strings`'s `w!` but yields mssf-pal's
/// own `PCWSTR` type so generated constants type-check against mssf-pal.
#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        const INPUT: &[u8] = $s.as_bytes();
        const OUTPUT_LEN: usize = $crate::utf16_len(INPUT) + 1;
        const OUTPUT: &[u16; OUTPUT_LEN] = {
            let mut buffer = [0; OUTPUT_LEN];
            let mut input_pos = 0;
            let mut output_pos = 0;
            while let Some((mut code_point, new_pos)) = $crate::decode_utf8_char(INPUT, input_pos) {
                input_pos = new_pos;
                if code_point <= 0xffff {
                    buffer[output_pos] = code_point as u16;
                    output_pos += 1;
                } else {
                    code_point -= 0x10000;
                    buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                    output_pos += 1;
                    buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                    output_pos += 1;
                }
            }
            &{ buffer }
        };
        $crate::PCWSTR::from_raw(OUTPUT.as_ptr())
    }};
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
