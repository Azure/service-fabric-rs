// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
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
pub enum FabricSecurityCredentials {
    // TODO: consider None (to clear previously set settings), X509Credentials2?
    FabricWindowsCredentials(FabricWindowsCredentials),
    FabricX509Credentials(FabricX509Credentials),
    //FabricX509Credentials2(FabricX509Credentials2),
    FabricClaimsCredentials(FabricClaimsCredentials),
}

trait FabricSecurityCredentialKind {
    fn apply_inner(&self, settings_interface: &IFabricClientSettings2) -> crate::Result<()>;
}

impl FabricSecurityCredentials {
    // TODO: may belong on the other side?
    pub fn apply(&self, settings_interface: &IFabricClientSettings2) -> crate::Result<()> {
        match &self {
            FabricSecurityCredentials::FabricX509Credentials(v) => {
                v as &dyn FabricSecurityCredentialKind
            }
            FabricSecurityCredentials::FabricClaimsCredentials(v) => {
                v as &dyn FabricSecurityCredentialKind
            }
            FabricSecurityCredentials::FabricWindowsCredentials(v) => {
                v as &dyn FabricSecurityCredentialKind
            }
        }
        .apply_inner(settings_interface)
    }
}