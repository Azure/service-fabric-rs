// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]

pub mod Microsoft;

// expose mod directly
#[cfg(feature = "ServiceFabric")]
pub use Microsoft::ServiceFabric::*;

// Special usage for mssf_pal.
// See mssf_pal documentations for why this is used this way.
#[cfg(feature = "ServiceFabric")]
use mssf_pal::*;
extern crate self as windows;
extern crate self as windows_core;
