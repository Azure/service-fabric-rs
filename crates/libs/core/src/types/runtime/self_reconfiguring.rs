// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Safe wrappers for the `FABRIC_SELF_RECONFIGURING_*` value types.
//!
//! Conversions are provided in the direction(s) each type is used by the
//! self-reconfiguring service surface: inbound (COM -> safe) for values the
//! Service Fabric runtime pushes to an instance (configuration request and
//! configuration-change request), outbound (safe -> COM) for values the author
//! produces (configuration report), and both directions for the simple enums
//! and identifier values that appear on both paths.

use std::marker::PhantomData;

use crate::WString;
use mssf_com::FabricTypes::{
    FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST,
    FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT,
    FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID,
    FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST,
    FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID,
    FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE,
    FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST,
    FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION,
    FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST,
    FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE,
    FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING,
    FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_INVALID,
    FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW, FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE,
    FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL,
    FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER, FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE,
    FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED,
    FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED,
    FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_INVALID,
};

/// Whether a self-reconfiguring instance is being opened as new or existing.
/// Safe wrapper for `FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfReconfiguringOpenMode {
    Invalid,
    New,
    Existing,
}

impl From<FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE> for SelfReconfiguringOpenMode {
    fn from(e: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE) -> Self {
        match e {
            FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW => SelfReconfiguringOpenMode::New,
            FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING => {
                SelfReconfiguringOpenMode::Existing
            }
            _ => SelfReconfiguringOpenMode::Invalid,
        }
    }
}

impl From<SelfReconfiguringOpenMode> for FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE {
    fn from(val: SelfReconfiguringOpenMode) -> Self {
        match val {
            SelfReconfiguringOpenMode::Invalid => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_INVALID
            }
            SelfReconfiguringOpenMode::New => FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_NEW,
            SelfReconfiguringOpenMode::Existing => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE_EXISTING
            }
        }
    }
}

/// Role of a self-reconfiguring instance within the configuration.
/// Safe wrapper for `FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfReconfiguringInstanceRole {
    None,
    Initial,
    Member,
}

impl From<&FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE> for SelfReconfiguringInstanceRole {
    fn from(r: &FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE) -> Self {
        match *r {
            FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL => {
                SelfReconfiguringInstanceRole::Initial
            }
            FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER => SelfReconfiguringInstanceRole::Member,
            _ => SelfReconfiguringInstanceRole::None,
        }
    }
}

impl From<&SelfReconfiguringInstanceRole> for FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE {
    fn from(val: &SelfReconfiguringInstanceRole) -> Self {
        match *val {
            SelfReconfiguringInstanceRole::None => FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE,
            SelfReconfiguringInstanceRole::Initial => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL
            }
            SelfReconfiguringInstanceRole::Member => FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER,
        }
    }
}

/// Activation state of a self-reconfiguring instance.
/// Safe wrapper for `FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfReconfiguringInstanceActivationState {
    Invalid,
    Activated,
    Deactivated,
}

impl From<&FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE>
    for SelfReconfiguringInstanceActivationState
{
    fn from(s: &FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE) -> Self {
        match *s {
            FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED => {
                SelfReconfiguringInstanceActivationState::Activated
            }
            FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED => {
                SelfReconfiguringInstanceActivationState::Deactivated
            }
            _ => SelfReconfiguringInstanceActivationState::Invalid,
        }
    }
}

impl From<&SelfReconfiguringInstanceActivationState>
    for FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE
{
    fn from(val: &SelfReconfiguringInstanceActivationState) -> Self {
        match *val {
            SelfReconfiguringInstanceActivationState::Invalid => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_INVALID
            }
            SelfReconfiguringInstanceActivationState::Activated => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED
            }
            SelfReconfiguringInstanceActivationState::Deactivated => {
                FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED
            }
        }
    }
}

/// Identifier of a configuration request.
/// Safe wrapper for `FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfReconfiguringConfigurationRequestId {
    pub generation_number: i64,
    pub sequence_number: i64,
}

impl From<&FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID>
    for SelfReconfiguringConfigurationRequestId
{
    fn from(r: &FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID) -> Self {
        Self {
            generation_number: r.GenerationNumber,
            sequence_number: r.SequenceNumber,
        }
    }
}

