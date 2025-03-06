// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{any::Any, ffi::c_void};

use mssf_com::{FabricClient::IFabricClientSettings2, FabricTypes::{FABRIC_SECURITY_CREDENTIALS, FABRIC_SECURITY_CREDENTIAL_KIND, FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS, FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS, FABRIC_SECURITY_CREDENTIAL_KIND_X509_2}};

// TODO: would this better live in common?

pub struct FabricWindowsCredentials{}

pub struct FabricX509Credentials{}

pub struct FabricClaimsCredentials
{}

#[non_exhaustive]
pub enum FabricSecurityCredentials{
    FabricWindowsCredentials(FabricWindowsCredentials),
    FabricX509Credentials(FabricX509Credentials),
    FabricClaimsCredentials(FabricClaimsCredentials)
}

impl FabricSecurityCredentials
{
    pub fn set(&self, settings_interface: &IFabricClientSettings2)->  windows_core::Result<()>
    {
        let triplet: (FABRIC_SECURITY_CREDENTIAL_KIND, Box<dyn std::any::Any>, * mut c_void) = match &self
        {
            FabricSecurityCredentials::FabricWindowsCredentials(_) => 
            {
                (FABRIC_SECURITY_CREDENTIAL_KIND_WINDOWS, todo!(), todo!())
            }
            FabricSecurityCredentials::FabricX509Credentials(_) => 
            {
                (FABRIC_SECURITY_CREDENTIAL_KIND_X509_2, todo!(), todo!())
            }
            FabricSecurityCredentials::FabricClaimsCredentials(_) => 
            {
                (FABRIC_SECURITY_CREDENTIAL_KIND_CLAIMS, todo!(), todo!())
            }
        }
       ;
       let (kind, value_source, value) = triplet;
        // SAFETY: settings_interface implements the required interface TODO more
        let credentials = Box::new(FABRIC_SECURITY_CREDENTIALS
        {
            Kind: kind,
            Value: value,
        });

        let result = unsafe { settings_interface.SetSecurityCredentials(std::ptr::null()) };

        result
    }
}