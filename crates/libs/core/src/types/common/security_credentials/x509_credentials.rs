// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
use std::ffi::c_void;

use mssf_com::FabricTypes::{
    FABRIC_PROTECTION_LEVEL, FABRIC_SECURITY_CREDENTIAL_KIND, FABRIC_SECURITY_CREDENTIAL_KIND_X509,
    FABRIC_X509_CREDENTIALS, FABRIC_X509_FIND_TYPE, FABRIC_X509_FIND_TYPE_FINDBYEXTENSION,
    FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME, FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT,
    FABRIC_X509_STORE_LOCATION, FABRIC_X509_STORE_LOCATION_CURRENTUSER,
    FABRIC_X509_STORE_LOCATION_INVALID, FABRIC_X509_STORE_LOCATION_LOCALMACHINE,
};
use windows_core::{WString, PCWSTR};

use super::{
    FabricProtectionLevel, FabricSecurityCredentialKind, FabricSecurityCredentialKindWrapper,
};

/// How to find the X509 certificate.
#[non_exhaustive]
pub enum FabricX509FindType {
    FindByExtension { extension: WString },
    FindBySubjectName { subject_name: WString },
    FindByThumbprint { thumbprint: WString },
}

impl From<&FabricX509FindType> for FABRIC_X509_FIND_TYPE {
    fn from(value: &FabricX509FindType) -> Self {
        match value {
            FabricX509FindType::FindByExtension { extension: _ } => {
                FABRIC_X509_FIND_TYPE_FINDBYEXTENSION
            }
            FabricX509FindType::FindBySubjectName { subject_name: _ } => {
                FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME
            }
            FabricX509FindType::FindByThumbprint { thumbprint: _ } => {
                FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT
            }
        }
    }
}

/// What store location the certificate will be found in
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FabricX509StoreLocation {
    CurrentUser,
    LocalMachine,
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FabricX509StoreLocationConversionError {
    InvalidValue,
    UnknownValue(FABRIC_X509_STORE_LOCATION),
}

impl TryFrom<FABRIC_X509_STORE_LOCATION> for FabricX509StoreLocation {
    type Error = FabricX509StoreLocationConversionError;

    fn try_from(value: FABRIC_X509_STORE_LOCATION) -> Result<Self, Self::Error> {
        match value {
            FABRIC_X509_STORE_LOCATION_CURRENTUSER => Ok(FabricX509StoreLocation::CurrentUser),
            FABRIC_X509_STORE_LOCATION_LOCALMACHINE => Ok(FabricX509StoreLocation::LocalMachine),
            FABRIC_X509_STORE_LOCATION_INVALID => {
                Err(FabricX509StoreLocationConversionError::InvalidValue)
            }
            x => Err(FabricX509StoreLocationConversionError::UnknownValue(x)),
        }
    }
}

impl From<FabricX509StoreLocation> for FABRIC_X509_STORE_LOCATION {
    fn from(value: FabricX509StoreLocation) -> Self {
        match value {
            FabricX509StoreLocation::CurrentUser => FABRIC_X509_STORE_LOCATION_CURRENTUSER,
            FabricX509StoreLocation::LocalMachine => FABRIC_X509_STORE_LOCATION_LOCALMACHINE,
        }
    }
}

#[allow(non_snake_case, reason = "Consistency with underlying API")]
pub struct FabricX509Credentials {
    AllowedCommonNames: Vec<WString>,
    FindType: FabricX509FindType,
    // NB: FindValue in practice appears to always be a PCWSTR.
    // What that PCWSTR is depends on the FindType.
    // So it's been modeled as a Rust enum
    StoreLocation: FabricX509StoreLocation,
    StoreName: WString,
    ProtectionLevel: FabricProtectionLevel,
    // TODO: extensions?
}

pub(super) struct FabricX509CredentialsTemporaryData {
    /// a Vec<WString> can't be used as an array of PCWSTR
    /// NOTE: drop order in Rust is top to bottom. This should be dropped before allowed_common_names, to avoid dangling
    allowed_common_names_ffi: Box<[PCWSTR]>,
    /// allowed_common_names_ffi borrows data from this
    /// Technically, we could just borrow it from FabricX509Credentials
    /// But the lifetimes already are hard to follow and this shouldn't be at all hot.
    #[allow(dead_code, reason = "Must be kept alive")]
    allowed_common_names: Box<[WString]>,
}

impl FabricSecurityCredentialKind for FabricX509Credentials {
    type FfiType = FABRIC_X509_CREDENTIALS;
    type TemporaryData = FabricX509CredentialsTemporaryData;
    const KIND: FABRIC_SECURITY_CREDENTIAL_KIND = FABRIC_SECURITY_CREDENTIAL_KIND_X509;
    unsafe fn into_raw(
        &self,
    ) -> FabricSecurityCredentialKindWrapper<'_, Self, Self::FfiType, Self::TemporaryData> {
        let allowed_common_names = self.AllowedCommonNames.clone().into_boxed_slice();
        let allowed_common_names_ffi = allowed_common_names.iter().map(|x| x.as_pcwstr()).collect();
        let temporary_data = FabricX509CredentialsTemporaryData {
            allowed_common_names,
            allowed_common_names_ffi,
        };

        let find_type = FABRIC_X509_FIND_TYPE::from(&self.FindType);
        let find_value = match &self.FindType {
            FabricX509FindType::FindByExtension { extension } => extension.as_pcwstr(),
            FabricX509FindType::FindBySubjectName { subject_name } => subject_name.as_pcwstr(),
            FabricX509FindType::FindByThumbprint { thumbprint } => thumbprint.as_pcwstr(),
        }
        .as_ptr() as *mut c_void;
        let store_location = FABRIC_X509_STORE_LOCATION::from(self.StoreLocation);
        let store_name = self.StoreName.as_pcwstr();
        let protection_level = FABRIC_PROTECTION_LEVEL::from(self.ProtectionLevel);
        let value = Box::new(FABRIC_X509_CREDENTIALS {
            AllowedCommonNameCount: u32::try_from(temporary_data.allowed_common_names_ffi.len())
                .unwrap(),
            AllowedCommonNames: temporary_data.allowed_common_names_ffi.as_ptr(),
            FindType: find_type,
            FindValue: find_value,
            StoreLocation: store_location,
            StoreName: store_name,
            ProtectionLevel: protection_level,
            Reserved: std::ptr::null_mut(),
        });
        FabricSecurityCredentialKindWrapper {
            value,
            temporary_data,
            producing_credential: &self,
        }
    }
}
