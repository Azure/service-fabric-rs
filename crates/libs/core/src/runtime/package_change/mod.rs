// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
//! This module supports implementing callbacks when Service Fabric Packages are changed
//!
pub mod config;

/// The ways a given Service Fabric Package (e.g. ConfigurationPackage or DataPackage) can change
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PackageChangeEvent<T> {
    Addition { new_package: T },
    Removal { previous_package: T },
    Modification { previous_package: T, new_package: T },
}

pub type ConfigurationPackageChangeEvent = PackageChangeEvent<super::config::ConfigurationPackage>;
