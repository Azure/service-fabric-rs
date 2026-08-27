// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::mem::{BoxPool, GetRawWithBoxPool};
use crate::time::filetime_to_system_time;
use crate::types::HealthState;
use crate::{WString, types::Uri};
use bitflags::bitflags;
use mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS;
use mssf_com::{
    FabricClient::IFabricGetNodeListResult2,
    FabricTypes::{
        FABRIC_NODE_ID, FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION_EX1,
        FABRIC_NODE_QUERY_DESCRIPTION_EX2, FABRIC_NODE_QUERY_DESCRIPTION_EX3,
        FABRIC_NODE_QUERY_RESULT_ITEM, FABRIC_NODE_QUERY_RESULT_ITEM_EX1,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX2, FABRIC_NODE_QUERY_RESULT_ITEM_EX3,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX4, FABRIC_NODE_QUERY_RESULT_ITEM_EX5,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX6, FABRIC_NODE_QUERY_RESULT_ITEM_EX7,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX8, FABRIC_NODE_QUERY_RESULT_ITEM_EX9, FABRIC_PAGING_STATUS,
        FABRIC_QUERY_NODE_STATUS_FILTER_ALL, FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT,
        FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED, FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_DOWN, FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED, FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN,
        FABRIC_QUERY_NODE_STATUS_FILTER_UP,
    },
};
use std::ffi::c_void;
use std::time::SystemTime;

#[derive(Debug, Default, Clone)]
pub struct PagingStatus {
    pub continuation_token: WString,
}

impl From<&FABRIC_PAGING_STATUS> for PagingStatus {
    fn from(value: &FABRIC_PAGING_STATUS) -> Self {
        Self {
            continuation_token: WString::from(value.ContinuationToken),
        }
    }
}

#[derive(Default, Debug)]
pub struct PagedQueryDescription {
    pub continuation_token: Option<WString>,
    pub max_results: Option<i32>,
}

bitflags! {
    #[derive(Debug)]
    pub struct NodeStatusFilter: u32{
        const All = FABRIC_QUERY_NODE_STATUS_FILTER_ALL.0 as u32;
        const Default = FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT.0 as u32;
        const Disabled = FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED.0 as u32;
        const Disabling = FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING.0 as u32;
        const Down = FABRIC_QUERY_NODE_STATUS_FILTER_DOWN.0 as u32;
        const Enabling = FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING.0 as u32;
        const Removed = FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED.0 as u32;
        const Unknown = FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN.0 as u32;
        const Up = FABRIC_QUERY_NODE_STATUS_FILTER_UP.0 as u32;
    }
}

impl Default for NodeStatusFilter {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Default, Debug)]
pub struct NodeQueryDescription {
    pub node_name_filter: Option<WString>,
    pub node_status_filter: NodeStatusFilter,
    pub paged_query: PagedQueryDescription,
}

impl GetRawWithBoxPool<FABRIC_NODE_QUERY_DESCRIPTION> for NodeQueryDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_NODE_QUERY_DESCRIPTION {
        let ex3 = pool.push(Box::new(FABRIC_NODE_QUERY_DESCRIPTION_EX3 {
            MaxResults: self.paged_query.max_results.unwrap_or(0),
            Reserved: std::ptr::null_mut(),
        }));
        let ex2 = pool.push(Box::new(FABRIC_NODE_QUERY_DESCRIPTION_EX2 {
            NodeStatusFilter: self.node_status_filter.bits(),
            Reserved: ex3 as *const _ as *mut c_void,
        }));
        let ex1 = pool.push(Box::new(FABRIC_NODE_QUERY_DESCRIPTION_EX1 {
            ContinuationToken: self.paged_query.continuation_token.as_ref().into(),
            Reserved: ex2 as *const _ as *mut c_void,
        }));
        FABRIC_NODE_QUERY_DESCRIPTION {
            NodeNameFilter: self.node_name_filter.as_ref().into(),
            Reserved: ex1 as *const _ as *mut c_void,
        }
    }
}

#[derive(Debug)]
pub struct NodeListResult {
    pub paging_status: Option<PagingStatus>,
    pub nodes: Vec<NodeQueryResultItem>,
}

