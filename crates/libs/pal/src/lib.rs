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
pub use windows_core::{
    implement, AsImpl, ComObject, ComObjectInner, ComObjectInterface, CopyType, Error,
    IInspectable, IInspectable_Vtbl, IUnknown, IUnknownImpl, IUnknown_Vtbl, Interface,
    InterfaceRef, OutParam, OutRef, Param, ParamValue, Ref, Result, RuntimeName, RuntimeType,
    StaticComObject, Type, TypeKind, BOOL, DYNAMIC_CAST_IID, GUID, HRESULT
};

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
