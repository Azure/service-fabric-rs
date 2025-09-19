// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricClient::{IFabricApplicationHealthResult, IFabricGetApplicationListResult2},
    FabricTypes::{
        FABRIC_APPLICATION_DEFINITION_KIND, FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION,
        FABRIC_APPLICATION_QUERY_DESCRIPTION, FABRIC_APPLICATION_QUERY_DESCRIPTION_EX1,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX2, FABRIC_APPLICATION_QUERY_DESCRIPTION_EX3,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX4, FABRIC_APPLICATION_QUERY_RESULT_ITEM,
        FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX1, FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX2,
        FABRIC_APPLICATION_STATUS, FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER,
    },
};
use windows_core::{PCWSTR, WString};

use crate::{
    mem::{GetRaw, GetRawWithBoxPool},
    types::{
        ApplicationHealthPolicy, HealthEventsFilter, HealthState, HealthStateFilterFlags,
        PagingStatus, Uri,
    },
};

// FABRIC_APPLICATION_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ApplicationQueryDescription {
    pub application_name_filter: Option<Uri>,
    pub application_type_name_filter: Option<WString>,
    pub application_definition_kind_filter: ApplicationDefinitionKindFilter,
    pub continuation_token: Option<WString>,
    pub exclude_application_parameters: Option<bool>,
    pub max_results: Option<i32>,
}

// FABRIC_APPLICATION_DEFINITION_KIND_FILTER
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ApplicationDefinitionKindFilter: i32 {
        const All = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_ALL.0;
        const Compose = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_COMPOSE.0;
        const Default = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_DEFAULT.0;
        const MeshApplicationDescription = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_MESH_APPLICATION_DESCRIPTION.0;
        const ServiceFabricApplicationDescription = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_FILTER_SERVICE_FABRIC_APPLICATION_DESCRIPTION.0;
    }
}
impl Default for ApplicationDefinitionKindFilter {
    fn default() -> Self {
        Self::Default
    }
}

// FABRIC_APPLICATION_DEFINITION_KIND
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum ApplicationDefinitionKind {
    Invalid = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_INVALID.0,
    ServiceFabricApplicationDescription = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_DESCRIPTION.0,
    Compose = mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_COMPOSE.0,
    MeshApplicationDescription =
        mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION.0,
}

impl From<FABRIC_APPLICATION_DEFINITION_KIND> for ApplicationDefinitionKind {
    fn from(value: FABRIC_APPLICATION_DEFINITION_KIND) -> Self {
        match value {
            mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_INVALID => {
                ApplicationDefinitionKind::Invalid
            }
            mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_SERVICE_FABRIC_APPLICATION_DESCRIPTION => {
                ApplicationDefinitionKind::ServiceFabricApplicationDescription
            }
            mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_COMPOSE => {
                ApplicationDefinitionKind::Compose
            }
            mssf_com::FabricTypes::FABRIC_APPLICATION_DEFINITION_KIND_MESH_APPLICATION_DESCRIPTION => {
                ApplicationDefinitionKind::MeshApplicationDescription
            }
            _ => ApplicationDefinitionKind::Invalid,
        }
    }
}

// FABRIC_APPLICATION_STATUS
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum ApplicationStatus {
    Creating = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_CREATING.0,
    Deleting = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_DELETING.0,
    Failed = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_FAILED.0,
    Invalid = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_INVALID.0,
    Ready = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_READY.0,
    Upgrading = mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_UPGRADING.0,
}

impl From<FABRIC_APPLICATION_STATUS> for ApplicationStatus {
    fn from(value: FABRIC_APPLICATION_STATUS) -> Self {
        match value {
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_CREATING => {
                ApplicationStatus::Creating
            }
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_DELETING => {
                ApplicationStatus::Deleting
            }
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_FAILED => ApplicationStatus::Failed,
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_INVALID => ApplicationStatus::Invalid,
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_READY => ApplicationStatus::Ready,
            mssf_com::FabricTypes::FABRIC_APPLICATION_STATUS_UPGRADING => {
                ApplicationStatus::Upgrading
            }
            _ => ApplicationStatus::Invalid,
        }
    }
}

