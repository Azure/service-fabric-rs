// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! mssf utilities and extensions

#[cfg(feature = "tokio")]
pub mod tokio;

// Requires tokio_util
#[cfg(feature = "tokio")]
pub mod resolve;

#[cfg(feature = "tokio")]
pub mod monitoring;
