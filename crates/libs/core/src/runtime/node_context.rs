use std::time::Duration;

use mssf_com::FabricRuntime::{
    FabricBeginGetNodeContext, FabricEndGetNodeContext, IFabricNodeContextResult,
    IFabricNodeContextResult2,
};
use windows_core::{Interface, HSTRING};

use crate::{strings::HSTRINGWrap, sync::fabric_begin_end_proxy};

pub fn get_com_node_context(
    timeoutMilliseconds: u32,
) -> crate::sync::FabricReceiver<::windows_core::Result<IFabricNodeContextResult>> {
    fabric_begin_end_proxy(
        move |callback| unsafe { FabricBeginGetNodeContext(timeoutMilliseconds, callback) },
        move |ctx| {
            unsafe { FabricEndGetNodeContext(ctx) }.map(|raw| {
                assert!(!raw.is_null());
                unsafe { IFabricNodeContextResult::from_raw(raw) }
            })
        },
    )
}

#[derive(Debug)]
pub struct NodeId {
    pub low: u64,
    pub high: u64,
}

#[derive(Debug)]
pub struct NodeContext {
    com: IFabricNodeContextResult,
    pub node_name: HSTRING,
    pub node_type: HSTRING,
    pub ip_address_or_fqdn: HSTRING,
    pub node_instance_id: u64,
    pub node_id: NodeId,
}

impl NodeContext {
    // Get the node context from SF runtime
    pub async fn get(timeout: Duration) -> ::windows_core::Result<Self> {
        let com = get_com_node_context(timeout.as_millis().try_into().unwrap()).await?;
        Ok(Self::convert(&com))
    }

    pub fn get_sync(timeout: Duration) -> ::windows_core::Result<Self> {
        let com = get_com_node_context(timeout.as_millis().try_into().unwrap()).blocking_recv()?;
        Ok(Self::convert(&com))
    }

    fn convert(com: &IFabricNodeContextResult) -> Self {
        let raw = unsafe { com.get_NodeContext() };
        assert!(!raw.is_null());
        let raw_ref = unsafe { raw.as_ref() }.unwrap();
        Self {
            com: com.clone(),
            node_name: HSTRINGWrap::from(raw_ref.NodeName).into(),
            node_type: HSTRINGWrap::from(raw_ref.NodeType).into(),
            ip_address_or_fqdn: HSTRINGWrap::from(raw_ref.IPAddressOrFQDN).into(),
            node_instance_id: raw_ref.NodeInstanceId,
            node_id: NodeId {
                low: raw_ref.NodeId.Low,
                high: raw_ref.NodeId.High,
            },
        }
    }

    // Retrieves the directory path for the directory at node level.
    pub fn get_directory(&self, logical_directory_name: &HSTRING) -> windows_core::Result<HSTRING> {
        let com2 = self.com.cast::<IFabricNodeContextResult2>()?;
        let dir = unsafe { com2.GetDirectory(logical_directory_name) }?;
        Ok(HSTRINGWrap::from(&dir).into())
    }
}
