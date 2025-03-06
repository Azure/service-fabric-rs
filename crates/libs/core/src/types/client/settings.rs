// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
use std::{num::NonZeroU32, ptr};

use mssf_com::{
    FabricClient::{IFabricClientSettings2, IFabricClientSettingsResult},
    FabricTypes::{
        FABRIC_CLIENT_SETTINGS, FABRIC_CLIENT_SETTINGS_EX1, FABRIC_CLIENT_SETTINGS_EX2,
        FABRIC_CLIENT_SETTINGS_EX3, FABRIC_CLIENT_SETTINGS_EX4,
    },
};
use windows_core::WString;

use crate::strings::WStringWrap;

/// Represents the value of a client setting
///
pub enum FabricClientSettingValue<T> {
    /// Set the value to the provided value
    Set(T),
    /// Value was retrieved from Service Fabric
    Retrieved(T),
    /// Use whatever value GetSettings returns
    Default,
    /// The version of Service Fabric Client found at runtime does not support this setting
    Unsupported,
}

impl<T> Default for FabricClientSettingValue<T> {
    fn default() -> Self {
        Self::Default
    }
}

/// A idiomatic Rust version of FABRIC_CLIENT_SETTINGS
#[allow(non_snake_case, reason = "For consistency with underlying COM api")]
pub struct FabricClientSettings {
    // FabricClientSettings::FromPublicApi validates ranges for many of these.
    // Where possible, disallow trying to set values that will be rejected there anyway
    // And get a niche optimization at the same time
    // FABRIC_CLIENT_SETTINGS
    pub PartitionLocationCacheLimit: FabricClientSettingValue<NonZeroU32>,
    pub ServiceChangePollIntervalInSeconds: FabricClientSettingValue<NonZeroU32>,
    /// Note: ConnectionInitializationTimeoutInSeconds must be greater than or equal to ServiceChangePollIntervalInSecond
    // TODO: consider enforcing this before even calling into ServiceFabric?
    pub ConnectionInitializationTimeoutInSeconds: FabricClientSettingValue<NonZeroU32>,
    // TODO: document what 0 means for this value, it appears to be allowed
    pub KeepAliveIntervalInSeconds: FabricClientSettingValue<u32>,
    pub HealthOperationTimeoutInSeconds: FabricClientSettingValue<NonZeroU32>,
    // TODO: document what 0 means for this value, it appears to be allowed
    pub HealthReportSendIntervalInSeconds: FabricClientSettingValue<u32>,

    // FABRIC_CLIENT_SETTINGS_EX1
    /// May not be longer than 256 characters
    pub ClientFriendlyName: FabricClientSettingValue<WString>,

    pub PartitionLocationCacheBucketCount: FabricClientSettingValue<u32>,
    /// Note: 0 is accepted in the C++ API, but is replaced with th e default vlaue under the hood
    pub HealthReportRetrySendIntervalInSeconds: FabricClientSettingValue<NonZeroU32>,

    // FABRIC_CLIENT_SETTINGS_EX2
    pub NotificationGatewayConnectionTimeoutInSeconds: FabricClientSettingValue<NonZeroU32>,
    pub NotificationCacheUpdateTimeoutInSeconds: FabricClientSettingValue<NonZeroU32>,
    // FABRIC_CLIENT_SETTINGS_EX3
    // TODO: presumably Zero is not a valid value for this
    pub AuthTokenBufferSize: FabricClientSettingValue<u32>,

    // FABRIC_CLIENT_SETTINGS_EX4
    // Note: ConnectionIdleTimeoutInSeconds is deprecated and must be 0, so we don't expose it.

    //  TODO: we're missing FABRIC_CLIENT_SETTINGS_EX5 struct definition
    // FABRIC_CLIENT_SETTINGS_EX5
    pub AllowHealthReportCleanup: FabricClientSettingValue<bool>,
    pub HealthReportDropTransientReportTtlThresholdInSeconds: FabricClientSettingValue<u32>,
}

