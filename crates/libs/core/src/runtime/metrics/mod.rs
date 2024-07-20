// mod for handling fabric metrics

// FABRIC_LOAD_METRIC

use std::marker::PhantomData;

use mssf_com::FabricTypes::FABRIC_LOAD_METRIC;
use windows_core::{HSTRING, PCWSTR};

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
