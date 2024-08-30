// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![doc(html_no_source)]
// extern crate windows;
#![allow(non_snake_case)]
pub mod ServiceFabric;

// expose mod directly
pub use ServiceFabric::*;

#[cfg(all(target_os = "windows", feature = "bundled_libs"))]
pub use mssf_metadata;

// In linux force to pull in pal lib for linking
#[cfg(target_os = "linux")]
extern crate mssf_pal;