impl FabricClientSettings {
    /// FABRIC_CLIENT_SETTINGS and FABRIC_CLIENT_SETTINGS_EX*
    /// Uses a common Win32 API pattern to allow extensibility; each struct ends with an opaque pointer to the next extension, if supported
    /// This reuslts in a lot of repetitive, tricky unsafe code in Rust, but it follows a simple pattern.
    /// So encapsulate that pattern into a generic function.
    /// SAFETY: caller promises that the *mut core::ffi::c_void is actually of type Next
    unsafe fn get_next<Current, Next, F>(input: Option<Current>, accessor: F) -> Option<Next>
    where
        Next: Copy + Clone,
        F: FnOnce(&Current) -> *mut core::ffi::c_void,
    {
        input
            .map(|val| {
                let reserved: *mut core::ffi::c_void = accessor(&val);
                if !reserved.is_null() {
                    // SAFETY: caller promises that the *mut c_void returned by accessor, if non-null, is actually a *mut Next
                    let next_ptr: *mut Next = unsafe { std::mem::transmute(reserved) };
                    assert!(next_ptr.is_aligned());
                    // SAFETY: pointer is valid and deferencable (null checked and alignment checked above)
                    let next = unsafe { ptr::read(next_ptr) };
                    Some(next)
                } else {
                    None
                }
            })
            .flatten()
    }

    /// Inner scope; helps enforce IFabricClientSettingsResult outliving the derived pointers
    fn get_from_com_inner(result: &mut IFabricClientSettingsResult) -> FabricClientSettings {
        #![allow(non_snake_case, reason = "consistency with field definitions")]
        // SAFETY: FABRIC_CLIENT_SETTINGS_EX1.ClientFriendlyName is only accessed while IFabricClientSettingsResult is in scope
        let ptr = unsafe { result.get_Settings() };
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());
        // SAFETY: ptr is not null, deferenceable, and not mutated concurrently
        // Note: this read/copy doesn't free us from lifetime concerns, as there are heap-allocated string pointers e.g. in FABRIC_CLIENT_SETTINGS_EX1.
        let val = unsafe { ptr::read(ptr) };

        

        // FABRIC_CLIENT_SETTING
        let PartitionLocationCacheLimit = FabricClientSettingValue::Retrieved(
            NonZeroU32::new(val.PartitionLocationCacheLimit)
                .expect("PartitionLocationCacheLimit should be non-zero"),
        );
        let ServiceChangePollIntervalInSeconds = FabricClientSettingValue::Retrieved(
            NonZeroU32::new(val.ServiceChangePollIntervalInSeconds)
                .expect("ServiceChangePollIntervalInSeconds should be non-zero"),
        );
        let ConnectionInitializationTimeoutInSeconds = FabricClientSettingValue::Retrieved(
            NonZeroU32::new(val.ConnectionInitializationTimeoutInSeconds)
                .expect("ConnectionInitializationTimeoutInSeconds should be non-zero"),
        );
        let KeepAliveIntervalInSeconds =
            FabricClientSettingValue::Retrieved(val.KeepAliveIntervalInSeconds);
        let HealthOperationTimeoutInSeconds = FabricClientSettingValue::Retrieved(
            NonZeroU32::new(val.HealthOperationTimeoutInSeconds)
                .expect("HealthOperationTimeoutInSeconds should be non-zero"),
        );
        let HealthReportSendIntervalInSeconds =
            FabricClientSettingValue::Retrieved(val.HealthReportSendIntervalInSeconds);

