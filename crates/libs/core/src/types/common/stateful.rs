// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricTypes::{
    FABRIC_REPLICA_ROLE, FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY, FABRIC_REPLICA_ROLE_IDLE_SECONDARY,
    FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
};

#[derive(PartialEq, Clone, Debug)]
pub enum ReplicaRole {
    ActiveSecondary,
    IdleSecondary,
    None,
    Primary,
    Unknown,
}

impl From<&FABRIC_REPLICA_ROLE> for ReplicaRole {
    fn from(r: &FABRIC_REPLICA_ROLE) -> Self {
        match *r {
            FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY => ReplicaRole::ActiveSecondary,
            FABRIC_REPLICA_ROLE_IDLE_SECONDARY => ReplicaRole::IdleSecondary,
            FABRIC_REPLICA_ROLE_NONE => ReplicaRole::None,
            FABRIC_REPLICA_ROLE_PRIMARY => ReplicaRole::Primary,
            _ => ReplicaRole::Unknown,
        }
    }
}

impl From<&ReplicaRole> for FABRIC_REPLICA_ROLE {
    fn from(val: &ReplicaRole) -> Self {
        match *val {
            ReplicaRole::ActiveSecondary => FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
            ReplicaRole::IdleSecondary => FABRIC_REPLICA_ROLE_IDLE_SECONDARY,
            ReplicaRole::None => FABRIC_REPLICA_ROLE_NONE,
            ReplicaRole::Primary => FABRIC_REPLICA_ROLE_PRIMARY,
            ReplicaRole::Unknown => FABRIC_REPLICA_ROLE_NONE,
        }
    }
}
