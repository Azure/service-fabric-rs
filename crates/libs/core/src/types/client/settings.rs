// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
use std::{ffi::c_void, num::NonZeroU32, ptr};

use mssf_com::{
    FabricClient::IFabricClientSettings2,
    FabricTypes::{
        FABRIC_CLIENT_SETTINGS, FABRIC_CLIENT_SETTINGS_EX1, FABRIC_CLIENT_SETTINGS_EX2,
        FABRIC_CLIENT_SETTINGS_EX3, FABRIC_CLIENT_SETTINGS_EX4,
    },
};
use windows_core::{WString, PCWSTR};

use crate::strings::WStringWrap;

/// A idiomatic Rust version of FABRIC_CLIENT_SETTINGS
///
/// Note: you can default fields you're not interested in like so:
/// ```
/// let my_settings = FabricClientSettings {
///  PartitionLocationCacheLimit: Option<NonZeroU32>(NonZeroU32::new(1).expect("Non-zero value")),
///  ..Default::default()
/// };
/// ```
#[derive(Default, Clone)]
#[allow(non_snake_case, reason = "For consistency with underlying COM api")]
pub struct FabricClientSettings {
    // FabricClientSettings::FromPublicApi validates ranges for many of these.
    // Where possible, disallow trying to set values that will be rejected there anyway
    // And get a niche optimization at the same time
    // FABRIC_CLIENT_SETTINGS
    pub PartitionLocationCacheLimit: Option<NonZeroU32>,
    pub ServiceChangePollIntervalInSeconds: Option<NonZeroU32>,
    /// Note: ConnectionInitializationTimeoutInSeconds must be greater than or equal to ServiceChangePollIntervalInSecond
    // TODO: consider enforcing this before even calling into ServiceFabric?
    pub ConnectionInitializationTimeoutInSeconds: Option<NonZeroU32>,
    // TODO: document what 0 means for this value, it appears to be allowed
    pub KeepAliveIntervalInSeconds: Option<u32>,
    pub HealthOperationTimeoutInSeconds: Option<NonZeroU32>,
    // TODO: document what 0 means for this value, it appears to be allowed
    pub HealthReportSendIntervalInSeconds: Option<u32>,

    // FABRIC_CLIENT_SETTINGS_EX1
    /// May not be longer than 256 characters
    pub ClientFriendlyName: Option<WString>,

    pub PartitionLocationCacheBucketCount: Option<u32>,
    /// Note: 0 is accepted in the C++ API, but is replaced with th e default vlaue under the hood
    pub HealthReportRetrySendIntervalInSeconds: Option<NonZeroU32>,

    // FABRIC_CLIENT_SETTINGS_EX2
    pub NotificationGatewayConnectionTimeoutInSeconds: Option<NonZeroU32>,
    pub NotificationCacheUpdateTimeoutInSeconds: Option<NonZeroU32>,
    // FABRIC_CLIENT_SETTINGS_EX3
    // TODO: presumably Zero is not a valid value for this
    pub AuthTokenBufferSize: Option<u32>,

    // FABRIC_CLIENT_SETTINGS_EX4
    // Note: ConnectionIdleTimeoutInSeconds is deprecated and must be 0, so we don't expose it.

    //  TODO: we're missing FABRIC_CLIENT_SETTINGS_EX5 struct definition
    // FABRIC_CLIENT_SETTINGS_EX5
    pub AllowHealthReportCleanup: Option<bool>,
    pub HealthReportDropTransientReportTtlThresholdInSeconds: Option<u32>,
}

