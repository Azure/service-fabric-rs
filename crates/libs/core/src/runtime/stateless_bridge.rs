// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// windows::core::implement macro generates snake case types.
#![allow(non_camel_case_types)]

use std::sync::Arc;

use crate::{
    runtime::stateless::StatelessServicePartition,
    strings::HSTRINGWrap,
    sync::{fabric_begin_bridge, fabric_end_bridge},
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
use tracing::info;
use windows::core::implement;
use windows_core::HSTRING;

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

impl<E, F> IFabricStatelessServiceFactory_Impl for StatelessServiceFactoryBridge<E, F>
where
    E: Executor,
    F: StatelessServiceFactory,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateInstance(
        &self,
        servicetypename: &::windows_core::PCWSTR,
        servicename: FABRIC_URI,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows_core::GUID,
        instanceid: i64,
    ) -> ::windows_core::Result<IFabricStatelessServiceInstance> {
        info!("StatelessServiceFactoryBridge::CreateInstance");
        let p_servicename = ::windows_core::PCWSTR::from_raw(servicename.0);
        let h_servicename = HSTRING::from_wide(unsafe { p_servicename.as_wide() }).unwrap();
        let h_servicetypename = HSTRING::from_wide(unsafe { servicetypename.as_wide() }).unwrap();
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

impl<E, S> IFabricStatelessServiceInstance_Impl for IFabricStatelessServiceInstanceBridge<E, S>
where
    E: Executor,
    S: StatelessServiceInstance + 'static,
{
    fn BeginOpen(
        &self,
        partition: ::core::option::Option<&IFabricStatelessServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatelessServiceInstanceBridge::BeginOpen");
        let partition_cp = partition.unwrap().clone();
        let partition_bridge = StatelessServicePartition::new(partition_cp);
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move {
            inner
                .open(&partition_bridge)
                .await
                .map(|s| IFabricStringResult::from(HSTRINGWrap::from(s)))
        })
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricStatelessServiceInstanceBridge::EndOpen");
        fabric_end_bridge(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatelessServiceInstanceBridge::BeginClose");
        let inner = self.inner.clone();
        fabric_begin_bridge(&self.rt, callback, async move { inner.close().await })
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricStatelessServiceInstanceBridge::EndClose");
        fabric_end_bridge(context)
    }

    fn Abort(&self) {
        info!("IFabricStatelessServiceInstanceBridge::Abort");
        self.inner.abort()
    }
}
