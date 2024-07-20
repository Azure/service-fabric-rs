// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// mod for handling fabric metrics
use mssf_com::FabricTypes::FABRIC_LOAD_METRIC;
use std::marker::PhantomData;
use windows_core::{HSTRING, PCWSTR};

// FABRIC_LOAD_METRIC
pub struct LoadMetric {
    // TODO: support static string without heap allocation
    pub name: HSTRING,
    pub value: u32,
}

impl LoadMetric {
    pub fn new(name: HSTRING, value: u32) -> Self {
        Self { name, value }
    }
}

// result has the lifetime of the original value
impl From<&LoadMetric> for FABRIC_LOAD_METRIC {
    fn from(value: &LoadMetric) -> Self {
        Self {
            Name: PCWSTR(value.name.as_ptr()),
            Value: value.value,
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// Temporary type to hold the buffer of raw metrics
/// passed into SF api.
pub struct LoadMetricListRef<'a> {
    metrics: Vec<FABRIC_LOAD_METRIC>,
    owner: PhantomData<&'a Vec<LoadMetric>>,
}

impl<'a> LoadMetricListRef<'a> {
    pub fn from_slice(v: &[LoadMetric]) -> Self {
        let metrics = v.iter().map(FABRIC_LOAD_METRIC::from).collect::<Vec<_>>();
        Self {
            metrics,
            owner: PhantomData,
        }
    }

    // Get the raw slice for passing into SF com api.
    pub fn as_raw_slice(&self) -> &[FABRIC_LOAD_METRIC] {
        self.metrics.as_slice()
    }
}