impl FabricClientSettings {
    /// FABRIC_CLIENT_SETTINGS and FABRIC_CLIENT_SETTINGS_EX*
    /// Uses a common Win32 API pattern to allow extensibility; each struct ends with an opaque pointer to the next extension, if supported
    /// This resuslts in a lot of repetitive, tricky unsafe code in Rust, but it follows a simple pattern.
    /// So encapsulate that pattern into a generic function.
    /// SAFETY: caller promises that the *mut core::ffi::c_void is actually of type Next
    unsafe fn get_next<Current, Next, F>(val: &Current, accessor: F) -> &Next
    where
        F: FnOnce(&Current) -> *mut core::ffi::c_void,
    {
        let reserved: *mut core::ffi::c_void = accessor(&val);
        // SAFETY: caller promises that the *mut c_void returned by accessor, if non-null, is actually a *mut Next
        let next_ptr: *mut Next = unsafe { std::mem::transmute(reserved) };
        // Even FABRIC_CLIENT_SETTINGS_EX5 is from 2020 (so 5 years old). If it's null, fatal error
        assert!(!next_ptr.is_null() && next_ptr.is_aligned());
        // SAFETY: pointer is valid and deferencable (null checked and alignment checked above)
        let next: &Next = unsafe { std::mem::transmute(next_ptr) };
        next
    }
}

impl From<&FABRIC_CLIENT_SETTINGS> for FabricClientSettings {
    fn from(val: &FABRIC_CLIENT_SETTINGS) -> Self {
        #![allow(non_snake_case, reason = "consistency with field definitions")]
        // This is just wordy enough to warrant a macro. Especially since if they are somehow zero, we'd like a nice message
        macro_rules! GetNonZeroU32 {
            ($parent:expr, $field:ident) => {
                Some(
                    NonZeroU32::new($parent.$field)
                        .expect(concat!(stringify!($field), " should be non-zero")),
                )
            };
        }

        // SAFETY: FABRIC_CLIENT_SETTINGS.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX1
        let ex1: &FABRIC_CLIENT_SETTINGS_EX1 =
            unsafe { Self::get_next(val, |x: &FABRIC_CLIENT_SETTINGS| x.Reserved) };
        // Note: it's critical that ex1 cannout outlive Result, as that's the only thing keeping ClientFriendlyName alive
        let client_friendly_name = WStringWrap::from(ex1.ClientFriendlyName).into_wstring();

        // SAFETY: FABRIC_CLIENT_SETTINGS_EX1.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX2
        let ex2: &FABRIC_CLIENT_SETTINGS_EX2 =
            unsafe { Self::get_next(ex1, |x: &FABRIC_CLIENT_SETTINGS_EX1| x.Reserved) };
        // SAFETY: FABRIC_CLIENT_SETTINGS_EX2.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX3
        let ex3: &FABRIC_CLIENT_SETTINGS_EX3 =
            unsafe { Self::get_next(ex2, |x: &FABRIC_CLIENT_SETTINGS_EX2| x.Reserved) };

        // FABRIC_CLIENT_SETTINGS_EX4 contained a single now-deprecated setting. We only need it to get the pointer to FABRIC_CLIENT_SETTINGS_EX5
        // SAFETY: FABRIC_CLIENT_SETTINGS_EX3.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX4
        let _ex4: &FABRIC_CLIENT_SETTINGS_EX4 =
            unsafe { Self::get_next(ex3, |x: &FABRIC_CLIENT_SETTINGS_EX3| x.Reserved) };

        FabricClientSettings {
            // FABRIC_CLIENT_SETTING
            PartitionLocationCacheLimit: GetNonZeroU32!(val, PartitionLocationCacheLimit),
            ServiceChangePollIntervalInSeconds: GetNonZeroU32!(
                val,
                ServiceChangePollIntervalInSeconds
            ),
            ConnectionInitializationTimeoutInSeconds: GetNonZeroU32!(
                val,
                ConnectionInitializationTimeoutInSeconds
            ),
            KeepAliveIntervalInSeconds: Some(val.KeepAliveIntervalInSeconds),
            HealthOperationTimeoutInSeconds: GetNonZeroU32!(val, HealthOperationTimeoutInSeconds),
            HealthReportSendIntervalInSeconds: Some(val.HealthReportSendIntervalInSeconds),
            // FABRIC_CLIENT_SETTINGS_EX1
            ClientFriendlyName: Some(client_friendly_name),
            PartitionLocationCacheBucketCount: Some(ex1.PartitionLocationCacheBucketCount),
            HealthReportRetrySendIntervalInSeconds: GetNonZeroU32!(
                ex1,
                HealthReportRetrySendIntervalInSeconds
            ),
            // FABRIC_CLIENT_SETTINGS_EX2
            NotificationGatewayConnectionTimeoutInSeconds: GetNonZeroU32!(
                ex2,
                NotificationGatewayConnectionTimeoutInSeconds
            ),
            NotificationCacheUpdateTimeoutInSeconds: GetNonZeroU32!(
                ex2,
                NotificationCacheUpdateTimeoutInSeconds
            ),
            // FABRIC_CLIENT_SETTINGS_EX3 only has a deprecated setting
            AuthTokenBufferSize: Some(ex3.AuthTokenBufferSize),
            // FABRIC_CLIENT_SETTINGS_EX4 only has a deprecated setting
            // FABRIC_CLIENT_SETTINGS_EX5
            // TODO: waiting on IDL update
            AllowHealthReportCleanup: None,
            HealthReportDropTransientReportTtlThresholdInSeconds: None,
        }
    }
}

