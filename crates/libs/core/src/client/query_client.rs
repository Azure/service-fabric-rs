// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![cfg_attr(
    not(feature = "tokio_async"),
    allow(unused_imports, reason = "code configured out")
)]
use std::{ffi::c_void, time::Duration};

use mssf_com::{
    FabricClient::{
        IFabricGetNodeListResult2, IFabricGetPartitionListResult2,
        IFabricGetPartitionLoadInformationResult, IFabricGetReplicaListResult2,
        IFabricQueryClient10,
    },
    FabricTypes::{
        FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION_EX1,
        FABRIC_NODE_QUERY_DESCRIPTION_EX2, FABRIC_NODE_QUERY_DESCRIPTION_EX3,
        FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION, FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
    },
};

#[cfg(feature = "tokio_async")]
use crate::sync::{fabric_begin_end_proxy2, CancellationToken, FabricReceiver2};
use crate::{
    strings::get_pcwstr_from_opt,
    types::{
        NodeList, NodeQueryDescription, PartitionLoadInformation,
        PartitionLoadInformationQueryDescription, ServicePartitionList,
        ServicePartitionQueryDescription, ServiceReplicaList, ServiceReplicaQueryDescription,
    },
};

#[derive(Debug, Clone)]
pub struct QueryClient {
    com: IFabricQueryClient10,
}

// Internal implementation block
// Internal functions focuses on changing SF callback to async future,
// while the public apis impl focuses on type conversion.
impl QueryClient {
    #[cfg(feature = "tokio_async")]
    pub fn get_node_list_internal(
        &self,
        query_description: &FABRIC_NODE_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<::windows_core::Result<IFabricGetNodeListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();

        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginGetNodeList(query_description, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetNodeList2(ctx) },
            cancellation_token,
        )
    }

    #[cfg(feature = "tokio_async")]
    fn get_partition_list_internal(
        &self,
        desc: &FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<IFabricGetPartitionListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginGetPartitionList(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetPartitionList2(ctx) },
            cancellation_token,
        )
    }

    #[cfg(feature = "tokio_async")]
    fn get_replica_list_internal(
        &self,
        desc: &FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<IFabricGetReplicaListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginGetReplicaList(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetReplicaList2(ctx) },
            cancellation_token,
        )
    }

    #[cfg(feature = "tokio_async")]
    fn get_partition_load_information_internal(
        &self,
        desc: &FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver2<crate::Result<IFabricGetPartitionLoadInformationResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy2(
            move |callback| unsafe {
                com1.BeginGetPartitionLoadInformation(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetPartitionLoadInformation(ctx) },
            cancellation_token,
        )
    }
}

impl QueryClient {
    pub fn get_com(&self) -> IFabricQueryClient10 {
        self.com.clone()
    }

    pub fn from_com(com: IFabricQueryClient10) -> Self {
        Self { com: com.clone() }
    }

    #[cfg(feature = "tokio_async")]
    // List nodes in the cluster
    pub async fn get_node_list(
        &self,
        desc: &NodeQueryDescription,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> windows_core::Result<NodeList> {
        // Note that the SF raw structs are scoped to avoid having them across await points.
        // This makes api Send. All FabricClient api should follow this pattern.
        let com = {
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
            self.get_node_list_internal(
                &arg,
                timeout.as_millis().try_into().unwrap(),
                cancellation_token,
            )
        }
        .await??;
        Ok(NodeList::from_com(com))
    }

    #[cfg(feature = "tokio_async")]
    pub async fn get_partition_list(
        &self,
        desc: &ServicePartitionQueryDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<ServicePartitionList> {
        let com = {
            let raw: FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION = desc.into();
            let mili = timeout.as_millis() as u32;
            self.get_partition_list_internal(&raw, mili, cancellation_token)
        }
        .await??;
        Ok(ServicePartitionList::new(com))
    }

    #[cfg(feature = "tokio_async")]
    pub async fn get_replica_list(
        &self,
        desc: &ServiceReplicaQueryDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<ServiceReplicaList> {
        let com = {
            let raw: FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION = desc.into();
            let mili = timeout.as_millis() as u32;
            self.get_replica_list_internal(&raw, mili, cancellation_token)
        }
        .await??;
        Ok(ServiceReplicaList::new(com))
    }

    #[cfg(feature = "tokio_async")]
    pub async fn get_partition_load_information(
        &self,
        desc: &PartitionLoadInformationQueryDescription,
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<PartitionLoadInformation> {
        let com = {
            let raw: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION = desc.into();
            let timeout_ms = timeout.as_micros() as u32;
            self.get_partition_load_information_internal(&raw, timeout_ms, cancellation_token)
        }
        .await??;
        Ok(PartitionLoadInformation::new(com))
    }
}
