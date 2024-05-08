// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(non_snake_case)]

// lib that contains all common extensions for the raw fabric apis.

pub mod client;
pub mod conf;
pub mod debug;
mod iter;
pub mod runtime;
pub mod strings;
pub mod sync;

// re-export some windows types
pub use windows_core::{Error, Result, GUID, HSTRING, PCWSTR};
