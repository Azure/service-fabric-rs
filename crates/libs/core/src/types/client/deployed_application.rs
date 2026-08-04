// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricDeployedApplicationHealthResult;
use mssf_com::FabricTypes::{
    FABRIC_DEPLOYED_APPLICATION_HEALTH, FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
    FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1,
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
}
