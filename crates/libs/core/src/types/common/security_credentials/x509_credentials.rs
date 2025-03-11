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
        // technically speaking, doesn't need to be null in this case. but being paranoid
        let allowed_common_names_ptr = if allowed_common_names.is_empty() {
            std::ptr::null()
        } else {
            allowed_common_names.as_ptr()
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
        let result = unsafe { settings_interface.SetSecurityCredentials(&security_credentials) }
            .map_err(crate::Error::from);
        #[cfg(miri)] // TODO: investigate what's wrong with windows_core::implement drop implement.
        Box::leak(Box::new(settings_interface));
        result
    }
}

#[cfg(test)]
mod test {
    use mssf_com::FabricClient::IFabricClientSettings2;
    use mssf_com::FabricTypes::{
        FABRIC_E_INVALID_CREDENTIALS, FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN,
        FABRIC_PROTECTION_LEVEL_NONE,
    };
    use std::ptr;
    use std::sync::{Arc, Mutex};

    use crate::strings::WStringWrap;
    use crate::types::mockifabricclientsettings::test_constants::*;
    use crate::types::mockifabricclientsettings::test_utilities::check_array_parameter;
    use crate::types::mockifabricclientsettings::MockIFabricClientSettings;

    use super::*;
    const TEST_STORE_1: &str = "TEST_STORE_1";
    const TEST_STORE_2: &str = "TEST_STORE_2";
    fn make_credentials() -> FabricX509Credentials {
        FabricX509Credentials {
            AllowedCommonNames: vec![
                WString::from(TEST_SERVER_NAME_2),
                WString::from(TEST_SERVER_NAME_1),
            ],
            FindType: FabricX509FindType::FindByThumbprint {
                thumbprint: WString::from(TEST_THUMBPRINT_1),
            },
            StoreLocation: FabricX509StoreLocation::LocalMachine,
            StoreName: WString::from(TEST_STORE_1),
            ProtectionLevel: FabricProtectionLevel::EncryptAndSign,
        }
    }

    fn make_credentials_with_empty_vecs() -> FabricX509Credentials {
        FabricX509Credentials {
            AllowedCommonNames: vec![],
            FindType: FabricX509FindType::FindBySubjectName {
                subject_name: WString::from(TEST_SERVER_NAME_1),
            },
            StoreLocation: FabricX509StoreLocation::CurrentUser,
            StoreName: WString::from(TEST_STORE_2),
            ProtectionLevel: FabricProtectionLevel::None,
        }
    }

    #[test]
    fn x509_credentials_nonempty_failure() {
        let mock = MockIFabricClientSettings::new_all_methods_fail();
        let creds = make_credentials();
        let result = creds.apply_inner(mock.into());
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn x509_credentials_empty_failure() {
        let mock = IFabricClientSettings2::from(MockIFabricClientSettings::new_all_methods_fail());
        let creds = make_credentials_with_empty_vecs();
        let result = creds.apply_inner(mock);
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn x509_credentials_empty_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_copy: FABRIC_SECURITY_CREDENTIALS = unsafe { ptr::read(creds) };
                assert_eq!(creds_copy.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_X509);

                let value = creds_copy.Value as *const FABRIC_X509_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_copy = unsafe { ptr::read(value) };
                // SAFETY: AllowedCommonNameCount and AllowedCommonNames go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [],
                        value_copy.AllowedCommonNameCount,
                        value_copy.AllowedCommonNames,
                    )
                };

                assert_eq!(value_copy.FindType, FABRIC_X509_FIND_TYPE_FINDBYSUBJECTNAME);
                let find_val_ptr = value_copy.FindValue as *const u16;
                assert!(!find_val_ptr.is_null() && find_val_ptr.is_aligned());
                let val_str = WStringWrap::from(PCWSTR::from_raw(find_val_ptr))
                    .into_wstring()
                    .to_string_lossy();
                assert_eq!(val_str.as_str(), TEST_SERVER_NAME_1);
                assert_eq!(
                    value_copy.StoreLocation,
                    FABRIC_X509_STORE_LOCATION_CURRENTUSER
                );
                assert_eq!(
                    WStringWrap::from(value_copy.StoreName)
                        .into_wstring()
                        .to_string_lossy()
                        .as_str(),
                    TEST_STORE_2
                );

                assert_eq!(value_copy.ProtectionLevel, FABRIC_PROTECTION_LEVEL_NONE);
                assert!(value_copy.Reserved.is_null());

                Ok(())
            },
        ));
        // SF might reject this in reality - that's ok, we're making sure our code doesn't have UB
        let creds = make_credentials_with_empty_vecs();
        let result = creds.apply_inner(com.into());
        assert_eq!(result, Ok(()));
        let actual_call_count = *call_counter.lock().expect("Not poisioned");
        assert_eq!(actual_call_count, 1)
    }

    #[test]
    fn x509_credentials_filled_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_copy: FABRIC_SECURITY_CREDENTIALS = unsafe { ptr::read(creds) };
                assert_eq!(creds_copy.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_X509);

                let value = creds_copy.Value as *const FABRIC_X509_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_copy = unsafe { ptr::read(value) };
                // SAFETY: AllowedCommonNameCount and AllowedCommonNames go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [TEST_SERVER_NAME_2, TEST_SERVER_NAME_1],
                        value_copy.AllowedCommonNameCount,
                        value_copy.AllowedCommonNames,
                    )
                };

                assert_eq!(value_copy.FindType, FABRIC_X509_FIND_TYPE_FINDBYTHUMBPRINT);
                let find_val_ptr = value_copy.FindValue as *const u16;
                assert!(!find_val_ptr.is_null() && find_val_ptr.is_aligned());
                let val_str = WStringWrap::from(PCWSTR::from_raw(find_val_ptr))
                    .into_wstring()
                    .to_string_lossy();
                assert_eq!(val_str.as_str(), TEST_THUMBPRINT_1);
                assert_eq!(
                    value_copy.StoreLocation,
                    FABRIC_X509_STORE_LOCATION_LOCALMACHINE
                );
                assert_eq!(
                    WStringWrap::from(value_copy.StoreName)
                        .into_wstring()
                        .to_string_lossy()
                        .as_str(),
                    TEST_STORE_1
                );

                assert_eq!(
                    value_copy.ProtectionLevel,
                    FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN
                );
                assert!(value_copy.Reserved.is_null());

                Ok(())
            },
        ));
        // SF might reject this in reality - that's ok, we're making sure our code doesn't have UB
        let creds = make_credentials();
        let result = creds.apply_inner(com.into());
        assert_eq!(result, Ok(()));
        let actual_call_count = *call_counter.lock().expect("Not poisioned");
        assert_eq!(actual_call_count, 1)
    }
}
