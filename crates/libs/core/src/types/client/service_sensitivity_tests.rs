// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use super::*;
use mssf_com::FabricTypes::SERVICE_SENSITIVITY_DESCRIPTION;

// Walk the actual ABI chain emitted for UpdateService while its pool is alive.
unsafe fn update_sensitivity(
    raw: &FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION,
) -> *const SERVICE_SENSITIVITY_DESCRIPTION {
    // SAFETY: callers provide a description emitted by get_raw_with_pool and
    // keep that pool alive. Each Reserved pointer has the documented EX type.
    unsafe {
        let ex1 = &*raw
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX1>();
        let ex2 = &*ex1
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX2>();
        let ex3 = &*ex2
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX3>();
        let ex4 = &*ex3
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX4>();
        let ex5 = &*ex4
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX5>();
        let ex6 = &*ex5
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX6>();
        let ex7 = &*ex6
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX7>();
        let ex8 = &*ex7
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX8>();
        let ex9 = &*ex8
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX9>();
        let ex10 = &*ex9
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX10>();
        let ex11 = &*ex10
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX11>();
        let ex12 = &*ex11
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION_EX12>();
        assert!(ex12.Reserved.is_null());
        ex12.ServiceSensitivityDescription
    }
}

#[test]
fn sensitivity_update_marshals_all_fields_and_explicit_false() {
    for enabled in [true, false] {
        let mut pool = BoxPool::new();
        let raw = {
            let desc = ServiceUpdateDescription::Stateful(
                StatefulServiceUpdateDescription::new().with_service_sensitivity(
                    ServiceSensitivityDescription {
                        primary_default_sensitivity: 11,
                        secondary_default_sensitivity: 22,
                        auxiliary_default_sensitivity: 33,
                        is_maximum_sensitivity: enabled,
                    },
                ),
            );
            desc.get_raw_with_pool(&mut pool)
        };
        // Growing the pool must not invalidate any of the boxed ABI objects.
        for value in 0..64 {
            let _ptr = pool.push(Box::new(value));
        }
        assert_eq!(raw.Kind, FABRIC_SERVICE_DESCRIPTION_KIND_STATEFUL);
        // SAFETY: the stateful description and its full chain are owned by pool.
        let stateful = unsafe {
            &*raw
                .Value
                .cast::<FABRIC_STATEFUL_SERVICE_UPDATE_DESCRIPTION>()
        };
        assert_eq!(
            stateful.Flags,
            StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY
                .bits()
        );
        // SAFETY: pool is alive and the sensitivity setter populated EX12.
        let sensitivity = unsafe { &*update_sensitivity(stateful) };
        assert_eq!(sensitivity.PrimaryDefaultSensitivity, 11);
        assert_eq!(sensitivity.SecondaryDefaultSensitivity, 22);
        assert_eq!(sensitivity.AuxiliaryDefaultSensitivity, 33);
        assert_eq!(sensitivity.IsMaximumSensitivity, enabled);
        assert!(sensitivity.Reserved.is_null());
    }
}

#[test]
fn omitted_sensitivity_does_not_set_flag_or_pointer() {
    let desc = StatefulServiceUpdateDescription::new().with_target_replica_set_size(5);
    let mut pool = BoxPool::new();
    let raw = desc.get_raw_with_pool(&mut pool);
    assert_eq!(
        raw.Flags,
        StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE
            .bits()
    );
    assert_eq!(raw.TargetReplicaSetSize, 5);
    // SAFETY: get_raw_with_pool populated the chain and pool is still alive.
    assert!(unsafe { update_sensitivity(&raw) }.is_null());
}

#[test]
fn sensitivity_setter_composes_and_latest_value_wins() {
    let desc = StatefulServiceUpdateDescription::new()
        .with_target_replica_set_size(5)
        .with_service_sensitivity(ServiceSensitivityDescription {
            is_maximum_sensitivity: true,
            ..Default::default()
        })
        .with_service_sensitivity(ServiceSensitivityDescription::default());
    let mut pool = BoxPool::new();
    let raw = desc.get_raw_with_pool(&mut pool);
    assert_eq!(
        raw.Flags,
        (StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_TARGET_REPLICA_SET_SIZE
            | StatefulServiceUpdateDescriptionFlags::FABRIC_STATEFUL_SERVICE_SERVICE_SENSITIVITY)
            .bits()
    );
    assert_eq!(raw.TargetReplicaSetSize, 5);
    // SAFETY: the sensitivity setter populated EX12 and pool is still alive.
    let sensitivity = unsafe { &*update_sensitivity(&raw) };
    assert!(!sensitivity.IsMaximumSensitivity);
    assert_eq!(sensitivity.PrimaryDefaultSensitivity, 0);
}

#[test]
fn create_service_sensitivity_uses_same_conversion() {
    let settings = StatefulServiceFailoverSettings {
        service_sensitivity: Some(ServiceSensitivityDescription {
            primary_default_sensitivity: 1,
            secondary_default_sensitivity: 2,
            auxiliary_default_sensitivity: 3,
            is_maximum_sensitivity: true,
        }),
        ..Default::default()
    };
    let mut pool = BoxPool::new();
    let raw = settings
        .get_raw_with_pool(&mut pool)
        .expect("settings provided");
    assert_eq!(
        raw.Flags,
        StatefulServiceFailoverSettingsFlags::SERVICE_SENSITIVITY.bits()
    );
    // SAFETY: get_raw_with_pool owns the complete failover-settings chain in pool.
    let sensitivity = unsafe {
        let ex1 = &*raw
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX1>();
        let ex2 = &*ex1
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX2>();
        let ex3 = &*ex2
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX3>();
        let ex4 = &*ex3
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX4>();
        let ex5 = &*ex4
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX5>();
        let ex6 = &*ex5
            .Reserved
            .cast::<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS_EX6>();
        &*ex6.ServiceSensitivityDescription
    };
    assert_eq!(sensitivity.PrimaryDefaultSensitivity, 1);
    assert_eq!(sensitivity.SecondaryDefaultSensitivity, 2);
    assert_eq!(sensitivity.AuxiliaryDefaultSensitivity, 3);
    assert!(sensitivity.IsMaximumSensitivity);
    assert!(sensitivity.Reserved.is_null());
}
