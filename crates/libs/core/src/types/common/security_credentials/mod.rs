// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricClientSettings2;

/// Placeholder
#[non_exhaustive]
pub enum FabricSecurityCredentials {}

impl FabricSecurityCredentials {
    /// Note: only overrides non-default settings; leaves any settings set previously that don't explicitly have new values alone
    pub fn apply(&self, _settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        // Placeholder
        Ok(())
    }
}
