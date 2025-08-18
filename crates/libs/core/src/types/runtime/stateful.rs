// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// stateful_types contains type wrappers for sf stateful raw types

use std::{ffi::c_void, marker::PhantomData};

use crate::{PCWSTR, WString};
use mssf_com::FabricTypes::{
    FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1,
    FABRIC_REPLICA_OPEN_MODE, FABRIC_REPLICA_OPEN_MODE_EXISTING, FABRIC_REPLICA_OPEN_MODE_INVALID,
    FABRIC_REPLICA_OPEN_MODE_NEW, FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_ALL,
    FABRIC_REPLICA_SET_QUORUM_INVALID, FABRIC_REPLICA_SET_QUORUM_MODE,
    FABRIC_REPLICA_SET_WRITE_QUORUM, FABRIC_REPLICA_STATUS, FABRIC_REPLICA_STATUS_DOWN,
    FABRIC_REPLICA_STATUS_INVALID, FABRIC_REPLICA_STATUS_UP,
};

use crate::{strings::WStringWrap, types::ReplicaRole};

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

/// Represents the current version of the partition in Service Fabric.
///
/// An Epoch is a configuration number for the partition as a whole.
/// When the configuration of the replica set changes, for example when the Primary replica changes,
/// the operations that are replicated from the new Primary replica are said to be a new Epoch
/// from the ones which were sent by the old Primary replica.
/// The fact that the Primary has changed is not directly visible to Secondary replicas,
/// which are usually unaffected by the failure that affected the original Primary replica.
/// To track that the Primary replica has changed has to be communicated to the Secondary replica.
///
/// Most services can ignore the details of the inner fields of the Epoch as it is usually sufficient
/// to know that the Epoch has changed and to compare Epochs to determine relative ordering of
/// operations and events in the system. Comparison operations are provided for this purpose
//
// Following the c# implementation:
// https://github.com/microsoft/service-fabric/blob/887a7e5bd2de155adab9d4a74c68faa9e691ee0f/src/prod/src/managed/Api/src/System/Fabric/Epoch.cs#L274
// Ordering of epoch is exactly the order of default struct fields, i.e. data loss number is compared first,
// and then configuration number. So the simple derive of Ord does the job.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
    pub data_loss_number: i64,
    pub configuration_number: i64,
}

impl Epoch {
    pub fn new(data_loss_number: i64, configuration_number: i64) -> Self {
        Self {
            data_loss_number,
            configuration_number,
        }
    }
}

impl From<&FABRIC_EPOCH> for Epoch {
    fn from(e: &FABRIC_EPOCH) -> Self {
        Epoch {
            data_loss_number: e.DataLossNumber,
            configuration_number: e.ConfigurationNumber,
        }
    }
}

