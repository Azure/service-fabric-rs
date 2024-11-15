// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
//! This module supports implementing callbacks when Service Fabric Packages are changed
//!
pub(super) mod config;

/// The ways a given Service Fabric Package (e.g. ConfigurationPackage or DataPackage) can change
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PackageChangeType {
    Addition,
    Removal,
    Modification,
}
