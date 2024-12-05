// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{
    runtime::stateless::StatelessServicePartition, strings::HSTRINGWrap, sync::BridgeContext3,
};
use mssf_com::{
    FabricCommon::IFabricStringResult,
    FabricRuntime::{
        IFabricStatelessServiceFactory, IFabricStatelessServiceFactory_Impl,
        IFabricStatelessServiceInstance, IFabricStatelessServiceInstance_Impl,
        IFabricStatelessServicePartition,
    },
    FabricTypes::FABRIC_URI,
};
use tracing::debug;
use windows_core::implement;

use super::{
    executor::Executor,
    stateless::{StatelessServiceFactory, StatelessServiceInstance},
};

#[implement(IFabricStatelessServiceFactory)]
pub struct StatelessServiceFactoryBridge<E, F>
where
    E: Executor + 'static,
    F: StatelessServiceFactory + 'static,
{
    inner: F,
    rt: E,
}

impl<E, F> StatelessServiceFactoryBridge<E, F>
where
    E: Executor,
    F: StatelessServiceFactory,
{
    pub fn create(factory: F, rt: E) -> StatelessServiceFactoryBridge<E, F> {
        StatelessServiceFactoryBridge::<E, F> { inner: factory, rt }
    }
}

impl<E, F> IFabricStatelessServiceFactory_Impl for StatelessServiceFactoryBridge_Impl<E, F>
where
    E: Executor,
    F: StatelessServiceFactory,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateInstance(
        &self,
        servicetypename: &crate::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &crate::GUID,
        instanceid: i64,
    ) -> crate::Result<IFabricStatelessServiceInstance> {
        debug!("StatelessServiceFactoryBridge::CreateInstance");
        let h_servicename = HSTRINGWrap::from(crate::PCWSTR(servicename.0)).into();
        let h_servicetypename = HSTRINGWrap::from(*servicetypename).into();
        let data = unsafe {
            if !initializationdata.is_null() {
                std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
            } else {
                &[]
            }
        };

        let instance = self.inner.create_instance(
            &h_servicetypename,
            &h_servicename,
            data,
            partitionid,
            instanceid,
        )?;
        let rt = self.rt.clone();
        let instance_bridge = IFabricStatelessServiceInstanceBridge::create(instance, rt);

        Ok(instance_bridge.into())
    }
}

// bridge from safe service instance to com
#[implement(IFabricStatelessServiceInstance)]

struct IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance + 'static,
{
    inner: Arc<S>,
    rt: E,
}

impl<E, S> IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance,
{
    pub fn create(instance: S, rt: E) -> IFabricStatelessServiceInstanceBridge<E, S>
    where
        S: StatelessServiceInstance,
    {
        IFabricStatelessServiceInstanceBridge {
            inner: Arc::new(instance),
            rt,
        }
    }
}

impl<E, S> IFabricStatelessServiceInstance_Impl for IFabricStatelessServiceInstanceBridge_Impl<E, S>
where
    E: Executor,
    S: StatelessServiceInstance + 'static,
{
    fn BeginOpen(
        &self,
        partition: ::core::option::Option<&IFabricStatelessServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        debug!("IFabricStatelessServiceInstanceBridge::BeginOpen");
        let partition_cp = partition.unwrap().clone();
        let partition_bridge = StatelessServicePartition::new(partition_cp);
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move {
            inner
                .open(&partition_bridge, token)
                .await
                .map(|s| IFabricStringResult::from(HSTRINGWrap::from(s)))
        })
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<IFabricStringResult> {
        debug!("IFabricStatelessServiceInstanceBridge::EndOpen");
        BridgeContext3::result(context)?
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> crate::Result<super::IFabricAsyncOperationContext> {
        debug!("IFabricStatelessServiceInstanceBridge::BeginClose");
        let inner = self.inner.clone();
        let (ctx, token) = BridgeContext3::make(callback);
        ctx.spawn(&self.rt, async move { inner.close(token).await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> crate::Result<()> {
        debug!("IFabricStatelessServiceInstanceBridge::EndClose");
        BridgeContext3::result(context)?
    }

    fn Abort(&self) {
        debug!("IFabricStatelessServiceInstanceBridge::Abort");
        self.inner.abort()
    }
}
