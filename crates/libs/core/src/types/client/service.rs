// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::{ffi::c_void, marker::PhantomData};

use mssf_com::{
    FabricClient::IFabricGetServiceListResult2,
    FabricTypes::{
        FABRIC_NAMED_REPARTITION_DESCRIPTION, FABRIC_SERVICE_DESCRIPTION,
        FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL, FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
        FABRIC_SERVICE_HEALTH_STATE, FABRIC_SERVICE_PARTITION_KIND,
        FABRIC_SERVICE_PARTITION_KIND_INVALID, FABRIC_SERVICE_PARTITION_KIND_NAMED,
        FABRIC_SERVICE_QUERY_DESCRIPTION, FABRIC_SERVICE_QUERY_DESCRIPTION_EX1,
        FABRIC_SERVICE_QUERY_DESCRIPTION_EX2, FABRIC_SERVICE_QUERY_DESCRIPTION_EX3,
        FABRIC_SERVICE_QUERY_RESULT_ITEM, FABRIC_SERVICE_UPDATE_DESCRIPTION,
        FABRIC_STATEFUL_SERVICE_DESCRIPTION, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1,
        FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2, FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3,
        FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4, FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX1,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX2,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX3,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX4,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX5,
        FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX6, FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11,
        FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12, FABRIC_STATELESS_SERVICE_DESCRIPTION,
        FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2,
        FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3, FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4,
        FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION,
        FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX1,
        FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX2,
        FABRIC_STATELESS_SERVICE_UPDATE_DESCRIPTION_EX3,
    },
};
use windows_core::{PCWSTR, WString};

use crate::{
    mem::{BoxPool, GetRaw, GetRawWithBoxPool},
    types::{
        ApplicationHealthPolicy, HealthEvent, HealthEventsFilter, HealthState,
        HealthStateFilterFlags, MoveCost, PagingStatus, PartitionSchemeDescription,
        ServicePackageActivationMode, Uri,
    },
};

pub enum ServiceDescription {
    // Invalid,
    Stateful(StatefulServiceDescription), // FABRIC_STATEFUL_SERVICE_DESCRIPTION
    Stateless(StatelessServiceDescription), // FABRIC_STATELESS_SERVICE_DESCRIPTION
}

#[derive(Debug)]
pub struct StatefulServiceDescription {
    // common
    // Note: if application_name is not set, SF com api will succeed but the service does not show up in the SF explorer.
    // May need to file a bug with SF team.
    application_name: Uri,
    service_name: Uri,
    service_type_name: WString,
    initialization_data: Option<Vec<u8>>,
    partition_scheme: PartitionSchemeDescription,
    // stateful
    min_replica_set_size: i32,
    target_replica_set_size: i32,
    // common
    placement_contraints: WString,
    _correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    _metrics: Vec<WString>,      // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    has_persistent_state: bool,
    // ex1
    _policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    failover_settings: StatefulServiceFailoverSettings,
    // ex2
    default_move_cost: Option<MoveCost>, // TODO: FABRIC_MOVE_COST
    // ex3
    service_package_activation_mode: ServicePackageActivationMode,
    _service_dns_name: Option<WString>, // TODO: FABRIC_SERVICE_DNS_NAME
    // ex4
    _service_scaling_policys: Vec<WString>, // TODO: FABRIC_SERVICE_SCALING_POLICY
}

impl StatefulServiceDescription {
    /// Following the csharp validator code:
    /// The required fields are here, but optional fields can be set by other setters.
    pub fn new(
        application_name: Uri,
        service_name: Uri,
        service_type_name: WString,
        partition_scheme: PartitionSchemeDescription,
    ) -> Self {
        Self {
            application_name,
            service_name,
            service_type_name,
            initialization_data: None,
            partition_scheme,
            min_replica_set_size: 1,
            target_replica_set_size: 1,
            placement_contraints: WString::default(),
            _correlations: Vec::new(),
            _metrics: Vec::new(),
            has_persistent_state: false,
            _policy_list: Vec::new(),
            failover_settings: StatefulServiceFailoverSettings::default(),
            default_move_cost: None,
            service_package_activation_mode: ServicePackageActivationMode::default(),
            _service_dns_name: None,
            _service_scaling_policys: Vec::new(),
        }
    }

    // TODO: add more setters for the fields.

    pub fn with_initialization_data(mut self, initialization_data: Vec<u8>) -> Self {
        self.initialization_data = Some(initialization_data);
        self
    }

    pub fn with_min_replica_set_size(mut self, min_replica_set_size: i32) -> Self {
        self.min_replica_set_size = min_replica_set_size;
        self
    }
    pub fn with_target_replica_set_size(mut self, target_replica_set_size: i32) -> Self {
        self.target_replica_set_size = target_replica_set_size;
        self
    }
    pub fn with_has_persistent_state(mut self, has_persistent_state: bool) -> Self {
        self.has_persistent_state = has_persistent_state;
        self
    }

    pub fn with_default_move_cost(mut self, default_move_cost: MoveCost) -> Self {
        self.default_move_cost = Some(default_move_cost);
        self
    }
    pub fn with_service_activation_mode(
        mut self,
        service_package_activation_mode: ServicePackageActivationMode,
    ) -> Self {
        self.service_package_activation_mode = service_package_activation_mode;
        self
    }

    // Failover settings setters

    pub fn with_replica_restart_wait_duration_seconds(mut self, seconds: u32) -> Self {
        self.failover_settings.replica_restart_wait_duration_seconds = Some(seconds);
        self
    }

