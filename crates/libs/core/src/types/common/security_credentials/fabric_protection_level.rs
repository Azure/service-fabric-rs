// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use mssf_com::FabricTypes::{
    FABRIC_PROTECTION_LEVEL, FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN, FABRIC_PROTECTION_LEVEL_NONE,
    FABRIC_PROTECTION_LEVEL_SIGN,
};

/// The Fabric Protection Level
/// See https://learn.microsoft.com/en-us/dotnet/api/system.fabric.protectionlevel?view=azure-dotnet
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum FabricProtectionLevel {
    #[default]
    None,
    Sign,
    EncryptAndSign,
}

#[derive(Debug)]
#[allow(dead_code, reason = "For error handling")]
pub struct FabricProtectionLevelUnknownValueError(pub FABRIC_PROTECTION_LEVEL);

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
