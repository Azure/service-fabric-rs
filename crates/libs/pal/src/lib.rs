// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#[cfg(unix)]
pub mod pal;

// expose all windows core has except string types
pub mod imp {
    pub use windows_core::imp::*;
}
pub use windows_core::{
    from_raw_borrowed, implement, AsImpl, ComObject, ComObjectInner, ComObjectInterface, CopyType,
    Error, IInspectable, IInspectable_Vtbl, IUnknown, IUnknownImpl, IUnknown_Vtbl, Interface,
    InterfaceRef, OutParam, Param, ParamValue, Result, RuntimeName, RuntimeType, Type, TypeKind,
    DYNAMIC_CAST_IID, GUID, HRESULT,
};

// provide other implemenations missing for linux
// extern crate self as windows_core;
// This overrides the original on windows as well.
mod strings;
pub use strings::*;

// pal implementation for windows types
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Win32;
