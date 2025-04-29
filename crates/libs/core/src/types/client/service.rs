// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![cfg_attr(
    not(feature = "tokio_async"),
    allow(dead_code, reason = "code configured out")
)]

use std::ffi::c_void;

use mssf_com::FabricTypes::{
    FABRIC_NAMED_REPARTITION_DESCRIPTION, FABRIC_SERVICE_DESCRIPTION,
    FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL, FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
    FABRIC_SERVICE_PARTITION_KIND, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_UPDATE_DESCRIPTION,
    FABRIC_STATEFUL_SERVICE_DESCRIPTION, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1,
    FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3,
    FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4, FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION,
    FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1, FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2,
    FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3, FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4,
    FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5, FABRIC_STATELESS_SERVICE_DESCRIPTION,
    FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2,
    FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4,
    FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION, FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX1,
    FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX2,
    FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX3, FABRIC_URI,
};
use windows_core::{WString, PCWSTR};

use crate::types::{MoveCost, PartitionSchemeDescription, ServicePackageActivationMode, Uri};

pub enum ServiceDescription {
    // Invalid,
    Stateful(StatefulServiceDescription), // FABRIC_STATEFUL_SERVICE_DESCRIPTION
    Stateless(StatelessServiceDescription), // FABRIC_STATELESS_SERVICE_DESCRIPTION
}

#[derive(Debug, Default)]
pub struct StatefulServiceDescription {
    // common
    pub application_name: Uri,
    pub service_name: Uri,
    pub service_type_name: WString,
    pub initialization_data: Vec<u8>,
    pub partition_scheme: PartitionSchemeDescription,
    // stateful
    pub min_replica_set_size: i32,
    pub target_replica_set_size: i32,
    // common
    pub placement_contraints: WString,
    pub correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    pub metrics: Vec<WString>,      // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    pub has_persistent_state: bool,
    // ex1
    pub policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    pub failover_settings: WString, // TODO: FABRIC_SERVICE_PARTITION_KIND
    // ex2
    pub default_move_cost: Option<MoveCost>, // TODO: FABRIC_MOVE_COST
    // ex3
    pub service_package_activation_mode: ServicePackageActivationMode,
    pub service_dns_name: WString, // TODO: FABRIC_SERVICE_DNS_NAME
    // ex4
    pub service_scaling_policys: Vec<WString>, // TODO: FABRIC_SERVICE_SCALING_POLICY
}

pub(crate) struct StatefulServiceDescriptionRaw<'a> {
    internal: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3>,
    _internal_ex4: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4>,
    _owner: &'a StatefulServiceDescription,
}

impl StatefulServiceDescriptionRaw<'_> {
    pub fn as_ffi(&self) -> &FABRIC_STATEFUL_SERVICE_DESCRIPTION {
        self.internal.as_ref()
    }
}

impl StatefulServiceDescription {
    // Initializes the internal struct
    fn build_raw(&self) -> StatefulServiceDescriptionRaw {
        let ex4 = Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4 {
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: support scaling policies
            ScalingPolicyCount: 0,
            Reserved: std::ptr::null_mut(),
        });
        let ex3 = Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3 {
            ServiceDnsName: self.service_dns_name.as_pcwstr(),
            ServicePackageActivationMode: self.service_package_activation_mode.clone().into(),
            Reserved: ex4.as_ref() as *const _ as *mut c_void,
        });
        let ex2 = Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2 {
            IsDefaultMoveCostSpecified: self.default_move_cost.is_some(),
            DefaultMoveCost: self
                .default_move_cost
                .clone()
                .unwrap_or(MoveCost::Zero)
                .into(),
            Reserved: ex3.as_ref() as *const _ as *mut c_void,
        });
        let ex1 = Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1 {
            PolicyList: std::ptr::null_mut(),       // TODO:
            FailoverSettings: std::ptr::null_mut(), // TODO:
            Reserved: ex2.as_ref() as *const _ as *mut c_void,
        });

        let internal = Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            ServiceName: self.service_name.as_raw(),
            ServiceTypeName: self.service_type_name.as_pcwstr(),
            InitializationDataSize: self.initialization_data.len() as u32,
            InitializationData: self.initialization_data.as_ptr() as *mut u8,
            PartitionScheme: self.partition_scheme.as_raw().0,
            PartitionSchemeDescription: self.partition_scheme.as_raw().1,
            TargetReplicaSetSize: self.target_replica_set_size,
            MinReplicaSetSize: self.min_replica_set_size,
            PlacementConstraints: self.placement_contraints.as_pcwstr(), // TODO:
            CorrelationCount: 0,
            Correlations: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
            Metrics: std::ptr::null_mut(),      // TODO:
            MetricCount: 0,
            HasPersistedState: self.has_persistent_state,
            Reserved: ex1.as_ref() as *const _ as *mut c_void,
        });

        StatefulServiceDescriptionRaw {
            internal,
            _internal_ex1: ex1,
            _internal_ex2: ex2,
            _internal_ex3: ex3,
            _internal_ex4: ex4,
            _owner: self,
        }
    }
}