    pub fn with_quorum_loss_wait_duration_seconds(mut self, seconds: u32) -> Self {
        self.failover_settings.quorum_loss_wait_duration_seconds = Some(seconds);
        self
    }

    pub fn with_stand_by_replica_keep_duration_seconds(mut self, seconds: u32) -> Self {
        self.failover_settings
            .stand_by_replica_keep_duration_seconds = Some(seconds);
        self
    }

    pub fn with_service_placement_time_limit_seconds(mut self, seconds: u32) -> Self {
        self.failover_settings.service_placement_time_limit_seconds = Some(seconds);
        self
    }

    pub fn with_drop_source_replica_on_move(mut self, drop: bool) -> Self {
        self.failover_settings.drop_source_replica_on_move = Some(drop);
        self
    }

    pub fn with_is_singleton_replica_move_allowed_during_upgrade(mut self, allowed: bool) -> Self {
        self.failover_settings
            .is_singleton_replica_move_allowed_during_upgrade = Some(allowed);
        self
    }

    pub fn with_restore_replica_location_after_upgrade(mut self, restore: bool) -> Self {
        self.failover_settings
            .restore_replica_location_after_upgrade = Some(restore);
        self
    }

    pub fn with_auxiliary_replica_count(mut self, count: i32) -> Self {
        self.failover_settings.auxiliary_replica_count = Some(count);
        self
    }

    pub fn with_service_sensitivity(mut self, sensitivity: ServiceSensitivityDescription) -> Self {
        self.failover_settings.service_sensitivity = Some(sensitivity);
        self
    }
}

impl GetRawWithBoxPool<FABRIC_STATEFUL_SERVICE_DESCRIPTION> for StatefulServiceDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_STATEFUL_SERVICE_DESCRIPTION {
        let ex4 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX4 {
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: support scaling policies
            ScalingPolicyCount: 0,
            Reserved: std::ptr::null_mut(),
        }));
        let ex3 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX3 {
            ServiceDnsName: windows_core::PCWSTR::null(), // TODO: FABRIC_SERVICE_DNS_NAME
            ServicePackageActivationMode: self.service_package_activation_mode.into(),
            Reserved: ex4 as *const _ as *mut c_void,
        }));
        let ex2 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX2 {
            IsDefaultMoveCostSpecified: self.default_move_cost.is_some(),
            DefaultMoveCost: self.default_move_cost.unwrap_or(MoveCost::Zero).into(),
            Reserved: ex3 as *const _ as *mut c_void,
        }));

        let failover_settings_raw = self.failover_settings.get_raw_with_pool(pool);
        let failover_settings_ptr = failover_settings_raw
            .map(|s| pool.push(Box::new(s)) as *const _ as *mut _)
            .unwrap_or(std::ptr::null_mut());

        let ex1 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_DESCRIPTION_EX1 {
            PolicyList: std::ptr::null_mut(), // TODO:
            FailoverSettings: failover_settings_ptr,
            Reserved: ex2 as *const _ as *mut c_void,
        }));

        let (init_data, init_data_len) = self
            .initialization_data
            .as_ref()
            .map(|v| (v.as_ptr() as *mut u8, v.len() as u32))
            .unwrap_or((std::ptr::null_mut(), 0));

        FABRIC_STATEFUL_SERVICE_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            ServiceName: self.service_name.as_raw(),
            ServiceTypeName: self.service_type_name.as_pcwstr(),
            InitializationDataSize: init_data_len,
            InitializationData: init_data,
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
            Reserved: ex1 as *const _ as *mut c_void,
        }
    }
}

pub struct StatelessServiceDescription {
    // common
    pub application_name: Uri,
    pub service_name: Uri,
    pub service_type_name: WString,
    pub initialization_data: Option<Vec<u8>>,
    pub partition_scheme_description: PartitionSchemeDescription,
    // stateless
    pub instance_count: i32,
    // common
    pub placement_contraints: WString,
    _correlations: Vec<WString>, // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
    _metrics: Vec<WString>,      // TODO: FABRIC_SERVICE_LOAD_METRIC_DESCRIPTION
    // ex1
    _policy_list: Vec<WString>, // TODO: FABRIC_SERVICE_PLACEMENT_POLICY_DESCRIPTION
    // ex2
    default_move_cost: Option<MoveCost>, // TODO: FABRIC_MOVE_COST
    // ex3
    service_package_activation_mode: ServicePackageActivationMode, // TODO: FABRIC_SERVICE_PACKAGE_ACTIVATION_MODE
    _service_dns_name: WString,                                    // TODO: FABRIC_SERVICE_DNS_NAME
    // ex4
    _service_scaling_policys: Vec<WString>, // TODO: FABRIC_SERVICE_SCALING_POLICY
}
impl StatelessServiceDescription {
    pub fn new(
        application_name: Uri,
        service_name: Uri,
        service_type_name: WString,
        partition_scheme_description: PartitionSchemeDescription,
    ) -> Self {
        Self {
            application_name,
            service_name,
            service_type_name,
            initialization_data: None,
            partition_scheme_description,
            instance_count: 1,
            placement_contraints: WString::default(),
            _correlations: Vec::new(),
            _metrics: Vec::new(),
            _policy_list: Vec::new(),
            default_move_cost: None,
            service_package_activation_mode: ServicePackageActivationMode::default(),
            _service_dns_name: WString::default(),
            _service_scaling_policys: Vec::new(),
        }
    }

