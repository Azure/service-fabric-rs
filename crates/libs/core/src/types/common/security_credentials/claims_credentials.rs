// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::{ffi::c_void, ptr::addr_of_mut};

use mssf_com::FabricTypes::{
    FABRIC_CLAIMS_CREDENTIALS, FABRIC_CLAIMS_CREDENTIALS_EX1,
    FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS, FABRIC_SECURITY_CREDENTIALS,
};
use windows_core::{PCWSTR, WString};

use super::{FabricProtectionLevel, FabricSecurityCredentialKind};

#[allow(non_snake_case, reason = "Consistency with underlying API")]
pub struct FabricClaimsCredentials {
    pub ServerCommonNames: Vec<WString>,
    pub IssuerThumbprints: Vec<WString>,
    pub LocalClaims: WString,
    pub ProtectionLevel: FabricProtectionLevel,
    // FABRIC_CLAIMS_CREDENTIALS_EX1
    pub ServerThumbprints: Vec<WString>,
}

impl FabricSecurityCredentialKind for FabricClaimsCredentials {
    fn apply_inner(
        &self,
        settings_interface: mssf_com::FabricClient::IFabricClientSettings2,
    ) -> crate::Result<()> {
        let server_thumbprints: Box<[PCWSTR]> = self
            .ServerThumbprints
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        // Maybe a bit paranoid, but let's make sure we use a null ptr if it's an empty boxed slice
        fn slice_to_ptr(val: &[PCWSTR]) -> *const PCWSTR {
            if val.is_empty() {
                std::ptr::null()
            } else {
                val.as_ptr()
            }
        }
        let mut ex1 = FABRIC_CLAIMS_CREDENTIALS_EX1 {
            ServerThumbprintCount: u32::try_from(server_thumbprints.len()).unwrap(),
            ServerThumbprints: slice_to_ptr(&server_thumbprints),
            Reserved: std::ptr::null_mut(),
        };

        let server_common_names: Box<[PCWSTR]> = self
            .ServerCommonNames
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let issuer_thumbprints: Box<[PCWSTR]> = self
            .IssuerThumbprints
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let mut value = FABRIC_CLAIMS_CREDENTIALS {
            ServerCommonNameCount: u32::try_from(server_common_names.len()).unwrap(),
            ServerCommonNames: slice_to_ptr(&server_common_names),
            IssuerThumbprintCount: u32::try_from(issuer_thumbprints.len()).unwrap(),
            IssuerThumbprints: slice_to_ptr(&issuer_thumbprints),
            LocalClaims: self.LocalClaims.as_pcwstr(),
            ProtectionLevel: self.ProtectionLevel.into(),
            Reserved: addr_of_mut!(ex1) as *mut c_void,
        };

        let security_credentials = FABRIC_SECURITY_CREDENTIALS {
            Kind: FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS,
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
        FABRIC_PROTECTION_LEVEL_SIGN,
    };
    use std::sync::{Arc, Mutex};

    use crate::strings::WStringWrap;
    use crate::types::mockifabricclientsettings::MockIFabricClientSettings;
    use crate::types::mockifabricclientsettings::test_constants::*;
    use crate::types::mockifabricclientsettings::test_utilities::check_array_parameter;

    use super::*;
    fn make_credentials() -> FabricClaimsCredentials {
        FabricClaimsCredentials {
            ServerCommonNames: vec![WString::from(TEST_SERVER_NAME_1)],
            IssuerThumbprints: vec![
                WString::from(TEST_THUMBPRINT_1),
                WString::from(TEST_THUMBPRINT_2),
            ],
            LocalClaims: WString::from(TEST_CLAIMS),
            ProtectionLevel: FabricProtectionLevel::EncryptAndSign,
            ServerThumbprints: vec![
                WString::from(TEST_THUMBPRINT_3),
                WString::from(TEST_THUMBPRINT_4),
            ],
        }
    }

    fn make_credentials_with_empty_vecs() -> FabricClaimsCredentials {
        FabricClaimsCredentials {
            ServerCommonNames: vec![],
            IssuerThumbprints: vec![],
            LocalClaims: WString::new(),
            ProtectionLevel: FabricProtectionLevel::Sign,
            ServerThumbprints: vec![],
        }
    }

    #[test]
    fn claims_credentials_nonempty_failure() {
        let mock = MockIFabricClientSettings::new_all_methods_fail();
        let creds = make_credentials();
        let result = creds.apply_inner(mock.into());
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn claims_credentials_empty_failure() {
        let mock = IFabricClientSettings2::from(MockIFabricClientSettings::new_all_methods_fail());
        let creds = make_credentials_with_empty_vecs();
        let result = creds.apply_inner(mock);
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn claims_credentials_empty_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_ref: &FABRIC_SECURITY_CREDENTIALS = unsafe { creds.as_ref() }.unwrap();
                assert_eq!(creds_ref.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS);

                let value = creds_ref.Value as *const FABRIC_CLAIMS_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_ref = unsafe { value.as_ref() }.unwrap();
                // SAFETY: IssuerThumbprintCount and IssuerThumbprints go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [],
                        value_ref.IssuerThumbprintCount,
                        value_ref.IssuerThumbprints,
                    )
                };
                // SAFETY: test code. Should point to a null byte even when None.
                assert!(unsafe { value_ref.LocalClaims.is_empty() });
                assert_eq!(value_ref.ProtectionLevel, FABRIC_PROTECTION_LEVEL_SIGN);
                // SAFETY: ServerCommonNameCount and ServerCommonNames go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [],
                        value_ref.ServerCommonNameCount,
                        value_ref.ServerCommonNames,
                    )
                };

                let ex1 = value_ref.Reserved as *const FABRIC_CLAIMS_CREDENTIALS_EX1;
                assert!(!ex1.is_null() && ex1.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let ex1_ref = unsafe { ex1.as_ref() }.unwrap();
                // SAFETY: ServerThumbprintCount and ServerThumbprints go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [],
                        ex1_ref.ServerThumbprintCount,
                        ex1_ref.ServerThumbprints,
                    )
                };

                assert!(ex1_ref.Reserved.is_null());

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
    fn claims_credentials_filled_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_ref: &FABRIC_SECURITY_CREDENTIALS = unsafe { creds.as_ref() }.unwrap();
                assert_eq!(creds_ref.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS);

                let value = creds_ref.Value as *const FABRIC_CLAIMS_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_ref = unsafe { value.as_ref() }.unwrap();
                // SAFETY: IssuerThumbprintCount and IssuerThumbprints go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [TEST_THUMBPRINT_1, TEST_THUMBPRINT_2],
                        value_ref.IssuerThumbprintCount,
                        value_ref.IssuerThumbprints,
                    )
                };

                let local_claim = WStringWrap::from(value_ref.LocalClaims)
                    .into_wstring()
                    .to_string_lossy();
                assert_eq!(&local_claim, TEST_CLAIMS);

                assert_eq!(
                    value_ref.ProtectionLevel,
                    FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN
                );
                // SAFETY: ServerCommonNameCount and ServerCommonNames go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [TEST_SERVER_NAME_1],
                        value_ref.ServerCommonNameCount,
                        value_ref.ServerCommonNames,
                    )
                };

                let ex1 = value_ref.Reserved as *const FABRIC_CLAIMS_CREDENTIALS_EX1;
                assert!(!ex1.is_null() && ex1.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let ex1_ref = unsafe { ex1.as_ref() }.unwrap();
                // SAFETY: ServerThumbprintCount and ServerThumbprints go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [TEST_THUMBPRINT_3, TEST_THUMBPRINT_4],
                        ex1_ref.ServerThumbprintCount,
                        ex1_ref.ServerThumbprints,
                    )
                };

                assert!(ex1_ref.Reserved.is_null());

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
