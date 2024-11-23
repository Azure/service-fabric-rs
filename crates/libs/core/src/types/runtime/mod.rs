// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// Runtime related types.

pub mod health;
pub mod stateful;
pub mod store;

use mssf_com::FabricTypes::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION;

use crate::HSTRING;

#[derive(Debug)]
pub struct EndpointResourceDescription {
    pub name: HSTRING,
    pub protocol: HSTRING,
    pub r#type: HSTRING,
    pub port: u32,
    pub certificate_name: HSTRING,
}

impl From<&FABRIC_ENDPOINT_RESOURCE_DESCRIPTION> for EndpointResourceDescription {
    fn from(e: &FABRIC_ENDPOINT_RESOURCE_DESCRIPTION) -> Self {
        EndpointResourceDescription {
            name: HSTRING::from_wide(unsafe { e.Name.as_wide() }).unwrap(),
            protocol: HSTRING::from_wide(unsafe { e.Protocol.as_wide() }).unwrap(),
            r#type: HSTRING::from_wide(unsafe { e.Type.as_wide() }).unwrap(),
            port: e.Port,
            certificate_name: HSTRING::from_wide(unsafe { e.CertificateName.as_wide() }).unwrap(),
        }
    }
}
