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
