// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#[cfg(unix)]
pub mod pal;

// expose all windows core has
pub use windows_core::*;

// provide other implemenations missing for linux
// extern crate self as windows_core;
// This overrides the original on windows as well.

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl AsRef<PCWSTR> for PCWSTR {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl windows_core::TypeKind for PCWSTR {
    type TypeKind = windows_core::CopyType;
}