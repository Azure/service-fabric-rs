use std::{ffi::c_void, time::Duration};

use crate::client::IFabricQueryClient10Wrap;
use bitflags::bitflags;
use mssf_com::{
    FabricCommon::FabricClient::{IFabricGetNodeListResult, IFabricQueryClient10},
    FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION_EX1,
    FABRIC_NODE_QUERY_DESCRIPTION_EX2, FABRIC_NODE_QUERY_RESULT_ITEM,
    FABRIC_QUERY_NODE_STATUS_FILTER_ALL, FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT,
    FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED, FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING,
    FABRIC_QUERY_NODE_STATUS_FILTER_DOWN, FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING,
    FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED, FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN,
    FABRIC_QUERY_NODE_STATUS_FILTER_UP,
};
use windows_core::{HSTRING, PCWSTR};

pub struct QueryClient {
    _com: IFabricQueryClient10,
    gen_wrap: IFabricQueryClient10Wrap,
}

impl QueryClient {
    pub fn from_com(com: IFabricQueryClient10) -> Self {
        Self {
            _com: com.clone(),
            gen_wrap: IFabricQueryClient10Wrap::from_com(com),
        }
    }

    // List nodes in the cluster
    pub async fn get_node_list(
        &self,
        desc: &NodeQueryDescription,
        timeout: Duration,
    ) -> windows_core::Result<NodeList> {
        let fu;
        {
            let ex2 = FABRIC_NODE_QUERY_DESCRIPTION_EX2 {
                NodeStatusFilter: desc.node_status_filter.bits(),
                Reserved: std::ptr::null_mut(),
            };

            let ex1 = FABRIC_NODE_QUERY_DESCRIPTION_EX1 {
                ContinuationToken: get_pcwstr_from_opt(&desc.continuation_token),
                Reserved: std::ptr::addr_of!(ex2) as *mut c_void,
            };

            let arg = FABRIC_NODE_QUERY_DESCRIPTION {
                NodeNameFilter: get_pcwstr_from_opt(&desc.node_name_filter),
                Reserved: std::ptr::addr_of!(ex1) as *mut c_void,
            };
            fu = self
                .gen_wrap
                .GetNodeList(&arg, timeout.as_millis().try_into().unwrap());
        }
        let res = fu.await?;
        Ok(NodeList::from_com(res))
    }
}

// note that hstring must be valid for pcwstr lifetime
fn get_pcwstr_from_opt(opt: &Option<HSTRING>) -> PCWSTR {
    match opt {
        Some(x) => PCWSTR(x.as_ptr()),
        None => PCWSTR::null(),
    }
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
    pub continuation_token: Option<HSTRING>,
    pub node_status_filter: NodeStatusFilter,
}

pub struct NodeList {
    com: IFabricGetNodeListResult,
}

impl NodeList {
    fn from_com(com: IFabricGetNodeListResult) -> Self {
        Self { com }
    }
    pub fn iter(&self) -> NodeListIter {
        NodeListIter::new(self.com.clone())
    }
}

pub struct NodeListIter {
    _owner: IFabricGetNodeListResult,
    count: u32, // total
    index: u32,
    curr: *const FABRIC_NODE_QUERY_RESULT_ITEM,
}

#[derive(Debug)]
pub struct Node {
    pub name: HSTRING,
}

impl Iterator for NodeListIter {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        // get the curr out
        let raw = unsafe { self.curr.as_ref().unwrap() };
        let res = Node {
            name: HSTRING::from_wide(unsafe { raw.NodeName.as_wide() }).unwrap(),
        };
        self.index += 1;
        self.curr = unsafe { self.curr.offset(1) };
        Some(res)
    }
}

impl NodeListIter {
    fn new(com: IFabricGetNodeListResult) -> Self {
        let list = unsafe { com.get_NodeList().as_ref().unwrap() };

        let count = list.Count;
        let item = list.Items;
        Self {
            _owner: com,
            count,
            index: 0,
            curr: item,
        }
    }
}
