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
    FABRIC_SECURITY_CREDENTIALS, FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS,
    FABRIC_WINDOWS_CREDENTIALS,
};
use windows_core::{WString, PCWSTR};

use super::{FabricProtectionLevel, FabricSecurityCredentialKind};

/// A wrapper around FABRIC_WINDOWS_CREDENTIALS
#[allow(non_snake_case, reason = "Consistency with underlying API")]
pub struct FabricWindowsCredentials {
    RemoteSpn: WString,
    RemoteIdentities: Vec<WString>,
    ProtectionLevel: FabricProtectionLevel,
}

impl FabricSecurityCredentialKind for FabricWindowsCredentials {
    fn apply_inner(
        &self,
        settings_interface: &mssf_com::FabricClient::IFabricClientSettings2,
    ) -> crate::Result<()> {
        let remote_identities: Box<[PCWSTR]> = self
            .RemoteIdentities
            .iter()
            .map(WString::as_pcwstr)
            .collect();
        let mut value = FABRIC_WINDOWS_CREDENTIALS {
            RemoteSpn: self.RemoteSpn.as_pcwstr(),
            RemoteIdentityCount: u32::try_from(remote_identities.len()).unwrap(),
            RemoteIdentities: remote_identities.as_ptr(),
            ProtectionLevel: self.ProtectionLevel.into(),
            Reserved: ptr::null_mut(),
        };
        let security_credentials = FABRIC_SECURITY_CREDENTIALS {
            Kind: FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS,
            Value: addr_of_mut!(value) as *mut c_void,
        };

        // SAFETY: COM interop. SetSecurityCredentials does not retain reference to the passed in data after function returns.
        unsafe { settings_interface.SetSecurityCredentials(&security_credentials) }
            .map_err(crate::Error::from)
    }
}