impl FabricClientSettings {
    /// Get the current settings via the COM interface
    pub fn get_defaults(com: &IFabricClientSettings2) -> FabricClientSettings {
        // TODO: error handling?
        // SAFETY: IFabricClientSettings2 implements this COM interface
        // TODO: replace this with GetFabricClientDefaultSettings
        let result = unsafe { com.GetSettings() }.expect("GetSettings failed");
        // Note: inner scope outlives result, which is critical as otherwise we'd have UB.
        let converted = {
            // SAFETY: FABRIC_CLIENT_SETTINGS_EX1.ClientFriendlyName is only accessed while IFabricClientSettingsResult is in scope
            let ptr = unsafe { result.get_Settings() };
            assert!(!ptr.is_null() && ptr.is_aligned());
            // SAFETY: both preconditions are asserted above
            let my_ref: &FABRIC_CLIENT_SETTINGS = unsafe { std::mem::transmute(ptr) };
            FabricClientSettings::from(my_ref)
        };
        drop(result);
        converted
    }
}

/// Combine explicitly provided settings with current effective settings
/// TODO: tests
fn combine_settings_with_overrides(
    base_client_settings: FabricClientSettings,
    overlay_client_settings: FabricClientSettings,
) -> FabricClientSettings {
    fn merge_pair<T>(base: Option<T>, overlay: Option<T>) -> Option<T> {
        match &overlay {
            // If Set, overlay obviously wins
            Some(_) => overlay,
            // If Default, overlay has no value, use the lower priority value
            None => base,
        }
    }
    // This macro is maybe a bit unnecessary. But it means there's only 2 places that have to match up
    // When combined with long enough variable names, it wraps nicely and is legible
    // We could mutate the structure in place and reduce it to a single repetition of the field name,
    // but then it's easy to accidentally forget to add a new setting.
    macro_rules! Merge {
        ($base:expr, $overlay:expr, $field:ident) => {
            merge_pair($base.$field, $overlay.$field)
        };
    }
    FabricClientSettings {
        PartitionLocationCacheLimit: Merge!(
            base_client_settings,
            overlay_client_settings,
            PartitionLocationCacheLimit
        ),
        ServiceChangePollIntervalInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            ServiceChangePollIntervalInSeconds
        ),
        ConnectionInitializationTimeoutInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            ConnectionInitializationTimeoutInSeconds
        ),
        KeepAliveIntervalInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            KeepAliveIntervalInSeconds
        ),
        HealthOperationTimeoutInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            HealthOperationTimeoutInSeconds
        ),
        HealthReportSendIntervalInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            HealthReportSendIntervalInSeconds
        ),
        ClientFriendlyName: Merge!(
            base_client_settings,
            overlay_client_settings,
            ClientFriendlyName
        ),
        PartitionLocationCacheBucketCount: Merge!(
            base_client_settings,
            overlay_client_settings,
            PartitionLocationCacheBucketCount
        ),
        HealthReportRetrySendIntervalInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            HealthReportRetrySendIntervalInSeconds
        ),
        NotificationGatewayConnectionTimeoutInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            NotificationGatewayConnectionTimeoutInSeconds
        ),
        NotificationCacheUpdateTimeoutInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            NotificationCacheUpdateTimeoutInSeconds
        ),
        AuthTokenBufferSize: Merge!(
            base_client_settings,
            overlay_client_settings,
            AuthTokenBufferSize
        ),
        AllowHealthReportCleanup: Merge!(
            base_client_settings,
            overlay_client_settings,
            AllowHealthReportCleanup
        ),
        HealthReportDropTransientReportTtlThresholdInSeconds: Merge!(
            base_client_settings,
            overlay_client_settings,
            HealthReportDropTransientReportTtlThresholdInSeconds
        ),
    }
}

