// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FABRIC_ESE_LOCAL_STORE_SETTINGS, FABRIC_LOCAL_STORE_KIND, FABRIC_LOCAL_STORE_KIND_ESE,
    FABRIC_LOCAL_STORE_KIND_INVALID, FABRIC_REPLICATOR_SETTINGS,
    FABRIC_TRANSACTION_ISOLATION_LEVEL, FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE, FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT,
};
use windows_core::PCWSTR;

#[derive(Default)]
pub struct ReplicatorSettings {
    pub Flags: u32,
    pub RetryIntervalMilliseconds: u32,
    pub BatchAcknowledgementIntervalMilliseconds: u32,
    pub ReplicatorAddress: ::windows_core::HSTRING,
    pub RequireServiceAck: bool,
    pub InitialReplicationQueueSize: u32,
    pub MaxReplicationQueueSize: u32,
    pub InitialCopyQueueSize: u32,
    pub MaxCopyQueueSize: u32,
    //pub SecurityCredentials: *const FABRIC_SECURITY_CREDENTIALS,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl ReplicatorSettings {
    pub fn get_raw(&self) -> FABRIC_REPLICATOR_SETTINGS {
        FABRIC_REPLICATOR_SETTINGS {
            Flags: self.Flags,
            RetryIntervalMilliseconds: self.RetryIntervalMilliseconds,
            BatchAcknowledgementIntervalMilliseconds: self.BatchAcknowledgementIntervalMilliseconds,
            ReplicatorAddress: PCWSTR::from_raw(self.ReplicatorAddress.as_ptr()),
            RequireServiceAck: self.RequireServiceAck.into(),
            InitialReplicationQueueSize: self.InitialReplicationQueueSize,
            MaxReplicationQueueSize: self.MaxReplicationQueueSize,
            InitialCopyQueueSize: self.InitialCopyQueueSize,
            MaxCopyQueueSize: self.MaxCopyQueueSize,
            SecurityCredentials: std::ptr::null(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

pub enum LocalStoreKind {
    Ese,
    Invalid,
}

impl From<LocalStoreKind> for FABRIC_LOCAL_STORE_KIND {
    fn from(val: LocalStoreKind) -> Self {
        match val {
            LocalStoreKind::Ese => FABRIC_LOCAL_STORE_KIND_ESE,
            LocalStoreKind::Invalid => FABRIC_LOCAL_STORE_KIND_INVALID,
        }
    }
}

#[derive(Default)]
pub struct EseLocalStoreSettings {
    // FABRIC_ESE_LOCAL_STORE_SETTINGS
    pub DbFolderPath: ::windows_core::HSTRING,
    pub LogFileSizeInKB: i32,
    pub LogBufferSizeInKB: i32,
    pub MaxCursors: i32,
    pub MaxVerPages: i32,
    pub MaxAsyncCommitDelayInMilliseconds: i32,
    // pub Reserved: *mut ::core::ffi::c_void,
}

impl EseLocalStoreSettings {
    pub fn get_raw(&self) -> FABRIC_ESE_LOCAL_STORE_SETTINGS {
        FABRIC_ESE_LOCAL_STORE_SETTINGS {
            DbFolderPath: windows_core::PCWSTR::from_raw(self.DbFolderPath.as_ptr()),
            LogFileSizeInKB: self.LogFileSizeInKB,
            LogBufferSizeInKB: self.LogBufferSizeInKB,
            MaxCursors: self.MaxCursors,
            MaxVerPages: self.MaxVerPages,
            MaxAsyncCommitDelayInMilliseconds: self.MaxAsyncCommitDelayInMilliseconds,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_TRANSACTION_ISOLATION_LEVEL
pub enum TransactionIsolationLevel {
    Default,
    ReadCommitted,
    ReadUncomitted,
    RepeatableRead,
    Serializable,
    Snapshot,
}

impl From<FABRIC_TRANSACTION_ISOLATION_LEVEL> for TransactionIsolationLevel {
    fn from(e: FABRIC_TRANSACTION_ISOLATION_LEVEL) -> Self {
        match e {
            FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT => TransactionIsolationLevel::Default,
            FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED => {
                TransactionIsolationLevel::ReadCommitted
            }
            FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED => {
                TransactionIsolationLevel::ReadUncomitted
            }
            FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ => {
                TransactionIsolationLevel::RepeatableRead
            }
            FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE => {
                TransactionIsolationLevel::Serializable
            }
            FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT => TransactionIsolationLevel::Snapshot,
            _ => TransactionIsolationLevel::Default,
        }
    }
}