    pub fn with_initialization_data(mut self, initialization_data: Vec<u8>) -> Self {
        self.initialization_data = Some(initialization_data);
        self
    }
    pub fn with_instance_count(mut self, instance_count: i32) -> Self {
        self.instance_count = instance_count;
        self
    }
    pub fn with_placement_constraints(mut self, placement_contraints: WString) -> Self {
        self.placement_contraints = placement_contraints;
        self
    }
    pub fn with_default_move_cost(mut self, default_move_cost: MoveCost) -> Self {
        self.default_move_cost = Some(default_move_cost);
        self
    }
    pub fn with_service_activation_mode(
        mut self,
        service_package_activation_mode: ServicePackageActivationMode,
    ) -> Self {
        self.service_package_activation_mode = service_package_activation_mode;
        self
    }
}
impl GetRawWithBoxPool<FABRIC_STATELESS_SERVICE_DESCRIPTION> for StatelessServiceDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_STATELESS_SERVICE_DESCRIPTION {
        let ex4 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4 {
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: support scaling policies
            ScalingPolicyCount: 0,
            Reserved: std::ptr::null_mut(),
        }));
        let ex3 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3 {
            ServiceDnsName: windows_core::PCWSTR::null(), // TODO: FABRIC_SERVICE_DNS_NAME
            ServicePackageActivationMode: self.service_package_activation_mode.into(),
            Reserved: ex4 as *const _ as *mut c_void,
        }));
        let ex2 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2 {
            IsDefaultMoveCostSpecified: self.default_move_cost.is_some(),
            DefaultMoveCost: self.default_move_cost.unwrap_or(MoveCost::Zero).into(),
            Reserved: ex3 as *const _ as *mut c_void,
        }));
        let ex1 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1 {
            PolicyList: std::ptr::null_mut(), // TODO:
            Reserved: ex2 as *const _ as *mut c_void,
        }));

        let (init_data, init_data_len) = self
            .initialization_data
            .as_ref()
            .map(|v| (v.as_ptr() as *mut u8, v.len() as u32))
            .unwrap_or((std::ptr::null_mut(), 0));

        FABRIC_STATELESS_SERVICE_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            ServiceName: self.service_name.as_raw(),
            ServiceTypeName: self.service_type_name.as_pcwstr(),
            InitializationDataSize: init_data_len,
            InitializationData: init_data,
            PartitionScheme: self.partition_scheme_description.as_raw().0,
            PartitionSchemeDescription: self.partition_scheme_description.as_raw().1,
            InstanceCount: self.instance_count,
            PlacementConstraints: self.placement_contraints.as_pcwstr(), // TODO:
            CorrelationCount: 0,
            Correlations: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_CORRELATION_DESCRIPTION
            Metrics: std::ptr::null_mut(),      // TODO:
            MetricCount: 0,
            Reserved: ex1 as *const _ as *mut c_void,
        }
    }
}