impl FabricClientSettings {
    /// Note: only overrides non-default settings; leaves any settings set previously that don't explicitly have new values alone
    pub fn set(&self, settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        // SAFETY: setting_interface implements the required COM interface.
        let existing_settings = FabricClientSettings::get_defaults(settings_interface);
        let new_settings = combine_settings_with_overrides(existing_settings, self.clone());
        new_settings.set_inner(settings_interface)
    }

    fn set_inner(&self, settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        // TODO: ex5 missing from IDL?
        let ex4 = FABRIC_CLIENT_SETTINGS_EX4 {
            // Deprecated, should always be zero
            ConnectionIdleTimeoutInSeconds: 0,
            Reserved: ptr::null_mut(),
        };

        let ex3 = FABRIC_CLIENT_SETTINGS_EX3 {
            AuthTokenBufferSize: self.AuthTokenBufferSize.unwrap(),
            Reserved: std::ptr::addr_of!(ex4) as *mut c_void,
        };

        let ex2 = FABRIC_CLIENT_SETTINGS_EX2 {
            NotificationGatewayConnectionTimeoutInSeconds: self
                .NotificationGatewayConnectionTimeoutInSeconds
                .unwrap()
                .into(),
            NotificationCacheUpdateTimeoutInSeconds: self
                .NotificationCacheUpdateTimeoutInSeconds
                .unwrap()
                .into(),
            Reserved: std::ptr::addr_of!(ex3) as *mut c_void,
        };
        // Note: &self reference ensures client_friendly_name is not mutable,
        // and remains valid for duration of this function
        // It should always be Some (since if SF gives back null, WStringWrap would have created an empty string)
        // But the SF API also treats null as if it were empty string, so if self.ClientFriendlyName is somehow None, we'll just map that to null.
        // SF side copies the string and does not retain a reference, so safety conditions are met.
        let client_friendly_name = self
            .ClientFriendlyName
            .as_ref()
            .map_or(PCWSTR::null(), |x| x.as_pcwstr());

        let ex1 = FABRIC_CLIENT_SETTINGS_EX1 {
            ClientFriendlyName: client_friendly_name,
            PartitionLocationCacheBucketCount: self.PartitionLocationCacheBucketCount.unwrap(),
            HealthReportRetrySendIntervalInSeconds: self
                .HealthReportRetrySendIntervalInSeconds
                .unwrap()
                .into(),
            Reserved: std::ptr::addr_of!(ex2) as *mut c_void,
        };

        let val = FABRIC_CLIENT_SETTINGS {
            PartitionLocationCacheLimit: self.PartitionLocationCacheLimit.unwrap().into(),
            ServiceChangePollIntervalInSeconds: self
                .ServiceChangePollIntervalInSeconds
                .unwrap()
                .into(),
            ConnectionInitializationTimeoutInSeconds: self
                .ConnectionInitializationTimeoutInSeconds
                .unwrap()
                .into(),
            KeepAliveIntervalInSeconds: self.KeepAliveIntervalInSeconds.unwrap(),
            HealthOperationTimeoutInSeconds: self.HealthOperationTimeoutInSeconds.unwrap().into(),
            HealthReportSendIntervalInSeconds: self.HealthReportSendIntervalInSeconds.unwrap(),
            Reserved: std::ptr::addr_of!(ex1) as *mut c_void,
        };

        // CALL THE FUNCTION:
        let val_ptr = std::ptr::addr_of!(val);
        // SAFETY: val is valid for the duration of the call
        unsafe { settings_interface.SetSettings(val_ptr) }
    }
}
