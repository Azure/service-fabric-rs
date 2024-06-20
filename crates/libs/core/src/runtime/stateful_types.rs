// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_types contains type wrappers for sf stateful raw types

use std::{ffi::c_void, marker::PhantomData};

use mssf_com::{
    FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1,
    FABRIC_REPLICA_OPEN_MODE, FABRIC_REPLICA_OPEN_MODE_EXISTING, FABRIC_REPLICA_OPEN_MODE_INVALID,
    FABRIC_REPLICA_OPEN_MODE_NEW, FABRIC_REPLICA_ROLE, FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
    FABRIC_REPLICA_ROLE_IDLE_SECONDARY, FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
    FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_ALL,
    FABRIC_REPLICA_SET_QUORUM_INVALID, FABRIC_REPLICA_SET_QUORUM_MODE,
    FABRIC_REPLICA_SET_WRITE_QUORUM, FABRIC_REPLICA_STATUS, FABRIC_REPLICA_STATUS_DOWN,
    FABRIC_REPLICA_STATUS_INVALID, FABRIC_REPLICA_STATUS_UP,
};
use windows_core::PCWSTR;

use crate::strings::HSTRINGWrap;

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

#[derive(Clone, Debug, PartialEq)]
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

// Safe wrapping for FABRIC_REPLICA_SET_CONFIGURATION
#[derive(Debug)]
pub struct ReplicaSetConfig {
    pub Replicas: Vec<ReplicaInfo>,
    pub WriteQuorum: u32,
}

impl From<&FABRIC_REPLICA_SET_CONFIGURATION> for ReplicaSetConfig {
    fn from(r: &FABRIC_REPLICA_SET_CONFIGURATION) -> Self {
        let mut res = ReplicaSetConfig {
            Replicas: vec![],
            WriteQuorum: r.WriteQuorum,
        };
        // fill the vec
        for i in 0..r.ReplicaCount {
            let replica = unsafe { r.Replicas.offset(i as isize) };
            let replica_ref = unsafe { replica.as_ref().unwrap() };
            res.Replicas.push(ReplicaInfo::from(replica_ref))
        }
        res
    }
}

impl ReplicaSetConfig {
    // view has the lifetime as self
    pub fn get_view(&self) -> ReplicaSetConfigView {
        // fast return for raw types needed by COM
        let mut replica_raw_cache: Vec<FABRIC_REPLICA_INFORMATION> = vec![];
        let mut replica_ex1_cache: Vec<FABRIC_REPLICA_INFORMATION_EX1> = vec![];
        // prepare vec cache
        for replica in &self.Replicas {
            let (info, ex1) = replica.get_raw_parts();
            replica_raw_cache.push(info);
            replica_ex1_cache.push(ex1);
        }

        // stitch the raw parts together
        let info_iter = replica_raw_cache.iter_mut();
        let mut ex1_iter = replica_ex1_cache.iter();
        for i in info_iter {
            // 2 vec has the same length, this cannot fail
            let ex = ex1_iter.next().unwrap();
            i.Reserved = ex as *const FABRIC_REPLICA_INFORMATION_EX1 as *mut c_void;
        }

        let config = FABRIC_REPLICA_SET_CONFIGURATION {
            ReplicaCount: replica_raw_cache.len() as u32,
            Replicas: replica_raw_cache.as_ptr(),
            WriteQuorum: self.WriteQuorum,
            Reserved: std::ptr::null_mut(),
        };

        ReplicaSetConfigView {
            _replica_raw_vec: replica_raw_cache,
            _replica_ex1_vec: replica_ex1_cache,
            raw: config,
            _phantom: PhantomData,
        }
    }
}

// View is not movable because it has raw pointer links in it.
// And it has the same lifetime as the config.
pub struct ReplicaSetConfigView<'a> {
    _replica_raw_vec: Vec<FABRIC_REPLICA_INFORMATION>,
    _replica_ex1_vec: Vec<FABRIC_REPLICA_INFORMATION_EX1>,
    raw: FABRIC_REPLICA_SET_CONFIGURATION,
    _phantom: PhantomData<&'a ReplicaSetConfig>,
}

impl ReplicaSetConfigView<'_> {
    // returns the config that can be passed to SF com api.
    pub fn get_raw(&self) -> &FABRIC_REPLICA_SET_CONFIGURATION {
        &self.raw
    }
}

// Safe wrapping for FABRIC_REPLICA_INFORMATION
#[derive(Debug, PartialEq, Clone)]
pub struct ReplicaInfo {
    pub Id: i64,
    pub Role: Role,
    pub Status: ReplicaStatus,
    pub ReplicatorAddress: ::windows_core::HSTRING,
    pub CurrentProgress: i64,
    pub CatchUpCapability: i64,
    pub MustCatchUp: bool,
}

