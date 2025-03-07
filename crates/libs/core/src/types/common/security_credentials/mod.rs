// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
use mssf_com::FabricClient::IFabricClientSettings2;

mod claims_credentials;
pub use claims_credentials::*;
mod fabric_protection_level;
pub use fabric_protection_level::*;
mod windows_credentials;
pub use windows_credentials::*;
mod x509_credentials;
pub use x509_credentials::*;

/// Idiomatic FABRIC_SECURITY_CREDENTIALS wrapper
/// Currently, just a placeholder
#[non_exhaustive]
pub enum FabricSecurityCredentials {}

impl FabricSecurityCredentials {
    /// Note: only overrides non-default settings; leaves any settings set previously that don't explicitly have new values alone
    pub(crate) fn apply(&self, _settings_interface: &IFabricClientSettings2) -> crate::Result<()> {
        // Placeholder
        Ok(())
    }
}
