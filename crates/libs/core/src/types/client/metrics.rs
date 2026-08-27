// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Module that defines metrics-related types through FabricClient

use std::time::SystemTime;

use crate::WString;
use crate::time::filetime_to_system_time;
use mssf_com::FabricTypes::FABRIC_LOAD_METRIC_REPORT;

/// Wrapper for FABRIC_LOAD_METRIC_REPORT
#[derive(Debug, Clone)]
pub struct LoadMetricReport {
    pub name: WString,
    pub value: u32,
    pub last_reported_utc: std::time::SystemTime,
}

impl From<&FABRIC_LOAD_METRIC_REPORT> for LoadMetricReport {
    fn from(value: &FABRIC_LOAD_METRIC_REPORT) -> Self {
        Self {
            name: WString::from(value.Name),
            value: value.Value,
            last_reported_utc: filetime_to_system_time(value.LastReportedUtc)
                .unwrap_or(SystemTime::UNIX_EPOCH),
        }
    }
}
