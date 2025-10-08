// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// Runtime related types.

pub mod health;
pub mod stateful;
pub mod store;

use mssf_com::FabricTypes::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION;

use crate::WString;

#[derive(Debug)]
pub struct EndpointResourceDescription {
    pub name: WString,
    pub protocol: WString,
    pub r#type: WString,
    pub port: u32,
    pub certificate_name: WString,
}

impl From<&FABRIC_ENDPOINT_RESOURCE_DESCRIPTION> for EndpointResourceDescription {
    fn from(e: &FABRIC_ENDPOINT_RESOURCE_DESCRIPTION) -> Self {
        EndpointResourceDescription {
            name: WString::from(e.Name),
            protocol: WString::from(e.Protocol),
            r#type: WString::from(e.Type),
            port: e.Port,
            certificate_name: WString::from(e.CertificateName),
        }
    }
}
