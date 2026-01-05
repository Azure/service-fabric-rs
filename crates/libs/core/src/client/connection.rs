// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// --------------------------------------------------

use mssf_com::{
    FabricClient::{
        IFabricClientConnectionEventHandler, IFabricClientConnectionEventHandler_Impl,
        IFabricClientConnectionEventHandler2, IFabricClientConnectionEventHandler2_Impl,
        IFabricGatewayInformationResult,
    },
    FabricTypes::FABRIC_CLAIMS_RETRIEVAL_METADATA,
};

use crate::{WString, types::NodeId};

/// Internal trait that rust code implements that can be bridged into IFabricClientConnectionEventHandler.
/// Not exposed to user.
pub trait ClientConnectionEventHandler: 'static {
    fn on_connected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
    fn on_disconnected(&self, info: &GatewayInformationResult) -> crate::Result<()>;
    fn on_claims_retrieval(&self, metadata: ClaimsRetrievalMetadata) -> crate::Result<WString>;
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
#[windows_core::implement(IFabricClientConnectionEventHandler2)]
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
        let h1: IFabricClientConnectionEventHandler2 = Self::new(inner).into();
        use windows_core::Interface;
        h1.cast().unwrap()
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

impl<T> IFabricClientConnectionEventHandler2_Impl for ClientConnectionEventHandlerBridge_Impl<T>
where
    T: ClientConnectionEventHandler,
{
    fn OnClaimsRetrieval(
        &self,
        metadata: *const mssf_com::Microsoft::ServiceFabric::FabricTypes::FABRIC_CLAIMS_RETRIEVAL_METADATA,
    ) -> crate::WinResult<mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult>
    {
        let meta = unsafe { metadata.as_ref().unwrap() };
        let claims_meta = ClaimsRetrievalMetadata::from(meta);
        let result = self
            .inner
            .on_claims_retrieval(claims_meta)
            .map_err(crate::WinError::from)?;

        let string_result = crate::strings::StringResult::new(result);
        Ok(string_result.into())
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

pub trait ClaimsRetrievalFn:
    Fn(ClaimsRetrievalMetadata) -> crate::Result<WString> + 'static
{
}
impl<T: Fn(ClaimsRetrievalMetadata) -> crate::Result<WString> + 'static> ClaimsRetrievalFn for T {}

/// Lambda implementation of the ClientConnectionEventHandler trait.
/// This is used in FabricClientBuilder to build handler from functions.
pub struct LambdaClientConnectionNotificationHandler {
    f_conn: Option<Box<dyn ConnectionNotificationFn>>,
    f_disconn: Option<Box<dyn ConnectionNotificationFn>>,
    f_claims: Option<Box<dyn ClaimsRetrievalFn>>,
}

impl LambdaClientConnectionNotificationHandler {
    pub fn new() -> Self {
        Self {
            f_conn: None,
            f_disconn: None,
            f_claims: None,
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

    /// Set the on_claims_retrieval callback.
    pub fn set_f_claims(&mut self, f: impl ClaimsRetrievalFn) {
        self.f_claims = Some(Box::new(f));
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

    fn on_claims_retrieval(&self, metadata: ClaimsRetrievalMetadata) -> crate::Result<WString> {
        if let Some(f) = &self.f_claims {
            f(metadata)
        } else {
            // Not implemented
            Err(crate::ErrorCode::E_NOTIMPL.into())
        }
    }
}

/// FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA
#[derive(Debug, Clone, PartialEq)]
pub struct AadClaimsRetrievalMetadata {
    pub authority: WString,
    pub tenant_id: WString,
    pub cluster_application: WString,
    pub client_application: WString,
    pub client_redirect: WString,
    // ex1
    pub login_endpoint: WString,
}

/// FABRIC_CLAIMS_RETRIEVAL_METADATA
#[derive(Debug, Clone, PartialEq)]
pub enum ClaimsRetrievalMetadata {
    AAD(AadClaimsRetrievalMetadata),
    None,
}

impl From<&FABRIC_CLAIMS_RETRIEVAL_METADATA> for ClaimsRetrievalMetadata {
    fn from(value: &FABRIC_CLAIMS_RETRIEVAL_METADATA) -> Self {
        match value.Kind {
            mssf_com::FabricTypes::FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_AAD => {
                let aad_meta = unsafe {
                    (value.Value
                        as *const mssf_com::FabricTypes::FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA)
                        .as_ref()
                        .unwrap()
                };
                let ex1 = unsafe {
                    (aad_meta.Reserved
                        as *const mssf_com::FabricTypes::FABRIC_AAD_CLAIMS_RETRIEVAL_METADATA_EX1)
                        .as_ref()
                };

                ClaimsRetrievalMetadata::AAD(AadClaimsRetrievalMetadata {
                    authority: WString::from(aad_meta.Authority),
                    tenant_id: WString::from(aad_meta.TenantId),
                    cluster_application: WString::from(aad_meta.ClusterApplication),
                    client_application: WString::from(aad_meta.ClientApplication),
                    client_redirect: WString::from(aad_meta.ClientRedirect),
                    login_endpoint: ex1.map_or(WString::new(), |v| WString::from(v.LoginEndpoint)),
                })
            }
            mssf_com::FabricTypes::FABRIC_CLAIMS_RETRIEVAL_METADATA_KIND_NONE => {
                ClaimsRetrievalMetadata::None
            }
            _ => ClaimsRetrievalMetadata::None,
        }
    }
}
