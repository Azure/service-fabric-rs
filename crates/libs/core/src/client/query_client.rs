use std::{ffi::c_void, time::Duration};

use crate::{client::IFabricQueryClient10Wrap, unsafe_pwstr_to_hstring};
use bitflags::bitflags;
use mssf_com::{
    FabricCommon::FabricClient::{IFabricGetNodeListResult2, IFabricQueryClient10},
    FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION_EX1,
    FABRIC_NODE_QUERY_DESCRIPTION_EX2, FABRIC_NODE_QUERY_DESCRIPTION_EX3,
    FABRIC_NODE_QUERY_RESULT_ITEM, FABRIC_NODE_QUERY_RESULT_ITEM_EX1,
    FABRIC_NODE_QUERY_RESULT_ITEM_EX2, FABRIC_PAGING_STATUS, FABRIC_QUERY_NODE_STATUS_FILTER_ALL,
    FABRIC_QUERY_NODE_STATUS_FILTER_DEFAULT, FABRIC_QUERY_NODE_STATUS_FILTER_DISABLED,
    FABRIC_QUERY_NODE_STATUS_FILTER_DISABLING, FABRIC_QUERY_NODE_STATUS_FILTER_DOWN,
    FABRIC_QUERY_NODE_STATUS_FILTER_ENABLING, FABRIC_QUERY_NODE_STATUS_FILTER_REMOVED,
    FABRIC_QUERY_NODE_STATUS_FILTER_UNKNOWN, FABRIC_QUERY_NODE_STATUS_FILTER_UP,
};
use windows_core::{HSTRING, PCWSTR};

pub struct QueryClient {
    com: IFabricQueryClient10,
    _gen_wrap: IFabricQueryClient10Wrap,
}

impl QueryClient {
    pub fn from_com(com: IFabricQueryClient10) -> Self {
        Self {
            com: com.clone(),
            _gen_wrap: IFabricQueryClient10Wrap::from_com(com),
        }
    }

    // manually wrapping the com call since this is a irregular api.
    pub fn get_node_list_internal(
        &self,
        queryDescription: &FABRIC_NODE_QUERY_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<IFabricGetNodeListResult2>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            // Note we use the v2 api
            let res = unsafe { self.com.EndGetNodeList2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetNodeList(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
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
            let ex3 = FABRIC_NODE_QUERY_DESCRIPTION_EX3 {
                MaxResults: match desc.paged_query.max_results {
                    Some(x) => x,
                    None => 0,
                },
                Reserved: std::ptr::null_mut(),
            };

            let ex2 = FABRIC_NODE_QUERY_DESCRIPTION_EX2 {
                NodeStatusFilter: desc.node_status_filter.bits(),
                Reserved: std::ptr::addr_of!(ex3) as *mut c_void,
            };

            let ex1 = FABRIC_NODE_QUERY_DESCRIPTION_EX1 {
                ContinuationToken: get_pcwstr_from_opt(&desc.paged_query.continuation_token),
                Reserved: std::ptr::addr_of!(ex2) as *mut c_void,
            };

            let arg = FABRIC_NODE_QUERY_DESCRIPTION {
                NodeNameFilter: get_pcwstr_from_opt(&desc.node_name_filter),
                Reserved: std::ptr::addr_of!(ex1) as *mut c_void,
            };
            fu = self.get_node_list_internal(&arg, timeout.as_millis().try_into().unwrap());
        }
        let res = fu.await?;
        Ok(NodeList::from_com(res))
    }
}

pub struct PagingStatus {
    pub continuation_token: HSTRING,
}

impl From<&FABRIC_PAGING_STATUS> for PagingStatus {
    fn from(value: &FABRIC_PAGING_STATUS) -> Self {
        Self {
            continuation_token: unsafe_pwstr_to_hstring(value.ContinuationToken),
        }
    }
}

#[derive(Default, Debug)]
pub struct PagedQueryDescription {
    pub continuation_token: Option<HSTRING>,
    pub max_results: Option<i32>,
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
    pub node_status_filter: NodeStatusFilter,
    pub paged_query: PagedQueryDescription,
}

pub struct NodeList {
    com: IFabricGetNodeListResult2,
}

impl NodeList {
    fn from_com(com: IFabricGetNodeListResult2) -> Self {
        Self { com }
    }
    pub fn iter(&self) -> NodeListIter {
        NodeListIter::new(self.com.clone())
    }
    pub fn get_paging_status(&self) -> PagingStatus {
        let raw = unsafe { self.com.get_PagingStatus().as_ref().unwrap() };
        raw.into()
    }
}

pub struct NodeListIter {
    _owner: IFabricGetNodeListResult2,
    count: u32, // total
    index: u32,
    curr: *const FABRIC_NODE_QUERY_RESULT_ITEM,
}

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

impl Iterator for NodeListIter {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        // get the curr out
        let raw = unsafe { self.curr.as_ref().unwrap() };
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
        let res = Node {
            name: HSTRING::from_wide(unsafe { raw.NodeName.as_wide() }).unwrap(),
            ip_address_or_fqdn: unsafe_pwstr_to_hstring(raw.IpAddressOrFQDN),
            node_type: unsafe_pwstr_to_hstring(raw.NodeType),
            code_version: unsafe_pwstr_to_hstring(raw.CodeVersion),
            config_version: unsafe_pwstr_to_hstring(raw.ConfigVersion),
            node_up_time_in_seconds: raw.NodeUpTimeInSeconds,
            is_seed_node: raw.IsSeedNode.as_bool(),
            upgrade_domain: unsafe_pwstr_to_hstring(raw.UpgradeDomain),
            fault_domain: unsafe_pwstr_to_hstring(windows_core::PCWSTR(raw.FaultDomain)),
            node_instance_id: raw2.NodeInstanceId,
        };
        self.index += 1;
        self.curr = unsafe { self.curr.offset(1) };
        Some(res)
    }
}

impl NodeListIter {
    fn new(com: IFabricGetNodeListResult2) -> Self {
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
