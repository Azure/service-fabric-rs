// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use mssf_com::{FabricClient::{IFabricClientSettings2, IFabricClientSettings2_Impl, IFabricClientSettingsResult, IFabricClientSettings_Impl}, FabricTypes::{FABRIC_CLIENT_SETTINGS, FABRIC_E_INVALID_CONFIGURATION, FABRIC_E_INVALID_CREDENTIALS, FABRIC_SECURITY_CREDENTIALS}};
use windows::Win32::Foundation::E_NOTIMPL;

/// A convenience struct to help test code which interacts with IFabricClientSettings2
#[windows_core::implement(IFabricClientSettings2)]
pub(crate) struct MockIFabricClientSettings
{
    pub set_security_credentials_mock: Box<dyn Fn(*const FABRIC_SECURITY_CREDENTIALS) -> windows_core::Result<()>>,
    pub set_keepalive_mock: Box<dyn Fn(u32) -> windows_core::Result<()>>,
    pub get_settings_mock: Box<dyn Fn() -> windows_core::Result<IFabricClientSettingsResult>>,
    pub set_settings_mock: Box<dyn Fn(*const FABRIC_CLIENT_SETTINGS) -> windows_core::Result<()>>
}

impl MockIFabricClientSettings
{
    pub fn new_all_methods_fail() -> Self
    {
        Self
        {
            set_security_credentials_mock: Box::new(|_| Err(crate::Error::from(FABRIC_E_INVALID_CREDENTIALS).into())),
            set_keepalive_mock: Box::new(|_| Err(crate::Error::from(FABRIC_E_INVALID_CONFIGURATION).into())),
            get_settings_mock: Box::new(|| Err(crate::Error::from(E_NOTIMPL).into())),
            set_settings_mock:  Box::new(|_| Err(crate::Error::from(FABRIC_E_INVALID_CONFIGURATION).into())),
        }
    }

    pub fn new_with_security_credentials_mock(mock: Box<dyn Fn(*const FABRIC_SECURITY_CREDENTIALS) -> windows_core::Result<()>>) -> Self
    {
        Self
        {
            set_security_credentials_mock: mock,
            set_keepalive_mock: Box::new(|_| panic!("Unexpected call to SetKeepAlive")),
            get_settings_mock: Box::new(|| panic!("Unexpected call to GetSettings")),
            set_settings_mock:  Box::new(|_| panic!("Unexpected call to SetSettings")),
        }
    }
}

impl IFabricClientSettings_Impl for MockIFabricClientSettings_Impl
{
    fn SetSecurityCredentials(
        &self,
        securitycredentials: *const FABRIC_SECURITY_CREDENTIALS,
    ) -> windows_core::Result<()> {
        (self.set_security_credentials_mock)(securitycredentials)
    }

    fn SetKeepAlive(&self, keepaliveintervalinseconds: u32) -> windows_core::Result<()> {
        (self.set_keepalive_mock)(keepaliveintervalinseconds)
    }
}
    
impl IFabricClientSettings2_Impl for MockIFabricClientSettings_Impl
{
    fn GetSettings(&self) -> windows_core::Result<IFabricClientSettingsResult> {
        (self.get_settings_mock)()
    }

    fn SetSettings(
        &self,
        fabricclientsettings: *const FABRIC_CLIENT_SETTINGS,
    ) -> windows_core::Result<()> {
        (self.set_settings_mock)(fabricclientsettings)
    }
}