        // FABRIC_CLIENT_SETTINGS_EX1
        let ex1: Option<FABRIC_CLIENT_SETTINGS_EX1> =
            unsafe { Self::get_next(Some(val), |x: &FABRIC_CLIENT_SETTINGS| x.Reserved) };
        // Note: it's critical that ex1 cannout outlive Result, as that's the only thing keeping ClientFriendlyName alive
        let ClientFriendlyName = ex1.map_or(FabricClientSettingValue::Unsupported, |v| {
            FabricClientSettingValue::Retrieved(
                WStringWrap::from(v.ClientFriendlyName).into_wstring(),
            )
        });
        let PartitionLocationCacheBucketCount = ex1
            .map_or(FabricClientSettingValue::Unsupported, |v| {
                FabricClientSettingValue::Retrieved(v.PartitionLocationCacheBucketCount)
            });
        let HealthReportRetrySendIntervalInSeconds =
            ex1.map_or(FabricClientSettingValue::Unsupported, |v| {
                FabricClientSettingValue::Retrieved(
                    NonZeroU32::new(v.HealthReportRetrySendIntervalInSeconds)
                        .expect("HealthReportRetrySendIntervalInSeconds should be non-zero"),
                )
            });

        // FABRIC_CLIENT_SETTINGS_EX2
        let ex2: Option<FABRIC_CLIENT_SETTINGS_EX2> =
            unsafe { Self::get_next(ex1, |x: &FABRIC_CLIENT_SETTINGS_EX1| x.Reserved) };
        let NotificationGatewayConnectionTimeoutInSeconds =
            ex2.map_or(FabricClientSettingValue::Unsupported, |v| {
                FabricClientSettingValue::Retrieved(
                    NonZeroU32::new(v.NotificationGatewayConnectionTimeoutInSeconds)
                        .expect("NotificationGatewayConnectionTimeoutInSeconds should be non-zero"),
                )
            });
        let NotificationCacheUpdateTimeoutInSeconds =
            ex2.map_or(FabricClientSettingValue::Unsupported, |v| {
                FabricClientSettingValue::Retrieved(
                    NonZeroU32::new(v.NotificationCacheUpdateTimeoutInSeconds)
                        .expect("NotificationCacheUpdateTimeoutInSeconds should be non-zero"),
                )
            });

        // FABRIC_CLIENT_SETTINGS_EX3
        let ex3: Option<FABRIC_CLIENT_SETTINGS_EX3> =
            unsafe { Self::get_next(ex2, |x: &FABRIC_CLIENT_SETTINGS_EX2| x.Reserved) };
        let AuthTokenBufferSize = ex3.map_or(FabricClientSettingValue::Unsupported, |v| {
            FabricClientSettingValue::Retrieved(v.AuthTokenBufferSize)
        });

        // FABRIC_CLIENT_SETTINGS_EX4 contained a single now-deprecated setting
        let _ex4: Option<FABRIC_CLIENT_SETTINGS_EX4> =
            unsafe { Self::get_next(ex3, |x: &FABRIC_CLIENT_SETTINGS_EX3| x.Reserved) };

        // FABRIC_CLIENT_SETTINGS_EX5
        // TODO: waiting on IDL update
        let AllowHealthReportCleanup = FabricClientSettingValue::Unsupported;
        let HealthReportDropTransientReportTtlThresholdInSeconds =
            FabricClientSettingValue::Unsupported;

        FabricClientSettings {
            PartitionLocationCacheLimit,
            ServiceChangePollIntervalInSeconds,
            ConnectionInitializationTimeoutInSeconds,
            KeepAliveIntervalInSeconds,
            HealthOperationTimeoutInSeconds,
            HealthReportSendIntervalInSeconds,
            ClientFriendlyName,
            PartitionLocationCacheBucketCount,
            HealthReportRetrySendIntervalInSeconds,
            NotificationGatewayConnectionTimeoutInSeconds,
            NotificationCacheUpdateTimeoutInSeconds,
            AuthTokenBufferSize,
            AllowHealthReportCleanup,
            HealthReportDropTransientReportTtlThresholdInSeconds,
        }
    }

    pub fn get_from_com(com: &IFabricClientSettings2) -> FabricClientSettings {
        // TODO: error handling?
        // SAFETY: IFabricClientSettings2 implements this COM interface
        let mut result = unsafe { com.GetSettings() }.expect("GetSettings failed");
        Self::get_from_com_inner(&mut result)
    }
}
