// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{
    ErrorCode, runtime::SelfReconfiguringServicePartition, strings::StringResult,
    sync::BridgeContext, types::Uri,
};
use mssf_com::{
    FabricCommon::IFabricStringResult,
    FabricRuntime::{
        IFabricSelfReconfiguringConfigurationChangeRequest, IFabricSelfReconfiguringServiceFactory,
        IFabricSelfReconfiguringServiceFactory_Impl, IFabricSelfReconfiguringServiceInstance,
        IFabricSelfReconfiguringServiceInstance_Impl, IFabricSelfReconfiguringServicePartition,
    },
    FabricTypes::{
        FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST,
        FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE, FABRIC_URI,
    },
};
use windows_core::{WString, implement};

use crate::runtime::{
    executor::Executor,
    {ISelfReconfiguringServiceFactory, ISelfReconfiguringServiceInstance},
};
use crate::types::{
    SelfReconfiguringConfigurationChangeRequest, SelfReconfiguringConfigurationRequest,
    SelfReconfiguringOpenMode,
};

#[implement(IFabricSelfReconfiguringServiceFactory)]
pub struct SelfReconfiguringServiceFactoryBridge<E>
where
    E: Executor + 'static,
{
    inner: Box<dyn ISelfReconfiguringServiceFactory>,
    rt: E,
}

impl<E> SelfReconfiguringServiceFactoryBridge<E>
where
    E: Executor,
{
    pub fn create(
        factory: Box<dyn ISelfReconfiguringServiceFactory>,
        rt: E,
    ) -> SelfReconfiguringServiceFactoryBridge<E> {
        SelfReconfiguringServiceFactoryBridge { inner: factory, rt }
    }
}

impl<E> IFabricSelfReconfiguringServiceFactory_Impl
    for SelfReconfiguringServiceFactoryBridge_Impl<E>
where
    E: Executor,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn CreateInstance(
        &self,
        servicetypename: &crate::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &crate::GUID,
        instanceid: i64,
    ) -> crate::WinResult<IFabricSelfReconfiguringServiceInstance> {
        let h_servicename = Uri::from(servicename);
        let h_servicetypename = WString::from(*servicetypename);
        let data = unsafe {
            if !initializationdata.is_null() {
                std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
            } else {
                &[]
            }
        };

        let instance = self.inner.create_instance(
            h_servicetypename,
            h_servicename,
            data,
            *partitionid,
            instanceid,
        )?;
        let rt = self.rt.clone();
        let instance_bridge = IFabricSelfReconfiguringServiceInstanceBridge::create(instance, rt);

        Ok(instance_bridge.into())
    }
}

// bridge from safe service instance to com
#[implement(IFabricSelfReconfiguringServiceInstance)]
pub(crate) struct IFabricSelfReconfiguringServiceInstanceBridge<E>
where
    E: Executor,
{
    inner: Arc<Box<dyn ISelfReconfiguringServiceInstance>>,
    rt: E,
}

impl<E> IFabricSelfReconfiguringServiceInstanceBridge<E>
where
    E: Executor,
{
    pub fn create(
        instance: Box<dyn ISelfReconfiguringServiceInstance>,
        rt: E,
    ) -> IFabricSelfReconfiguringServiceInstanceBridge<E> {
        IFabricSelfReconfiguringServiceInstanceBridge {
            inner: Arc::new(instance),
            rt,
        }
    }
}

impl<E> IFabricSelfReconfiguringServiceInstance_Impl
    for IFabricSelfReconfiguringServiceInstanceBridge_Impl<E>
where
    E: Executor,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginOpen(
        &self,
        openmode: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE,
        partition: windows_core::Ref<IFabricSelfReconfiguringServicePartition>,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let partition_cp = partition.unwrap().clone();
        let partition_bridge = SelfReconfiguringServicePartition::new(partition_cp);
        let open_mode = SelfReconfiguringOpenMode::from(openmode);
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(Arc::new(partition_bridge), open_mode, token)
                .await
                .map(|s| IFabricStringResult::from(StringResult::new(s)))
                .map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndOpen(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<IFabricStringResult> {
        BridgeContext::result(context)?
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn RequestConfiguration(
        &self,
        configurationrequest: *const FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST,
    ) -> crate::WinResult<()> {
        if configurationrequest.is_null() {
            return Err(ErrorCode::E_POINTER.into());
        }
        let request =
            SelfReconfiguringConfigurationRequest::from(unsafe { &*configurationrequest });
        self.inner
            .request_configuration(request)
            .map_err(crate::WinError::from)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn RequestConfigurationChange(
        &self,
        configurationchangerequest: windows_core::Ref<
            IFabricSelfReconfiguringConfigurationChangeRequest,
        >,
    ) -> crate::WinResult<()> {
        let com = configurationchangerequest.ok()?;
        let raw = unsafe { com.get_ConfigurationChangeRequest() };
        if raw.is_null() {
            return Err(ErrorCode::E_POINTER.into());
        }
        let change = SelfReconfiguringConfigurationChangeRequest::from(unsafe { &*raw });
        self.inner
            .request_configuration_change(change)
            .map_err(crate::WinError::from)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn BeginClose(
        &self,
        callback: windows_core::Ref<super::IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<super::IFabricAsyncOperationContext> {
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext::make(callback);
        ctx.spawn(&self.rt, async move {
            inner.close(token).await.map_err(crate::WinError::from)
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"), err)
    )]
    fn EndClose(
        &self,
        context: windows_core::Ref<super::IFabricAsyncOperationContext>,
    ) -> crate::WinResult<()> {
        BridgeContext::result(context)?
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, ret(level = "debug"))
    )]
    fn Abort(&self) {
        self.inner.abort()
    }
}
