// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
//! Handle callbacks for configuration package changes
//! TODO: We probably should also provide a helpful callback to use in conjunction with the config-rs support (so that it processes configuration changes)
use mssf_com::FabricRuntime::{
    IFabricCodePackageActivationContext6, IFabricConfigurationPackageChangeHandler,
    IFabricConfigurationPackageChangeHandler_Impl,
};
use windows_core::implement;

use crate::runtime::{config::ConfigurationPackage, CodePackageActivationContext};

use super::ConfigurationPackageChangeEvent;

/// Rust trait to turn rust code into IFabricConfigurationPackageChangeHandler.
/// Not exposed to user
pub trait ConfigurationPackageChangeEventHandler: 'static {
    fn on_change(&self, change: &ConfigurationPackageChangeEvent) -> crate::Result<()>;
}

// Bridge implementation for the change handler to turn rust code into SF com object.
#[implement(IFabricConfigurationPackageChangeHandler)]
#[allow(non_camel_case_types)] // Suppress lint for _Impl struct
pub struct ConfigurationPackageChangeEventHandlerBridge<T>
where
    T: ConfigurationPackageChangeEventHandler,
{
    inner: T,
}

impl<T> ConfigurationPackageChangeEventHandlerBridge<T>
where
    T: ConfigurationPackageChangeEventHandler,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> IFabricConfigurationPackageChangeHandler_Impl
    for ConfigurationPackageChangeEventHandlerBridge<T>
where
    T: ConfigurationPackageChangeEventHandler,
{
    fn OnPackageAdded(
        &self,
        _source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let new_package = ConfigurationPackage::from_com(configpackage.unwrap().clone());
        let event = ConfigurationPackageChangeEvent::Addition { new_package };
        // TODO: unwrap, or should we change the return type of the lambda to be the empty type?
        self.inner.on_change(&event).unwrap();
    }

    fn OnPackageRemoved(
        &self,
        _source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let previous_package = ConfigurationPackage::from_com(configpackage.unwrap().clone());
        let event = ConfigurationPackageChangeEvent::Removal { previous_package };
        self.inner.on_change(&event).unwrap();
    }

    fn OnPackageModified(
        &self,
        _source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        previousconfigpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let new_package = ConfigurationPackage::from_com(configpackage.unwrap().clone());
        let previous_package =
            ConfigurationPackage::from_com(previousconfigpackage.unwrap().clone());
        let event = ConfigurationPackageChangeEvent::Modification {
            previous_package,
            new_package,
        };
        self.inner.on_change(&event).unwrap();
    }
}

/// Lambda implementation of ConfigurationPackageChangeEventHandler trait.
/// This is used in FabricClientBuilder to build function into handler.
/// Not exposed to user.
/// Strictly speaking we don't need this layer. But it would allow us to open the door to trait implementations someday
struct LambdaConfigurationPackageEventHandler<T>
where
    T: Fn(&ConfigurationPackageChangeEvent) -> crate::Result<()> + 'static,
{
    f: T,
}

impl<T> LambdaConfigurationPackageEventHandler<T>
where
    T: Fn(&ConfigurationPackageChangeEvent) -> crate::Result<()> + 'static,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> ConfigurationPackageChangeEventHandler for LambdaConfigurationPackageEventHandler<T>
where
    T: Fn(&ConfigurationPackageChangeEvent) -> crate::Result<()> + 'static,
{
    fn on_change(&self, change: &ConfigurationPackageChangeEvent) -> crate::Result<()> {
        (self.f)(change)
    }
}

/// This struct ensures that the handle is retained and deregistered before the implementation is dropped
struct ConfigurationPackageChangeHandle {
    activation_ctx: IFabricCodePackageActivationContext6,
    handle: i64,
}

impl ConfigurationPackageChangeHandle {
    pub fn new(
        activation_context: &CodePackageActivationContext,
        implementation: IFabricConfigurationPackageChangeHandler,
    ) -> crate::Result<Self> {
        let activation_ctx = activation_context.get_com();
        let handle =
            unsafe { activation_ctx.RegisterConfigurationPackageChangeHandler(&implementation) }?;

        Ok(Self {
            activation_ctx,
            handle,
        })
    }
}

impl Drop for ConfigurationPackageChangeHandle {
    fn drop(&mut self) {
        unsafe {
            self.activation_ctx
                .UnregisterConfigurationPackageChangeHandler(self.handle)
        }
        .unwrap();
    }
}
