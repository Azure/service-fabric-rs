// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricNodeHealthResult;
use mssf_com::FabricTypes::{FABRIC_HEALTH_EVENT, FABRIC_HEALTH_EVENT_LIST};
use windows_core::Win32::Foundation::FILETIME;

use crate::iter::{FabricIter, FabricListAccessor};
use crate::{GUID, WString};

use crate::types::{HealthInformation, HealthState};

/// FABRIC_HEALTH_REPORT
#[derive(Debug, Clone)]
pub enum HealthReport {
    Invalid,
    StatefulServiceReplica(StatefulServiceReplicaHealthReport),
    StatelessServiceInstance(StatelessServiceInstanceHealthReport),
    Partition(PartitionHealthReport),
    Node(NodeHealthReport),
    Service(ServiceHealthReport),
    Application(ApplicationHealthReport),
    DeployedApplication(DeployedApplicationHealthReport),
    DeployedServicePackage(DeployedServicePackageHealthReport),
    Cluster(ClusterHealthReport),
}

/// FABRIC_STATEFUL_SERVICE_REPLICA_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct StatefulServiceReplicaHealthReport {
    pub partition_id: GUID,
    pub replica_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_STATELESS_SERVICE_INSTANCE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct StatelessServiceInstanceHealthReport {
    pub partition_id: GUID,
    pub instance_id: i64,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_PARTITION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct PartitionHealthReport {
    pub partition_id: GUID,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_NODE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct NodeHealthReport {
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_SERVICE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ServiceHealthReport {
    pub service_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ApplicationHealthReport {
    pub application_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_DEPLOYED_APPLICATION_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedApplicationHealthReport {
    pub application_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct DeployedServicePackageHealthReport {
    pub application_name: WString,
    pub service_manifest_name: WString,
    pub node_name: WString,
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_CLUSTER_HEALTH_REPORT
#[derive(Debug, Clone)]
pub struct ClusterHealthReport {
    pub health_information: HealthInformation,
    // TODO: Implement reserved fields
    // pub reserved: *mut std::ffi::c_void,
}

/// FABRIC_CLUSTER_HEALTH_POLICY
#[derive(Debug, Clone)]
pub struct ClusterHealthPolicy {
    pub consider_warning_as_error: bool,
    pub max_percent_unhealthy_nodes: u8,
    pub max_percent_unhealthy_applications: u8,
}

/// FABRIC_HEALTH_EVENTS_FILTER
#[derive(Debug, Clone)]
pub struct HealthEventsFilter {
    pub health_state_filter: HealthStateFilterFlags, // FABRIC_HEALTH_STATE_FILTER
}

bitflags::bitflags! {
    /// bitflag of FABRIC_HEALTH_STATE_FILTER
    #[derive(Debug, Clone)]
    pub struct HealthStateFilterFlags: i32{
        const ALL = mssf_com::FabricTypes::FABRIC_HEALTH_STATE_FILTER_ALL.0;
        const ERROR = mssf_com::FabricTypes::FABRIC_HEALTH_STATE_FILTER_ERROR.0;
        const WARNING = mssf_com::FabricTypes::FABRIC_HEALTH_STATE_FILTER_WARNING.0;
        const OK = mssf_com::FabricTypes::FABRIC_HEALTH_STATE_FILTER_OK.0;
        const NONE = mssf_com::FabricTypes::FABRIC_HEALTH_STATE_FILTER_NONE.0;
    }
}

/// FABRIC_NODE_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct NodeHealthQueryDescription {
    /// Node name is required.
    pub node_name: WString,
    pub health_policy: Option<ClusterHealthPolicy>,
    pub events_filter: Option<HealthEventsFilter>,
}

/// IFabricNodeHealthResult and FABRIC_NODE_HEALTH
pub struct NodeHealthResult {
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<crate::types::HealthEvent>,
}

impl NodeHealthResult {
    pub fn from_com(com: &IFabricNodeHealthResult) -> Self {
        let raw = unsafe { com.get_NodeHealth().as_ref().unwrap() };
        Self {
            node_name: WString::from(raw.NodeName),
            aggregated_health_state: (&raw.AggregatedHealthState).into(),
            health_events: HealthEventList { com: com.clone() }.iter().collect(),
        }
    }
}

/// FABRIC_HEALTH_EVENT
pub struct HealthEvent {
    pub health_information: HealthInformation,
    pub source_utc_timestamp: FILETIME,
    pub last_modified_utc_timestamp: FILETIME,
    pub is_expired: bool,
}

impl From<&FABRIC_HEALTH_EVENT> for HealthEvent {
    fn from(value: &FABRIC_HEALTH_EVENT) -> Self {
        Self {
            health_information: unsafe { value.HealthInformation.as_ref().unwrap().into() },
            source_utc_timestamp: value.SourceUtcTimestamp,
            last_modified_utc_timestamp: value.LastModifiedUtcTimestamp,
            is_expired: value.IsExpired,
        }
    }
}

// FABRIC_HEALTH_EVENT_LIST
pub struct HealthEventList {
    com: IFabricNodeHealthResult,
}

type HealthEventListIter<'a> = FabricIter<'a, FABRIC_HEALTH_EVENT, HealthEvent, HealthEventList>;

impl HealthEventList {
    fn get_raw_list(&self) -> Option<&FABRIC_HEALTH_EVENT_LIST> {
        unsafe { self.com.get_NodeHealth().as_ref()?.HealthEvents.as_ref() }
    }
    pub fn iter(&self) -> HealthEventListIter<'_> {
        FabricIter::new(self, self)
    }
}
impl FabricListAccessor<FABRIC_HEALTH_EVENT> for HealthEventList {
    fn get_count(&self) -> u32 {
        let raw = self.get_raw_list();
        raw.map(|r| r.Count).unwrap_or(0)
    }

    fn get_first_item(&self) -> *const FABRIC_HEALTH_EVENT {
        let raw = self.get_raw_list();
        raw.map(|r| r.Items).unwrap_or(std::ptr::null())
    }
}