pub struct StatelessServiceDescription {
    // common
    pub application_name: WString,
    pub service_name: WString,
    pub service_type_name: WString,
    pub initialization_data: Vec<u8>,
    pub partition_scheme_description: PartitionSchemeDescription,
    // stateless
    pub instance_count: i32,
    // common
    pub placement_contraints: WString,
    pub correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    pub metrics: Vec<WString>,      // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    // ex1
    pub policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    // ex2
    pub default_move_cost: Option<MoveCost>, // TODO: FABRIC_MOVE_COST
    // ex3
    pub service_package_activation_mode: ServicePackageActivationMode, // TODO: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE
    pub service_dns_name: WString, // TODO: FABRIC_SERVICE_DNS_NAME
    // ex4
    pub service_scaling_policys: Vec<WString>, // TODO: FABRIC_SERVICE_SCALING_POLICY
}

pub(crate) struct StatelessServiceDescriptionRaw<'a> {
    internal: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3>,
    _internal_ex4: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4>,
    // String buffers memory owner
    _owner: &'a StatelessServiceDescription,
}
impl StatelessServiceDescriptionRaw<'_> {
    pub fn as_ffi(&self) -> &FABRIC_STATELESS_SERVICE_DESCRIPTION {
        self.internal.as_ref()
    }
}

impl StatelessServiceDescription {
    fn build_raw(&self) -> StatelessServiceDescriptionRaw {
        let ex4 = Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4 {
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: support scaling policies
            ScalingPolicyCount: 0,
            Reserved: std::ptr::null_mut(),
        });
        let ex3 = Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3 {
            ServiceDnsName: self.service_dns_name.as_pcwstr(),
            ServicePackageActivationMode: self.service_package_activation_mode.clone().into(),
            Reserved: ex4.as_ref() as *const _ as *mut c_void,
        });
        let ex2 = Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2 {
            IsDefaultMoveCostSpecified: self.default_move_cost.is_some(),
            DefaultMoveCost: self
                .default_move_cost
                .clone()
                .unwrap_or(MoveCost::Zero)
                .into(),
            Reserved: ex3.as_ref() as *const _ as *mut c_void,
        });
        let ex1 = Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1 {
            PolicyList: std::ptr::null_mut(), // TODO:
            Reserved: ex2.as_ref() as *const _ as *mut c_void,
        });
        let internal = Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION {
            ApplicationName: FABRIC_URI(self.application_name.as_ptr() as *mut u16),
            ServiceName: FABRIC_URI(self.service_name.as_ptr() as *mut u16),
            ServiceTypeName: self.service_type_name.as_pcwstr(),
            InitializationDataSize: self.initialization_data.len() as u32,
            InitializationData: self.initialization_data.as_ptr() as *mut u8,
            PartitionScheme: self.partition_scheme_description.as_raw().0,
            PartitionSchemeDescription: self.partition_scheme_description.as_raw().1,
            InstanceCount: self.instance_count,
            PlacementConstraints: self.placement_contraints.as_pcwstr(), // TODO:
            CorrelationCount: 0,
            Correlations: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
            Metrics: std::ptr::null_mut(),      // TODO:
            MetricCount: 0,
            Reserved: ex1.as_ref() as *const _ as *mut c_void,
        });
        StatelessServiceDescriptionRaw {
            internal,
            _internal_ex1: ex1,
            _internal_ex2: ex2,
            _internal_ex3: ex3,
            _internal_ex4: ex4,
            _owner: self,
        }
    }
}

