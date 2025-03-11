// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::{ffi::c_void, ptr::addr_of_mut};

use mssf_com::FabricTypes::{
    FABRIC_PROTECTION_LEVEL, FABRIC_SECURITY_CREDENTIALS, FABRIC_SECURITY_CREDENTIAL_KIND_X509,
    FABRIC_X509_CREDENTIALS, FABRIC_X509_FIND_TYPE, FABRIC_X509_FIND_TYPE_FINDBYEXTENSION,
    FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME, FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT,
    FABRIC_X509_STORE_LOCATION, FABRIC_X509_STORE_LOCATION_CURRENTUSER,
    FABRIC_X509_STORE_LOCATION_INVALID, FABRIC_X509_STORE_LOCATION_LOCALMACHINE,
};
use windows_core::{WString, PCWSTR};

use super::{FabricProtectionLevel, FabricSecurityCredentialKind};

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

impl FabricSecurityCredentialKind for FabricX509Credentials {
    fn apply_inner(
        &self,
        settings_interface: mssf_com::FabricClient::IFabricClientSettings2,
    ) -> crate::Result<()> {
        let allowed_common_names: Box<[PCWSTR]> = self
            .AllowedCommonNames
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let allowed_common_names_ptr = allowed_common_names.as_ptr();

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

        let mut value = FABRIC_X509_CREDENTIALS {
            AllowedCommonNameCount: u32::try_from(allowed_common_names.len()).unwrap(),
            AllowedCommonNames: allowed_common_names_ptr,
            FindType: find_type,
            FindValue: find_value,
            StoreLocation: store_location,
            StoreName: store_name,
            ProtectionLevel: protection_level,
            // TODO: extensions
            Reserved: std::ptr::null_mut(),
        };
        let security_credentials = FABRIC_SECURITY_CREDENTIALS {
            Kind: FABRIC_SECURITY_CREDENTIAL_KIND_X509,
            Value: addr_of_mut!(value) as *mut c_void,
        };

        // SAFETY: COM interop. SetSecurityCredentials does not retain reference to the passed in data after function returns.
        unsafe { settings_interface.SetSecurityCredentials(&security_credentials) }
            .map_err(crate::Error::from)
    }
}
