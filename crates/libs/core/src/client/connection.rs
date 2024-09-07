// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// --------------------------------------------------

use mssf_com::FabricClient::{
    IFabricClientConnectionEventHandler, IFabricClientConnectionEventHandler_Impl,
    IFabricGatewayInformationResult,
};

use crate::{strings::HSTRINGWrap, types::NodeId};

pub trait ClientConnectionEventHandler: 'static {
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
}

// IFabricGatewayInformationResult
#[derive(Debug, Clone)]
pub struct GatewayInformationResult {
    pub node_address: crate::HSTRING,
    pub node_id: NodeId,
    pub node_instance_id: u64,
    pub node_name: crate::HSTRING,
}

impl GatewayInformationResult {
    fn from_com(com: &IFabricGatewayInformationResult) -> Self {
        let info = unsafe { com.get_GatewayInformation().as_ref().unwrap() };
        Self {
            node_address: HSTRINGWrap::from(info.NodeAddress).into(),
            node_id: info.NodeId.into(),
            node_instance_id: info.NodeInstanceId,
            node_name: HSTRINGWrap::from(info.NodeName).into(),
        }
    }
}

// IFabricClientConnectionEventHandler
#[windows_core::implement(IFabricClientConnectionEventHandler)]
pub struct ClientConnectionEventHandlerBridge<T>
where
    T: ClientConnectionEventHandler,
{
    inner: T,
}

impl<T> ClientConnectionEventHandlerBridge<T>
where
    T: ClientConnectionEventHandler,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn new_com(inner: T) -> IFabricClientConnectionEventHandler {
        Self::new(inner).into()
    }
}

impl<T> IFabricClientConnectionEventHandler_Impl for ClientConnectionEventHandlerBridge<T>
where
    T: ClientConnectionEventHandler,
{
    fn OnConnected(
        &self,
        gw_info: Option<&IFabricGatewayInformationResult>,
    ) -> windows_core::Result<()> {
        let info = GatewayInformationResult::from_com(gw_info.unwrap());
        self.inner.on_connected(&info)
    }

    fn OnDisconnected(
        &self,
        gw_info: Option<&IFabricGatewayInformationResult>,
    ) -> windows_core::Result<()> {
        let info = GatewayInformationResult::from_com(gw_info.unwrap());
        self.inner.on_disconnected(&info)
    }
}

// Default implementation of ClientConnectionEventHandler
pub struct DefaultClientConnectionEventHandler {}

impl ClientConnectionEventHandler for DefaultClientConnectionEventHandler {
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        tracing::debug!("on_connected: {:?}", info);
        Ok(())
    }

    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        tracing::debug!("on_disconnected: {:?}", info);
        Ok(())
    }
}

/// Turns a Fn into client connection notification handler.
pub struct LambdaClientConnectionNotificationHandler<T, K>
where
    T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    K: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
{
    f_conn: T,
    f_disconn: K,
}

impl<T, K> LambdaClientConnectionNotificationHandler<T, K>
where
    T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    K: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
{
    pub fn new(f_conn: T, f_disconn: K) -> Self {
        Self { f_conn, f_disconn }
    }
}

impl<T, K> ClientConnectionEventHandler for LambdaClientConnectionNotificationHandler<T, K>
where
    T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
    K: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static,
{
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        (self.f_conn)(info)
    }

    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        (self.f_disconn)(info)
    }
}
