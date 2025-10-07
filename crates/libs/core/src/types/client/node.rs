// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{WString, types::Uri};
use bitflags::bitflags;
use mssf_com::{
    FabricClient::IFabricGetNodeListResult2,
    FabricTypes::{
        FABRIC_NODE_ID, FABRIC_NODE_QUERY_RESULT_ITEM, FABRIC_NODE_QUERY_RESULT_ITEM_EX1,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX2, FABRIC_PAGING_STATUS,
        FABRIC_QUERY_NODE_STATUS_FILTER_ALL, FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT,
        FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED, FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_DOWN, FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED, FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN,
        FABRIC_QUERY_NODE_STATUS_FILTER_UP,
    },
};

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
    // pub node_status
    pub node_up_time_in_seconds: i64,
    // pub AggregatedHealthState
    pub is_seed_node: bool,
    pub upgrade_domain: WString,
    pub fault_domain: Uri,
    pub node_instance_id: u64,
}

impl From<&FABRIC_NODE_QUERY_RESULT_ITEM> for NodeQueryResultItem {
    fn from(value: &FABRIC_NODE_QUERY_RESULT_ITEM) -> Self {
        let raw = value;
        // TODO: get node id. integrate with another PR
        let raw1 = unsafe {
            (raw.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX1)
                .as_ref()
                .unwrap()
        };
        let raw2 = unsafe {
            (raw1.Reserved as *const FABRIC_NODE_QUERY_RESULT_ITEM_EX2)
                .as_ref()
                .unwrap()
        };
        NodeQueryResultItem {
            name: WString::from(raw.NodeName),
            ip_address_or_fqdn: WString::from(raw.IpAddressOrFQDN),
            node_type: WString::from(raw.NodeType),
            code_version: WString::from(raw.CodeVersion),
            config_version: WString::from(raw.ConfigVersion),
            node_up_time_in_seconds: raw.NodeUpTimeInSeconds,
            is_seed_node: raw.IsSeedNode,
            upgrade_domain: WString::from(raw.UpgradeDomain),
            fault_domain: Uri::from(raw.FaultDomain),
            node_instance_id: raw2.NodeInstanceId,
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
