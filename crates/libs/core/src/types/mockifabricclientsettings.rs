// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
use mssf_com::{FabricClient::{IFabricClientSettings2, IFabricClientSettings2_Impl, IFabricClientSettingsResult, IFabricClientSettings_Impl}, FabricTypes::{FABRIC_CLIENT_SETTINGS, FABRIC_E_INVALID_CONFIGURATION, FABRIC_E_INVALID_CREDENTIALS, FABRIC_SECURITY_CREDENTIALS}};
use windows::Win32::Foundation::E_NOTIMPL;
pub(crate) mod test_constants
{
    pub const TEST_SERVER_NAME_1: &str = "test.contoso.com";
    pub const TEST_SERVER_NAME_2: &str = "test2.contoso.com";
    pub const TEST_THUMBPRINT_1: &str = "ABCDEF01234567890";
    pub const TEST_THUMBPRINT_2: &str = "ABCDEF01234567891";
    pub const TEST_THUMBPRINT_3: &str = "FFABCDEF0123456789";
    pub const TEST_THUMBPRINT_4: &str = "FFABCDEF0123456789";
    pub const TEST_CLAIMS: &str = "mock_claims_here";
}

pub(crate) mod test_utilities
{
    use windows_core::PCWSTR;


    /// # SAFETY
    /// * This is test code, intended to be used with Miri
    /// to validate that all reads SF might do of a 
    /// pointer / length pair are defined behavior
    /// * Caller is responsible for ensuring that the actual_len and actual_values_start parameters go together 
    /// * Caller is responsible for ensuring that actual_values_start is valid for dereference for N elements at the time of the call, if non-null
    pub unsafe fn check_array_parameter<const N: usize>(expected_values: [&str; N], actual_len: u32, actual_values_start: *const PCWSTR)
    {
        let expected_len = u32::try_from(N).unwrap();
        assert_eq!(expected_len, actual_len);
        if expected_len == 0
        {
            assert!(actual_values_start.is_null());
            return;
        }
        if expected_len == actual_len
        {
            for i in 0..N
            {
                // SAFETY: caller promises that actual_values_start is valid for deference for N elements
                let actual_value_ptr = unsafe { actual_values_start.add(i) };
                assert!(!actual_value_ptr.is_null(), "Pointer at index {} should not be null", i);
                assert!(actual_value_ptr.is_aligned(), "Pointer at index {} should be aligned", i);
                // SAFETY: caller promises it's within lifetime. non-null and alignment is checked above
                let actual_value = unsafe { std::ptr::read(actual_value_ptr) };
            }
        }
    }
}
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