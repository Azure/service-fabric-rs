// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Module that defines metrics-related types through FabricClient

use std::time::SystemTime;

use mssf_com::{
    FabricClient::IFabricGetPartitionLoadInformationResult, FabricTypes::FABRIC_LOAD_METRIC_REPORT,
};
use windows_core::WString;

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::WStringWrap,
};

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
            name: WStringWrap::from(value.Name).into(),
            value: value.Value,
            last_reported_utc: SystemTime::UNIX_EPOCH, // TODO: convert Win32 FILETIME to SystemTime in Unix or Win32 depending on the platform
        }
    }
}

/// Wrapper of the load metric report list from the primary replica of the partition
pub struct PrimaryLoadMetricReportList {
    com: IFabricGetPartitionLoadInformationResult,
}

impl FabricListAccessor<FABRIC_LOAD_METRIC_REPORT> for PrimaryLoadMetricReportList {
    fn get_count(&self) -> u32 {
        unsafe {
            self.com
                .get_PartitionLoadInformation()
                .as_ref()
                .unwrap()
                .PrimaryLoadMetricReports
                .as_ref()
                .unwrap()
                .Count
        }
    }

    fn get_first_item(&self) -> *const FABRIC_LOAD_METRIC_REPORT {
        unsafe {
            self.com
                .get_PartitionLoadInformation()
                .as_ref()
                .unwrap()
                .PrimaryLoadMetricReports
                .as_ref()
                .unwrap()
                .Items
        }
    }
}

impl PrimaryLoadMetricReportList {
    pub fn new(com: IFabricGetPartitionLoadInformationResult) -> Self {
        Self { com }
    }

    pub fn iter(&self) -> PrimaryLoadMetricReportListIter {
        PrimaryLoadMetricReportListIter::new(self, self)
    }
}

/// Wrapper of the load metric report list from the secondary replicas of the partition
pub struct SecondaryLoadMetricReportList {
    com: IFabricGetPartitionLoadInformationResult,
}

impl FabricListAccessor<FABRIC_LOAD_METRIC_REPORT> for SecondaryLoadMetricReportList {
    fn get_count(&self) -> u32 {
        unsafe {
            self.com
                .get_PartitionLoadInformation()
                .as_ref()
                .unwrap()
                .SecondaryLoadMetricReports
                .as_ref()
                .unwrap()
                .Count
        }
    }

    fn get_first_item(&self) -> *const FABRIC_LOAD_METRIC_REPORT {
        unsafe {
            self.com
                .get_PartitionLoadInformation()
                .as_ref()
                .unwrap()
                .SecondaryLoadMetricReports
                .as_ref()
                .unwrap()
                .Items
        }
    }
}

impl SecondaryLoadMetricReportList {
    pub fn new(com: IFabricGetPartitionLoadInformationResult) -> Self {
        Self { com }
    }

    pub fn iter(&self) -> SecondaryLoadMetricReportListIter {
        SecondaryLoadMetricReportListIter::new(self, self)
    }
}

type PrimaryLoadMetricReportListIter<'a> =
    FabricIter<'a, FABRIC_LOAD_METRIC_REPORT, LoadMetricReport, PrimaryLoadMetricReportList>;
type SecondaryLoadMetricReportListIter<'a> =
    FabricIter<'a, FABRIC_LOAD_METRIC_REPORT, LoadMetricReport, SecondaryLoadMetricReportList>;