impl From<&IFabricGetNodeListResult2> for NodeListResult {
    fn from(com: &IFabricGetNodeListResult2) -> Self {
        let paging_status = unsafe { com.get_PagingStatus().as_ref() }.map(|s| s.into());
        let nodes = unsafe { com.get_NodeList().as_ref() }
            .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
            .unwrap_or_default();
        Self {
            paging_status,
            nodes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeQueryResultItem {
    pub name: WString,
    pub ip_address_or_fqdn: WString,
    pub node_type: WString,
    pub code_version: WString,
    pub config_version: WString,
    pub status: NodeStatus,
    pub node_up_time_in_seconds: i64,
    pub health_state: HealthState,
    pub is_seed_node: bool,
    pub upgrade_domain: WString,
    pub fault_domain: Uri,
    // ex1
    pub node_id: NodeId,
    // ex2
    pub node_instance_id: u64,
    // ex3
    // TODO: NodeDeactivationInfo
    // ex4
    pub is_stopped: bool,
    // ex5
    pub node_down_time_in_seconds: i64,
    // ex6
    pub node_up_at: SystemTime,
    pub node_down_at: SystemTime,
    // ex7
    pub infrastructure_placement_id: WString,
    // ex8
    // TODO: NodeTags
    // ex9
    pub is_node_by_node_upgrade_in_progress: bool,
}

impl From<&FABRIC_NODE_QUERY_RESULT_ITEM> for NodeQueryResultItem {
    fn from(value: &FABRIC_NODE_QUERY_RESULT_ITEM) -> Self {
        let raw = value;
        // TODO: get node id. integrate with another PR
        let ex1 = unsafe {
            (raw.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX1)
                .as_ref()
                .unwrap()
        };
        let ex2 = unsafe {
            (ex1.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX2)
                .as_ref()
                .unwrap()
        };
        let ex3 =
            unsafe { (ex2.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX3).as_ref() }.unwrap();
        let ex4 =
            unsafe { (ex3.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX4).as_ref() }.unwrap();
        let ex5 =
            unsafe { (ex4.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX5).as_ref() }.unwrap();
        let ex6 =
            unsafe { (ex5.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX6).as_ref() }.unwrap();
        let ex7 =
            unsafe { (ex6.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX7).as_ref() }.unwrap();
        let ex8 =
            unsafe { (ex7.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX8).as_ref() }.unwrap();
        let ex9 =
            unsafe { (ex8.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX9).as_ref() }.unwrap();
        NodeQueryResultItem {
            name: WString::from(raw.NodeName),
            ip_address_or_fqdn: WString::from(raw.IpAddressOrFQDN),
            node_type: WString::from(raw.NodeType),
            code_version: WString::from(raw.CodeVersion),
            config_version: WString::from(raw.ConfigVersion),
            status: value.NodeStatus.into(),
            node_up_time_in_seconds: raw.NodeUpTimeInSeconds,
            health_state: (&value.AggregatedHealthState).into(),
            is_seed_node: raw.IsSeedNode,
            upgrade_domain: WString::from(raw.UpgradeDomain),
            fault_domain: Uri::from(raw.FaultDomain),
            node_id: ex1.NodeId.into(),
            node_instance_id: ex2.NodeInstanceId,
            is_stopped: ex4.IsStopped,
            node_down_time_in_seconds: ex5.NodeDownTimeInSeconds,
            node_up_at: filetime_to_system_time(ex6.NodeUpAt).unwrap_or(SystemTime::UNIX_EPOCH),
            node_down_at: filetime_to_system_time(ex6.NodeDownAt).unwrap_or(SystemTime::UNIX_EPOCH),
            infrastructure_placement_id: WString::from(ex7.InfrastructurePlacementID),
            is_node_by_node_upgrade_in_progress: ex9.IsNodeByNodeUpgradeInProgress,
        }
    }
}

// FABRIC_NODE_ID
#[derive(Debug, Clone)]
pub struct NodeId {
    pub low: u64,
    pub high: u64,
}

impl From<FABRIC_NODE_ID> for NodeId {
    fn from(value: FABRIC_NODE_ID) -> Self {
        Self {
            low: value.Low,
            high: value.High,
        }
    }
}

// FABRIC_QUERY_NODE_STATUS
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum NodeStatus {
    Up = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_UP.0,
    Down = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DOWN.0,
    Enabling = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_ENABLING.0,
    Disabled = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DISABLED.0,
    Disabling = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DISABLING.0,
    Removed = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_REMOVED.0,
    Unknown = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_UNKNOWN.0,
    Invalid = mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_INVALID.0,
}
impl From<FABRIC_QUERY_NODE_STATUS> for NodeStatus {
    fn from(value: FABRIC_QUERY_NODE_STATUS) -> Self {
        match value {
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_UP => NodeStatus::Up,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DOWN => NodeStatus::Down,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_ENABLING => NodeStatus::Enabling,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DISABLED => NodeStatus::Disabled,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_DISABLING => NodeStatus::Disabling,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_REMOVED => NodeStatus::Removed,
            mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_UNKNOWN => NodeStatus::Unknown,
            _ => NodeStatus::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use windows_core::Win32::Foundation::FILETIME;

    use super::*;

    #[test]
    fn test_node_query_result_item_from_raw() {
        let node_name = WString::from("Node1");
        let ip_address = WString::from("127.0.0.1");
        let node_type = WString::from("NodeType1");
        let code_version = WString::from("1.0.0");
        let config_version = WString::from("2.0.0");
        let upgrade_domain = WString::from("UD1");
        let fault_domain = Uri::from("fabric:/FD1");
        let infra_placement_id = WString::from("infra-1");

        let ex9 = FABRIC_NODE_QUERY_RESULT_ITEM_EX9 {
            IsNodeByNodeUpgradeInProgress: true,
            Reserved: std::ptr::null_mut(),
        };
        let ex8 = FABRIC_NODE_QUERY_RESULT_ITEM_EX8 {
            NodeTags: std::ptr::null_mut(),
            Reserved: std::ptr::addr_of!(ex9) as *mut c_void,
        };
        let ex7 = FABRIC_NODE_QUERY_RESULT_ITEM_EX7 {
            InfrastructurePlacementID: infra_placement_id.as_pcwstr(),
            Reserved: std::ptr::addr_of!(ex8) as *mut c_void,
        };
        let ex6 = FABRIC_NODE_QUERY_RESULT_ITEM_EX6 {
            NodeUpAt: FILETIME {
                dwLowDateTime: 100,
                dwHighDateTime: 200,
            },
            NodeDownAt: FILETIME {
                dwLowDateTime: 300,
                dwHighDateTime: 400,
            },
            Reserved: std::ptr::addr_of!(ex7) as *mut c_void,
        };
        let ex5 = FABRIC_NODE_QUERY_RESULT_ITEM_EX5 {
            NodeDownTimeInSeconds: 123,
            Reserved: std::ptr::addr_of!(ex6) as *mut c_void,
        };
        let ex4 = FABRIC_NODE_QUERY_RESULT_ITEM_EX4 {
            IsStopped: true,
            Reserved: std::ptr::addr_of!(ex5) as *mut c_void,
        };
        let ex3 = FABRIC_NODE_QUERY_RESULT_ITEM_EX3 {
            NodeDeactivationInfo: std::ptr::null(),
            Reserved: std::ptr::addr_of!(ex4) as *mut c_void,
        };
        let ex2 = FABRIC_NODE_QUERY_RESULT_ITEM_EX2 {
            NodeInstanceId: 42,
            Reserved: std::ptr::addr_of!(ex3) as *mut c_void,
        };
        let ex1 = FABRIC_NODE_QUERY_RESULT_ITEM_EX1 {
            NodeId: FABRIC_NODE_ID {
                Low: 1,
                High: 2,
                Reserved: std::ptr::null_mut(),
            },
            Reserved: std::ptr::addr_of!(ex2) as *mut c_void,
        };
        let raw = FABRIC_NODE_QUERY_RESULT_ITEM {
            NodeName: node_name.as_pcwstr(),
            IpAddressOrFQDN: ip_address.as_pcwstr(),
            NodeType: node_type.as_pcwstr(),
            CodeVersion: code_version.as_pcwstr(),
            ConfigVersion: config_version.as_pcwstr(),
            NodeStatus: mssf_com::FabricTypes::FABRIC_QUERY_NODE_STATUS_UP,
            NodeUpTimeInSeconds: 555,
            AggregatedHealthState: mssf_com::FabricTypes::FABRIC_HEALTH_STATE_OK,
            IsSeedNode: true,
            UpgradeDomain: upgrade_domain.as_pcwstr(),
            FaultDomain: fault_domain.as_raw(),
            Reserved: std::ptr::addr_of!(ex1) as *mut c_void,
        };

        let item = NodeQueryResultItem::from(&raw);
        assert_eq!(item.name.to_string_lossy(), "Node1");
        assert_eq!(item.ip_address_or_fqdn.to_string_lossy(), "127.0.0.1");
        assert_eq!(item.node_type.to_string_lossy(), "NodeType1");
        assert_eq!(item.code_version.to_string_lossy(), "1.0.0");
        assert_eq!(item.config_version.to_string_lossy(), "2.0.0");
        assert!(matches!(item.status, NodeStatus::Up));
        assert_eq!(item.node_up_time_in_seconds, 555);
        assert_eq!(item.health_state, HealthState::Ok);
        assert!(item.is_seed_node);
        assert_eq!(item.upgrade_domain.to_string_lossy(), "UD1");
        assert_eq!(item.fault_domain.to_string(), "fabric:/FD1");
        assert_eq!(item.node_id.low, 1);
        assert_eq!(item.node_id.high, 2);
        assert_eq!(item.node_instance_id, 42);
        assert!(item.is_stopped);
        assert_eq!(item.node_down_time_in_seconds, 123);
        assert_eq!(
            item.node_up_at,
            filetime_to_system_time(FILETIME {
                dwLowDateTime: 100,
                dwHighDateTime: 200,
            })
            .unwrap()
        );
        assert_eq!(
            item.node_down_at,
            filetime_to_system_time(FILETIME {
                dwLowDateTime: 300,
                dwHighDateTime: 400,
            })
            .unwrap()
        );
        assert_eq!(
            item.infrastructure_placement_id.to_string_lossy(),
            "infra-1"
        );
        assert!(item.is_node_by_node_upgrade_in_progress);
    }
}
