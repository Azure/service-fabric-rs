// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Safe types describing the progress of a Service Fabric application upgrade.

use mssf_com::FabricClient::IFabricApplicationUpgradeProgressResult2;
use mssf_com::FabricTypes::{
    FABRIC_APPLICATION_UPGRADE_STATE, FABRIC_APPLICATION_UPGRADE_STATE_FAILED,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_COMPLETED,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_PENDING,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS,
    FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_PENDING, FABRIC_UPGRADE_DOMAIN_STATE,
    FABRIC_UPGRADE_DOMAIN_STATE_COMPLETED, FABRIC_UPGRADE_DOMAIN_STATE_IN_PROGRESS,
    FABRIC_UPGRADE_DOMAIN_STATE_PENDING, FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
};
use windows_core::WString;

use crate::types::Uri;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationUpgradeState {
    Invalid,
    RollingBackInProgress,
    RollingBackCompleted,
    RollingForwardPending,
    RollingForwardInProgress,
    RollingForwardCompleted,
    Failed,
    RollingBackPending,
}

impl ApplicationUpgradeState {
    /// Returns true when the application is actively going through an upgrade
    /// (rolling forward or back, pending or in progress) or the upgrade failed.
    /// Returning failed as true, since a failed state is actionable.
    /// The completed / invalid states return false.
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            Self::RollingForwardPending
                | Self::RollingForwardInProgress
                | Self::RollingBackPending
                | Self::RollingBackInProgress
                | Self::Failed
        )
    }
}

impl From<FABRIC_APPLICATION_UPGRADE_STATE> for ApplicationUpgradeState {
    fn from(value: FABRIC_APPLICATION_UPGRADE_STATE) -> Self {
        match value {
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_IN_PROGRESS => {
                Self::RollingBackInProgress
            }
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_COMPLETED => Self::RollingBackCompleted,
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_PENDING => Self::RollingForwardPending,
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_IN_PROGRESS => {
                Self::RollingForwardInProgress
            }
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_FORWARD_COMPLETED => {
                Self::RollingForwardCompleted
            }
            FABRIC_APPLICATION_UPGRADE_STATE_FAILED => Self::Failed,
            FABRIC_APPLICATION_UPGRADE_STATE_ROLLING_BACK_PENDING => Self::RollingBackPending,
            _ => Self::Invalid,
        }
    }
}

impl std::fmt::Display for ApplicationUpgradeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Invalid => "Invalid",
            Self::RollingBackInProgress => "RollingBackInProgress",
            Self::RollingBackCompleted => "RollingBackCompleted",
            Self::RollingForwardPending => "RollingForwardPending",
            Self::RollingForwardInProgress => "RollingForwardInProgress",
            Self::RollingForwardCompleted => "RollingForwardCompleted",
            Self::Failed => "Failed",
            Self::RollingBackPending => "RollingBackPending",
        };
        write!(f, "{s}")
    }
}

/// The state of a single upgrade domain within an application upgrade.
/// Maps from `FABRIC_UPGRADE_DOMAIN_STATE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeDomainState {
    Invalid,
    Pending,
    InProgress,
    Completed,
}

impl From<FABRIC_UPGRADE_DOMAIN_STATE> for UpgradeDomainState {
    fn from(value: FABRIC_UPGRADE_DOMAIN_STATE) -> Self {
        match value {
            FABRIC_UPGRADE_DOMAIN_STATE_PENDING => Self::Pending,
            FABRIC_UPGRADE_DOMAIN_STATE_IN_PROGRESS => Self::InProgress,
            FABRIC_UPGRADE_DOMAIN_STATE_COMPLETED => Self::Completed,
            _ => Self::Invalid,
        }
    }
}

impl std::fmt::Display for UpgradeDomainState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Invalid => "Invalid",
            Self::Pending => "Pending",
            Self::InProgress => "InProgress",
            Self::Completed => "Completed",
        };
        write!(f, "{s}")
    }
}

/// The status of a single upgrade domain.
/// Maps from `FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION`.
#[derive(Debug, Clone)]
pub struct UpgradeDomainStatus {
    pub name: WString,
    pub state: UpgradeDomainState,
}

impl From<&FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION> for UpgradeDomainStatus {
    fn from(value: &FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION) -> Self {
        Self {
            name: WString::from(value.Name),
            state: value.State.into(),
        }
    }
}

/// Describes the progress of an application upgrade, including the per
/// upgrade-domain status.
/// Maps from `IFabricApplicationUpgradeProgressResult2`.
#[derive(Debug, Clone)]
pub struct ApplicationUpgradeProgress {
    pub application_name: Uri,
    pub application_type_name: WString,
    pub target_application_type_version: WString,
    pub upgrade_state: ApplicationUpgradeState,
    pub upgrade_domains: Vec<UpgradeDomainStatus>,
}

impl ApplicationUpgradeProgress {
    /// Returns true when the application is actively going through an upgrade.
    /// See [`ApplicationUpgradeState::is_active`].
    pub fn is_active(&self) -> bool {
        self.upgrade_state.is_active()
    }
}

impl From<&IFabricApplicationUpgradeProgressResult2> for ApplicationUpgradeProgress {
    fn from(com: &IFabricApplicationUpgradeProgressResult2) -> Self {
        // SAFETY: `com` is a valid COM result object; the getters return values
        // and pointers owned by `com`, which are only read while it is alive.
        unsafe {
            let application_name = Uri::from(com.get_ApplicationName());
            let application_type_name = WString::from(com.get_ApplicationTypeName());
            let target_application_type_version =
                WString::from(com.get_TargetApplicationTypeVersion());
            let upgrade_state = ApplicationUpgradeState::from(com.get_UpgradeState());

            // `GetUpgradeDomains` writes the domain count into `item_count` and
            // returns a pointer to `item_count` contiguous descriptions owned by
            // `com`.
            let mut item_count: u32 = 0;
            let domains_ptr = com
                .GetUpgradeDomains(&mut item_count)
                .unwrap_or(std::ptr::null_mut());
            let upgrade_domains = if domains_ptr.is_null() || item_count == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(domains_ptr, item_count as usize)
                    .iter()
                    .map(UpgradeDomainStatus::from)
                    .collect()
            };

            Self {
                application_name,
                application_type_name,
                target_application_type_version,
                upgrade_state,
                upgrade_domains,
            }
        }
    }
}
