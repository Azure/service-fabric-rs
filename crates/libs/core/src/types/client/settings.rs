// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
use std::{ffi::c_void, num::NonZeroU32, ptr};

use mssf_com::{
    FabricClient::{IFabricClientSettings2, IFabricClientSettingsResult},
    FabricTypes::{
        FABRIC_CLIENT_SETTINGS, FABRIC_CLIENT_SETTINGS_EX1, FABRIC_CLIENT_SETTINGS_EX2,
        FABRIC_CLIENT_SETTINGS_EX3, FABRIC_CLIENT_SETTINGS_EX4,
    },
};
use windows_core::WString;

use crate::strings::WStringWrap;

/// What level of Fabric Client setting support does the client library have?
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[non_exhaustive]
pub enum FabricClientSettingSupportLevel {
    Unknown = -1,
    Basic = 0,
    EX1 = 1,
    EX2 = 2,
    EX3 = 3,
    EX4 = 4,
    #[allow(dead_code, reason = "TODO: idl update")]
    EX5 = 5,
}

impl Default for FabricClientSettingSupportLevel {
    fn default() -> Self {
        FabricClientSettingSupportLevel::Unknown
    }
}

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
    /// What level of support does the client library have?
    /// Should be left as FabricClientSettingSupportLevel::default()
    /// As explicitly setting a value in user code has no effect.
    pub SupportLevel: FabricClientSettingSupportLevel,
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
        let valRead = unsafe { ptr::read(ptr) };
        let mut support_level = FabricClientSettingSupportLevel::Basic;
        // Always Some, but makes it cleaner if we reuse the optional handling
        let val = Some(valRead);

        // Handle type conversion
        macro_rules! SettingParse {
            (NonZeroU32, $e:expr, $field:ident) => { NonZeroU32::new($e).expect(concat!(stringify!($field), " should be non-zero")) };
            (u32, $e:expr, $field:ident) => { $e };
            {WString, $e:expr, $field:ident} => { WStringWrap::from($e).into_wstring()};
        }

        // Handle the possibility an _EX field might be unsupported
        macro_rules! Setting {
            ($setting_ty:tt, $parent:expr, $field:ident) => {
                $parent.map_or(None, |v| Some(SettingParse!($setting_ty, v.$field, $field)))
            };
        }

        // FABRIC_CLIENT_SETTING
        let PartitionLocationCacheLimit = Setting!(NonZeroU32, val, PartitionLocationCacheLimit);
        let ServiceChangePollIntervalInSeconds =
            Setting!(NonZeroU32, val, ServiceChangePollIntervalInSeconds);
        let ConnectionInitializationTimeoutInSeconds =
            Setting!(NonZeroU32, val, ConnectionInitializationTimeoutInSeconds);
        let KeepAliveIntervalInSeconds = Setting!(u32, val, KeepAliveIntervalInSeconds);

        let HealthOperationTimeoutInSeconds =
            Setting!(NonZeroU32, val, HealthOperationTimeoutInSeconds);
        let HealthReportSendIntervalInSeconds =
            Setting!(u32, val, HealthReportSendIntervalInSeconds);

        // SAFETY: FABRIC_CLIENT_SETTINGS.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX1
        let ex1: Option<FABRIC_CLIENT_SETTINGS_EX1> =
            unsafe { Self::get_next(val, |x: &FABRIC_CLIENT_SETTINGS| x.Reserved) };
        if ex1.is_some() {
            support_level = FabricClientSettingSupportLevel::EX1;
        }
        // Note: it's critical that ex1 cannout outlive Result, as that's the only thing keeping ClientFriendlyName alive
        let ClientFriendlyName = Setting!(WString, ex1, ClientFriendlyName);
        let PartitionLocationCacheBucketCount =
            Setting!(u32, ex1, PartitionLocationCacheBucketCount);
        let HealthReportRetrySendIntervalInSeconds =
            Setting!(NonZeroU32, ex1, HealthReportRetrySendIntervalInSeconds);

        // SAFETY: FABRIC_CLIENT_SETTINGS_EX1.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX2
        let ex2: Option<FABRIC_CLIENT_SETTINGS_EX2> =
            unsafe { Self::get_next(ex1, |x: &FABRIC_CLIENT_SETTINGS_EX1| x.Reserved) };
        if ex2.is_some() {
            support_level = FabricClientSettingSupportLevel::EX2;
        }
        let NotificationGatewayConnectionTimeoutInSeconds = Setting!(
            NonZeroU32,
            ex2,
            NotificationGatewayConnectionTimeoutInSeconds
        );
        let NotificationCacheUpdateTimeoutInSeconds =
            Setting!(NonZeroU32, ex2, NotificationCacheUpdateTimeoutInSeconds);

        // SAFETY: FABRIC_CLIENT_SETTINGS_EX2.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX3
        let ex3: Option<FABRIC_CLIENT_SETTINGS_EX3> =
            unsafe { Self::get_next(ex2, |x: &FABRIC_CLIENT_SETTINGS_EX2| x.Reserved) };
        if ex3.is_some() {
            support_level = FabricClientSettingSupportLevel::EX3;
        }
        let AuthTokenBufferSize = Setting!(u32, ex3, AuthTokenBufferSize);

        // SAFETY: FABRIC_CLIENT_SETTINGS_EX3.Reserved, if non-null, is really a *mut FABRIC_CLIENT_SETTINGS_EX4
        let ex4: Option<FABRIC_CLIENT_SETTINGS_EX4> =
            unsafe { Self::get_next(ex3, |x: &FABRIC_CLIENT_SETTINGS_EX3| x.Reserved) };
        if ex4.is_some() {
            support_level = FabricClientSettingSupportLevel::EX4;
        }
        // FABRIC_CLIENT_SETTINGS_EX4 contained a single now-deprecated setting. We only need it to get the pointer to FABRIC_CLIENT_SETTINGS_EX5

        // FABRIC_CLIENT_SETTINGS_EX5
        // TODO: waiting on IDL update
        let AllowHealthReportCleanup = None;
        let HealthReportDropTransientReportTtlThresholdInSeconds = None;

        FabricClientSettings {
            SupportLevel: support_level,
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

    /// Get the current settings via the COM interface
    pub fn get_from_com(com: &IFabricClientSettings2) -> FabricClientSettings {
        // TODO: error handling?
        // SAFETY: IFabricClientSettings2 implements this COM interface
        let mut result = unsafe { com.GetSettings() }.expect("GetSettings failed");
        Self::get_from_com_inner(&mut result)
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
        SupportLevel: base_client_settings.SupportLevel,
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

/// Helpers for calling SetSettings

/// Get a pointer to the contents of a Box if Some, else ptr::null_mut()
/// Safe because the pointer lives as long as the mutable borrow, so there are no additional preconditions implied
fn option_box_into_raw<T>(maybe_box: Option<Box<T>>) -> *mut c_void {
    maybe_box.map_or(ptr::null_mut(), Box::into_raw) as *mut c_void
}

/// SAFETY: caller asserts that maybe_ptr either points to dereferenceable, unaliased T instance, or is nullptr
unsafe fn option_box_from_raw<T>(maybe_ptr: *mut c_void) -> Option<Box<T>> {
    if maybe_ptr.is_null() {
        None
    } else {
        // Would imply memory corruption or allocator not respecting alignment
        assert!(maybe_ptr.is_aligned());
        // SAFETY: caller promises that the pointer is to *mut T if not nullptr, and we checked nullptr above
        let definitely_box = unsafe { Box::from_raw(maybe_ptr as *mut T) };
        Some(definitely_box)
    }
}

/// Get a required setting
fn get_required<T: Copy>(val: &Option<T>) -> T {
    match val {
        Some(v) => *v,
        None => {
            panic!("Required setting")
        }
    }
}

impl FabricClientSettings {
    /// Note: only overrides non-default settings; leaves any settings set previously that don't explicitly have new values alone
    pub fn set(&self, settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        // SAFETY: setting_interface implements the required COM interface.
        let existing_settings = FabricClientSettings::get_from_com(&settings_interface);
        let new_settings = combine_settings_with_overrides(existing_settings, self.clone());
        new_settings.set_inner(settings_interface)
    }

    fn set_inner(&self, settings_interface: &IFabricClientSettings2) -> windows_core::Result<()> {
        assert!(self.SupportLevel >= FabricClientSettingSupportLevel::Basic);

        // This is... rather tricky code. But it makes for a very clean API for end users, so we live with it.
        // The basic idea is for each group of settings the client understands, we allocate the struct
        // Going in reverse order, (FABRIC_CLIENT_SETTINGS_EXN -> FABRIC_CLIENT_SETTINGS_EX1 -> FABRIC_CLIENT_SETTINGS)
        // as this allows us to convert each Box into an owning raw pointer as we go down
        // In a single pass.
        //
        // Then, we call the function
        //
        // However, to make sure we don't violate Shared ^ Mutable, we don't retain the raw pointers
        // So we then walk the chain of structures in the forward direction
        // (i.e. FABRIC_CLIENT_SETTINGS -> FABRIC_CLIENT_SETTINGS_EX1 -> FABRIC_CLIENT_SETTINGS_EXN)
        //
        // It's probably possible to avoid the heap allocations with more work (maybe via Pin??), while still avoiding them moving while we have pointers to them.
        // But it's doubtful this function is called enough to be worth it.

        // BUILD STRUCTS
        let ex4 = if self.SupportLevel >= FabricClientSettingSupportLevel::EX4 {
            Some(Box::new(FABRIC_CLIENT_SETTINGS_EX4 {
                ConnectionIdleTimeoutInSeconds: get_required(
                    &self.ConnectionInitializationTimeoutInSeconds,
                )
                .into(),
                Reserved: ptr::null_mut(),
            }))
        } else {
            None
        };

        let ex3 = if self.SupportLevel >= FabricClientSettingSupportLevel::EX3 {
            Some(Box::new(FABRIC_CLIENT_SETTINGS_EX3 {
                AuthTokenBufferSize: get_required(&self.AuthTokenBufferSize),
                Reserved: option_box_into_raw(ex4),
            }))
        } else {
            None
        };

        let ex2 = if self.SupportLevel >= FabricClientSettingSupportLevel::EX2 {
            Some(Box::new(FABRIC_CLIENT_SETTINGS_EX2 {
                NotificationGatewayConnectionTimeoutInSeconds: get_required(
                    &self.NotificationGatewayConnectionTimeoutInSeconds,
                )
                .into(),
                NotificationCacheUpdateTimeoutInSeconds: get_required(
                    &self.NotificationCacheUpdateTimeoutInSeconds,
                )
                .into(),
                Reserved: option_box_into_raw(ex3),
            }))
        } else {
            None
        };

        let ex1 = if self.SupportLevel >= FabricClientSettingSupportLevel::EX1 {
            // Note: &self reference ensures client_friendly_name is not mutable,
            // and remains valid for duration of this function
            // SF side copies the string and does not retain a reference, so safety conditions are met.
            // TODO: this as_pcwstr function should be unsafe
            let client_friendly_name = match &self.ClientFriendlyName {
                Some(v) => v.as_pcwstr(),
                None => {
                    panic!("Required setting")
                }
            };
            Some(Box::new(FABRIC_CLIENT_SETTINGS_EX1 {
                ClientFriendlyName: client_friendly_name,
                PartitionLocationCacheBucketCount: get_required(
                    &self.PartitionLocationCacheBucketCount,
                ),
                HealthReportRetrySendIntervalInSeconds: get_required(
                    &self.HealthReportRetrySendIntervalInSeconds,
                )
                .into(),
                Reserved: option_box_into_raw(ex2),
            }))
        } else {
            None
        };

        let val = Box::new(FABRIC_CLIENT_SETTINGS {
            PartitionLocationCacheLimit: get_required(&self.PartitionLocationCacheLimit).into(),
            ServiceChangePollIntervalInSeconds: get_required(
                &self.ServiceChangePollIntervalInSeconds,
            )
            .into(),
            ConnectionInitializationTimeoutInSeconds: get_required(
                &self.ConnectionInitializationTimeoutInSeconds,
            )
            .into(),
            KeepAliveIntervalInSeconds: get_required(&self.KeepAliveIntervalInSeconds).into(),
            HealthOperationTimeoutInSeconds: get_required(&self.HealthOperationTimeoutInSeconds)
                .into(),
            HealthReportSendIntervalInSeconds: get_required(
                &self.HealthReportSendIntervalInSeconds,
            )
            .into(),
            Reserved: option_box_into_raw(ex1),
        });
        let raw_val = Box::into_raw(val);

        // CALL THE FUNCTION:
        // SAFETY: raw_val is valid for the duration of the call
        let result =
            unsafe { settings_interface.SetSettings(raw_val as *const FABRIC_CLIENT_SETTINGS) };

        // Walk the structs forward, avoiding any possibility of dangling reverses
        // The drop statements and asserts are probably overkill,
        // but this is pretty gnarly and this function should be quite cold.
        // SAFETY: raw_val is a valid FABRIC_CLIENT_SETTINGS and not aliased, as SetSettings does not retain the pointer
        let reboxed = unsafe { Box::from_raw(raw_val) };

        // SAFETY: if FABRIC_CLIENT_SETTINGS.Reserved is non-null, it points to a FABRIC_CLIENT_SETTINGS_EX1 allocated above
        let maybe_ex1: Option<Box<FABRIC_CLIENT_SETTINGS_EX1>> =
            unsafe { option_box_from_raw(reboxed.Reserved) };
        assert_eq!(
            maybe_ex1.is_some(),
            self.SupportLevel >= FabricClientSettingSupportLevel::EX1
        );
        // Make absolutely sure FABRIC_CLIENT_SETTINGS.Reserved can't become a dangling pointer
        drop(reboxed);

        let maybe_ex2: Option<Box<FABRIC_CLIENT_SETTINGS_EX2>> = maybe_ex1
            .as_ref()
            .map(|ex1| {
                // SAFETY: if FABRIC_CLIENT_SETTINGS_EX1.Reserved is non-null, it points to a FABRIC_CLIENT_SETTINGS_EX2 allocated above
                unsafe { option_box_from_raw(ex1.Reserved) }
            })
            .flatten();
        assert_eq!(
            maybe_ex2.is_some(),
            self.SupportLevel >= FabricClientSettingSupportLevel::EX2
        );
        // Make absolutely sure FABRIC_CLIENT_SETTINGS_EX1.Reserved can't become a dangling pointer
        drop(maybe_ex1);

        let maybe_ex3: Option<Box<FABRIC_CLIENT_SETTINGS_EX3>> = maybe_ex2
            .as_ref()
            .map(|ex2| {
                // SAFETY: if FABRIC_CLIENT_SETTINGS_EX2.Reserved is non-null, it points to a FABRIC_CLIENT_SETTINGS_EX3 allocated above
                unsafe { option_box_from_raw(ex2.Reserved) }
            })
            .flatten();
        assert_eq!(
            maybe_ex3.is_some(),
            self.SupportLevel >= FabricClientSettingSupportLevel::EX3
        );
        // Make absolutely sure FABRIC_CLIENT_SETTINGS_EX2.Reserved can't become a dangling pointer
        drop(maybe_ex2);

        let maybe_ex4: Option<Box<FABRIC_CLIENT_SETTINGS_EX3>> = maybe_ex3
            .as_ref()
            .map(|ex3| {
                // SAFETY: if FABRIC_CLIENT_SETTINGS_EX3.Reserved is non-null, it points to a FABRIC_CLIENT_SETTINGS_EX4 allocated above
                unsafe { option_box_from_raw(ex3.Reserved) }
            })
            .flatten();
        assert_eq!(
            maybe_ex4.is_some(),
            self.SupportLevel >= FabricClientSettingSupportLevel::EX4
        );
        // Make absolutely sure FABRIC_CLIENT_SETTINGS_EX2.Reserved can't become a dangling pointer
        drop(maybe_ex3);

        // The last EX is special, should not have anything following it
        if let Some(ex4) = maybe_ex4.as_ref() {
            // Very paranoid, but if this isn't null, something has gone very wrong
            assert!(ex4.Reserved.is_null())
        }
        // Drop the outermost EX for good measure
        drop(maybe_ex4);

        result
    }
}
