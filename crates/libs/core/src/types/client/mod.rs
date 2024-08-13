// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// This mod contains fabric client related types
mod partition;
use mssf_com::FabricTypes::{
    FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION, FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NAME_PREFIX,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_NONE,
    FABRIC_SERVICE_NOTIFICATION_FILTER_FLAGS_PRIMARY_ONLY, FABRIC_URI,
};
pub use partition::*;
mod node;
pub use node::*;
mod replica;
pub use replica::*;
use windows_core::HSTRING;

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
    pub name: HSTRING,
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