impl From<&FABRIC_REPLICA_INFORMATION> for ReplicaInfo {
    fn from(r: &FABRIC_REPLICA_INFORMATION) -> Self {
        let ex1 = r.Reserved as *const FABRIC_REPLICA_INFORMATION_EX1;
        let mut must_catchup = false;
        if !ex1.is_null() {
            if let Some(ex1ref) = unsafe { ex1.as_ref() } {
                must_catchup = ex1ref.MustCatchup.as_bool();
            }
        }
        ReplicaInfo {
            Id: r.Id,
            Role: r.Role.into(),
            Status: r.Status.into(),
            ReplicatorAddress: HSTRINGWrap::from(r.ReplicatorAddress).into(),
            CurrentProgress: r.CurrentProgress,
            CatchUpCapability: r.CatchUpCapability,
            MustCatchUp: must_catchup,
        }
    }
}

impl ReplicaInfo {
    // The parts has the same lifetime as self.
    // Caller needs to stitch the parts together.
    pub fn get_raw_parts(&self) -> (FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1) {
        let info = FABRIC_REPLICA_INFORMATION {
            Id: self.Id,
            Role: self.Role.clone().into(),
            Status: self.Status.clone().into(),
            ReplicatorAddress: PCWSTR::from_raw(self.ReplicatorAddress.as_ptr()),
            CurrentProgress: self.CurrentProgress,
            CatchUpCapability: self.CatchUpCapability,
            Reserved: std::ptr::null_mut(),
        };
        let ex1 = FABRIC_REPLICA_INFORMATION_EX1 {
            MustCatchup: self.MustCatchUp.into(),
            Reserved: std::ptr::null_mut(),
        };
        (info, ex1)
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

#[cfg(test)]
mod test {
    use std::ffi::c_void;

    use mssf_com::{FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1};
    use windows_core::HSTRING;

    use super::{ReplicaInfo, ReplicaSetConfig};

    // caller needs to stitch the reserved ptr.
    fn create_test_data(id: i64) -> (FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1) {
        let ex1 = FABRIC_REPLICA_INFORMATION_EX1 {
            MustCatchup: true.into(),
            Reserved: std::ptr::null_mut(),
        };
        let info = FABRIC_REPLICA_INFORMATION {
            Id: id,
            Role: mssf_com::FABRIC_REPLICA_ROLE_PRIMARY,
            Status: mssf_com::FABRIC_REPLICA_STATUS_UP,
            ReplicatorAddress: windows_core::PCWSTR::null(),
            CurrentProgress: 123,
            CatchUpCapability: 123,
            Reserved: std::ptr::null_mut(),
        };
        (info, ex1)
    }

    #[test]
    fn test_replica_info_conv() {
        let (mut info, ex1) = create_test_data(123);
        info.Reserved = std::ptr::addr_of!(ex1) as *mut c_void;

        // test raw -> wrap
        let wrap = ReplicaInfo::from(&info);
        assert_eq!(wrap.Id, 123);
        assert!(wrap.MustCatchUp);

        // test wrap -> raw
        let (info_b, ex1_b) = wrap.get_raw_parts();
        assert_eq!(info.CurrentProgress, info_b.CurrentProgress);
        assert_eq!(ex1.MustCatchup, ex1_b.MustCatchup);
    }

    #[test]
    fn test_replica_set_config_conv() {
        let replica1 = ReplicaInfo {
            Id: 1,
            Role: super::Role::Primary,
            Status: super::ReplicaStatus::Up,
            ReplicatorAddress: HSTRING::from("addr1"),
            CurrentProgress: 123,
            CatchUpCapability: 123,
            MustCatchUp: true,
        };

        let replica2 = ReplicaInfo {
            Id: 2,
            Role: super::Role::ActiveSecondary,
            Status: super::ReplicaStatus::Up,
            ReplicatorAddress: HSTRING::from("addr2"),
            CurrentProgress: 120,
            CatchUpCapability: 120,
            MustCatchUp: false,
        };

        let config_a = ReplicaSetConfig {
            Replicas: vec![replica1.clone(), replica2.clone()],
            WriteQuorum: 2,
        };

        // test wrap -> raw type conversion
        let view = config_a.get_view();
        let raw = view.get_raw();
        assert_eq!(raw.ReplicaCount, 2);
        assert_eq!(raw.WriteQuorum, 2);

        // test raw type -> wrap conversion
        let config_b = ReplicaSetConfig::from(raw);
        assert_eq!(config_b.Replicas.len(), 2);
        let replica1_b = &config_b.Replicas[0];
        let replica2_b = &config_b.Replicas[1];
        assert_eq!(&replica1, replica1_b);
        assert_eq!(&replica2, replica2_b);
    }
}
