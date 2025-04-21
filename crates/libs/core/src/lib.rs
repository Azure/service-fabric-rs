// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
//! # Features
//! All features are enabled by default unless otherwise noted.
//! For most scenarios, you'll want the features. However, in some scenarios, such as:
//! - integrating Rust into an existing Service Fabric Application written in another language
//! - when you are using the lower-level COM API to do something more custom
//!   You might not need all of the functionality that the mssf-core crate provides
//!   In this case, you can configure only what you need to reduce dependencies and compile times.
//!
//! * ** config_source **  -
//!   Provides an implementation of config::Source. Requires config_rs crate
//!
//! * ** Tokio **  -
//!   A lot of the sophoisticated functionality in this crate requires Tokio.
//!   However, even without tokio, some of the higher level wrappers over COM types have utility.

// lib that contains all common extensions for the raw fabric apis.

// SF lib entrypoint apis.
pub mod api;
pub use api::API_TABLE;
pub mod client;
#[cfg(feature = "config_source")]
pub mod conf;
pub mod debug;
mod error;
pub use error::{Error, ErrorCode, Result};
mod iter;
pub mod runtime;
pub mod strings;
pub mod sync;
pub mod types;

// re-export some windows types
pub use windows_core::{Interface, WString, GUID, HRESULT, PCWSTR};
// Note cannot re-export windows_core::implement because the macro using it has hard coded mod name.
/// Windows error type.
pub use windows_core::Error as WinError;
pub use windows_core::Result as WinResult;
