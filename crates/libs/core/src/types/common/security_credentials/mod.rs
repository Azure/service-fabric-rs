// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricClient::IFabricClientSettings2,
    FabricTypes::{
        FABRIC_PROTECTION_LEVEL, FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN,
        FABRIC_PROTECTION_LEVEL_NONE, FABRIC_PROTECTION_LEVEL_SIGN,
    },
};

mod claims_credentials;
pub use claims_credentials::*;
mod windows_credentials;
pub use windows_credentials::*;
mod x509_credentials;
pub use x509_credentials::*;

#[non_exhaustive]
pub enum FabricSecurityCredentials {
    // TODO: implement them all
    // TODO: None?
    //FabricWindowsCredentials(FabricWindowsCredentials),
    FabricX509Credentials(FabricX509Credentials),
    //FabricX509Credentials2(FabricX509Credentials2),
    FabricClaimsCredentials(FabricClaimsCredentials),
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FabricProtectionLevel {
    None,
    Sign,
    EncryptAndSign,
}

#[derive(Debug)]
#[allow(dead_code, reason = "For error handling")]
pub struct FabricProtectionLevelUnknownValueError(FABRIC_PROTECTION_LEVEL);

impl TryFrom<FABRIC_PROTECTION_LEVEL> for FabricProtectionLevel {
    type Error = FabricProtectionLevelUnknownValueError;

    fn try_from(value: FABRIC_PROTECTION_LEVEL) -> Result<Self, Self::Error> {
        match value {
            FABRIC_PROTECTION_LEVEL_NONE => Ok(FabricProtectionLevel::None),
            FABRIC_PROTECTION_LEVEL_SIGN => Ok(FabricProtectionLevel::Sign),
            FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN => Ok(FabricProtectionLevel::EncryptAndSign),
            x => Err(FabricProtectionLevelUnknownValueError(x)),
        }
    }
}

impl From<FabricProtectionLevel> for FABRIC_PROTECTION_LEVEL {
    fn from(value: FabricProtectionLevel) -> Self {
        match value {
            FabricProtectionLevel::None => FABRIC_PROTECTION_LEVEL_NONE,
            FabricProtectionLevel::Sign => FABRIC_PROTECTION_LEVEL_SIGN,
            FabricProtectionLevel::EncryptAndSign => FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN,
        }
    }
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
        }
        .apply_inner(settings_interface)
    }
}
