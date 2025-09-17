// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::{IFabricClusterHealthResult, IFabricNodeHealthResult};
use mssf_com::FabricTypes::{
    FABRIC_APPLICATION_HEALTH_POLICY, FABRIC_APPLICATION_HEALTH_POLICY_MAP,
    FABRIC_APPLICATION_HEALTH_POLICY_MAP_ITEM, FABRIC_APPLICATION_HEALTH_STATES_FILTER,
    FABRIC_CLUSTER_HEALTH, FABRIC_CLUSTER_HEALTH_POLICY, FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION,
    FABRIC_HEALTH_EVENT, FABRIC_HEALTH_EVENTS_FILTER, FABRIC_NODE_HEALTH_STATES_FILTER,
    FABRIC_SERVICE_TYPE_HEALTH_POLICY, FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP,
    FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP_ITEM,
};
use windows_core::Win32::Foundation::FILETIME;

use crate::mem::{BoxPool, GetRaw, GetRawWithBoxPool};
use crate::{GUID, WString};

use crate::types::{HealthInformation, HealthState, Uri};

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

impl GetRaw<FABRIC_HEALTH_EVENTS_FILTER> for HealthEventsFilter {
    fn get_raw(&self) -> FABRIC_HEALTH_EVENTS_FILTER {
        FABRIC_HEALTH_EVENTS_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
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
#[derive(Debug, Clone)]
pub struct NodeHealthResult {
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<crate::types::HealthEvent>,
}

impl NodeHealthResult {
    pub fn from_com(com: &IFabricNodeHealthResult) -> Self {
        let raw = unsafe { com.get_NodeHealth().as_ref().unwrap() };
        let health_event_list = unsafe { raw.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        Self {
            node_name: WString::from(raw.NodeName),
            aggregated_health_state: (&raw.AggregatedHealthState).into(),
            health_events: health_event_list,
        }
    }
}

/// FABRIC_HEALTH_EVENT
#[derive(Debug, Clone)]
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

// FABRIC_NODE_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct NodeHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags, // FABRIC_HEALTH_STATE_FILTER
}

impl GetRaw<FABRIC_NODE_HEALTH_STATES_FILTER> for NodeHealthStatesFilter {
    fn get_raw(&self) -> FABRIC_NODE_HEALTH_STATES_FILTER {
        FABRIC_NODE_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_NODE_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct NodeHealthState {
    pub node_name: WString,
    pub aggregated_health_state: HealthState,
}

impl From<&mssf_com::FabricTypes::FABRIC_NODE_HEALTH_STATE> for NodeHealthState {
    fn from(value: &mssf_com::FabricTypes::FABRIC_NODE_HEALTH_STATE) -> Self {
        Self {
            node_name: WString::from(value.NodeName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
        }
    }
}

// FABRIC_APPLICATION_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct ApplicationHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags, // FABRIC_HEALTH_STATE_FILTER
}

impl GetRaw<FABRIC_APPLICATION_HEALTH_STATES_FILTER> for ApplicationHealthStatesFilter {
    fn get_raw(&self) -> FABRIC_APPLICATION_HEALTH_STATES_FILTER {
        FABRIC_APPLICATION_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_APPLICATION_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct ApplicationHealthState {
    pub application_name: Uri,
    pub aggregated_health_state: HealthState,
}

impl From<&mssf_com::FabricTypes::FABRIC_APPLICATION_HEALTH_STATE> for ApplicationHealthState {
    fn from(value: &mssf_com::FabricTypes::FABRIC_APPLICATION_HEALTH_STATE) -> Self {
        Self {
            application_name: Uri::from(value.ApplicationName),
            aggregated_health_state: (&value.AggregatedHealthState).into(),
        }
    }
}

// FABRIC_SERVICE_TYPE_HEALTH_POLICY
#[derive(Debug, Clone, Default)]
pub struct ServiceTypeHealthPolicy {
    pub max_percent_unhealthy_services: u8,
    pub max_percent_unhealthy_partitions_per_service: u8,
    pub max_percent_unhealthy_replicas_per_partition: u8,
}
impl GetRaw<FABRIC_SERVICE_TYPE_HEALTH_POLICY> for ServiceTypeHealthPolicy {
    fn get_raw(&self) -> FABRIC_SERVICE_TYPE_HEALTH_POLICY {
        FABRIC_SERVICE_TYPE_HEALTH_POLICY {
            MaxPercentUnhealthyServices: self.max_percent_unhealthy_services,
            MaxPercentUnhealthyPartitionsPerService: self
                .max_percent_unhealthy_partitions_per_service,
            MaxPercentUnhealthyReplicasPerPartition: self
                .max_percent_unhealthy_replicas_per_partition,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_APPLICATION_HEALTH_POLICY
#[derive(Debug, Clone, Default)]
pub struct ApplicationHealthPolicy {
    pub consider_warning_as_error: bool,
    pub max_percent_unhealthy_partitions: u8,
    pub default_service_type_health_policy: Option<ServiceTypeHealthPolicy>,
    pub service_type_health_policy_map:
        Option<std::collections::HashMap<WString, ServiceTypeHealthPolicy>>,
}

impl GetRawWithBoxPool<FABRIC_APPLICATION_HEALTH_POLICY> for ApplicationHealthPolicy {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_APPLICATION_HEALTH_POLICY {
        let default_service_type_health_policy = self
            .default_service_type_health_policy
            .as_ref()
            .map(|policy| pool.push(Box::new(policy.get_raw())));
        // build the policy map FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP
        let service_type_health_policy_map =
            self.service_type_health_policy_map
                .as_ref()
                .map(|policies| {
                    let mut items = Vec::new();
                    for (name, policy) in policies {
                        let raw_svc_tp_policy = pool.push(Box::new(policy.get_raw()));
                        items.push(FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP_ITEM {
                            ServiceTypeName: name.as_pcwstr(),
                            ServiceTypeHealthPolicy: raw_svc_tp_policy,
                        });
                    }
                    let (count, items) = pool.push_vec(items);
                    // save the items on heap_holder to extend the lifetime
                    pool.push(Box::new(FABRIC_SERVICE_TYPE_HEALTH_POLICY_MAP {
                        Count: count as u32,
                        Items: items as *mut _,
                    }))
                });
        FABRIC_APPLICATION_HEALTH_POLICY {
            ConsiderWarningAsError: self.consider_warning_as_error,
            DefaultServiceTypeHealthPolicy: default_service_type_health_policy
                .unwrap_or(std::ptr::null()),
            ServiceTypeHealthPolicyMap: service_type_health_policy_map.unwrap_or(std::ptr::null()),
            MaxPercentUnhealthyDeployedApplications: self.max_percent_unhealthy_partitions,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ClusterHealthQueryDescription {
    pub health_policy: Option<ClusterHealthPolicy>,
    pub application_health_policy_map:
        Option<std::collections::HashMap<Uri, ApplicationHealthPolicy>>,
    pub events_filter: Option<HealthEventsFilter>,
    pub nodes_filter: Option<NodeHealthStatesFilter>,
    pub applications_filter: Option<ApplicationHealthStatesFilter>,
}

impl GetRawWithBoxPool<FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION> for ClusterHealthQueryDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION {
        let health_policy = self.health_policy.as_ref().map(|policy| {
            let raw = Box::new(FABRIC_CLUSTER_HEALTH_POLICY {
                ConsiderWarningAsError: policy.consider_warning_as_error,
                MaxPercentUnhealthyNodes: policy.max_percent_unhealthy_nodes,
                MaxPercentUnhealthyApplications: policy.max_percent_unhealthy_applications,
                Reserved: std::ptr::null_mut(),
            });
            pool.push(raw)
        });
        let application_health_policy_map =
            self.application_health_policy_map.as_ref().map(|policies| {
                let mut items = Vec::new();
                for (name, policy) in policies {
                    let raw_policy = Box::new(policy.get_raw_with_pool(pool));
                    let raw_policy = pool.push(raw_policy);
                    items.push(FABRIC_APPLICATION_HEALTH_POLICY_MAP_ITEM {
                        ApplicationName: name.as_raw(),
                        HealthPolicy: raw_policy,
                    });
                }
                let (count, items) = pool.push_vec(items);
                // save the items on heap_holder to extend the lifetime
                pool.push(Box::new(FABRIC_APPLICATION_HEALTH_POLICY_MAP {
                    Count: count as u32,
                    Items: items as *mut _,
                }))
            });
        FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION {
            HealthPolicy: health_policy.unwrap_or(std::ptr::null()),
            ApplicationHealthPolicyMap: application_health_policy_map.unwrap_or(std::ptr::null()),
            EventsFilter: self
                .events_filter
                .as_ref()
                .map(|f| pool.push(Box::new(f.get_raw())))
                .unwrap_or(std::ptr::null()),
            NodesFilter: self
                .nodes_filter
                .as_ref()
                .map(|f| pool.push(Box::new(f.get_raw())))
                .unwrap_or(std::ptr::null()),
            ApplicationsFilter: self
                .applications_filter
                .as_ref()
                .map(|f| pool.push(Box::new(f.get_raw())))
                .unwrap_or(std::ptr::null()),
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// FABRIC_CLUSTER_HEALTH
#[derive(Debug, Clone)]
pub struct ClusterHealth {
    pub aggregated_health_state: HealthState,
    pub node_health_states: Vec<NodeHealthState>,
    pub application_health_states: Vec<ApplicationHealthState>,
    pub health_events: Vec<HealthEvent>,
    // TODO: health evaluations
    // TODO: health statistics
}

impl From<&FABRIC_CLUSTER_HEALTH> for ClusterHealth {
    fn from(value: &FABRIC_CLUSTER_HEALTH) -> Self {
        let ex1 = unsafe {
            (value.Reserved as *const mssf_com::FabricTypes::FABRIC_CLUSTER_HEALTH_EX1)
                .as_ref()
                .unwrap()
        };
        let ex2 = unsafe {
            (ex1.Reserved as *const mssf_com::FabricTypes::FABRIC_CLUSTER_HEALTH_EX2)
                .as_ref()
                .unwrap()
        };
        let _ex3 = unsafe {
            (ex2.Reserved as *const mssf_com::FabricTypes::FABRIC_CLUSTER_HEALTH_EX3)
                .as_ref()
                .unwrap()
        };
        let node_health_list = unsafe { ex1.NodeHealthStates.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        let application_health_list = unsafe { ex1.ApplicationHealthStates.as_ref() }
            .map_or(vec![], |list| {
                crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
            });
        let health_event_list = unsafe { ex1.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });
        Self {
            aggregated_health_state: (&value.AggregatedHealthState).into(),
            node_health_states: node_health_list,
            application_health_states: application_health_list,
            health_events: health_event_list,
        }
    }
}

impl From<&IFabricClusterHealthResult> for ClusterHealth {
    fn from(value: &IFabricClusterHealthResult) -> Self {
        let raw = unsafe { value.get_ClusterHealth().as_ref().unwrap() };
        raw.into()
    }
}
