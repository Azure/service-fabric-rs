// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::{
    IFabricDeployedServicePackageHealthResult, IFabricGetDeployedServicePackageListResult,
};
use mssf_com::FabricTypes::{
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE, FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER, FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION, FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM, FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX1, FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX2, FABRIC_DEPLOYMENT_STATUS, FABRIC_DEPLOYMENT_STATUS_ACTIVATING, FABRIC_DEPLOYMENT_STATUS_ACTIVE, FABRIC_DEPLOYMENT_STATUS_DEACTIVATING, FABRIC_DEPLOYMENT_STATUS_DOWNLOADING, FABRIC_DEPLOYMENT_STATUS_FAILED, FABRIC_DEPLOYMENT_STATUS_INVALID, FABRIC_DEPLOYMENT_STATUS_RAN_TO_COMPLETION, FABRIC_DEPLOYMENT_STATUS_UPGRADING,
};
use windows_core::{PCWSTR, WString};

use crate::mem::{BoxPool, GetRaw, GetRawWithBoxPool};
use crate::types::{
    ApplicationHealthPolicy, HealthEventsFilter, HealthState, HealthStateFilterFlags, Uri,
};

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags,
}

impl GetRaw<FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER>
    for DeployedServicePackageHealthStatesFilter
{
    fn get_raw(&self) -> FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER {
        FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealthState {
    pub application_name: Uri,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
}

impl From<&FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE> for DeployedServicePackageHealthState {
    fn from(value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE) -> Self {
        Self {
            application_name: Uri::from(value.ApplicationName),
            service_manifest_name: WString::from(value.ServiceManifestName),
            node_name: WString::from(value.NodeName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
        }
    }
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct DeployedServicePackageHealthQueryDescription {
    pub application_name: Uri,
    pub node_name: WString,
    pub service_manifest_name: WString,
    pub health_policy: Option<ApplicationHealthPolicy>,
    pub events_filter: Option<HealthEventsFilter>,
    /// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1::ServicePackageActivationId
    pub service_package_activation_id: Option<WString>,
}

impl GetRawWithBoxPool<FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION>
    for DeployedServicePackageHealthQueryDescription
{
    fn get_raw_with_pool(
        &self,
        pool: &mut BoxPool,
    ) -> FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION {
        let health_policy = self
            .health_policy
            .as_ref()
            .map(|p| {
                let b = Box::new(p.get_raw_with_pool(pool));
                pool.push(b)
            })
            .unwrap_or_default();

        let events_filter = self
            .events_filter
            .as_ref()
            .map(|f| {
                let b = Box::new(f.get_raw());
                pool.push(b)
            })
            .unwrap_or_default();

        let ex1 = pool.push(Box::new(
            FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1 {
                ServicePackageActivationId: self
                    .service_package_activation_id
                    .as_ref()
                    .map_or(PCWSTR::null(), |s| s.as_pcwstr()),
                Reserved: std::ptr::null_mut(),
            },
        ));

        FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            NodeName: self.node_name.as_pcwstr(),
            ServiceManifestName: self.service_manifest_name.as_pcwstr(),
            HealthPolicy: health_policy,
            EventsFilter: events_filter,
            Reserved: ex1 as *mut _,
        }
    }
}

/// IFabricDeployedServicePackageHealthResult and FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealth {
    pub application_name: Uri,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<crate::types::HealthEvent>,
}

impl From<&IFabricDeployedServicePackageHealthResult> for DeployedServicePackageHealth {
    fn from(value: &IFabricDeployedServicePackageHealthResult) -> Self {
        let raw = unsafe { value.get_DeployedServicePackageHealth().as_ref().unwrap() };
        raw.into()
    }
}

impl From<&FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH> for DeployedServicePackageHealth {
    fn from(value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH) -> Self {
        let health_events = unsafe { value.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        Self {
            application_name: Uri::from(value.ApplicationName),
            service_manifest_name: WString::from(value.ServiceManifestName),
            node_name: WString::from(value.NodeName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
            health_events,
        }
    }
}

// FABRIC_DEPLOYMENT_STATUS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentStatus {
    Invalid,
    Downloading,
    Activating,
    Active,
    Upgrading,
    Deactivating,
    RanToCompletion,
    Failed,
}

impl From<FABRIC_DEPLOYMENT_STATUS> for DeploymentStatus {
    fn from(value: FABRIC_DEPLOYMENT_STATUS) -> Self {
        match value {
            FABRIC_DEPLOYMENT_STATUS_DOWNLOADING => Self::Downloading,
            FABRIC_DEPLOYMENT_STATUS_ACTIVATING => Self::Activating,
            FABRIC_DEPLOYMENT_STATUS_ACTIVE => Self::Active,
            FABRIC_DEPLOYMENT_STATUS_UPGRADING => Self::Upgrading,
            FABRIC_DEPLOYMENT_STATUS_DEACTIVATING => Self::Deactivating,
            FABRIC_DEPLOYMENT_STATUS_RAN_TO_COMPLETION => Self::RanToCompletion,
            FABRIC_DEPLOYMENT_STATUS_FAILED => Self::Failed,
            FABRIC_DEPLOYMENT_STATUS_INVALID => Self::Invalid,
            _ => Self::Invalid,
        }
    }
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct DeployedServicePackageQueryDescription {
    pub node_name: WString,
    pub application_name: Uri,
    pub service_manifest_name_filter: Option<WString>,
}

impl From<&DeployedServicePackageQueryDescription>
    for FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION
{
    fn from(value: &DeployedServicePackageQueryDescription) -> Self {
        Self {
            NodeName: value.node_name.as_pcwstr(),
            ApplicationName: value.application_name.as_raw(),
            ServiceManifestNameFilter: value
                .service_manifest_name_filter
                .as_ref()
                .map_or(PCWSTR::null(), |s| s.as_pcwstr()),
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct DeployedServicePackageQueryResultItem {
    pub service_manifest_name: WString,
    pub service_manifest_version: WString,
    pub status: DeploymentStatus,
    // ex1
    pub service_package_activation_id: WString,
    // ex2
    pub health_state: HealthState,
}

impl From<&FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM>
    for DeployedServicePackageQueryResultItem
{
    fn from(value: &FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM) -> Self {
        let ex1 = unsafe {
            (value.Reserved as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX1).as_ref()
        }.unwrap();
        let ex2 = unsafe {
            (ex1.Reserved as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX2).as_ref()
        }.unwrap();
        Self {
            service_manifest_name: WString::from(value.ServiceManifestName),
            service_manifest_version: WString::from(value.ServiceManifestVersion),
            status: value.DeployedServicePackageStatus.into(),
            service_package_activation_id: WString::from(ex1.ServicePackageActivationId),
            health_state: (&ex2.HealthState).into(),
        }
    }
}

/// IFabricGetDeployedServicePackageListResult
#[derive(Debug, Clone)]
pub struct DeployedServicePackageList {
    pub items: Vec<DeployedServicePackageQueryResultItem>,
}

impl From<&IFabricGetDeployedServicePackageListResult> for DeployedServicePackageList {
    fn from(value: &IFabricGetDeployedServicePackageListResult) -> Self {
        let items = unsafe { value.get_DeployedServicePackageList().as_ref() }
            .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
            .unwrap_or_default();
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HealthStateFilterFlags;

    #[test]
    fn test_deployed_service_package_health_query_description_raw() {
        let desc = DeployedServicePackageHealthQueryDescription {
            application_name: Uri::from("fabric:/App1"),
            node_name: WString::from("Node1"),
            service_manifest_name: WString::from("Pkg1"),
            health_policy: None,
            events_filter: Some(HealthEventsFilter {
                health_state_filter: HealthStateFilterFlags::ERROR,
            }),
            service_package_activation_id: Some(WString::from("activation-1")),
        };
        let mut pool = BoxPool::new();
        let raw = desc.get_raw_with_pool(&mut pool);
        assert_eq!(WString::from(raw.NodeName), WString::from("Node1"));
        assert_eq!(
            WString::from(raw.ServiceManifestName),
            WString::from("Pkg1")
        );
        assert!(!raw.EventsFilter.is_null());
        assert_eq!(
            unsafe { (*raw.EventsFilter).HealthStateFilter },
            HealthStateFilterFlags::ERROR.bits() as u32
        );
        let ex1 = unsafe {
            (raw.Reserved as *const FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1)
                .as_ref()
                .unwrap()
        };
        assert_eq!(
            WString::from(ex1.ServicePackageActivationId),
            WString::from("activation-1")
        );
    }

    #[test]
    fn test_deployed_service_package_health_states_filter_raw() {
        let filter = DeployedServicePackageHealthStatesFilter {
            health_state_filter: HealthStateFilterFlags::WARNING,
        };
        let raw = filter.get_raw();
        assert_eq!(
            raw.HealthStateFilter,
            HealthStateFilterFlags::WARNING.bits() as u32
        );
    }

    #[test]
    fn test_deployed_service_package_query_description_raw() {
        let desc = DeployedServicePackageQueryDescription {
            node_name: WString::from("Node1"),
            application_name: Uri::from("fabric:/App1"),
            service_manifest_name_filter: Some(WString::from("Pkg1")),
        };
        let raw = FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION::from(&desc);
        assert_eq!(WString::from(raw.NodeName).to_string_lossy(), "Node1");
        assert_eq!(Uri::from(raw.ApplicationName).to_string(), "fabric:/App1");
        assert_eq!(
            WString::from(raw.ServiceManifestNameFilter).to_string_lossy(),
            "Pkg1"
        );
    }

    #[test]
    fn test_deployed_service_package_query_result_item_from_raw() {
        let manifest_name = WString::from("Pkg1");
        let manifest_version = WString::from("1.0.0");
        let activation_id = WString::from("activation-1");
        let ex2 = FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX2 {
            HealthState: mssf_com::FabricTypes::FABRIC_HEALTH_STATE_OK,
            Reserved: std::ptr::null_mut(),
        };
        let ex1 = FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM_EX1 {
            ServicePackageActivationId: activation_id.as_pcwstr(),
            Reserved: std::ptr::addr_of!(ex2) as *mut _,
        };
        let raw = FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_ITEM {
            ServiceManifestName: manifest_name.as_pcwstr(),
            ServiceManifestVersion: manifest_version.as_pcwstr(),
            DeployedServicePackageStatus: FABRIC_DEPLOYMENT_STATUS_ACTIVE,
            Reserved: std::ptr::addr_of!(ex1) as *mut _,
        };
        let item = DeployedServicePackageQueryResultItem::from(&raw);
        assert_eq!(item.service_manifest_name.to_string_lossy(), "Pkg1");
        assert_eq!(item.service_manifest_version.to_string_lossy(), "1.0.0");
        assert_eq!(item.status, DeploymentStatus::Active);
        assert_eq!(
            item.service_package_activation_id.to_string_lossy(),
            "activation-1"
        );
        assert_eq!(item.health_state, HealthState::Ok);
    }
}
