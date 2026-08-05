// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Crate error type.

use crate::address::AddressError;

/// Errors surfaced by `mssf-net` configuration and serving.
#[derive(Debug)]
pub enum Error {
    /// The mapping configuration is invalid (empty name, empty URI, ...).
    Config(String),
    /// The supplied address interpreter rejected an SF endpoint address.
    Address(AddressError),
    /// The SF-facing endpoint source failed.
    Source(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Config(m) => write!(f, "invalid xds mapping configuration: {m}"),
            Error::Address(e) => write!(f, "endpoint address interpretation failed: {e}"),
            Error::Source(m) => write!(f, "endpoint source failure: {m}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Address(e) => Some(e),
            _ => None,
        }
    }
}

impl From<AddressError> for Error {
    fn from(e: AddressError) -> Self {
        Error::Address(e)
    }
}
