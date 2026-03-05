// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::time::Duration;

use mssf_com::{
    FabricClient::{
        IFabricGetApplicationListResult2, IFabricGetDeployedServiceReplicaDetailResult,
        IFabricGetNodeListResult2, IFabricGetPartitionListResult2,
        IFabricGetPartitionLoadInformationResult, IFabricGetReplicaListResult2,
        IFabricQueryClient13,
    },
    FabricTypes::{
        FABRIC_APPLICATION_QUERY_DESCRIPTION,
        FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_DESCRIPTION,
        FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION, FABRIC_SERVICE_QUERY_DESCRIPTION,
        FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
    },
};

use crate::mem::{BoxPool, GetRawWithBoxPool};

use crate::types::{
    DeployedServiceReplicaDetailQueryDescription, DeployedServiceReplicaDetailQueryResult,
    GetPartitionLoadInformationResult, NodeListResult, NodeQueryDescription,
    PartitionLoadInformationQueryDescription, ServicePartitionList,
    ServicePartitionQueryDescription, ServiceReplicaList, ServiceReplicaQueryDescription,
};
use crate::{
    runtime::executor::BoxedCancelToken,
    sync::{FabricReceiver, fabric_begin_end_proxy},
    types::ServiceQueryDescription,
};

#[derive(Debug, Clone)]
pub struct QueryClient {
    com: IFabricQueryClient13,
}

// Internal implementation block
// Internal functions focuses on changing SF callback to async future,
// while the public apis impl focuses on type conversion.

impl QueryClient {
    pub fn get_node_list_internal(
        &self,
        query_description: &FABRIC_NODE_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetNodeListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();

        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetNodeList(query_description, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetNodeList2(ctx) },
            cancellation_token,
        )
    }

    pub fn get_application_list_internal(
        &self,
        query_description: &FABRIC_APPLICATION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetApplicationListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetApplicationList(query_description, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetApplicationList2(ctx) },
            cancellation_token,
        )
    }

    fn get_service_list_internal(
        &self,
        desc: &FABRIC_SERVICE_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<mssf_com::FabricClient::IFabricGetServiceListResult2>>
    {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetServiceList(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetServiceList2(ctx) },
            cancellation_token,
        )
    }

    fn get_partition_list_internal(
        &self,
        desc: &FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetPartitionListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetPartitionList(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetPartitionList2(ctx) },
            cancellation_token,
        )
    }

    fn get_replica_list_internal(
        &self,
        desc: &FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetReplicaListResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetReplicaList(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetReplicaList2(ctx) },
            cancellation_token,
        )
    }

    fn get_partition_load_information_internal(
        &self,
        desc: &FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetPartitionLoadInformationResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetPartitionLoadInformation(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetPartitionLoadInformation(ctx) },
            cancellation_token,
        )
    }

    fn get_deployed_replica_detail_internal(
        &self,
        desc: &FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricGetDeployedServiceReplicaDetailResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetDeployedReplicaDetail(desc, timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndGetDeployedReplicaDetail(ctx) },
            cancellation_token,
        )
    }
}

impl From<IFabricQueryClient13> for QueryClient {
    fn from(com: IFabricQueryClient13) -> Self {
        Self { com }
    }
}

impl From<QueryClient> for IFabricQueryClient13 {
    fn from(value: QueryClient) -> Self {
        value.com
    }
}

impl QueryClient {
    // List nodes in the cluster
    pub async fn get_node_list(
        &self,
        desc: &NodeQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<NodeListResult> {
        let com = {
            let mut pool = BoxPool::new();
            let arg = desc.get_raw_with_pool(&mut pool);
            self.get_node_list_internal(
                &arg,
                timeout.as_millis().try_into().unwrap(),
                cancellation_token,
            )
        }
        .await??;
        Ok(NodeListResult::from(&com))
    }

    pub async fn get_application_list(
        &self,
        desc: &crate::types::ApplicationQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<crate::types::ApplicationListResult> {
        let com = {
            let mut pool = BoxPool::new();
            let arg = desc.get_raw_with_pool(&mut pool);
            self.get_application_list_internal(
                &arg,
                timeout.as_millis().try_into().unwrap(),
                cancellation_token,
            )
        }
        .await??;
        Ok(crate::types::ApplicationListResult::from(&com))
    }
    pub async fn get_service_list(
        &self,
        desc: &ServiceQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<crate::types::ServiceListResult> {
        let com = {
            let mut pool = BoxPool::new();
            let arg = desc.get_raw_with_pool(&mut pool);
            self.get_service_list_internal(&arg, timeout.as_millis() as u32, cancellation_token)
        }
        .await??;
        Ok(crate::types::ServiceListResult::from(&com))
    }

    pub async fn get_partition_list(
        &self,
        desc: &ServicePartitionQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<ServicePartitionList> {
        let com = {
            let raw: FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION = desc.into();
            let mili = timeout.as_millis() as u32;
            self.get_partition_list_internal(&raw, mili, cancellation_token)
        }
        .await??;
        Ok(ServicePartitionList::from(&com))
    }

    pub async fn get_replica_list(
        &self,
        desc: &ServiceReplicaQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<ServiceReplicaList> {
        let com = {
            let raw: FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION = desc.into();
            let mili = timeout.as_millis() as u32;
            self.get_replica_list_internal(&raw, mili, cancellation_token)
        }
        .await??;
        Ok(ServiceReplicaList::from(&com))
    }

    pub async fn get_partition_load_information(
        &self,
        desc: &PartitionLoadInformationQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<GetPartitionLoadInformationResult> {
        let com = {
            let raw: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION = desc.into();
            let timeout_ms = timeout.as_micros() as u32;
            self.get_partition_load_information_internal(&raw, timeout_ms, cancellation_token)
        }
        .await??;
        Ok(GetPartitionLoadInformationResult::from(&com))
    }

    pub async fn get_deployed_replica_detail(
        &self,
        desc: &DeployedServiceReplicaDetailQueryDescription,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<DeployedServiceReplicaDetailQueryResult> {
        let com = {
            let raw: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION = desc.into();
            let timeout_ms = timeout.as_micros() as u32;
            self.get_deployed_replica_detail_internal(&raw, timeout_ms, cancellation_token)
        }
        .await??;
        Ok(DeployedServiceReplicaDetailQueryResult::new(com))
    }
}
