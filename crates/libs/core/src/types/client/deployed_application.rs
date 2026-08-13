// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::{
    IFabricDeployedApplicationHealthResult, IFabricGetDeployedApplicationListResult,
};
use mssf_com::FabricTypes::{
    FABRIC_DEPLOYED_APPLICATION_HEALTH, FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
    FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1,
    FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION, FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM,
};
use windows_core::WString;

use crate::mem::{BoxPool, GetRaw, GetRawWithBoxPool};
use crate::types::{
    ApplicationHealthPolicy, DeployedServicePackageHealthState,
    DeployedServicePackageHealthStatesFilter, HealthEventsFilter, HealthState, Uri,
};

/// FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct DeployedApplicationHealthQueryDescription {
    pub application_name: Uri,
    pub node_name: WString,
    pub health_policy: Option<ApplicationHealthPolicy>,
    pub events_filter: Option<HealthEventsFilter>,
    pub deployed_service_packages_filter: Option<DeployedServicePackageHealthStatesFilter>,
}

impl GetRawWithBoxPool<FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION>
    for DeployedApplicationHealthQueryDescription
{
    fn get_raw_with_pool(
        &self,
        pool: &mut BoxPool,
    ) -> FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION {
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

        let deployed_service_packages_filter = self
            .deployed_service_packages_filter
            .as_ref()
            .map(|f| {
                let b = Box::new(f.get_raw());
                pool.push(b)
            })
            .unwrap_or_default();

        // HealthStatisticsFilter is not yet exposed on the Rust wrapper; always request full statistics.
        let ex1 = pool.push(Box::new(
            FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
                HealthStatisticsFilter: std::ptr::null(),
                Reserved: std::ptr::null_mut(),
            },
        ));

        FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            NodeName: self.node_name.as_pcwstr(),
            HealthPolicy: health_policy,
            EventsFilter: events_filter,
            DeployedServicePackagesFilter: deployed_service_packages_filter,
            Reserved: ex1 as *mut _,
        }
    }
}

/// IFabricDeployedApplicationHealthResult and FABRIC_DEPLOYED_APPLICATION_HEALTH
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealth {
    pub application_name: Uri,
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<crate::types::HealthEvent>,
    pub deployed_service_package_health_states: Vec<DeployedServicePackageHealthState>,
}

impl From<&IFabricDeployedApplicationHealthResult> for DeployedApplicationHealth {
    fn from(value: &IFabricDeployedApplicationHealthResult) -> Self {
        let raw = unsafe { value.get_DeployedApplicationHealth().as_ref().unwrap() };
        raw.into()
    }
}

impl From<&FABRIC_DEPLOYED_APPLICATION_HEALTH> for DeployedApplicationHealth {
    fn from(value: &FABRIC_DEPLOYED_APPLICATION_HEALTH) -> Self {
        let health_events = unsafe { value.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        let deployed_service_package_health_states =
            unsafe { value.DeployedServicePackageHealthStates.as_ref() }.map_or(vec![], |list| {
                crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
            });
        Self {
            application_name: Uri::from(value.ApplicationName),
            node_name: WString::from(value.NodeName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
            health_events,
            deployed_service_package_health_states,
        }
    }
}

/// FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct DeployedApplicationQueryDescription {
    pub node_name: WString,
    pub application_name_filter: Option<Uri>,
}

impl From<&DeployedApplicationQueryDescription> for FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION {
    fn from(value: &DeployedApplicationQueryDescription) -> Self {
        Self {
            NodeName: value.node_name.as_pcwstr(),
            ApplicationNameFilter: value
                .application_name_filter
                .as_ref()
                .map_or(Uri::default().as_raw(), |u| u.as_raw()),
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct DeployedApplicationQueryResultItem {
    pub application_name: Uri,
    pub application_type_name: WString,
    pub status: super::DeploymentStatus,
}

impl From<&FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM> for DeployedApplicationQueryResultItem {
    fn from(value: &FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM) -> Self {
        Self {
            application_name: Uri::from(value.ApplicationName),
            application_type_name: WString::from(value.ApplicationTypeName),
            status: value.DeployedApplicationStatus.into(),
        }
    }
}

/// IFabricGetDeployedApplicationListResult
#[derive(Debug, Clone)]
pub struct DeployedApplicationList {
    pub items: Vec<DeployedApplicationQueryResultItem>,
}

impl From<&IFabricGetDeployedApplicationListResult> for DeployedApplicationList {
    fn from(value: &IFabricGetDeployedApplicationListResult) -> Self {
        let items = unsafe { value.get_DeployedApplicationList().as_ref() }
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
    fn test_deployed_application_health_query_description_raw() {
        let desc = DeployedApplicationHealthQueryDescription {
            application_name: Uri::from("fabric:/App1"),
            node_name: WString::from("Node1"),
            health_policy: None,
            events_filter: Some(HealthEventsFilter {
                health_state_filter: HealthStateFilterFlags::WARNING,
            }),
            deployed_service_packages_filter: Some(DeployedServicePackageHealthStatesFilter {
                health_state_filter: HealthStateFilterFlags::ERROR,
            }),
        };
        let mut pool = BoxPool::new();
        let raw = desc.get_raw_with_pool(&mut pool);
        assert_eq!(WString::from(raw.NodeName), WString::from("Node1"));
        assert!(!raw.EventsFilter.is_null());
        assert_eq!(
            unsafe { (*raw.EventsFilter).HealthStateFilter },
            HealthStateFilterFlags::WARNING.bits() as u32
        );
        assert!(!raw.DeployedServicePackagesFilter.is_null());
        assert_eq!(
            unsafe { (*raw.DeployedServicePackagesFilter).HealthStateFilter },
            HealthStateFilterFlags::ERROR.bits() as u32
        );
        assert!(!raw.Reserved.is_null());
    }

    #[test]
    fn test_deployed_application_query_description_raw() {
        let desc = DeployedApplicationQueryDescription {
            node_name: WString::from("Node1"),
            application_name_filter: Some(Uri::from("fabric:/App1")),
        };
        let raw = FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION::from(&desc);
        assert_eq!(WString::from(raw.NodeName).to_string_lossy(), "Node1");
        assert_eq!(
            Uri::from(raw.ApplicationNameFilter).to_string(),
            "fabric:/App1"
        );
    }

    #[test]
    fn test_deployed_application_query_description_no_filter_raw() {
        let desc = DeployedApplicationQueryDescription {
            node_name: WString::from("Node1"),
            application_name_filter: None,
        };
        let raw = FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION::from(&desc);
        assert_eq!(Uri::from(raw.ApplicationNameFilter).to_string(), "");
    }

    #[test]
    fn test_deployed_application_query_result_item_from_raw() {
        let app_name = Uri::from("fabric:/App1");
        let app_type_name = WString::from("AppType1");
        let raw = FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_ITEM {
            ApplicationName: app_name.as_raw(),
            ApplicationTypeName: app_type_name.as_pcwstr(),
            DeployedApplicationStatus: mssf_com::FabricTypes::FABRIC_DEPLOYMENT_STATUS_ACTIVE,
            Reserved: std::ptr::null_mut(),
        };
        let item = DeployedApplicationQueryResultItem::from(&raw);
        assert_eq!(item.application_name.to_string(), "fabric:/App1");
        assert_eq!(item.application_type_name.to_string_lossy(), "AppType1");
        assert_eq!(item.status, crate::types::DeploymentStatus::Active);
    }
}
