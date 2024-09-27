use std::time::Duration;

use mssf_com::FabricRuntime::{
    FabricBeginGetNodeContext, FabricEndGetNodeContext, FabricGetNodeContext,
    IFabricNodeContextResult, IFabricNodeContextResult2,
};
use windows_core::{Interface, HSTRING};

use crate::{
    strings::HSTRINGWrap,
    sync::{fabric_begin_end_proxy2, CancellationToken},
    types::NodeId,
};

pub fn get_com_node_context(
    timeout_milliseconds: u32,
    cancellation_token: Option<CancellationToken>,
) -> crate::sync::FabricReceiver2<::windows_core::Result<IFabricNodeContextResult>> {
    fabric_begin_end_proxy2(
        move |callback| unsafe { FabricBeginGetNodeContext(timeout_milliseconds, callback) },
        move |ctx| {
            unsafe { FabricEndGetNodeContext(ctx) }.map(|raw| {
                assert!(!raw.is_null());
                unsafe { IFabricNodeContextResult::from_raw(raw) }
            })
        },
        cancellation_token,
    )
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
    pub async fn get(
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> ::windows_core::Result<Self> {
        let com = get_com_node_context(timeout.as_millis().try_into().unwrap(), cancellation_token)
            .await??;
        Ok(Self::from(&com))
    }

    // Get the node context synchronously
    pub fn get_sync() -> ::windows_core::Result<Self> {
        let raw = unsafe { FabricGetNodeContext() }?;
        assert!(!raw.is_null());
        let com = unsafe { IFabricNodeContextResult::from_raw(raw) };
        Ok(Self::from(&com))
    }

    // Retrieves the directory path for the directory at node level.
    pub fn get_directory(&self, logical_directory_name: &HSTRING) -> windows_core::Result<HSTRING> {
        let com2 = self.com.cast::<IFabricNodeContextResult2>()?;
        let dir = unsafe { com2.GetDirectory(logical_directory_name) }?;
        Ok(HSTRINGWrap::from(&dir).into())
    }
}

impl From<&IFabricNodeContextResult> for NodeContext {
    fn from(value: &IFabricNodeContextResult) -> Self {
        let raw = unsafe { value.get_NodeContext() };
        assert!(!raw.is_null());
        let raw_ref = unsafe { raw.as_ref() }.unwrap();
        Self {
            com: value.clone(),
            node_name: HSTRINGWrap::from(raw_ref.NodeName).into(),
            node_type: HSTRINGWrap::from(raw_ref.NodeType).into(),
            ip_address_or_fqdn: HSTRINGWrap::from(raw_ref.IPAddressOrFQDN).into(),
            node_instance_id: raw_ref.NodeInstanceId,
            node_id: raw_ref.NodeId.into(),
        }
    }
}
