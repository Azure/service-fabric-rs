use mssf_com::FabricTypes::{FABRIC_HEALTH_INFORMATION, FABRIC_HEALTH_REPORT_SEND_OPTIONS};
use windows_core::PCWSTR;

use crate::{strings::HSTRINGWrap, types::HealthState, HSTRING};

pub type SequenceNumber = i64;

/// Unknown sequence number, which is an invalid sequence number that is not accepted by the health store.
pub const UNKNOWN_SEQUENCE_NUMBER: SequenceNumber = -1;

/// When a health client receives a report with Auto sequence number,
/// it replaces the auto sequence number with a valid sequence number.
/// The sequence number is guaranteed to increase in the same process.
pub const AUTO_SEQUENCE_NUMBER: SequenceNumber = 0;

/// FABRIC_HEALTH_INFORMATION
#[derive(Debug, Clone)]
pub struct HealthInformation {
    /// source name which identifies the watchdog/system component which generated the health information.
    pub source_id: HSTRING,
    /// the property of the health report.
    pub property: HSTRING,
    /// how long the health report is valid. Must be larger than TimeSpan.Zero.
    pub time_to_live_seconds: u32,
    /// health state that describes the severity of the monitored condition used for reporting.
    pub state: HealthState,
    /// description of the health information. It represents free text used to add human readable
    /// information about the monitored condition.
    pub description: HSTRING,
    /// sequence number associated with the health information, used by the health store for staleness detection.
    /// Must be greater than UNKNOWN_SEQUENCE_NUMBER.
    pub sequence_number: SequenceNumber,
    /// whether the report is removed from health store when it expires.
    /// If set to false, the report is treated as an error when expired.
    pub remove_when_expired: bool,
    // TODO: not in rust yet
    // health_report_id: HSTRING,
}

impl From<&FABRIC_HEALTH_INFORMATION> for HealthInformation {
    fn from(value: &FABRIC_HEALTH_INFORMATION) -> Self {
        Self {
            source_id: HSTRINGWrap::from(value.SourceId).into(),
            property: HSTRINGWrap::from(value.Property).into(),
            time_to_live_seconds: value.TimeToLiveSeconds,
            state: HealthState::from(&value.State),
            description: HSTRINGWrap::from(value.Description).into(),
            sequence_number: value.SequenceNumber,
            remove_when_expired: value.RemoveWhenExpired.as_bool(),
        }
    }
}

/// Result has the same life time as self.
impl From<&HealthInformation> for FABRIC_HEALTH_INFORMATION {
    fn from(value: &HealthInformation) -> Self {
        Self {
            SourceId: PCWSTR(value.source_id.as_ptr()),
            Property: PCWSTR(value.property.as_ptr()),
            TimeToLiveSeconds: value.time_to_live_seconds,
            State: (&value.state).into(),
            Description: PCWSTR(value.description.as_ptr()),
            SequenceNumber: value.sequence_number,
            RemoveWhenExpired: value.remove_when_expired.into(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthReportSendOption {
    pub immediate: bool,
}

impl From<&FABRIC_HEALTH_REPORT_SEND_OPTIONS> for HealthReportSendOption {
    fn from(value: &FABRIC_HEALTH_REPORT_SEND_OPTIONS) -> Self {
        Self {
            immediate: value.Immediate.into(),
        }
    }
}

impl From<&HealthReportSendOption> for FABRIC_HEALTH_REPORT_SEND_OPTIONS {
    fn from(value: &HealthReportSendOption) -> Self {
        Self {
            Immediate: value.immediate.into(),
            Reserved: std::ptr::null_mut(),
        }
    }
}