impl ApplicationQueryDescription {
    /// Caller is responsible for stiching the reserved parts together
    pub fn get_raw_parts(
        &self,
    ) -> (
        FABRIC_APPLICATION_QUERY_DESCRIPTION,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX1,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX2,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX3,
        FABRIC_APPLICATION_QUERY_DESCRIPTION_EX4,
    ) {
        let base = FABRIC_APPLICATION_QUERY_DESCRIPTION {
            ApplicationNameFilter: self
                .application_name_filter
                .as_ref()
                .map_or(mssf_com::FabricTypes::FABRIC_URI::default(), |u| u.as_raw()),
            Reserved: std::ptr::null_mut(),
        };
        let ex1 = FABRIC_APPLICATION_QUERY_DESCRIPTION_EX1 {
            ContinuationToken: self
                .continuation_token
                .as_ref()
                .map_or(PCWSTR::null(), |u| u.as_pcwstr()),
            Reserved: std::ptr::null_mut(),
        };
        let ex2 = FABRIC_APPLICATION_QUERY_DESCRIPTION_EX2 {
            ApplicationTypeNameFilter: self
                .application_type_name_filter
                .as_ref()
                .map_or(PCWSTR::null(), |u| u.as_pcwstr()),
            // by default do not include application parameters
            ExcludeApplicationParameters: self.exclude_application_parameters.unwrap_or(true),
            Reserved: std::ptr::null_mut(),
        };
        let ex3 = FABRIC_APPLICATION_QUERY_DESCRIPTION_EX3 {
            ApplicationDefinitionKindFilter: self.application_definition_kind_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        };
        let ex4 = FABRIC_APPLICATION_QUERY_DESCRIPTION_EX4 {
            MaxResults: self.max_results.unwrap_or(0), // 0 means no limit
            Reserved: std::ptr::null_mut(),
        };
        (base, ex1, ex2, ex3, ex4)
    }
}

// IFabricGetApplicationListResult2
pub struct ApplicationListResult {
    pub items: Vec<ApplicationQueryResultItem>,
    pub paging_status: Option<PagingStatus>,
}

impl From<&IFabricGetApplicationListResult2> for ApplicationListResult {
    fn from(value: &IFabricGetApplicationListResult2) -> Self {
        let items = unsafe { value.get_ApplicationList().as_ref() }
            .map(|arr| crate::iter::vec_from_raw_com(arr.Count as usize, arr.Items))
            .unwrap_or_default();

        let paging_status = unsafe { value.get_PagingStatus().as_ref() }.map(|ps| ps.into());
        Self {
            items,
            paging_status,
        }
    }
}

// FABRIC_APPLICATION_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct ApplicationQueryResultItem {
    pub application_name: Uri,
    pub application_type_name: WString,
    pub application_type_version: WString,
    pub status: ApplicationStatus,
    pub health_state: HealthState,
    // TODO:
    // pub application_parameters: Option<Vec<(WString, WString)>>,
    pub upgrade_type_version: WString,
    // TODO:
    // pub upgrade_parameters: WString,
    pub application_definition_kind: ApplicationDefinitionKind,
}

impl From<&FABRIC_APPLICATION_QUERY_RESULT_ITEM> for ApplicationQueryResultItem {
    fn from(value: &FABRIC_APPLICATION_QUERY_RESULT_ITEM) -> Self {
        let ex1 =
            unsafe { (value.Reserved as *const FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX1).as_ref() }
                .unwrap();
        let ex2 =
            unsafe { (ex1.Reserved as *const FABRIC_APPLICATION_QUERY_RESULT_ITEM_EX2).as_ref() }
                .unwrap();

        Self {
            application_name: Uri::from(value.ApplicationName),
            application_type_name: WString::from(value.ApplicationTypeName),
            application_type_version: WString::from(value.ApplicationTypeVersion),
            status: value.Status.into(),
            health_state: (&value.HealthState).into(),
            upgrade_type_version: WString::from(ex1.UpgradeTypeVersion),
            application_definition_kind: ApplicationDefinitionKind::from(
                ex2.ApplicationDefinitionKind,
            ),
        }
    }
}

// FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ApplicationHealthQueryDescription {
    pub application_name: Uri,
    pub health_policy: Option<ApplicationHealthPolicy>,
    pub events_filter: Option<HealthEventsFilter>,
    pub services_filter: Option<super::service::ServiceHealthStatesFilter>,
    // pub health_statistics_filter: Option<HealthStatisticsFilter>,
    pub deployed_applications_filter: Option<DeployedApplicationHealthStatesFilter>,
}

impl GetRawWithBoxPool<FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION>
    for ApplicationHealthQueryDescription
{
    fn get_raw_with_pool(
        &self,
        pool: &mut crate::mem::BoxPool,
    ) -> FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION {
        let ex1 = pool.push(Box::new(
            mssf_com::FabricTypes::FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION_EX1 {
                HealthStatisticsFilter: std::ptr::null(),
                Reserved: std::ptr::null_mut(),
            },
        ));

        FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            HealthPolicy: self.health_policy.as_ref().map_or(std::ptr::null(), |p| {
                let b = Box::new(p.get_raw_with_pool(pool));
                pool.push(b)
            }),
            EventsFilter: self
                .events_filter
                .as_ref()
                .map_or(std::ptr::null(), |f| pool.push(Box::new(f.get_raw()))),
            ServicesFilter: self
                .services_filter
                .as_ref()
                .map_or(std::ptr::null(), |f| pool.push(Box::new(f.get_raw()))),
            DeployedApplicationsFilter: self
                .deployed_applications_filter
                .as_ref()
                .map_or(std::ptr::null(), |f| pool.push(Box::new(f.get_raw()))),
            Reserved: ex1 as *mut _,
        }
    }
}

// FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags,
}

impl GetRaw<FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER>
    for DeployedApplicationHealthStatesFilter
{
    fn get_raw(&self) -> FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER {
        FABRIC_DEPLOYED_APPLICATION_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_APPLICATION_HEALTH
#[derive(Debug, Clone)]
pub struct ApplicationHealth {
    pub application_name: Uri,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<crate::types::HealthEvent>,
    pub deployed_applications_health_states: Vec<DeployedApplicationHealthState>,
    pub service_health_states: Vec<crate::types::ServiceHealthState>,
}

impl From<&IFabricApplicationHealthResult> for ApplicationHealth {
    fn from(value: &IFabricApplicationHealthResult) -> Self {
        Self::from(unsafe { value.get_ApplicationHealth().as_ref() }.unwrap())
    }
}

impl From<&mssf_com::FabricTypes::FABRIC_APPLICATION_HEALTH> for ApplicationHealth {
    fn from(value: &mssf_com::FabricTypes::FABRIC_APPLICATION_HEALTH) -> Self {
        let health_events = unsafe { value.HealthEvents.as_ref() }
            .map(|arr| {
                // Interestingly Items ptr here can be non-aligned.
                crate::iter::vec_from_raw_com(arr.Count as usize, arr.Items)
            })
            .unwrap_or_default();
        let deployed_applications_health_states =
            unsafe { value.DeployedApplicationHealthStates.as_ref() }
                .map(|arr| crate::iter::vec_from_raw_com(arr.Count as usize, arr.Items))
                .unwrap_or_default();
        let service_health_states = unsafe { value.ServiceHealthStates.as_ref() }
            .map(|arr| crate::iter::vec_from_raw_com(arr.Count as usize, arr.Items))
            .unwrap_or_default();

        Self {
            application_name: Uri::from(value.ApplicationName),
            aggregated_health_state: HealthState::from(&value.AggregatedHealthState),
            health_events,
            deployed_applications_health_states,
            service_health_states,
        }
    }
}

// FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealthState {
    pub application_name: Uri,
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
}

impl From<&mssf_com::FabricTypes::FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE>
    for DeployedApplicationHealthState
{
    fn from(value: &mssf_com::FabricTypes::FABRIC_DEPLOYED_APPLICATION_HEALTH_STATE) -> Self {
        Self {
            application_name: Uri::from(value.ApplicationName),
            node_name: WString::from(value.NodeName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
        }
    }
}
