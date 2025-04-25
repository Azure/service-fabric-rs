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
    FABRIC_SERVICE_DESCRIPTION, FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
    FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS, FABRIC_STATEFUL_SERVICE_DESCRIPTION,
    FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2,
    FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4,
    FABRIC_STATELESS_SERVICE_DESCRIPTION, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1,
    FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3,
    FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4, FABRIC_URI,
};
use windows_core::WString;

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

pub(crate) struct StatefulServiceDescriptionRaw {
    internal: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3>,
    _internal_ex4: Box<FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4>,
}

impl StatefulServiceDescriptionRaw {
    pub fn as_raw(&self) -> &FABRIC_STATEFUL_SERVICE_DESCRIPTION {
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

pub(crate) struct StatelessServiceDescriptionRaw {
    internal: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION>,
    _internal_ex1: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1>,
    _internal_ex2: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2>,
    _internal_ex3: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3>,
    _internal_ex4: Box<FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4>,
}
impl StatelessServiceDescriptionRaw {
    pub fn as_raw(&self) -> &FABRIC_STATELESS_SERVICE_DESCRIPTION {
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
        }
    }
}

pub(crate) enum ServiceDescriptionRaw {
    Stateful(StatefulServiceDescriptionRaw),
    Stateless(StatelessServiceDescriptionRaw),
}

impl ServiceDescriptionRaw {
    pub(crate) fn as_raw(&self) -> FABRIC_SERVICE_DESCRIPTION {
        match self {
            ServiceDescriptionRaw::Stateful(ref desc) => FABRIC_SERVICE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
                Value: desc.as_raw() as *const _ as *mut c_void,
            },
            ServiceDescriptionRaw::Stateless(ref desc) => FABRIC_SERVICE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                Value: desc.as_raw() as *const _ as *mut c_void,
            },
        }
    }
}

impl ServiceDescription {
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