impl GetRawWithBoxPool<FABRIC_SERVICE_DESCRIPTION> for ServiceDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_SERVICE_DESCRIPTION {
        match self {
            ServiceDescription::Stateful(desc) => {
                let raw = desc.get_raw_with_pool(pool);
                let raw_ptr = pool.push(Box::new(raw));
                FABRIC_SERVICE_DESCRIPTION {
                    Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
                    Value: raw_ptr as *const _ as *mut c_void,
                }
            }
            ServiceDescription::Stateless(desc) => {
                let raw = desc.get_raw_with_pool(pool);
                let raw_ptr = pool.push(Box::new(raw));
                FABRIC_SERVICE_DESCRIPTION {
                    Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                    Value: raw_ptr as *const _ as *mut c_void,
                }
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
    phantom: PhantomData<&'a NamedRepartitionDescription>,
}

impl NamedRepartitionDescription {
    pub(crate) fn as_raw(&self) -> NamedRepartitionDescriptionRaw<'_> {
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
            phantom: PhantomData,
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
    pub(crate) fn as_raw(&self) -> ServiceRepartitionDescriptionRaw<'_> {
        match self {
            ServiceRepartitionDescription::Named(desc) => {
                ServiceRepartitionDescriptionRaw::Named(desc.as_raw())
            }
            ServiceRepartitionDescription::Invalid => ServiceRepartitionDescriptionRaw::Invalid,
        }
    }
}

impl ServiceRepartitionDescriptionRaw<'_> {
    pub(crate) fn as_ffi(&self) -> (FABRIC_SERVICE_PARTITION_KIND, *const c_void) {
        match self {
            ServiceRepartitionDescriptionRaw::Named(desc) => (
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
    /// FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_FLAGS
    /// Indicates what fields are set in the failover settings.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatefulServiceFailoverSettingsFlags: u32 {
        const NONE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_NONE.0 as u32;
        const REPLICA_RESTART_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_REPLICA_RESTART_WAIT_DURATION.0 as u32;
        const QUORUM_LOSS_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_QUORUM_LOSS_WAIT_DURATION.0 as u32;
        const STANDBY_REPLICA_KEEP_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_STANDBY_REPLICA_KEEP_DURATION.0 as u32;
        const SERVICE_PLACEMENT_TIME_LIMIT = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_PLACEMENT_TIME_LIMIT.0 as u32;
        const DROP_SOURCE_REPLICA_ON_MOVE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_DROP_SOURCE_REPLICA_ON_MOVE.0 as u32;
        const IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE.0 as u32;
        const RESTORE_REPLICA_LOCATION_AFTER_UPGRADE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE.0 as u32;
        const AUXILIARY_REPLICA_COUNT = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_AUXILIARY_REPLICA_COUNT.0 as u32;
        const SERVICE_SENSITIVITY = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SETTINGS_SERVICE_SENSITIVITY.0 as u32;
    }
}

impl Default for StatefulServiceFailoverSettingsFlags {
    fn default() -> Self {
        StatefulServiceFailoverSettingsFlags::NONE
    }
}

/// FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS (Base + EX1..EX6)
///
/// Wraps the failover settings chain. Each `Option` field corresponds to a flag bit.
/// Setting a field to `Some(...)` automatically sets the corresponding flag when building the raw type.
#[derive(Debug, Clone, Default)]
pub(crate) struct StatefulServiceFailoverSettings {
    /// Base: ReplicaRestartWaitDurationSeconds (flag bit 1)
    pub(crate) replica_restart_wait_duration_seconds: Option<u32>,
    /// Base: QuorumLossWaitDurationSeconds (flag bit 2)
    pub(crate) quorum_loss_wait_duration_seconds: Option<u32>,
    /// EX1: StandByReplicaKeepDurationSeconds (flag bit 4)
    pub(crate) stand_by_replica_keep_duration_seconds: Option<u32>,
    /// EX2: ServicePlacementTimeLimitSeconds (flag bit 8)
    pub(crate) service_placement_time_limit_seconds: Option<u32>,
    /// EX3: DropSourceReplicaOnMove (flag bit 16)
    pub(crate) drop_source_replica_on_move: Option<bool>,
    /// EX4 via REPLICA_LIFECYCLE_DESCRIPTION: IsSingletonReplicaMoveAllowedDuringUpgrade (flag bit 32)
    pub(crate) is_singleton_replica_move_allowed_during_upgrade: Option<bool>,
    /// EX4 via REPLICA_LIFECYCLE_DESCRIPTION: RestoreReplicaLocationAfterUpgrade (flag bit 64)
    pub(crate) restore_replica_location_after_upgrade: Option<bool>,
    /// EX5: AuxiliaryReplicaCount (flag bit 128)
    pub(crate) auxiliary_replica_count: Option<i32>,
    /// EX6 via SERVICE_SENSITIVITY_DESCRIPTION (flag bit 256)
    pub(crate) service_sensitivity: Option<ServiceSensitivityDescription>,
}

/// Wrapper for SERVICE_SENSITIVITY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ServiceSensitivityDescription {
    pub primary_default_sensitivity: u32,
    pub secondary_default_sensitivity: u32,
    pub auxiliary_default_sensitivity: u32,
    pub is_maximum_sensitivity: bool,
}

impl StatefulServiceFailoverSettings {
    /// Compute the flags bitmask from which fields are set.
    fn compute_flags(&self) -> StatefulServiceFailoverSettingsFlags {
        let mut flags = StatefulServiceFailoverSettingsFlags::NONE;
        if self.replica_restart_wait_duration_seconds.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::REPLICA_RESTART_WAIT_DURATION;
        }
        if self.quorum_loss_wait_duration_seconds.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::QUORUM_LOSS_WAIT_DURATION;
        }
        if self.stand_by_replica_keep_duration_seconds.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::STANDBY_REPLICA_KEEP_DURATION;
        }
        if self.service_placement_time_limit_seconds.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::SERVICE_PLACEMENT_TIME_LIMIT;
        }
        if self.drop_source_replica_on_move.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::DROP_SOURCE_REPLICA_ON_MOVE;
        }
        if self
            .is_singleton_replica_move_allowed_during_upgrade
            .is_some()
        {
            flags |= StatefulServiceFailoverSettingsFlags::IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE;
        }
        if self.restore_replica_location_after_upgrade.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
        }
        if self.auxiliary_replica_count.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::AUXILIARY_REPLICA_COUNT;
        }
        if self.service_sensitivity.is_some() {
            flags |= StatefulServiceFailoverSettingsFlags::SERVICE_SENSITIVITY;
        }
        flags
    }
}

