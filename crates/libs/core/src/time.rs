// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use windows_core::Win32::Foundation::FILETIME;

/// A [`FILETIME`] tick is 100 nanoseconds
const FILETIME_NS_PER_TICK: u64 = 100;
const FILETIME_TICKS_PER_SECOND: u64 = 10_000_000;
/// Number of [`FILETIME`] ticks between its epoch (`1601-01-01 00:00:00 UTC`)
/// and Rust's [`UNIX_EPOCH`] (`1970-01-01 00:00:00 UTC`). The two epochs are
/// 134,774 days, or 11,644,473,600 seconds, apart:
/// `11,644,473,600 seconds * 10,000,000 ticks/second`.
const FILETIME_UNIX_EPOCH_OFFSET_TICKS: u64 = 116_444_736_000_000_000;

/// Converts a Windows [`FILETIME`] to a [`SystemTime`], preserving its
/// 100-nanosecond precision. Both epochs denote UTC instants, although
/// [`SystemTime`] itself has no timezone. Returns [`None`] when the result is
/// outside the range supported by the current platform.
pub(crate) fn try_filetime_to_system_time(filetime: FILETIME) -> Option<SystemTime> {
    // FILETIME represents one unsigned 64-bit tick count as separate high and
    // low 32-bit words. Shifting the high word restores bits 32 through 63.
    let ticks = (u64::from(filetime.dwHighDateTime) << 32) | u64::from(filetime.dwLowDateTime);

    // Express the value as a positive duration on the appropriate side of the
    // Unix epoch so this also handles FILETIME values before 1970.
    let (after_unix_epoch, delta_ticks) = if ticks >= FILETIME_UNIX_EPOCH_OFFSET_TICKS {
        (true, ticks - FILETIME_UNIX_EPOCH_OFFSET_TICKS)
    } else {
        (false, FILETIME_UNIX_EPOCH_OFFSET_TICKS - ticks)
    };

    // Duration stores whole seconds and a nanosecond remainder.
    let delta = Duration::new(
        delta_ticks / FILETIME_TICKS_PER_SECOND,
        ((delta_ticks % FILETIME_TICKS_PER_SECOND) * FILETIME_NS_PER_TICK) as u32,
    );

    if after_unix_epoch {
        UNIX_EPOCH.checked_add(delta)
    } else {
        UNIX_EPOCH.checked_sub(delta)
    }
}
