// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use super::*;
use mssf_com::FabricTypes::SERVICE_SENSITIVITY_DESCRIPTION;

// Walk the actual ABI chain emitted for UpdateService while its pool is alive.
unsafe fn get_sensitivity(
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
        let sensitivity = unsafe { &*get_sensitivity(stateful) };
        assert_eq!(sensitivity.PrimaryDefaultSensitivity, 11);
        assert_eq!(sensitivity.SecondaryDefaultSensitivity, 22);
        assert_eq!(sensitivity.AuxiliaryDefaultSensitivity, 33);
        assert_eq!(sensitivity.IsMaximumSensitivity, enabled);
        assert!(sensitivity.Reserved.is_null());
    }
}
