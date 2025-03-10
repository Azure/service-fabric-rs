// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::{ffi::c_void, ptr::addr_of_mut};

use mssf_com::FabricTypes::{
    FABRIC_CLAIMS_CREDENTIALS, FABRIC_CLAIMS_CREDENTIALS_EX1, FABRIC_SECURITY_CREDENTIALS,
    FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS,
};
use windows_core::{WString, PCWSTR};

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
        settings_interface: &mssf_com::FabricClient::IFabricClientSettings2,
    ) -> crate::Result<()> {
        let server_thumbprints: Box<[PCWSTR]> = self
            .ServerThumbprints
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let mut ex1 = FABRIC_CLAIMS_CREDENTIALS_EX1 {
            ServerThumbprintCount: u32::try_from(server_thumbprints.len()).unwrap(),
            ServerThumbprints: server_thumbprints.as_ptr(),
            Reserved: std::ptr::null_mut(),
        };

        let server_common_names: Box<[PCWSTR]> = self
            .ServerCommonNames
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let issuer_thumbprints: Box<[PCWSTR]> = self
            .ServerCommonNames
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let mut value = FABRIC_CLAIMS_CREDENTIALS {
            ServerCommonNameCount: u32::try_from(server_common_names.len()).unwrap(),
            ServerCommonNames: server_common_names.as_ptr(),
            IssuerThumbprintCount: u32::try_from(issuer_thumbprints.len()).unwrap(),
            IssuerThumbprints: issuer_thumbprints.as_ptr(),
            LocalClaims: self.LocalClaims.as_pcwstr(),
            ProtectionLevel: self.ProtectionLevel.into(),
            Reserved: addr_of_mut!(ex1) as *mut c_void,
        };

        let security_credentials = FABRIC_SECURITY_CREDENTIALS {
            Kind: FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS,
            Value: addr_of_mut!(value) as *mut c_void,
        };

        // SAFETY: COM interop. SetSecurityCredentials does not retain reference to the passed in data after function returns.
        unsafe { settings_interface.SetSecurityCredentials(&security_credentials) }
            .map_err(crate::Error::from)
    }
}


#[cfg(test)]
mod test
{
    use mssf_com::FabricTypes::FABRIC_E_INVALID_CREDENTIALS;

    use crate::types::mockifabricclientsettings::MockIFabricClientSettings;

    use super::*;
    fn make_credentials() -> FabricClaimsCredentials
    {
        FabricClaimsCredentials
        {
            ServerCommonNames: vec![WString::from("test.contoso.com")],
            IssuerThumbprints: vec![WString::from("ABCDEF0123456789"), WString::from("ABCDEF0123456789")],
            LocalClaims: WString::from("mock_claims_here"),
            ProtectionLevel: FabricProtectionLevel::EncryptAndSign,
            ServerThumbprints: vec![WString::from("FFABCDEF0123456789"), WString::from("FFABCDEF0123456789")],
        }

    }
    
    fn make_credentials_with_empty_vecs() -> FabricClaimsCredentials
    {
        FabricClaimsCredentials
        {
            ServerCommonNames: vec![],
            IssuerThumbprints: vec![],
            LocalClaims: WString::new(),
            ProtectionLevel: FabricProtectionLevel::EncryptAndSign,
            ServerThumbprints: vec![],
        }

    }

    #[test]
    fn claims_credentials_nonempty_failure()
    {
        let com = MockIFabricClientSettings::new_all_methods_fail();
        let creds = make_credentials();
        let result  = creds.apply_inner(&com.into());
        assert_eq!(result, Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS)))
    }

    #[test]
    fn claims_credentials_empty_failure()
    {
        let com = MockIFabricClientSettings::new_all_methods_fail();
        let creds = make_credentials_with_empty_vecs();
        let result  = creds.apply_inner(&com.into());
        assert_eq!(result, Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS)))
    }
    
}