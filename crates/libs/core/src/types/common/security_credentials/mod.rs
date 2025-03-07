// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricClient::IFabricClientSettings2,
    FabricTypes::{
        FABRIC_PROTECTION_LEVEL, FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN,
        FABRIC_PROTECTION_LEVEL_NONE, FABRIC_PROTECTION_LEVEL_SIGN, FABRIC_SECURITY_CREDENTIALS,
        FABRIC_SECURITY_CREDENTIAL_KIND,
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
    //FabricClaimsCredentials(FabricClaimsCredentials),
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
/// Struct to enforce lifetime rules for FabricSecurityCredentialKind implementations
/// value may not outlive 'cred, as it may borrow from it
struct FabricSecurityCredentialKindWrapper<'cred, T, U, V> {
    // TODO: avoid the box?
    /// Note: drop order here is critical. temporary_data may need to outlive value to avoid dangling references
    value: Box<U>,
    /// Any temporary data that needs to be retained until after the function
    #[allow(
        dead_code,
        reason = "Ensures any temporary data pointed to by value U lives long enough"
    )]
    temporary_data: V,
    #[allow(
        dead_code,
        reason = "Ensures any permanent data pointed to by value U lives long enough"
    )]
    producing_credential: &'cred T,
}
trait FabricSecurityCredentialKind {
    type FfiType;
    type TemporaryData;
    const KIND: FABRIC_SECURITY_CREDENTIAL_KIND;
    /// SAFETY: caller takes responsibility for ensuring the corresponding cleanup_raw call happens
    unsafe fn into_raw<'a>(
        &'a self,
    ) -> FabricSecurityCredentialKindWrapper<'a, Self, Self::FfiType, Self::TemporaryData>
    where
        Self: Sized;
}

impl FabricSecurityCredentials {
    // TODO: may belong on the other side?
    pub fn set(&self, settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        match &self {
            FabricSecurityCredentials::FabricX509Credentials(v) => {
                Self::set_inner(v, settings_interface)
            }
        }
    }

    fn set_inner<T: FabricSecurityCredentialKind>(
        val: &T,
        settings_interface: &IFabricClientSettings2,
    ) -> windows_core::Result<()> {
        // SAFETY: we call val.cleanup_raw after calling into Service Fabric
        let mut wrapper = unsafe { val.into_raw() };
        let value = wrapper.value.as_mut();
        let value_ptr = value as *mut <T as FabricSecurityCredentialKind>::FfiType;
        let value_ptr_erased = value_ptr as *mut std::ffi::c_void;
        let result = {
            let security_credentials = FABRIC_SECURITY_CREDENTIALS {
                Kind: T::KIND,
                Value: value_ptr_erased,
            };

            // SAFETY: COM interop. SetSecurityCredentials does not retain reference to the passed in data after function returns.
            unsafe { settings_interface.SetSecurityCredentials(&security_credentials) }
            // security_credentials leaves scope here, so security_credentials.Value (which points to wrapper.value) cannot dangle
        };

        // Drop order of wrapper ensures that wrapper.value drops before temporary_data
        // Again ensuring no dangling pointers
        drop(wrapper);

        result
    }
}
