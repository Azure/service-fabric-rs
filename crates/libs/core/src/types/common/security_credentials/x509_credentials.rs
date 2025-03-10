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

use mssf_com::FabricTypes::{
    FABRIC_X509_FIND_TYPE, FABRIC_X509_FIND_TYPE_FINDBYEXTENSION,
    FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME, FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT,
    FABRIC_X509_STORE_LOCATION, FABRIC_X509_STORE_LOCATION_CURRENTUSER,
    FABRIC_X509_STORE_LOCATION_INVALID, FABRIC_X509_STORE_LOCATION_LOCALMACHINE,
};
use windows_core::WString;

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
