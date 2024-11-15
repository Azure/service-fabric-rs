// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use mssf_com::FabricRuntime::{IFabricConfigurationPackageChangeHandler, IFabricConfigurationPackageChangeHandler_Impl};

use crate::runtime::config::ConfigurationPackage;

#[derive(PartialEq, Eq, Copy, Clone)]
enum ConfigurationPackageChangeType
{
    Added,
    Removed,
    Modified
}

/// Rust trait to turn rust code into IFabricConfigurationPackageChangeHandler.
/// Not exposed to user
pub trait PackageChangeEventHandler: 'static {
    fn on_change(&self, change_type: ConfigurationPackageChangeType, package: Option<&ConfigurationPackage>) -> crate::Result<()>;
}

// Bridge implementation for the change handler to turn rust code into SF com object.
#[windows_core::implement(IFabricConfigurationPackageChangeHandler)]
pub struct PackageChangeEventHandlerBridge<T>
where
    T: PackageChangeEventHandler,
{
    inner: T,
}

impl<T> PackageChangeEventHandlerBridge<T>
where
    T: PackageChangeEventHandler,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn new_com(inner: T) -> IFabricConfigurationPackageChangeHandler {
        Self::new(inner).into()
    }
}

impl<T> IFabricConfigurationPackageChangeHandler_Impl for PackageChangeEventHandlerBridge<T>
where
    T: PackageChangeEventHandler,
{
    fn OnPackageAdded(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let package: Option<ConfigurationPackage> = configpackage.map(|c| ConfigurationPackage::from_com(c));
        self.inner.on_change(ConfigurationPackageChangeType::Added, package.as_ref())
    }

    fn OnPackageRemoved(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        // Presumably, configpackage is None in this case. But we won't assume that, just in case we're wrong
        let package: Option<ConfigurationPackage> = configpackage.map(|c| ConfigurationPackage::from_com(c));
        self.inner.on_change(ConfigurationPackageChangeType::Removed, package.as_ref())
    }

    fn OnPackageModified(
        &self,
        source: Option<&mssf_com::FabricRuntime::IFabricCodePackageActivationContext>,
        previousconfigpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
        configpackage: Option<&mssf_com::FabricRuntime::IFabricConfigurationPackage>,
    ) {
        let package: Option<ConfigurationPackage> = configpackage.map(|c| ConfigurationPackage::from_com(c));
        self.inner.on_change(ConfigurationPackageChangeType::Modified, package.as_ref())
    }
}

/// Lambda implementation of PackageChangeEventHandler trait.
/// This is used in FabricClientBuilder to build function into handler.
/// Not exposed to user.
pub struct LambdaChangeNotificationHandler<T>
where
    T: Fn(ConfigurationPackageChangeType, &ConfigurationPackage) -> crate::Result<()> + 'static,
{
    f: T,
}

impl<T> LambdaChangeNotificationHandler<T>
where
    T: Fn(ConfigurationPackageChangeType, &ConfigurationPackage) -> crate::Result<()> + 'static,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> PackageChangeEventHandler for LambdaChangeNotificationHandler<T>
where
    T: Fn(ConfigurationPackageChangeType, &ConfigurationPackage) -> crate::Result<()> + 'static,
{
    fn on_change(&self, change_type: ConfigurationPackageChangeType, package: Option<&ConfigurationPackage>) -> crate::Result<()> {
        self.f(change_type, package)
    }
}