impl GetRawWithBoxPool<Option<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS>>
    for StatefulServiceFailoverSettings
{
    /// Returns `Some` with the failover settings struct if any fields are set, or `None` otherwise.
    /// All allocations are placed in the provided `pool`.
    fn get_raw_with_pool(
        &self,
        pool: &mut BoxPool,
    ) -> Option<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS> {
        let flags = self.compute_flags();
        if flags == StatefulServiceFailoverSettingsFlags::NONE {
            return None;
        }

        // EX6: ServiceSensitivityDescription
        let service_sensitivity_ptr = if let Some(s) = &self.service_sensitivity {
            pool.push(Box::new(
                mssf_com::FabricTypes::SERVICE_SENSITIVITY_DESCRIPTION {
                    PrimaryDefaultSensitivity: s.primary_default_sensitivity,
                    SecondaryDefaultSensitivity: s.secondary_default_sensitivity,
                    AuxiliaryDefaultSensitivity: s.auxiliary_default_sensitivity,
                    IsMaximumSensitivity: s.is_maximum_sensitivity,
                    Reserved: std::ptr::null_mut(),
                },
            )) as *const _ as *mut _
        } else {
            std::ptr::null_mut()
        };
        let ex6 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX6 {
            ServiceSensitivityDescription: service_sensitivity_ptr,
            Reserved: std::ptr::null_mut(),
        }));

        // EX5: AuxiliaryReplicaCount
        let ex5 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX5 {
            AuxiliaryReplicaCount: self.auxiliary_replica_count.unwrap_or(0),
            Reserved: ex6 as *const _ as *mut c_void,
        }));

        // EX4: ReplicaLifecycleDescription
        let replica_lifecycle_ptr = if self
            .is_singleton_replica_move_allowed_during_upgrade
            .is_some()
            || self.restore_replica_location_after_upgrade.is_some()
        {
            pool.push(Box::new(
                mssf_com::FabricTypes::REPLICA_LIFECYCLE_DESCRIPTION {
                    IsIsSingletonReplicaMoveAllowedDuringUpgradeSpecified: self
                        .is_singleton_replica_move_allowed_during_upgrade
                        .is_some(),
                    IsSingletonReplicaMoveAllowedDuringUpgrade: self
                        .is_singleton_replica_move_allowed_during_upgrade
                        .unwrap_or(false),
                    IsRestoreReplicaLocationAfterUpgradeSpecified: self
                        .restore_replica_location_after_upgrade
                        .is_some(),
                    RestoreReplicaLocationAfterUpgrade: self
                        .restore_replica_location_after_upgrade
                        .unwrap_or(false),
                    Reserved: std::ptr::null_mut(),
                },
            )) as *const _ as *mut _
        } else {
            std::ptr::null_mut()
        };
        let ex4 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX4 {
            ReplicaLifecycleDescription: replica_lifecycle_ptr,
            Reserved: ex5 as *const _ as *mut c_void,
        }));

        // EX3: DropSourceReplicaOnMove
        let ex3 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX3 {
            DropSourceReplicaOnMove: self.drop_source_replica_on_move.unwrap_or(false),
            Reserved: ex4 as *const _ as *mut c_void,
        }));

        // EX2: ServicePlacementTimeLimitSeconds
        let ex2 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX2 {
            ServicePlacementTimeLimitSeconds: self
                .service_placement_time_limit_seconds
                .unwrap_or(0),
            Reserved: ex3 as *const _ as *mut c_void,
        }));

        // EX1: StandByReplicaKeepDurationSeconds
        let ex1 = pool.push(Box::new(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX1 {
            StandByReplicaKeepDurationSeconds: self
                .stand_by_replica_keep_duration_seconds
                .unwrap_or(0),
            Reserved: ex2 as *const _ as *mut c_void,
        }));

        // Base
        Some(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS {
            Flags: flags.bits(),
            ReplicaRestartWaitDurationSeconds: self
                .replica_restart_wait_duration_seconds
                .unwrap_or(0),
            QuorumLossWaitDurationSeconds: self.quorum_loss_wait_duration_seconds.unwrap_or(0),
            Reserved: ex1 as *const _ as *mut c_void,
        })
    }
}

bitflags::bitflags! {
    /// FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_FLAGS
    /// Indicates what fields are set in the description.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatefulServiceUpdateDescriptionFlags: u32 {
        const FABRIC_STATEFUL_SERVICE_NONE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_NONE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_REPLICA_RESTART_WAIT_DURATION.0 as u32;
        const FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUORUM_LOSS_WAIT_DURATION.0 as u32;
        const FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_STANDBY_REPLICA_KEEP_DURATION.0 as u32;
        const FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_MIN_REPLICA_SET_SIZE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_PLACEMENT_CONSTRAINTS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_POLICY_LIST = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_POLICY_LIST.0 as u32;
        const FABRIC_STATEFUL_SERVICE_CORRELATIONS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_CORRELATIONS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_METRICS = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_METRICS.0 as u32;
        const FABRIC_STATEFUL_SERVICE_MOVE_COST = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_MOVE_COST.0 as u32;
        const FABRIC_STATEFUL_SERVICE_SCALING_POLICY = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SCALING_POLICY.0 as u32;
        const FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT.0 as u32;
        const FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME.0 as u32;
        const FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_PLACE = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_PLACE.0 as u32;
        const FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_RUN = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_TAGS_REQUIRED_TO_RUN.0 as u32;
        const FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT.0 as u32;
        const FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY = mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY.0 as u32;
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
    pub(crate) fn build_raw(&self) -> ServiceUpdateDescriptionRaw<'_> {
        match self {
            ServiceUpdateDescription::Stateful(desc) => {
                ServiceUpdateDescriptionRaw::Stateful(desc.build_raw())
            }
            ServiceUpdateDescription::Stateless(desc) => {
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
    // ex6
    service_placement_time_limit_seconds: u32,
    // ex7
    drop_source_replica_on_move: bool,
    // ex8
    service_dns_name: WString,
    // ex9 - ReplicaLifecycleDescription
    is_singleton_replica_move_allowed_during_upgrade: Option<bool>,
    restore_replica_location_after_upgrade: Option<bool>,
    // ex10 - TagsDescription (not yet wired - complex type)
    // ex11
    auxiliary_replica_count: i32,
    // ex12 - ServiceSensitivityDescription (not yet wired - complex type)
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

    pub fn with_service_placement_time_limit_seconds(
        mut self,
        service_placement_time_limit_seconds: u32,
    ) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_SERVICE_PLACEMENT_TIME_LIMIT;
        self.service_placement_time_limit_seconds = service_placement_time_limit_seconds;
        self
    }

    pub fn with_drop_source_replica_on_move(mut self, drop_source_replica_on_move: bool) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_DROP_SOURCE_REPLICA_ON_MOVE;
        self.drop_source_replica_on_move = drop_source_replica_on_move;
        self
    }

    pub fn with_service_dns_name(mut self, service_dns_name: WString) -> Self {
        self.flags |=
            StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_SERVICE_DNS_NAME;
        self.service_dns_name = service_dns_name;
        self
    }

    pub fn with_is_singleton_replica_move_allowed_during_upgrade(mut self, allowed: bool) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_IS_SINGLETON_REPLICA_MOVE_ALLOWED_DURING_UPGRADE;
        self.is_singleton_replica_move_allowed_during_upgrade = Some(allowed);
        self
    }

    pub fn with_restore_replica_location_after_upgrade(mut self, restore: bool) -> Self {
        self.flags |= StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_RESTORE_REPLICA_LOCATION_AFTER_UPGRADE;
        self.restore_replica_location_after_upgrade = Some(restore);
        self
    }

    pub fn with_auxiliary_replica_count(mut self, auxiliary_replica_count: i32) -> Self {
        self.flags |=
            StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_AUXILIARY_REPLICA_COUNT;
        self.auxiliary_replica_count = auxiliary_replica_count;
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
    _internal_ex6: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6>,
    _internal_ex7: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7>,
    _internal_ex8: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8>,
    _internal_ex9: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9>,
    _internal_ex10: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10>,
    _internal_ex11: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11>,
    _internal_ex12: Box<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12>,
    _repartition_owner: ServiceRepartitionDescriptionRaw<'a>,
    _replica_lifecycle_owner: Box<mssf_com::FabricTypes::REPLICA_LIFECYCLE_DESCRIPTION>,
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
            ServiceUpdateDescriptionRaw::Stateful(desc) => FABRIC_SERVICE_UPDATE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
            ServiceUpdateDescriptionRaw::Stateless(desc) => FABRIC_SERVICE_UPDATE_DESCRIPTION {
                Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                Value: desc.as_ffi() as *const _ as *mut c_void,
            },
        }
    }
}

