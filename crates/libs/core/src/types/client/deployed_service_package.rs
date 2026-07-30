// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricDeployedServicePackageHealthResult;
use mssf_com::FabricTypes::{
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH,
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION,
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION_EX1,
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATE,
    FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_STATES_FILTER,
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
}
