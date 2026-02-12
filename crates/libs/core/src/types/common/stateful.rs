// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricTypes::{
    FABRIC_REPLICA_ROLE, FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY,
    FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY, FABRIC_REPLICA_ROLE_IDLE_AUXILIARY,
    FABRIC_REPLICA_ROLE_IDLE_SECONDARY, FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
    FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY, FABRIC_REPLICA_ROLE_UNKNOWN,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ReplicaRole {
    ActiveSecondary,
    ActiveAuxiliary,
    IdleAuxiliary,
    IdleSecondary,
    None,
    Primary,
    PrimaryAuxiliary,
    Unknown,
}

impl From<&FABRIC_REPLICA_ROLE> for ReplicaRole {
    fn from(r: &FABRIC_REPLICA_ROLE) -> Self {
        match *r {
            FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY => ReplicaRole::ActiveAuxiliary,
            FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY => ReplicaRole::ActiveSecondary,
            FABRIC_REPLICA_ROLE_IDLE_AUXILIARY => ReplicaRole::IdleAuxiliary,
            FABRIC_REPLICA_ROLE_IDLE_SECONDARY => ReplicaRole::IdleSecondary,
            FABRIC_REPLICA_ROLE_NONE => ReplicaRole::None,
            FABRIC_REPLICA_ROLE_PRIMARY => ReplicaRole::Primary,
            FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY => ReplicaRole::PrimaryAuxiliary,
            _ => ReplicaRole::Unknown,
        }
    }
}

impl From<&ReplicaRole> for FABRIC_REPLICA_ROLE {
    fn from(val: &ReplicaRole) -> Self {
        match *val {
            ReplicaRole::ActiveAuxiliary => FABRIC_REPLICA_ROLE_ACTIVE_AUXILIARY,
            ReplicaRole::ActiveSecondary => FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
            ReplicaRole::IdleAuxiliary => FABRIC_REPLICA_ROLE_IDLE_AUXILIARY,
            ReplicaRole::IdleSecondary => FABRIC_REPLICA_ROLE_IDLE_SECONDARY,
            ReplicaRole::None => FABRIC_REPLICA_ROLE_NONE,
            ReplicaRole::Primary => FABRIC_REPLICA_ROLE_PRIMARY,
            ReplicaRole::PrimaryAuxiliary => FABRIC_REPLICA_ROLE_PRIMARY_AUXILIARY,
            ReplicaRole::Unknown => FABRIC_REPLICA_ROLE_UNKNOWN,
        }
    }
}
