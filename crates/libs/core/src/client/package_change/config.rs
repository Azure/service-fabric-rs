// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
//! Handle callbacks for configuration package changes
//! TODO: We probably should also provide a helpful callback to use in conjunction with the config-rs support (so that it processes configuration changes)
use mssf_com::FabricRuntime::{
    IFabricConfigurationPackageChangeHandler, IFabricConfigurationPackageChangeHandler_Impl,
};

use crate::runtime::config::ConfigurationPackage;

use super::PackageChangeType;

#[derive(Debug, Clone)]
pub struct ConfigurationPackageChangeEvent {
    pub change_type: PackageChangeType,
    pub config_package: Option<ConfigurationPackage>,
    pub previous_config_package: Option<ConfigurationPackage>,
}

impl ConfigurationPackageChangeEvent {
    fn from_com(
        change_type: PackageChangeType,
        _source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        previous_configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) -> Self {
        let config_package: Option<ConfigurationPackage> =
            configpackage.map(|c| ConfigurationPackage::from_com(c.clone()));
        let previous_config_package: Option<ConfigurationPackage> =
            previous_configpackage.map(|c| ConfigurationPackage::from_com(c.clone()));
        Self {
            change_type,
            config_package,
            previous_config_package,
        }
    }
}

/// Rust trait to turn rust code into IFabricConfigurationPackageChangeHandler.
/// Not exposed to user
pub trait ConfigurationPackageChangeEventHandler: 'static {
    fn on_change(&self, change: &ConfigurationPackageChangeEvent) -> crate::Result<()>;
}

// Bridge implementation for the change handler to turn rust code into SF com object.
#[windows_core::implement(IFabricConfigurationPackageChangeHandler)]
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

    pub fn new_com(inner: T) -> IFabricConfigurationPackageChangeHandler {
        Self::new(inner).into()
    }
}

impl<T> IFabricConfigurationPackageChangeHandler_Impl
    for ConfigurationPackageChangeEventHandlerBridge<T>
where
    T: ConfigurationPackageChangeEventHandler,
{
    fn OnPackageAdded(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let event = ConfigurationPackageChangeEvent::from_com(
            PackageChangeType::Addition,
            source,
            configpackage,
            None,
        );
        // TODO: unwrap, or should we change the return type of the lambda to be the empty type?
        self.inner.on_change(&event).unwrap();
    }

    fn OnPackageRemoved(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let event = ConfigurationPackageChangeEvent::from_com(
            PackageChangeType::Removal,
            source,
            configpackage,
            None,
        );
        self.inner.on_change(&event).unwrap();
    }

    fn OnPackageModified(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        previousconfigpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let event = ConfigurationPackageChangeEvent::from_com(
            PackageChangeType::Modification,
            source,
            configpackage,
            previousconfigpackage,
        );
        self.inner.on_change(&event).unwrap();
    }
}

/// Lambda implementation of ConfigurationPackageChangeEventHandler trait.
/// This is used in FabricClientBuilder to build function into handler.
/// Not exposed to user.
pub(crate) struct LambdaConfigurationPackageEventHandler<T>
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
