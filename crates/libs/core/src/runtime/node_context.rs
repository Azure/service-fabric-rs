// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use crate::WString;
use crate::runtime::executor::BoxedCancelToken;
use crate::strings::StringResult;
use mssf_com::FabricRuntime::IFabricNodeContextResult2;

use crate::sync::fabric_begin_end_proxy;
use crate::types::NodeId;

pub fn get_com_node_context(
    timeout_milliseconds: u32,
    cancellation_token: Option<BoxedCancelToken>,
) -> crate::sync::FabricReceiver<crate::WinResult<IFabricNodeContextResult2>> {
    fabric_begin_end_proxy(
        move |callback| {
            crate::API_TABLE.fabric_begin_get_node_context(timeout_milliseconds, callback)
        },
        move |ctx| crate::API_TABLE.fabric_end_get_node_context(ctx),
        cancellation_token,
    )
}

#[derive(Debug)]
pub struct NodeContext {
    com: IFabricNodeContextResult2,
    pub node_name: WString,
    pub node_type: WString,
    pub ip_address_or_fqdn: WString,
    pub node_instance_id: u64,
    pub node_id: NodeId,
}

impl NodeContext {
    // Get the node context from SF runtime
    pub async fn get(
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<Self> {
        let com = get_com_node_context(timeout.as_millis().try_into().unwrap(), cancellation_token)
            .await??;
        Ok(Self::from(&com))
    }
}

impl NodeContext {
    // Get the node context synchronously
    pub fn get_sync() -> crate::Result<Self> {
        let com = crate::API_TABLE.fabric_get_node_context()?;
        Ok(Self::from(&com))
    }

    // Retrieves the directory path for the directory at node level.
    pub fn get_directory(&self, logical_directory_name: &WString) -> crate::Result<WString> {
        let dir = unsafe { self.com.GetDirectory(logical_directory_name.as_pcwstr()) }?;
        Ok(StringResult::from(&dir).into_inner())
    }
}

impl From<&IFabricNodeContextResult2> for NodeContext {
    fn from(value: &IFabricNodeContextResult2) -> Self {
        let raw = unsafe { value.get_NodeContext() };
        assert!(!raw.is_null());
        let raw_ref = unsafe { raw.as_ref() }.unwrap();
        Self {
            com: value.clone(),
            node_name: WString::from(raw_ref.NodeName),
            node_type: WString::from(raw_ref.NodeType),
            ip_address_or_fqdn: WString::from(raw_ref.IPAddressOrFQDN),
            node_instance_id: raw_ref.NodeInstanceId,
            node_id: raw_ref.NodeId.into(),
        }
    }
}
