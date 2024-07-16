// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{ffi::c_void, time::Duration};

use mssf_com::{
    FabricClient::{
        IFabricGetNodeListResult2, IFabricGetPartitionListResult2, IFabricQueryClient10,
    },
    FabricTypes::{
        FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION_EX1,
        FABRIC_NODE_QUERY_DESCRIPTION_EX2, FABRIC_NODE_QUERY_DESCRIPTION_EX3,
        FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
    },
};

use crate::{
    strings::get_pcwstr_from_opt,
    sync::{self, FabricReceiver},
    types::{
        NodeList, NodeQueryDescription, ServicePartitionList, ServicePartitionQueryDescription,
    },
};

pub struct QueryClient {
    com: IFabricQueryClient10,
}

// Internal implementation block
// Internal functions focuses on changing SF callback to async future,
// while the public apis impl focuses on type conversion.
impl QueryClient {
    pub fn get_node_list_internal(
        &self,
        queryDescription: &FABRIC_NODE_QUERY_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<IFabricGetNodeListResult2>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let com_cp = self.com.clone();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            // Note we use the v2 api
            let res = unsafe { com_cp.EndGetNodeList2(ctx) };
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

    fn get_partition_list_internal(
        &self,
        desc: &FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
    ) -> FabricReceiver<crate::Result<IFabricGetPartitionListResult2>> {
        let (tx, rx) = sync::oneshot_channel();
        let com_cp = self.com.clone();
        let callback = sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { com_cp.EndGetPartitionList2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetPartitionList(desc, timeout_milliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
}

impl QueryClient {
    pub fn from_com(com: IFabricQueryClient10) -> Self {
        Self { com: com.clone() }
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
                MaxResults: desc.paged_query.max_results.unwrap_or(0),
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

    pub async fn get_partition_list(
        &self,
        desc: &ServicePartitionQueryDescription,
        timeout: Duration,
    ) -> crate::Result<ServicePartitionList> {
        let raw: FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION = desc.into();
        let mili = timeout.as_millis() as u32;
        let com = self.get_partition_list_internal(&raw, mili).await?;
        Ok(ServicePartitionList::new(com))
    }
}