pub(crate) enum ServiceDescriptionRaw<'a> {
    Stateful(StatefulServiceDescriptionRaw<'a>),
    Stateless(StatelessServiceDescriptionRaw<'a>),
}

impl ServiceDescriptionRaw<'_> {
    pub(crate) fn as_ffi(&self) -> FABRIC_SERVICE_DESCRIPTION {
        match self {
            ServiceDescriptionRaw::Stateful(ref desc) => FABRIC_SERVICE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
            ServiceDescriptionRaw::Stateless(ref desc) => FABRIC_SERVICE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
        }
    }
}

impl ServiceDescription {
    /// The raw type contains the ffi pointers on heap to be used by SF.
    /// mssf build the raw type on the stack and call the SF API with it.
    pub(crate) fn build_raw(&self) -> ServiceDescriptionRaw {
        match self {
            ServiceDescription::Stateful(ref desc) => {
                ServiceDescriptionRaw::Stateful(desc.build_raw())
            }
            ServiceDescription::Stateless(ref desc) => {
                ServiceDescriptionRaw::Stateless(desc.build_raw())
            }
        }
    }
}

// Update API payloads
// ===================================================

#[derive(Debug, Default)]
pub enum ServiceRepartitionDescription {
    #[default]
    Invalid,
    Named(NamedRepartitionDescription), // FABRIC_SERVICE_REPARTITION_DESCRIPTION
}

// FABRIC_NAMED_REPARTITION_DESCRIPTION
#[derive(Debug)]
pub struct NamedRepartitionDescription {
    pub names_to_add: Vec<WString>,
    pub names_to_remove: Vec<WString>,
}

/// Holder for memories passed to the FFI
pub(crate) struct NamedRepartitionDescriptionRaw<'a> {
    _names_to_add: Vec<PCWSTR>,
    _names_to_remove: Vec<PCWSTR>,
    internal: Box<FABRIC_NAMED_REPARTITION_DESCRIPTION>,
    // owner of string buffers
    _owner: &'a NamedRepartitionDescription,
}

impl NamedRepartitionDescription {
    pub(crate) fn as_raw(&self) -> NamedRepartitionDescriptionRaw {
        let names_to_add = self
            .names_to_add
            .iter()
            .map(|s| s.as_pcwstr())
            .collect::<Vec<_>>();
        let names_to_remove = self
            .names_to_remove
            .iter()
            .map(|s| s.as_pcwstr())
            .collect::<Vec<_>>();
        let internal = Box::new(FABRIC_NAMED_REPARTITION_DESCRIPTION {
            NamesToAddCount: names_to_add.len() as u32,
            NamesToAdd: names_to_add.as_ptr(),
            NamesToRemoveCount: names_to_remove.len() as u32,
            NamesToRemove: names_to_remove.as_ptr(),
            Reserved: std::ptr::null_mut(),
        });
        NamedRepartitionDescriptionRaw {
            _names_to_add: names_to_add,
            _names_to_remove: names_to_remove,
            internal,
            _owner: self,
        }
    }
}

impl NamedRepartitionDescriptionRaw<'_> {
    pub(crate) fn as_ffi(&self) -> &FABRIC_NAMED_REPARTITION_DESCRIPTION {
        self.internal.as_ref()
    }
}

/// Memory holder for the repartition description passed to the FFI
pub(crate) enum ServiceRepartitionDescriptionRaw<'a> {
    Invalid,
    Named(NamedRepartitionDescriptionRaw<'a>),
}

impl ServiceRepartitionDescription {
    pub(crate) fn as_raw(&self) -> ServiceRepartitionDescriptionRaw {
        match self {
            ServiceRepartitionDescription::Named(ref desc) => {
                ServiceRepartitionDescriptionRaw::Named(desc.as_raw())
            }
            ServiceRepartitionDescription::Invalid => ServiceRepartitionDescriptionRaw::Invalid,
        }
    }
}

impl ServiceRepartitionDescriptionRaw<'_> {
    pub(crate) fn as_ffi(&self) -> (FABRIC_SERVICE_PARTITION_KIND, *const c_void) {
        match self {
            ServiceRepartitionDescriptionRaw::Named(ref desc) => (
                FABRIC_SERVICE_PARTITION_KIND_NAMED,
                desc.as_ffi() as *const _ as *const _,
            ),
            ServiceRepartitionDescriptionRaw::Invalid => {
                (FABRIC_SERVICE_PARTITION_KIND_INVALID, std::ptr::null_mut())
            }
        }
    }
}

