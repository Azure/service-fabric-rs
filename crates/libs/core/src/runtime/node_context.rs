use std::time::Duration;

use crate::{Interface, WString};
use mssf_com::FabricRuntime::{IFabricNodeContextResult, IFabricNodeContextResult2};

use crate::{
    strings::WStringWrap,
    sync::{fabric_begin_end_proxy2, CancellationToken},
    types::NodeId,
};

pub fn get_com_node_context(
    timeout_milliseconds: u32,
    cancellation_token: Option<CancellationToken>,
) -> crate::sync::FabricReceiver2<::windows_core::Result<IFabricNodeContextResult>> {
    fabric_begin_end_proxy2(
        move |callback| {
            crate::API_TABLE.fabric_begin_get_node_context(timeout_milliseconds, callback)
        },
        move |ctx| crate::API_TABLE.fabric_end_get_node_context(ctx),
        cancellation_token,
    )
}

#[derive(Debug)]
pub struct NodeContext {
    com: IFabricNodeContextResult,
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
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<Self> {
        let com = get_com_node_context(timeout.as_millis().try_into().unwrap(), cancellation_token)
            .await??;
        Ok(Self::from(&com))
    }

    // Get the node context synchronously
    pub fn get_sync() -> crate::Result<Self> {
        let com = crate::API_TABLE.fabric_get_node_context()?;
        Ok(Self::from(&com))
    }

    // Retrieves the directory path for the directory at node level.
    pub fn get_directory(&self, logical_directory_name: &WString) -> windows_core::Result<WString> {
        let com2 = self.com.cast::<IFabricNodeContextResult2>()?;
        let dir = unsafe { com2.GetDirectory(logical_directory_name.as_pcwstr()) }?;
        Ok(WStringWrap::from(&dir).into())
    }
}

impl From<&IFabricNodeContextResult> for NodeContext {
    fn from(value: &IFabricNodeContextResult) -> Self {
        let raw = unsafe { value.get_NodeContext() };
        assert!(!raw.is_null());
        let raw_ref = unsafe { raw.as_ref() }.unwrap();
        Self {
            com: value.clone(),
            node_name: WStringWrap::from(raw_ref.NodeName).into(),
            node_type: WStringWrap::from(raw_ref.NodeType).into(),
            ip_address_or_fqdn: WStringWrap::from(raw_ref.IPAddressOrFQDN).into(),
            node_instance_id: raw_ref.NodeInstanceId,
            node_id: raw_ref.NodeId.into(),
        }
    }
}