impl StatefulServiceUpdateDescription {
    pub(crate) fn build_raw(&self) -> StatefulServiceUpdateDescriptionRaw<'_> {
        let repartition_raw = self.repartition_description.as_raw();

        // Build replica lifecycle description for ex9
        let replica_lifecycle = Box::new(mssf_com::FabricTypes::REPLICA_LIFECYCLE_DESCRIPTION {
            IsIsSingletonReplicaMoveAllowedDuringUpgradeSpecified: self
                .is_singleton_replica_move_allowed_during_upgrade
                .is_some(),
            IsSingletonReplicaMoveAllowedDuringUpgrade: self
                .is_singleton_replica_move_allowed_during_upgrade
                .unwrap_or(false),
            IsRestoreReplicaLocationAfterUpgradeSpecified: self
                .restore_replica_location_after_upgrade
                .is_some(),
            RestoreReplicaLocationAfterUpgrade: self
                .restore_replica_location_after_upgrade
                .unwrap_or(false),
            Reserved: std::ptr::null_mut(),
        });

        let ex12 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12 {
            ServiceSensitivityDescription: std::ptr::null_mut(), // TODO: complex type
            Reserved: std::ptr::null_mut(),
        });
        let ex11 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11 {
            AuxiliaryReplicaCount: self.auxiliary_replica_count,
            Reserved: ex12.as_ref() as *const _ as *mut c_void,
        });
        let ex10 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10 {
            TagsDescription: std::ptr::null_mut(), // TODO: complex type
            Reserved: ex11.as_ref() as *const _ as *mut c_void,
        });
        let ex9 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9 {
            ReplicaLifecycleDescription: replica_lifecycle.as_ref() as *const _ as *mut _,
            Reserved: ex10.as_ref() as *const _ as *mut c_void,
        });
        let ex8 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8 {
            ServiceDnsName: self.service_dns_name.as_pcwstr(),
            Reserved: ex9.as_ref() as *const _ as *mut c_void,
        });
        let ex7 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7 {
            DropSourceReplicaOnMove: self.drop_source_replica_on_move,
            Reserved: ex8.as_ref() as *const _ as *mut c_void,
        });
        let ex6 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6 {
            ServicePlacementTimeLimitSeconds: self.service_placement_time_limit_seconds,
            Reserved: ex7.as_ref() as *const _ as *mut c_void,
        });
        let ex5 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5 {
            RepartitionDescription: repartition_raw.as_ffi().1 as *const _ as *mut _,
            RepartitionKind: repartition_raw.as_ffi().0,
            ScalingPolicyCount: 0,
            ServiceScalingPolicies: std::ptr::null_mut(), // TODO: FABRIC_SERVICE_SCALING_POLICY
            Reserved: ex6.as_ref() as *const _ as *mut c_void,
        });
        let ex4 = Box::new(FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4 {
            DefaultMoveCost: self.default_move_cost.into(),
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
            _internal_ex6: ex6,
            _internal_ex7: ex7,
            _internal_ex8: ex8,
            _internal_ex9: ex9,
            _internal_ex10: ex10,
            _internal_ex11: ex11,
            _internal_ex12: ex12,
            _repartition_owner: repartition_raw,
            _replica_lifecycle_owner: replica_lifecycle,
        }
    }
}

// FABRIC_SERVICE_HEALTH_STATES_FILTER
#[derive(Debug, Clone)]
pub struct ServiceHealthStatesFilter {
    pub health_state_filter: HealthStateFilterFlags,
}

