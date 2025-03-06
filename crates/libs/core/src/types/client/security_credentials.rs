// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricClientSettings2;

// TODO: would this better live in common?
pub struct FabricSecurityCredentials{}

impl FabricSecurityCredentials
{
    pub fn set(&self, settings_interface: &IFabricClientSettings2)->  windows_core::Result<()>
    {
        // SAFETY: settings_interface implements the required interface TODO more
        let result = unsafe { settings_interface.SetSecurityCredentials(std::ptr::null()) };

        result
    }
}