impl From<&Epoch> for FABRIC_EPOCH {
    fn from(val: &Epoch) -> Self {
        FABRIC_EPOCH {
            DataLossNumber: val.data_loss_number,
            ConfigurationNumber: val.configuration_number,
            Reserved: std::ptr::null_mut(),
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
    pub replicas: Vec<ReplicaInformation>,
    pub write_quorum: u32,
}

impl From<&FABRIC_REPLICA_SET_CONFIGURATION> for ReplicaSetConfig {
    fn from(r: &FABRIC_REPLICA_SET_CONFIGURATION) -> Self {
        let mut res = ReplicaSetConfig {
            replicas: vec![],
            write_quorum: r.WriteQuorum,
        };
        // fill the vec
        for i in 0..r.ReplicaCount {
            let replica = unsafe { r.Replicas.offset(i as isize) };
            let replica_ref = unsafe { replica.as_ref().unwrap() };
            res.replicas.push(ReplicaInformation::from(replica_ref))
        }
        res
    }
}

impl ReplicaSetConfig {
    // view has the lifetime as self
    pub fn get_view(&self) -> ReplicaSetConfigView<'_> {
        // fast return for raw types needed by COM
        let mut replica_raw_cache: Vec<FABRIC_REPLICA_INFORMATION> = vec![];
        let mut replica_ex1_cache: Vec<FABRIC_REPLICA_INFORMATION_EX1> = vec![];
        // prepare vec cache
        for replica in &self.replicas {
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
            WriteQuorum: self.write_quorum,
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

/// Safe wrapping for FABRIC_REPLICA_INFORMATION
#[derive(Debug, PartialEq, Clone)]
pub struct ReplicaInformation {
    pub id: i64,
    pub role: ReplicaRole,
    pub status: ReplicaStatus,
    pub replicator_address: WString,
    pub current_progress: i64,
    pub catch_up_capability: i64,
    /// indicating whether the replica must be caught up as part of a WaitForQuorumCatchup
    pub must_catch_up: bool,
}

impl From<&FABRIC_REPLICA_INFORMATION> for ReplicaInformation {
    fn from(r: &FABRIC_REPLICA_INFORMATION) -> Self {
        let ex1 = r.Reserved as *const FABRIC_REPLICA_INFORMATION_EX1;
        let mut must_catchup = false;
        if !ex1.is_null()
            && let Some(ex1ref) = unsafe { ex1.as_ref() }
        {
            must_catchup = ex1ref.MustCatchup;
        }
        ReplicaInformation {
            id: r.Id,
            role: (&r.Role).into(),
            status: r.Status.into(),
            replicator_address: WStringWrap::from(r.ReplicatorAddress).into(),
            current_progress: r.CurrentProgress,
            catch_up_capability: r.CatchUpCapability,
            must_catch_up: must_catchup,
        }
    }
}

impl ReplicaInformation {
    // The parts have the same lifetime as self.
    // Caller needs to stitch the parts together, i.e.
    // FABRIC_REPLICA_INFORMATION::Reserved needs to point at FABRIC_REPLICA_INFORMATION_EX1
    pub fn get_raw_parts(&self) -> (FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1) {
        let info = FABRIC_REPLICA_INFORMATION {
            Id: self.id,
            Role: (&self.role).into(),
            Status: self.status.clone().into(),
            ReplicatorAddress: PCWSTR::from_raw(self.replicator_address.as_ptr()),
            CurrentProgress: self.current_progress,
            CatchUpCapability: self.catch_up_capability,
            Reserved: std::ptr::null_mut(),
        };
        let ex1 = FABRIC_REPLICA_INFORMATION_EX1 {
            MustCatchup: self.must_catch_up,
            Reserved: std::ptr::null_mut(),
        };
        (info, ex1)
    }
}

// FABRIC_REPLICA_SET_QUORUM_MODE
#[derive(Debug, Clone, PartialEq)]
pub enum ReplicaSetQuorumMode {
    All,
    Invalid,
    Write,
}

impl From<FABRIC_REPLICA_SET_QUORUM_MODE> for ReplicaSetQuorumMode {
    fn from(r: FABRIC_REPLICA_SET_QUORUM_MODE) -> Self {
        match r {
            FABRIC_REPLICA_SET_QUORUM_ALL => ReplicaSetQuorumMode::All,
            FABRIC_REPLICA_SET_QUORUM_INVALID => ReplicaSetQuorumMode::Invalid,
            FABRIC_REPLICA_SET_WRITE_QUORUM => ReplicaSetQuorumMode::Write,
            _ => ReplicaSetQuorumMode::Invalid,
        }
    }
}

impl From<ReplicaSetQuorumMode> for FABRIC_REPLICA_SET_QUORUM_MODE {
    fn from(val: ReplicaSetQuorumMode) -> Self {
        match val {
            ReplicaSetQuorumMode::All => FABRIC_REPLICA_SET_QUORUM_ALL,
            ReplicaSetQuorumMode::Invalid => FABRIC_REPLICA_SET_QUORUM_INVALID,
            ReplicaSetQuorumMode::Write => FABRIC_REPLICA_SET_WRITE_QUORUM,
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::c_void;

    use crate::WString;
    use mssf_com::FabricTypes::{
        FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1, FABRIC_REPLICA_ROLE_PRIMARY,
        FABRIC_REPLICA_STATUS_UP,
    };

    use super::{Epoch, ReplicaInformation, ReplicaSetConfig};

    // caller needs to stitch the reserved ptr.
    fn create_test_data(id: i64) -> (FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_INFORMATION_EX1) {
        let ex1 = FABRIC_REPLICA_INFORMATION_EX1 {
            MustCatchup: true,
            Reserved: std::ptr::null_mut(),
        };
        let info = FABRIC_REPLICA_INFORMATION {
            Id: id,
            Role: FABRIC_REPLICA_ROLE_PRIMARY,
            Status: FABRIC_REPLICA_STATUS_UP,
            ReplicatorAddress: crate::PCWSTR::null(),
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
        let wrap = ReplicaInformation::from(&info);
        assert_eq!(wrap.id, 123);
        assert!(wrap.must_catch_up);

        // test wrap -> raw
        let (info_b, ex1_b) = wrap.get_raw_parts();
        assert_eq!(info.CurrentProgress, info_b.CurrentProgress);
        assert_eq!(ex1.MustCatchup, ex1_b.MustCatchup);
    }

    #[test]
    fn test_replica_set_config_conv() {
        let replica1 = ReplicaInformation {
            id: 1,
            role: super::ReplicaRole::Primary,
            status: super::ReplicaStatus::Up,
            replicator_address: WString::from("addr1"),
            current_progress: 123,
            catch_up_capability: 123,
            must_catch_up: true,
        };

        let replica2 = ReplicaInformation {
            id: 2,
            role: super::ReplicaRole::ActiveSecondary,
            status: super::ReplicaStatus::Up,
            replicator_address: WString::from("addr2"),
            current_progress: 120,
            catch_up_capability: 120,
            must_catch_up: false,
        };

        let config_a = ReplicaSetConfig {
            replicas: vec![replica1.clone(), replica2.clone()],
            write_quorum: 2,
        };

        // test wrap -> raw type conversion
        let view = config_a.get_view();
        let raw = view.get_raw();
        assert_eq!(raw.ReplicaCount, 2);
        assert_eq!(raw.WriteQuorum, 2);

        // test raw type -> wrap conversion
        let config_b = ReplicaSetConfig::from(raw);
        assert_eq!(config_b.replicas.len(), 2);
        let replica1_b = &config_b.replicas[0];
        let replica2_b = &config_b.replicas[1];
        assert_eq!(&replica1, replica1_b);
        assert_eq!(&replica2, replica2_b);
    }

    #[test]
    fn test_epoch_cmp() {
        assert!(Epoch::new(1, 2) < Epoch::new(1, 3));
        assert!(Epoch::new(1, 3) > Epoch::new(1, 2));
        assert!(Epoch::new(1, 2) < Epoch::new(2, 1));
        assert!(Epoch::new(2, 2) > Epoch::new(2, 1));
        assert!(Epoch::new(2, 2) == Epoch::new(2, 2));
    }
}
