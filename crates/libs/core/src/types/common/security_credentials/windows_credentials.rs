// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::{
    ffi::c_void,
    ptr::{self, addr_of_mut},
};

use mssf_com::FabricTypes::{
    FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS, FABRIC_SECURITY_CREDENTIALS,
    FABRIC_WINDOWS_CREDENTIALS,
};
use windows_core::{PCWSTR, WString};

use super::{FabricProtectionLevel, FabricSecurityCredentialKind};

/// A wrapper around FABRIC_WINDOWS_CREDENTIALS
#[allow(non_snake_case, reason = "Consistency with underlying API")]
pub struct FabricWindowsCredentials {
    pub RemoteSpn: WString,
    pub RemoteIdentities: Vec<WString>,
    pub ProtectionLevel: FabricProtectionLevel,
}

impl FabricSecurityCredentialKind for FabricWindowsCredentials {
    fn apply_inner(
        &self,
        settings_interface: mssf_com::FabricClient::IFabricClientSettings2,
    ) -> crate::Result<()> {
        let remote_identities: Box<[PCWSTR]> = self
            .RemoteIdentities
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let remote_identities_ptr = if remote_identities.is_empty() {
            std::ptr::null()
        } else {
            remote_identities.as_ptr()
        };
        let mut value = FABRIC_WINDOWS_CREDENTIALS {
            RemoteSpn: self.RemoteSpn.as_pcwstr(),
            RemoteIdentityCount: u32::try_from(remote_identities.len()).unwrap(),
            RemoteIdentities: remote_identities_ptr,
            ProtectionLevel: self.ProtectionLevel.into(),
            Reserved: ptr::null_mut(),
        };
        let security_credentials = FABRIC_SECURITY_CREDENTIALS {
            Kind: FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS,
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
    use std::sync::{Arc, Mutex};

    use crate::strings::WStringWrap;
    use crate::types::mockifabricclientsettings::MockIFabricClientSettings;
    use crate::types::mockifabricclientsettings::test_utilities::check_array_parameter;

    use super::*;
    const TEST_REMOTE_SPN_1: &str = "TEST_SPN_1";
    const TEST_REMOTE_IDENTITY_1: &str = "TEST_REMOTE_IDENTITY_1";
    const TEST_REMOTE_IDENTITY_2: &str = "TEST_REMOTE_IDENTITY_2";
    fn make_credentials() -> FabricWindowsCredentials {
        FabricWindowsCredentials {
            RemoteSpn: WString::from(TEST_REMOTE_SPN_1),
            RemoteIdentities: vec![
                WString::from(TEST_REMOTE_IDENTITY_1),
                WString::from(TEST_REMOTE_IDENTITY_2),
            ],
            ProtectionLevel: FabricProtectionLevel::EncryptAndSign,
        }
    }

    fn make_credentials_with_empty_vecs() -> FabricWindowsCredentials {
        FabricWindowsCredentials {
            RemoteSpn: WString::new(),
            RemoteIdentities: vec![],
            ProtectionLevel: FabricProtectionLevel::None,
        }
    }

    #[test]
    fn windows_credentials_nonempty_failure() {
        let mock = MockIFabricClientSettings::new_all_methods_fail();
        let creds = make_credentials();
        let result = creds.apply_inner(mock.into());
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn windows_credentials_empty_failure() {
        let mock = IFabricClientSettings2::from(MockIFabricClientSettings::new_all_methods_fail());
        let creds = make_credentials_with_empty_vecs();
        let result = creds.apply_inner(mock);
        assert_eq!(
            result,
            Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS))
        )
    }

    #[test]
    fn windows_credentials_empty_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_ref: &FABRIC_SECURITY_CREDENTIALS = unsafe { creds.as_ref() }.unwrap();
                assert_eq!(creds_ref.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS);

                let value = creds_ref.Value as *const FABRIC_WINDOWS_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_ref = unsafe { value.as_ref() }.unwrap();
                // SAFETY: RemoteIdentityCount and RemoteIdentities go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [],
                        value_ref.RemoteIdentityCount,
                        value_ref.RemoteIdentities,
                    )
                };

                value_ref.RemoteSpn.is_null();
                assert_eq!(value_ref.ProtectionLevel, FABRIC_PROTECTION_LEVEL_NONE);
                assert!(value_ref.Reserved.is_null());

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
    fn windows_credentials_filled_success() {
        let call_counter = Arc::new(Mutex::new(0));
        let call_counter_copy = Arc::clone(&call_counter);
        let com = MockIFabricClientSettings::new_with_security_credentials_mock(Box::new(
            move |creds: *const FABRIC_SECURITY_CREDENTIALS| {
                *call_counter_copy.lock().expect("Not poisoned") += 1;
                assert!(!creds.is_null() && creds.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let creds_ref: &FABRIC_SECURITY_CREDENTIALS = unsafe { creds.as_ref() }.unwrap();
                assert_eq!(creds_ref.Kind, FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS);

                let value = creds_ref.Value as *const FABRIC_WINDOWS_CREDENTIALS;
                assert!(!value.is_null() && value.is_aligned());
                // SAFETY: test code. non-null and alignment is checked above
                let value_ref = unsafe { value.as_ref() }.unwrap();
                // SAFETY: IssuerThumbprintCount and IssuerThumbprints go together. Should be valid for dereference.
                unsafe {
                    check_array_parameter(
                        [TEST_REMOTE_IDENTITY_1, TEST_REMOTE_IDENTITY_2],
                        value_ref.RemoteIdentityCount,
                        value_ref.RemoteIdentities,
                    )
                };

                let remote_spn = WStringWrap::from(value_ref.RemoteSpn)
                    .into_wstring()
                    .to_string_lossy();
                assert_eq!(&remote_spn, TEST_REMOTE_SPN_1);

                assert_eq!(
                    value_ref.ProtectionLevel,
                    FABRIC_PROTECTION_LEVEL_ENCRYPTANDSIGN
                );
                assert!(value_ref.Reserved.is_null());

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