impl GetRaw<mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_STATES_FILTER>
    for ServiceHealthStatesFilter
{
    fn get_raw(&self) -> mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_STATES_FILTER {
        mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_STATES_FILTER {
            HealthStateFilter: self.health_state_filter.bits() as u32,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_SERVICE_QUERY_DESCRIPTION
#[derive(Debug, Default, Clone)]
pub struct ServiceQueryDescription {
    pub application_name: Uri,
    pub service_name_filter: Option<Uri>,
    pub continuation_token: Option<WString>,
    pub service_type_name_filter: Option<WString>,
    pub max_results: Option<i32>,
}

impl ServiceQueryDescription {
    pub fn get_raw_parts(
        &self,
    ) -> (
        FABRIC_SERVICE_QUERY_DESCRIPTION,
        FABRIC_SERVICE_QUERY_DESCRIPTION_EX1,
        FABRIC_SERVICE_QUERY_DESCRIPTION_EX2,
        FABRIC_SERVICE_QUERY_DESCRIPTION_EX3,
    ) {
        let base = FABRIC_SERVICE_QUERY_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            ServiceNameFilter: self
                .service_name_filter
                .as_ref()
                .map_or(Uri::default().as_raw(), |uri| uri.as_raw()),
            Reserved: std::ptr::null_mut(),
        };
        let ex1 = FABRIC_SERVICE_QUERY_DESCRIPTION_EX1 {
            ContinuationToken: self
                .continuation_token
                .as_ref()
                .map_or(PCWSTR::null(), |s| s.as_pcwstr()),
            Reserved: std::ptr::null_mut(),
        };
        let ex2 = FABRIC_SERVICE_QUERY_DESCRIPTION_EX2 {
            ServiceTypeNameFilter: self
                .service_type_name_filter
                .as_ref()
                .map_or(PCWSTR::null(), |s| s.as_pcwstr()),
            Reserved: std::ptr::null_mut(),
        };
        let ex3 = FABRIC_SERVICE_QUERY_DESCRIPTION_EX3 {
            MaxResults: self.max_results.unwrap_or(0), // 0 means no limit
            Reserved: std::ptr::null_mut(),
        };
        (base, ex1, ex2, ex3)
    }
}

// IFabricGetServiceListResult
#[derive(Debug)]
pub struct ServiceListResult {
    pub items: Vec<ServiceQueryResultItem>,
    pub paging_status: Option<PagingStatus>,
}

impl From<&IFabricGetServiceListResult2> for ServiceListResult {
    fn from(value: &IFabricGetServiceListResult2) -> Self {
        let list = unsafe { value.get_ServiceList().as_ref().unwrap() };
        let items = crate::iter::vec_from_raw_com(list.Count as usize, list.Items);
        let paging_status_opt = unsafe { value.get_PagingStatus().as_ref() };
        ServiceListResult {
            items,
            paging_status: paging_status_opt.map(PagingStatus::from),
        }
    }
}

// FABRIC_SERVICE_HEALTH_STATE
#[derive(Debug, Clone)]
pub struct ServiceHealthState {
    pub service_name: Uri,
    pub aggregated_health_state: HealthState,
}

impl From<&FABRIC_SERVICE_HEALTH_STATE> for ServiceHealthState {
    fn from(value: &FABRIC_SERVICE_HEALTH_STATE) -> Self {
        Self {
            service_name: Uri::from(value.ServiceName),
            aggregated_health_state: HealthState::from(&value.AggregatedHealthState),
        }
    }
}
// FABRIC_SERVICE_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub enum ServiceQueryResultItem {
    // FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM
    Stateful(StatefulServiceQueryResultItem),
    // FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM
    Stateless(StatelessServiceQueryResultItem),
}

impl From<&FABRIC_SERVICE_QUERY_RESULT_ITEM> for ServiceQueryResultItem {
    fn from(value: &FABRIC_SERVICE_QUERY_RESULT_ITEM) -> Self {
        match value.Kind {
            mssf_com::FabricTypes::FABRIC_SERVICE_KIND_STATEFUL => {
                let item = unsafe {
                    (value.Value
                        as *const mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                ServiceQueryResultItem::Stateful(StatefulServiceQueryResultItem::from(item))
            }
            mssf_com::FabricTypes::FABRIC_SERVICE_KIND_STATELESS => {
                let item = unsafe {
                    (value.Value
                        as *const mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM)
                        .as_ref()
                        .unwrap()
                };
                ServiceQueryResultItem::Stateless(StatelessServiceQueryResultItem::from(item))
            }
            // TODO: may need to handle other kinds with newer sdks.
            _ => panic!("Unknown service query result kind"),
        }
    }
}

impl ServiceQueryResultItem {
    pub fn get_health_state(&self) -> HealthState {
        match self {
            ServiceQueryResultItem::Stateful(item) => item.health_state,
            ServiceQueryResultItem::Stateless(item) => item.health_state,
        }
    }

    pub fn get_service_name(&self) -> &Uri {
        match self {
            ServiceQueryResultItem::Stateful(item) => &item.service_name,
            ServiceQueryResultItem::Stateless(item) => &item.service_name,
        }
    }
}

// FABRIC_QUERY_SERVICE_STATUS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryServiceStatus {
    Unknown,
    Active,
    Upgrading,
    Deleting,
    Creating,
    Failed,
}

impl From<mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS> for QueryServiceStatus {
    fn from(value: mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS) -> Self {
        match value {
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_ACTIVE => QueryServiceStatus::Active,
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_UPGRADING => {
                QueryServiceStatus::Upgrading
            }
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_DELETING => {
                QueryServiceStatus::Deleting
            }
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_CREATING => {
                QueryServiceStatus::Creating
            }
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_FAILED => QueryServiceStatus::Failed,
            mssf_com::FabricTypes::FABRIC_QUERY_SERVICE_STATUS_UNKNOWN => {
                QueryServiceStatus::Unknown
            }
            _ => QueryServiceStatus::Unknown,
        }
    }
}

// FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct StatefulServiceQueryResultItem {
    pub service_name: Uri,
    pub service_type_name: WString,
    pub service_manifest_version: WString,
    pub has_persisted_state: bool,
    pub health_state: HealthState,
    pub service_status: QueryServiceStatus,
    pub is_service_group: bool,
}

impl From<&mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM>
    for StatefulServiceQueryResultItem
{
    fn from(value: &mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM) -> Self {
        let ex1 = unsafe {
            (value.Reserved
                as *const mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX1)
                .as_ref()
                .unwrap()
        };
        let ex2 = unsafe {
            (ex1.Reserved
                as *const mssf_com::FabricTypes::FABRIC_STATEFUL_SERVICE_QUERY_RESULT_ITEM_EX2)
                .as_ref()
                .unwrap()
        };

        Self {
            service_name: Uri::from(value.ServiceName),
            service_type_name: WString::from(value.ServiceTypeName),
            service_manifest_version: WString::from(value.ServiceManifestVersion),
            has_persisted_state: value.HasPersistedState,
            health_state: HealthState::from(&value.HealthState),
            service_status: QueryServiceStatus::from(ex1.ServiceStatus),
            is_service_group: ex2.IsServiceGroup,
        }
    }
}

// FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM
#[derive(Debug, Clone)]
pub struct StatelessServiceQueryResultItem {
    pub service_name: Uri,
    pub service_type_name: WString,
    pub service_manifest_version: WString,
    pub health_state: HealthState,
    pub service_status: QueryServiceStatus,
    pub is_service_group: bool,
}

impl From<&mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM>
    for StatelessServiceQueryResultItem
{
    fn from(value: &mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM) -> Self {
        let ex1 = unsafe {
            (value.Reserved
                as *const mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX1)
                .as_ref()
                .unwrap()
        };
        let ex2 = unsafe {
            (ex1.Reserved
                as *const mssf_com::FabricTypes::FABRIC_STATELESS_SERVICE_QUERY_RESULT_ITEM_EX2)
                .as_ref()
                .unwrap()
        };

        Self {
            service_name: Uri::from(value.ServiceName),
            service_type_name: WString::from(value.ServiceTypeName),
            service_manifest_version: WString::from(value.ServiceManifestVersion),
            health_state: HealthState::from(&value.HealthState),
            service_status: QueryServiceStatus::from(ex1.ServiceStatus),
            is_service_group: ex2.IsServiceGroup,
        }
    }
}

// FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION
#[derive(Debug, Clone, Default)]
pub struct ServiceHealthQueryDescription {
    pub service_name: Uri,
    pub health_policy: Option<ApplicationHealthPolicy>,
    pub events_filter: Option<HealthEventsFilter>,
    // TODO: implement other filters
    // pub partitions_filter: Option<PartitionHealthStatesFilter>,
    // pub health_statistics_filter: Option<HealthStatisticsFilter>,
}

impl crate::mem::GetRawWithBoxPool<mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION>
    for ServiceHealthQueryDescription
{
    fn get_raw_with_pool(
        &self,
        pool: &mut crate::mem::BoxPool,
    ) -> mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION {
        let health_policy = self
            .health_policy
            .as_ref()
            .map(|p| {
                let b = Box::new(p.get_raw_with_pool(pool));
                pool.push(b)
            })
            .unwrap_or_default();
        let events_filter = self
            .events_filter
            .as_ref()
            .map(|f| {
                let b = Box::new(f.get_raw());
                pool.push(b)
            })
            .unwrap_or_default();
        let ex1 = Box::new(
            mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION_EX1 {
                HealthStatisticsFilter: std::ptr::null_mut(),
                Reserved: std::ptr::null_mut(),
            },
        );
        let ex1 = pool.push(ex1);
        mssf_com::FabricTypes::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION {
            ServiceName: self.service_name.as_raw(),
            HealthPolicy: health_policy,
            EventsFilter: events_filter,
            PartitionsFilter: std::ptr::null_mut(),
            Reserved: ex1 as *mut c_void,
        }
    }
}

// IFabricServiceHealthResult
#[derive(Debug, Clone)]
pub struct ServiceHealthResult {
    pub service_name: Uri,
    pub aggregated_health_state: HealthState,
    pub health_events: Vec<HealthEvent>,
    // TODO: implement other fields
    // pub partitions_health: Vec<PartitionHealth>,
}

impl From<&mssf_com::FabricClient::IFabricServiceHealthResult> for ServiceHealthResult {
    fn from(value: &mssf_com::FabricClient::IFabricServiceHealthResult) -> Self {
        let raw = unsafe { value.get_ServiceHealth().as_ref().unwrap() };

        let health_event_list = unsafe { raw.HealthEvents.as_ref() }.map_or(vec![], |list| {
            crate::iter::vec_from_raw_com(list.Count as usize, list.Items)
        });

        Self {
            service_name: Uri::from(raw.ServiceName),
            aggregated_health_state: HealthState::from(&raw.AggregatedHealthState),
            health_events: health_event_list,
        }
    }
}