bitflags::bitflags! {
    /// FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS
    /// Indicates what fields are set in the description.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatefulServiceUpdateDescriptionFlags: u32 {
        const FABRIC_STATEFUL_SERVICE_NONE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_NONE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_CORRELATIONS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION.0 as u32;
        const FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION.0 as u32;
        const FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_POLICY_LIST = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_POLICY_LIST.0 as u32;
        const FABRIC_STATEFUL_SERVICE_CORRELATIONS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_CORRELATIONS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_METRICS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_METRICS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_MOVE_COST = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_MOVE_COST.0 as u32;
        const FABRIC_STATEFUL_SERVICE_SCALING_POLICY = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SCALING_POLICY.0 as u32;
    }
}
impl Default for StatefulServiceUpdateDescriptionFlags {
    fn default() -> Self {
        StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_NONE
    }
}

/// FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION
pub enum ServiceUpdateDescription {
    Stateful(StatefulServiceUpdateDescription), // FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION
    Stateless(StatelessServiceUpdateDescription), // FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION
}

impl ServiceUpdateDescription {
    /// The raw type contains the ffi pointers on heap to be used by SF.
    /// mssf build the raw type on the stack and call the SF API with it.
    pub(crate) fn build_raw(&self) -> ServiceUpdateDescriptionRaw {
        match self {
            ServiceUpdateDescription::Stateful(ref desc) => {
                ServiceUpdateDescriptionRaw::Stateful(desc.build_raw())
            }
            ServiceUpdateDescription::Stateless(ref desc) => {
                ServiceUpdateDescriptionRaw::Stateless(desc.build_raw())
            }
        }
    }
}

/// FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION
#[derive(Debug, Default)]
pub struct StatefulServiceUpdateDescription {
    flags: StatefulServiceUpdateDescriptionFlags,
    target_replica_set_size: i32,
    replica_restart_wait_duration_seconds: u32,
    quorum_loss_wait_duration_seconds: u32,
    // ex1
    stand_by_replica_keep_duration_seconds: u32,
    // ex2
    min_replica_set_size: i32,
    // ex3
    placement_contraints: WString,
    _policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    _correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    _metrics: Vec<WString>,     // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    // ex4
    default_move_cost: MoveCost, // TODO: FABRIC_MOVE_COST
    // ex5
    repartition_description: ServiceRepartitionDescription,
    _scaling_policys: Vec<WString>, // TODO: FABRIC_SERVICE_SCALING_POLICY
}

// setters for the fields
impl StatefulServiceUpdateDescription {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_target_replica_set_size(mut self, target_replica_set_size: i32) -> Self {
        self.flags |=
            StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE;
        self.target_replica_set_size = target_replica_set_size;
        self
    }

    pub fn with_replica_restart_wait_duration_seconds(
        mut self,
        replica_restart_wait_duration_seconds: u32,
    ) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION;
        self.replica_restart_wait_duration_seconds = replica_restart_wait_duration_seconds;
        self
    }
    pub fn with_quorum_loss_wait_duration_seconds(
        mut self,
        quorum_loss_wait_duration_seconds: u32,
    ) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION;
        self.quorum_loss_wait_duration_seconds = quorum_loss_wait_duration_seconds;
        self
    }
    pub fn with_stand_by_replica_keep_duration_seconds(
        mut self,
        stand_by_replica_keep_duration_seconds: u32,
    ) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION;
        self.stand_by_replica_keep_duration_seconds = stand_by_replica_keep_duration_seconds;
        self
    }
    pub fn with_min_replica_set_size(mut self, min_replica_set_size: i32) -> Self {
        self.flags |=
            StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE;
        self.min_replica_set_size = min_replica_set_size;
        self
    }

    pub fn with_move_cost(mut self, default_move_cost: MoveCost) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_MOVE_COST;
        self.default_move_cost = default_move_cost;
        self
    }

    pub fn with_repartition_description(
        mut self,
        repartition_description: ServiceRepartitionDescription,
    ) -> Self {
        // the ffi field is a ptr so no need to set the flag
        self.repartition_description = repartition_description;
        self
    }
}

/// FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION
pub struct StatelessServiceUpdateDescription {
    pub flags: StatefulServiceUpdateDescriptionFlags,
    pub instance_count: i32,
    // ex1
    pub placement_contraints: WString,
    pub policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    pub correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    pub metrics: Vec<WString>,     // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    // ex2
    pub default_move_cost: Option<MoveCost>, // TODO: FABRIC_MOVE_COST
    // ex5
    pub repartition_description: PartitionSchemeDescription, // TODO
    pub scaling_policys: Vec<WString>,                       // TODO: FABRIC_SERVICE_SCALING_POLICY
}

impl StatelessServiceUpdateDescription {
    fn build_raw(&self) -> StatelessServiceUpdateDescriptionRaw {
        unimplemented!()
    }
}

/// Temp enum to hold the raw data
pub(crate) enum ServiceUpdateDescriptionRaw<'a> {
    Stateful(StatefulServiceUpdateDescriptionRaw<'a>),
    Stateless(StatelessServiceUpdateDescriptionRaw),
}

/// Raw type
pub(crate) struct StatefulServiceUpdateDescriptionRaw<'a> {
    internal: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3>,
    _internal_ex4: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4>,
    _internal_ex5: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5>,
    _repartition_owner: ServiceRepartitionDescriptionRaw<'a>,
}

impl StatefulServiceUpdateDescriptionRaw<'_> {
    pub(crate) fn as_ffi(&self) -> &FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION {
        self.internal.as_ref()
    }
}

pub(crate) struct StatelessServiceUpdateDescriptionRaw {
    _internal: Box<FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX3>,
}

impl StatelessServiceUpdateDescriptionRaw {
    // TODO:
    pub(crate) fn as_ffi(&self) -> &FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION {
        unimplemented!()
        //self.internal.as_ref()
    }
}

impl ServiceUpdateDescriptionRaw<'_> {
    /// Must have the same lifetime as the original struct
    pub(crate) fn as_ffi(&self) -> FABRIC_SERVICE_UPDATE_DESCRIPTION {
        match self {
            ServiceUpdateDescriptionRaw::Stateful(ref desc) => FABRIC_SERVICE_UPDATE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
            ServiceUpdateDescriptionRaw::Stateless(ref desc) => FABRIC_SERVICE_UPDATE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
        }
    }
}

impl StatefulServiceUpdateDescription {
    pub(crate) fn build_raw(&self) -> StatefulServiceUpdateDescriptionRaw {
        let repartition_raw = self.repartition_description.as_raw();
        let ex5 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5 {
            RepartitionDescription: repartition_raw.as_ffi().1 as *const _ as *mut _,
            RepartitionKind: repartition_raw.as_ffi().0,
            ScalingPolicyCount: 0,
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_SCALING_POLICY
            Reserved: std::ptr::null_mut(),
        });
        let ex4 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4 {
            DefaultMoveCost: self.default_move_cost.clone().into(),
            Reserved: ex5.as_ref() as *const _ as *mut c_void,
        });
        let ex3 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3 {
            PlacementConstraints: self.placement_contraints.as_pcwstr(), // TODO:
            PolicyList: std::ptr::null_mut(),                            // TODO:
            CorrelationCount: 0,
            Correlations: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
            Metrics: std::ptr::null_mut(),      // TODO:
            MetricCount: 0,
            Reserved: ex4.as_ref() as *const _ as *mut c_void,
        });
        let ex2 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2 {
            MinReplicaSetSize: self.min_replica_set_size,
            Reserved: ex3.as_ref() as *const _ as *mut c_void,
        });
        let ex1 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1 {
            StandByReplicaKeepDurationSeconds: self.stand_by_replica_keep_duration_seconds,
            Reserved: ex2.as_ref() as *const _ as *mut c_void,
        });

        let internal = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION {
            Flags: self.flags.bits(),
            TargetReplicaSetSize: self.target_replica_set_size,
            ReplicaRestartWaitDurationSeconds: self.replica_restart_wait_duration_seconds,
            QuorumLossWaitDurationSeconds: self.quorum_loss_wait_duration_seconds,
            Reserved: ex1.as_ref() as *const _ as *mut c_void,
        });

        StatefulServiceUpdateDescriptionRaw {
            internal,
            _internal_ex1: ex1,
            _internal_ex2: ex2,
            _internal_ex3: ex3,
            _internal_ex4: ex4,
            _internal_ex5: ex5,
            _repartition_owner: repartition_raw,
        }
    }
}
