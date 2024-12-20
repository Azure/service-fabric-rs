// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricTypes::{
    FABRIC_CLIENT_ROLE, FABRIC_CLIENT_ROLE_ADMIN, FABRIC_CLIENT_ROLE_UNKNOWN,
    FABRIC_CLIENT_ROLE_USER, FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS, FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY, FABRIC_URI,
};

// This mod contains fabric client related types
mod partition;
pub use partition::*;
mod node;
pub use node::*;
mod replica;
use crate::WString;
pub use replica::*;
mod metrics;
pub use metrics::*;

// FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS
bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct ServiceNotificationFilterFlags: i32{
        const None = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE.0;
        const NamePrefix = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX.0;
        const PrimaryOnly = FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY.0;
    }
}

impl From<&ServiceNotificationFilterFlags> for FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS {
    fn from(value: &ServiceNotificationFilterFlags) -> Self {
        Self(value.bits())
    }
}

// FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION
#[derive(Debug, Clone)]
pub struct ServiceNotificationFilterDescription {
    pub name: WString,
    pub flags: ServiceNotificationFilterFlags,
}

impl From<&ServiceNotificationFilterDescription>
    for FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION
{
    /// The lifetime of the SF raw type returned must match the
    /// original struct.
    fn from(value: &ServiceNotificationFilterDescription) -> Self {
        Self {
            Name: FABRIC_URI(value.name.as_ptr() as *mut u16),
            Flags: (&value.flags).into(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_CLIENT_ROLE
#[derive(Debug, PartialEq, Clone)]
pub enum ClientRole {
    Unknown, // Default client role.
    User,    // User client role. Must set client certificate for tls endpoints.
    Admin,
    // ElevatedAdmin not supported by SF 6.x sdk yet.
}

impl From<FABRIC_CLIENT_ROLE> for ClientRole {
    fn from(value: FABRIC_CLIENT_ROLE) -> Self {
        match value {
            FABRIC_CLIENT_ROLE_UNKNOWN => Self::Unknown,
            FABRIC_CLIENT_ROLE_USER => Self::User,
            FABRIC_CLIENT_ROLE_ADMIN => Self::Admin,
            // FABRIC_CLIENT_ROLE_ELEVATED_ADMIN => Self::ElevatedAdmin,
            _ => Self::Unknown,
        }
    }
}

impl From<ClientRole> for FABRIC_CLIENT_ROLE {
    fn from(value: ClientRole) -> Self {
        match value {
            ClientRole::Unknown => FABRIC_CLIENT_ROLE_UNKNOWN,
            ClientRole::User => FABRIC_CLIENT_ROLE_USER,
            ClientRole::Admin => FABRIC_CLIENT_ROLE_ADMIN,
        }
    }
}
