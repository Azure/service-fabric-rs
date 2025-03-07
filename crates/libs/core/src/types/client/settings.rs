// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricClientSettings2;

/// A idiomatic Rust version of FABRIC_CLIENT_SETTINGS
///
/// Note: we may choose to add additional optional fields in future without considering that a SemVer breaking change.
/// You should default fields you're not interested in like so:
/// ```
/// # use std::num::NonZeroU32;
/// # use mssf_core::types::FabricClientSettings;
/// let my_settings = FabricClientSettings {
///  // TODO: uncomment in next PR
///  // PartitionLocationCacheLimit: Some(NonZeroU32::new(1).expect("Non-zero value")),
///  // Any other hypothetical settings you're interested in here,
///  ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct FabricClientSettings {}

impl FabricClientSettings {
    /// Note: only overrides non-default settings; leaves any settings set previously that don't explicitly have new values alone
    pub(crate) fn apply(&self, _settings_interface: &IFabricClientSettings2) -> crate::Result<()> {
        // Placeholder
        Ok(())
    }
}
