// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Module for handling fabric metrics

use crate::{WString, PCWSTR};
use mssf_com::FabricTypes::{
    FABRIC_LOAD_METRIC, FABRIC_MOVE_COST, FABRIC_MOVE_COST_HIGH, FABRIC_MOVE_COST_LOW,
    FABRIC_MOVE_COST_MEDIUM, FABRIC_MOVE_COST_ZERO,
};
use std::marker::PhantomData;

/// FABRIC_LOAD_METRIC
pub struct LoadMetric {
    // TODO: support static string without heap allocation
    pub name: WString,
    pub value: u32,
}

impl LoadMetric {
    pub fn new(name: WString, value: u32) -> Self {
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

/// Temporary type to hold the buffer of raw metrics passed into Service Fabric API call.
pub struct LoadMetricListRef<'a> {
    metrics: Vec<FABRIC_LOAD_METRIC>,
    owner: PhantomData<&'a [LoadMetric]>,
}

impl LoadMetricListRef<'_> {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MoveCost {
    #[default]
    Zero,
    Low,
    Medium,
    High,
    // VeryHigh,
}

impl From<FABRIC_MOVE_COST> for MoveCost {
    fn from(value: FABRIC_MOVE_COST) -> Self {
        match value {
            FABRIC_MOVE_COST_ZERO => Self::Zero,
            FABRIC_MOVE_COST_LOW => Self::Low,
            FABRIC_MOVE_COST_MEDIUM => Self::Medium,
            FABRIC_MOVE_COST_HIGH => Self::High,
            // Not supported in rust yet
            // FABRIC_MOVE_COST_VERYHIGH =>Self::VeryHigh,
            _ => Self::Zero,
        }
    }
}

impl From<MoveCost> for FABRIC_MOVE_COST {
    fn from(value: MoveCost) -> Self {
        match value {
            MoveCost::Zero => FABRIC_MOVE_COST_ZERO,
            MoveCost::Low => FABRIC_MOVE_COST_LOW,
            MoveCost::Medium => FABRIC_MOVE_COST_MEDIUM,
            MoveCost::High => FABRIC_MOVE_COST_HIGH,
        }
    }
}
