// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// This mod contains common types shared between FabricRuntime and FabricClient.
mod partition;
pub use partition::*;
mod stateful;
pub use stateful::*;
mod metrics;
pub use metrics::*;

use mssf_com::FabricTypes::{
    FABRIC_FAULT_TYPE, FABRIC_FAULT_TYPE_INVALID, FABRIC_FAULT_TYPE_PERMANENT,
    FABRIC_FAULT_TYPE_TRANSIENT, FABRIC_HEALTH_STATE, FABRIC_HEALTH_STATE_ERROR,
    FABRIC_HEALTH_STATE_INVALID, FABRIC_HEALTH_STATE_OK, FABRIC_HEALTH_STATE_UNKNOWN,
    FABRIC_HEALTH_STATE_WARNING,
};

// FABRIC_HEALTH_STATE
#[derive(Debug, PartialEq, Clone)]
pub enum HealthState {
    Invalid,
    Ok,
    Warning,
    Error,
    Unknown,
}

impl From<&FABRIC_HEALTH_STATE> for HealthState {
    fn from(value: &FABRIC_HEALTH_STATE) -> Self {
        match *value {
            FABRIC_HEALTH_STATE_INVALID => Self::Invalid,
            FABRIC_HEALTH_STATE_OK => Self::Ok,
            FABRIC_HEALTH_STATE_WARNING => Self::Warning,
            FABRIC_HEALTH_STATE_ERROR => Self::Error,
            FABRIC_HEALTH_STATE_UNKNOWN => Self::Unknown,
            _ => Self::Invalid,
        }
    }
}

impl From<&HealthState> for FABRIC_HEALTH_STATE {
    fn from(value: &HealthState) -> Self {
        match *value {
            HealthState::Invalid => FABRIC_HEALTH_STATE_INVALID,
            HealthState::Ok => FABRIC_HEALTH_STATE_OK,
            HealthState::Warning => FABRIC_HEALTH_STATE_WARNING,
            HealthState::Error => FABRIC_HEALTH_STATE_ERROR,
            HealthState::Unknown => FABRIC_HEALTH_STATE_UNKNOWN,
        }
    }
}

// FABRIC_FAULT_TYPE
#[derive(Debug, Clone, PartialEq)]
pub enum FaultType {
    Invalid,
    Permanent,
    Transient,
}

impl From<FABRIC_FAULT_TYPE> for FaultType {
    fn from(value: FABRIC_FAULT_TYPE) -> Self {
        match value {
            FABRIC_FAULT_TYPE_INVALID => Self::Invalid,
            FABRIC_FAULT_TYPE_PERMANENT => Self::Permanent,
            FABRIC_FAULT_TYPE_TRANSIENT => Self::Transient,
            _ => Self::Invalid,
        }
    }
}

impl From<FaultType> for FABRIC_FAULT_TYPE {
    fn from(value: FaultType) -> Self {
        match value {
            FaultType::Invalid => FABRIC_FAULT_TYPE_INVALID,
            FaultType::Permanent => FABRIC_FAULT_TYPE_PERMANENT,
            FaultType::Transient => FABRIC_FAULT_TYPE_TRANSIENT,
        }
    }
}