impl From<&SelfReconfiguringConfigurationRequestId>
    for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID
{
    fn from(val: &SelfReconfiguringConfigurationRequestId) -> Self {
        Self {
            GenerationNumber: val.generation_number,
            SequenceNumber: val.sequence_number,
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// Identifier of a configuration report.
/// Safe wrapper for `FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfReconfiguringConfigurationReportId {
    pub sequence_number: i64,
}

impl From<&FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID>
    for SelfReconfiguringConfigurationReportId
{
    fn from(r: &FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID) -> Self {
        Self {
            sequence_number: r.SequenceNumber,
        }
    }
}

impl From<&SelfReconfiguringConfigurationReportId>
    for FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID
{
    fn from(val: &SelfReconfiguringConfigurationReportId) -> Self {
        Self {
            SequenceNumber: val.sequence_number,
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// A configuration request pushed to the instance by Service Fabric.
/// Inbound safe wrapper for `FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfReconfiguringConfigurationRequest {
    pub request_id: SelfReconfiguringConfigurationRequestId,
}

impl From<&FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST>
    for SelfReconfiguringConfigurationRequest
{
    fn from(r: &FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST) -> Self {
        Self {
            request_id: SelfReconfiguringConfigurationRequestId::from(&r.RequestId),
        }
    }
}

/// A requested change to a single instance's role/activation state.
/// Inbound safe wrapper for `FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceChangeRequest {
    pub instance_id: i64,
    pub role: SelfReconfiguringInstanceRole,
    pub requested_role: SelfReconfiguringInstanceRole,
    pub activation_state: SelfReconfiguringInstanceActivationState,
    pub requested_activation_state: SelfReconfiguringInstanceActivationState,
    /// The endpoints string; empty when none was supplied.
    pub endpoints: WString,
}

impl From<&FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST> for InstanceChangeRequest {
    fn from(r: &FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST) -> Self {
        let endpoints = if r.Endpoints.is_null() {
            WString::default()
        } else {
            WString::from(r.Endpoints)
        };
        Self {
            instance_id: r.InstanceId,
            role: SelfReconfiguringInstanceRole::from(&r.Role),
            requested_role: SelfReconfiguringInstanceRole::from(&r.RequestedRole),
            activation_state: SelfReconfiguringInstanceActivationState::from(&r.ActivationState),
            requested_activation_state: SelfReconfiguringInstanceActivationState::from(
                &r.RequestedActivationState,
            ),
            endpoints,
        }
    }
}

/// A configuration-change request pushed to the instance by Service Fabric.
/// Inbound safe wrapper for `FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfReconfiguringConfigurationChangeRequest {
    pub request_id: SelfReconfiguringConfigurationRequestId,
    pub instances: Vec<InstanceChangeRequest>,
}

impl From<&FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST>
    for SelfReconfiguringConfigurationChangeRequest
{
    fn from(r: &FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST) -> Self {
        let instances = unsafe { r.Instances.as_ref() }
            .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
            .unwrap_or_default();
        Self {
            request_id: SelfReconfiguringConfigurationRequestId::from(&r.RequestId),
            instances,
        }
    }
}

/// The reported state of a single instance.
/// Outbound safe wrapper for `FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceInformation {
    pub instance_id: i64,
    pub role: SelfReconfiguringInstanceRole,
    pub activation_state: SelfReconfiguringInstanceActivationState,
}

impl From<&InstanceInformation> for FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION {
    fn from(val: &InstanceInformation) -> Self {
        Self {
            InstanceId: val.instance_id,
            Role: (&val.role).into(),
            ActivationState: (&val.activation_state).into(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

/// The configuration an instance reports back to Service Fabric.
/// Outbound safe wrapper for `FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfReconfiguringConfigurationReport {
    pub request_id: SelfReconfiguringConfigurationRequestId,
    pub report_id: SelfReconfiguringConfigurationReportId,
    pub instances: Vec<InstanceInformation>,
}

impl SelfReconfiguringConfigurationReport {
    /// Lowers this report into a view holding the raw COM representation.
    ///
    /// The returned view owns the raw instance items (heap `Vec`), the raw
    /// instance-information list (heap `Box`), and the raw report struct, so the
    /// nested pointers inside the report remain valid for the lifetime of the
    /// view. The view borrows `self` and is used only for the duration of a
    /// single Service Fabric API call.
    pub fn get_view(&self) -> SelfReconfiguringConfigurationReportView<'_> {
        let items = self
            .instances
            .iter()
            .map(FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION::from)
            .collect::<Vec<_>>();

        let list = Box::new(FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST {
            Count: items.len() as u32,
            Items: items.as_ptr(),
        });

        let raw = FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT {
            RequestId: (&self.request_id).into(),
            ReportId: (&self.report_id).into(),
            Instances: list.as_ref() as *const _,
            Reserved: std::ptr::null_mut(),
        };

        SelfReconfiguringConfigurationReportView {
            _items: items,
            _list: list,
            raw,
            _phantom: PhantomData,
        }
    }
}

/// Holds the raw COM representation of a [`SelfReconfiguringConfigurationReport`].
///
/// Not movable in spirit: the raw report contains pointers into the heap data
/// owned by this view. It has the same lifetime as the report it was created
/// from.
pub struct SelfReconfiguringConfigurationReportView<'a> {
    _items: Vec<FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION>,
    _list: Box<FABRIC_SELF_RECONFIGURING_INSTANCE_INFORMATION_LIST>,
    raw: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT,
    _phantom: PhantomData<&'a SelfReconfiguringConfigurationReport>,
}

impl SelfReconfiguringConfigurationReportView<'_> {
    /// Returns the raw report that can be passed to the Service Fabric COM API.
    pub fn get_raw(&self) -> &FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PCWSTR;
    use mssf_com::FabricTypes::FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST;

    #[test]
    fn open_mode_round_trip() {
        for m in [
            SelfReconfiguringOpenMode::Invalid,
            SelfReconfiguringOpenMode::New,
            SelfReconfiguringOpenMode::Existing,
        ] {
            let raw: FABRIC_SELF_RECONFIGURING_INSTANCE_OPEN_MODE = m.into();
            assert_eq!(SelfReconfiguringOpenMode::from(raw), m);
        }
    }

    #[test]
    fn instance_role_round_trip() {
        for r in [
            SelfReconfiguringInstanceRole::None,
            SelfReconfiguringInstanceRole::Initial,
            SelfReconfiguringInstanceRole::Member,
        ] {
            let raw: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE = (&r).into();
            assert_eq!(SelfReconfiguringInstanceRole::from(&raw), r);
        }
    }

    #[test]
    fn activation_state_round_trip() {
        for s in [
            SelfReconfiguringInstanceActivationState::Invalid,
            SelfReconfiguringInstanceActivationState::Activated,
            SelfReconfiguringInstanceActivationState::Deactivated,
        ] {
            let raw: FABRIC_SELF_RECONFIGURING_INSTANCE_ACTIVATION_STATE = (&s).into();
            assert_eq!(SelfReconfiguringInstanceActivationState::from(&raw), s);
        }
    }

    #[test]
    fn request_id_round_trip() {
        let id = SelfReconfiguringConfigurationRequestId {
            generation_number: 7,
            sequence_number: 42,
        };
        let raw: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID = (&id).into();
        assert_eq!(SelfReconfiguringConfigurationRequestId::from(&raw), id);
    }

    #[test]
    fn report_id_round_trip() {
        let id = SelfReconfiguringConfigurationReportId {
            sequence_number: 99,
        };
        let raw: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REPORT_ID = (&id).into();
        assert_eq!(SelfReconfiguringConfigurationReportId::from(&raw), id);
    }

    #[test]
    fn configuration_request_inbound() {
        let raw = FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST {
            RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID {
                GenerationNumber: 1,
                SequenceNumber: 2,
                Reserved: std::ptr::null_mut(),
            },
            Reserved: std::ptr::null_mut(),
        };
        let safe = SelfReconfiguringConfigurationRequest::from(&raw);
        assert_eq!(safe.request_id.generation_number, 1);
        assert_eq!(safe.request_id.sequence_number, 2);
    }

    #[test]
    fn change_request_inbound_empty() {
        let raw = FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST {
            RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID::default(),
            Instances: std::ptr::null(),
            Reserved: std::ptr::null_mut(),
        };
        let safe = SelfReconfiguringConfigurationChangeRequest::from(&raw);
        assert!(safe.instances.is_empty());
    }

    #[test]
    fn change_request_inbound_single_item() {
        let endpoints = WString::from("localhost:4321");
        let item = FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST {
            InstanceId: 12,
            Role: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_NONE,
            RequestedRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL,
            ActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED,
            RequestedActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED,
            Endpoints: PCWSTR(endpoints.as_ptr()),
            Reserved: std::ptr::null_mut(),
        };
        let list = FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST {
            Count: 1,
            Items: &item,
        };
        let raw = FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST {
            RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID::default(),
            Instances: &list,
            Reserved: std::ptr::null_mut(),
        };

        let safe = SelfReconfiguringConfigurationChangeRequest::from(&raw);
        assert_eq!(safe.instances.len(), 1);
        assert_eq!(safe.instances[0].instance_id, 12);
        assert_eq!(
            safe.instances[0].requested_role,
            SelfReconfiguringInstanceRole::Initial
        );
        assert_eq!(safe.instances[0].endpoints, WString::from("localhost:4321"));
    }

    #[test]
    fn change_request_inbound_items() {
        let endpoints = WString::from("localhost:1234");
        let items = [
            FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST {
                InstanceId: 10,
                Role: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL,
                RequestedRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER,
                ActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_DEACTIVATED,
                RequestedActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED,
                Endpoints: PCWSTR(endpoints.as_ptr()),
                Reserved: std::ptr::null_mut(),
            },
            // Second item with a null endpoints pointer.
            FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST {
                InstanceId: 11,
                Role: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER,
                RequestedRole: FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER,
                ActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED,
                RequestedActivationState: FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED,
                Endpoints: PCWSTR::null(),
                Reserved: std::ptr::null_mut(),
            },
        ];
        let list = FABRIC_SELF_RECONFIGURING_INSTANCE_CHANGE_REQUEST_LIST {
            Count: items.len() as u32,
            Items: items.as_ptr(),
        };
        let raw = FABRIC_SELF_RECONFIGURING_CONFIGURATION_CHANGE_REQUEST {
            RequestId: FABRIC_SELF_RECONFIGURING_CONFIGURATION_REQUEST_ID {
                GenerationNumber: 3,
                SequenceNumber: 4,
                Reserved: std::ptr::null_mut(),
            },
            Instances: &list,
            Reserved: std::ptr::null_mut(),
        };

        let safe = SelfReconfiguringConfigurationChangeRequest::from(&raw);
        assert_eq!(safe.request_id.generation_number, 3);
        assert_eq!(safe.instances.len(), 2);

        let first = &safe.instances[0];
        assert_eq!(first.instance_id, 10);
        assert_eq!(first.role, SelfReconfiguringInstanceRole::Initial);
        assert_eq!(first.requested_role, SelfReconfiguringInstanceRole::Member);
        assert_eq!(
            first.activation_state,
            SelfReconfiguringInstanceActivationState::Deactivated
        );
        assert_eq!(first.endpoints, WString::from("localhost:1234"));

        let second = &safe.instances[1];
        assert_eq!(second.instance_id, 11);
        assert_eq!(second.endpoints, WString::default());
    }

    #[test]
    fn report_outbound_empty() {
        let report = SelfReconfiguringConfigurationReport {
            request_id: SelfReconfiguringConfigurationRequestId {
                generation_number: 1,
                sequence_number: 2,
            },
            report_id: SelfReconfiguringConfigurationReportId { sequence_number: 5 },
            instances: vec![],
        };
        let view = report.get_view();
        let raw = view.get_raw();
        assert_eq!(raw.RequestId.GenerationNumber, 1);
        assert_eq!(raw.ReportId.SequenceNumber, 5);
        let list = unsafe { &*raw.Instances };
        assert_eq!(list.Count, 0);
    }

    #[test]
    fn report_outbound_items() {
        let report = SelfReconfiguringConfigurationReport {
            request_id: SelfReconfiguringConfigurationRequestId {
                generation_number: 8,
                sequence_number: 9,
            },
            report_id: SelfReconfiguringConfigurationReportId {
                sequence_number: 10,
            },
            instances: vec![
                InstanceInformation {
                    instance_id: 100,
                    role: SelfReconfiguringInstanceRole::Member,
                    activation_state: SelfReconfiguringInstanceActivationState::Activated,
                },
                InstanceInformation {
                    instance_id: 101,
                    role: SelfReconfiguringInstanceRole::Initial,
                    activation_state: SelfReconfiguringInstanceActivationState::Deactivated,
                },
            ],
        };
        let view = report.get_view();
        let raw = view.get_raw();
        let list = unsafe { &*raw.Instances };
        assert_eq!(list.Count, 2);

        let first = unsafe { &*list.Items.offset(0) };
        assert_eq!(first.InstanceId, 100);
        assert_eq!(first.Role, FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_MEMBER);
        assert_eq!(
            first.ActivationState,
            FABRIC_SELF_RECONFIGURING_INSTANCE_STATE_ACTIVATED
        );

        let second = unsafe { &*list.Items.offset(1) };
        assert_eq!(second.InstanceId, 101);
        assert_eq!(second.Role, FABRIC_SELF_RECONFIGURING_INSTANCE_ROLE_INITIAL);
    }
}
