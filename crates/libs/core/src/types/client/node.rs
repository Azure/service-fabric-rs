// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::HSTRINGWrap,
};
use bitflags::bitflags;
use mssf_com::{
    FabricClient::IFabricGetNodeListResult2,
    FabricTypes::{
        FABRIC_NODE_QUERY_RESULT_ITEM, FABRIC_NODE_QUERY_RESULT_ITEM_EX1,
        FABRIC_NODE_QUERY_RESULT_ITEM_EX2, FABRIC_PAGING_STATUS,
        FABRIC_QUERY_NODE_STATUS_FILTER_ALL, FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT,
        FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED, FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_DOWN, FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING,
        FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED, FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN,
        FABRIC_QUERY_NODE_STATUS_FILTER_UP,
    },
};
use windows_core::HSTRING;

pub struct PagingStatus {
    pub continuation_token: HSTRING,
}

impl From<&FABRIC_PAGING_STATUS> for PagingStatus {
    fn from(value: &FABRIC_PAGING_STATUS) -> Self {
        Self {
            continuation_token: HSTRINGWrap::from(value.ContinuationToken).into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct PagedQueryDescription {
    pub continuation_token: Option<HSTRING>,
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
    pub node_name_filter: Option<HSTRING>,
    pub node_status_filter: NodeStatusFilter,
    pub paged_query: PagedQueryDescription,
}

pub struct NodeList {
    com: IFabricGetNodeListResult2,
}

impl FabricListAccessor<FABRIC_NODE_QUERY_RESULT_ITEM> for NodeList {
    fn get_count(&self) -> u32 {
        let list = unsafe { self.com.get_NodeList().as_ref().unwrap() };
        list.Count
    }

    fn get_first_item(&self) -> *const FABRIC_NODE_QUERY_RESULT_ITEM {
        let list = unsafe { self.com.get_NodeList().as_ref().unwrap() };
        list.Items
    }
}

impl NodeList {
    pub fn from_com(com: IFabricGetNodeListResult2) -> Self {
        Self { com }
    }
    pub fn iter(&self) -> NodeListIter {
        NodeListIter::new(self, self)
    }
    pub fn get_paging_status(&self) -> Option<PagingStatus> {
        // If there is no more entries there is no paging status returned.
        let raw = unsafe { self.com.get_PagingStatus().as_ref() }?;
        Some(raw.into())
    }
}

type NodeListIter<'a> = FabricIter<'a, FABRIC_NODE_QUERY_RESULT_ITEM, Node, NodeList>;

#[derive(Debug)]
pub struct Node {
    pub name: HSTRING,
    pub ip_address_or_fqdn: HSTRING,
    pub node_type: HSTRING,
    pub code_version: HSTRING,
    pub config_version: HSTRING,
    // pub node_status
    pub node_up_time_in_seconds: i64,
    // pub AggregatedHealthState
    pub is_seed_node: bool,
    pub upgrade_domain: HSTRING,
    pub fault_domain: HSTRING,
    pub node_instance_id: u64,
}

impl From<&FABRIC_NODE_QUERY_RESULT_ITEM> for Node {
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
        Node {
            name: HSTRING::from_wide(unsafe { raw.NodeName.as_wide() }).unwrap(),
            ip_address_or_fqdn: HSTRINGWrap::from(raw.IpAddressOrFQDN).into(),
            node_type: HSTRINGWrap::from(raw.NodeType).into(),
            code_version: HSTRINGWrap::from(raw.CodeVersion).into(),
            config_version: HSTRINGWrap::from(raw.ConfigVersion).into(),
            node_up_time_in_seconds: raw.NodeUpTimeInSeconds,
            is_seed_node: raw.IsSeedNode.as_bool(),
            upgrade_domain: HSTRINGWrap::from(raw.UpgradeDomain).into(),
            fault_domain: HSTRINGWrap::from(windows_core::PCWSTR(raw.FaultDomain.0)).into(),
            node_instance_id: raw2.NodeInstanceId,
        }
    }
}
