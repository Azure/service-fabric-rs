// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// --------------------------------------------------

use mssf_com::FabricClient::{
    IFabricClientConnectionEventHandler, IFabricClientConnectionEventHandler_Impl,
    IFabricGatewayInformationResult,
};

use crate::{WString, types::NodeId};

/// Internal trait that rust code implements that can be bridged into IFabricClientConnectionEventHandler.
/// Not exposed to user.
pub trait ClientConnectionEventHandler: 'static {
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
}

/// FabricClient connection information.
/// Traslated from IFabricGatewayInformationResult
#[derive(Debug, Clone)]
pub struct GatewayInformationResult {
    pub node_address: crate::WString,
    pub node_id: NodeId,
    pub node_instance_id: u64,
    pub node_name: crate::WString,
}

impl From<&IFabricGatewayInformationResult> for GatewayInformationResult {
    fn from(com: &IFabricGatewayInformationResult) -> Self {
        let info = unsafe { com.get_GatewayInformation().as_ref().unwrap() };
        Self {
            node_address: WString::from(info.NodeAddress),
            node_id: info.NodeId.into(),
            node_instance_id: info.NodeInstanceId,
            node_name: WString::from(info.NodeName),
        }
    }
}

/// Bridge for IFabricClientConnectionEventHandler.
/// Turn rust trait into SF com object.
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

impl<T> IFabricClientConnectionEventHandler_Impl for ClientConnectionEventHandlerBridge_Impl<T>
where
    T: ClientConnectionEventHandler,
{
    fn OnConnected(
        &self,
        gw_info: windows_core::Ref<IFabricGatewayInformationResult>,
    ) -> crate::WinResult<()> {
        let info = GatewayInformationResult::from(gw_info.unwrap());
        self.inner
            .on_connected(&info)
            .map_err(crate::WinError::from)
    }

    fn OnDisconnected(
        &self,
        gw_info: windows_core::Ref<IFabricGatewayInformationResult>,
    ) -> crate::WinResult<()> {
        let info = GatewayInformationResult::from(gw_info.unwrap());
        self.inner
            .on_disconnected(&info)
            .map_err(crate::WinError::from)
    }
}

/// Connection notification function signature to avoid code repeatition.
/// Trait alias feature in rust (not yet stable) would eliminate this trait definition.
pub trait ConnectionNotificationFn:
    Fn(&GatewayInformationResult) -> crate::Result<()> + 'static
{
}
impl<T: Fn(&GatewayInformationResult) -> crate::Result<()> + 'static> ConnectionNotificationFn
    for T
{
}

/// Lambda implementation of the ClientConnectionEventHandler trait.
/// This is used in FabricClientBuilder to build handler from functions.
pub struct LambdaClientConnectionNotificationHandler {
    f_conn: Option<Box<dyn ConnectionNotificationFn>>,
    f_disconn: Option<Box<dyn ConnectionNotificationFn>>,
}

impl LambdaClientConnectionNotificationHandler {
    pub fn new() -> Self {
        Self {
            f_conn: None,
            f_disconn: None,
        }
    }

    /// Set the on_connected callback.
    pub fn set_f_conn(&mut self, f: impl ConnectionNotificationFn) {
        self.f_conn = Some(Box::new(f));
    }

    /// Set the on_disconnected callback.
    pub fn set_f_disconn(&mut self, f: impl ConnectionNotificationFn) {
        self.f_disconn = Some(Box::new(f));
    }
}

impl ClientConnectionEventHandler for LambdaClientConnectionNotificationHandler {
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        if let Some(f) = &self.f_conn {
            f(info)
        } else {
            Ok(())
        }
    }

    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()> {
        if let Some(f) = &self.f_disconn {
            f(info)
        } else {
            Ok(())
        }
    }
}
