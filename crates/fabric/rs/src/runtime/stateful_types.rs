// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_types contains type wrappers for sf stateful raw types

use fabric_base::{
    FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE,
    FABRIC_REPLICA_OPEN_MODE_EXISTING, FABRIC_REPLICA_OPEN_MODE_INVALID,
    FABRIC_REPLICA_OPEN_MODE_NEW, FABRIC_REPLICA_ROLE, FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
    FABRIC_REPLICA_ROLE_IDLE_SECONDARY, FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
    FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_ALL,
    FABRIC_REPLICA_SET_QUORUM_INVALID, FABRIC_REPLICA_SET_QUORUM_MODE,
    FABRIC_REPLICA_SET_WRITE_QUORUM, FABRIC_REPLICA_STATUS, FABRIC_REPLICA_STATUS_DOWN,
    FABRIC_REPLICA_STATUS_INVALID, FABRIC_REPLICA_STATUS_UP,
};
use windows_core::{HSTRING, PCWSTR};

#[derive(Debug)]
pub enum OpenMode {
    Invald,
    Existing,
    New,
}

impl From<FABRIC_REPLICA_OPEN_MODE> for OpenMode {
    fn from(e: FABRIC_REPLICA_OPEN_MODE) -> Self {
        match e {
            FABRIC_REPLICA_OPEN_MODE_EXISTING => OpenMode::Existing,
            FABRIC_REPLICA_OPEN_MODE_NEW => OpenMode::New,
            _ => OpenMode::Invald,
        }
    }
}
impl From<OpenMode> for FABRIC_REPLICA_OPEN_MODE {
    fn from(val: OpenMode) -> Self {
        match val {
            OpenMode::Invald => FABRIC_REPLICA_OPEN_MODE_INVALID,
            OpenMode::Existing => FABRIC_REPLICA_OPEN_MODE_EXISTING,
            OpenMode::New => FABRIC_REPLICA_OPEN_MODE_NEW,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Epoch {
    pub data_loss_number: i64,
    pub configuration_number: i64,
}

impl From<&FABRIC_EPOCH> for Epoch {
    fn from(e: &FABRIC_EPOCH) -> Self {
        Epoch {
            data_loss_number: e.DataLossNumber,
            configuration_number: e.ConfigurationNumber,
        }
    }
}

impl From<Epoch> for FABRIC_EPOCH {
    fn from(val: Epoch) -> Self {
        FABRIC_EPOCH {
            DataLossNumber: val.data_loss_number,
            ConfigurationNumber: val.configuration_number,
            Reserved: std::ptr::null_mut(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Role {
    ActiveSecondary,
    IdleSecondary,
    None,
    Primary,
    Unknown,
}

impl From<FABRIC_REPLICA_ROLE> for Role {
    fn from(r: FABRIC_REPLICA_ROLE) -> Self {
        match r {
            FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY => Role::ActiveSecondary,
            FABRIC_REPLICA_ROLE_IDLE_SECONDARY => Role::IdleSecondary,
            FABRIC_REPLICA_ROLE_NONE => Role::None,
            FABRIC_REPLICA_ROLE_PRIMARY => Role::Primary,
            _ => Role::Unknown,
        }
    }
}

impl From<Role> for FABRIC_REPLICA_ROLE {
    fn from(val: Role) -> Self {
        match val {
            Role::ActiveSecondary => FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
            Role::IdleSecondary => FABRIC_REPLICA_ROLE_IDLE_SECONDARY,
            Role::None => FABRIC_REPLICA_ROLE_NONE,
            Role::Primary => FABRIC_REPLICA_ROLE_PRIMARY,
            Role::Unknown => FABRIC_REPLICA_ROLE_NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ReplicaStatus {
    Invalid,
    Down,
    Up,
}

impl From<ReplicaStatus> for FABRIC_REPLICA_STATUS {
    fn from(val: ReplicaStatus) -> Self {
        match val {
            ReplicaStatus::Invalid => FABRIC_REPLICA_STATUS_INVALID,
            ReplicaStatus::Down => FABRIC_REPLICA_STATUS_DOWN,
            ReplicaStatus::Up => FABRIC_REPLICA_STATUS_UP,
        }
    }
}

impl From<FABRIC_REPLICA_STATUS> for ReplicaStatus {
    fn from(r: FABRIC_REPLICA_STATUS) -> Self {
        match r {
            FABRIC_REPLICA_STATUS_INVALID => ReplicaStatus::Invalid,
            FABRIC_REPLICA_STATUS_DOWN => ReplicaStatus::Down,
            FABRIC_REPLICA_STATUS_UP => ReplicaStatus::Up,
            _ => ReplicaStatus::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct ReplicaSetConfig {
    //pub ReplicaCount: u32,
    //pub Replicas: *const FABRIC_REPLICA_INFORMATION,
    pub Replicas: Vec<ReplicaInfo>,
    pub WriteQuorum: u32,
    // pub Reserved: *mut ::core::ffi::c_void,
    replica_raw_cache: Vec<FABRIC_REPLICA_INFORMATION>,
}

impl From<&FABRIC_REPLICA_SET_CONFIGURATION> for ReplicaSetConfig {
    fn from(r: &FABRIC_REPLICA_SET_CONFIGURATION) -> Self {
        let mut res = ReplicaSetConfig {
            Replicas: vec![],
            WriteQuorum: r.WriteQuorum,
            replica_raw_cache: vec![],
        };
        // fill the vec
        for i in 0..r.ReplicaCount {
            let replica = unsafe { r.Replicas.offset(i as isize) };
            let replica_ref = unsafe { replica.as_ref().unwrap() };
            res.Replicas.push(ReplicaInfo::from(replica_ref))
        }
        res.fill_cache_copy();
        res
    }
}

impl ReplicaSetConfig {
    pub fn get_raw(&self) -> FABRIC_REPLICA_SET_CONFIGURATION {
        FABRIC_REPLICA_SET_CONFIGURATION {
            ReplicaCount: self.replica_raw_cache.len() as u32,
            Replicas: self.replica_raw_cache.as_ptr(),
            WriteQuorum: self.WriteQuorum,
            Reserved: std::ptr::null_mut(),
        }
    }

    fn fill_cache_copy(&mut self) {
        self.replica_raw_cache.clear();
        for replica in &self.Replicas {
            self.replica_raw_cache.push(replica.get_raw());
        }
    }
}

#[derive(Debug)]
pub struct ReplicaInfo {
    pub Id: i64,
    pub Role: Role,
    pub Status: ReplicaStatus,
    pub ReplicatorAddress: ::windows_core::HSTRING,
    pub CurrentProgress: i64,
    pub CatchUpCapability: i64,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl From<&FABRIC_REPLICA_INFORMATION> for ReplicaInfo {
    fn from(r: &FABRIC_REPLICA_INFORMATION) -> Self {
        ReplicaInfo {
            Id: r.Id,
            Role: r.Role.into(),
            Status: r.Status.into(),
            ReplicatorAddress: HSTRING::from_wide(unsafe { r.ReplicatorAddress.as_wide() })
                .unwrap(),
            CurrentProgress: r.CurrentProgress,
            CatchUpCapability: r.CatchUpCapability,
        }
    }
}

impl ReplicaInfo {
    pub fn get_raw(&self) -> FABRIC_REPLICA_INFORMATION {
        FABRIC_REPLICA_INFORMATION {
            Id: self.Id,
            Role: self.Role.clone().into(),
            Status: self.Status.clone().into(),
            ReplicatorAddress: PCWSTR::from_raw(self.ReplicatorAddress.as_ptr()),
            CurrentProgress: self.CurrentProgress,
            CatchUpCapability: self.CatchUpCapability,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_REPLICA_SET_QUORUM_MODE
pub enum ReplicaSetQuarumMode {
    All,
    Invalid,
    Write,
}

impl From<FABRIC_REPLICA_SET_QUORUM_MODE> for ReplicaSetQuarumMode {
    fn from(r: FABRIC_REPLICA_SET_QUORUM_MODE) -> Self {
        match r {
            FABRIC_REPLICA_SET_QUORUM_ALL => ReplicaSetQuarumMode::All,
            FABRIC_REPLICA_SET_QUORUM_INVALID => ReplicaSetQuarumMode::Invalid,
            FABRIC_REPLICA_SET_WRITE_QUORUM => ReplicaSetQuarumMode::Write,
            _ => ReplicaSetQuarumMode::Invalid,
        }
    }
}

impl From<ReplicaSetQuarumMode> for FABRIC_REPLICA_SET_QUORUM_MODE {
    fn from(val: ReplicaSetQuarumMode) -> Self {
        match val {
            ReplicaSetQuarumMode::All => FABRIC_REPLICA_SET_QUORUM_ALL,
            ReplicaSetQuarumMode::Invalid => FABRIC_REPLICA_SET_QUORUM_INVALID,
            ReplicaSetQuarumMode::Write => FABRIC_REPLICA_SET_WRITE_QUORUM,
        }
    }
}
