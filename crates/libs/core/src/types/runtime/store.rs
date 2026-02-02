// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::PCWSTR;
use mssf_com::FabricTypes::{
    FABRIC_ESE_LOCAL_STORE_SETTINGS, FABRIC_LOCAL_STORE_KIND, FABRIC_LOCAL_STORE_KIND_ESE,
    FABRIC_LOCAL_STORE_KIND_INVALID, FABRIC_REPLICATOR_SETTINGS,
    FABRIC_TRANSACTION_ISOLATION_LEVEL, FABRIC_TRANSACTION_ISOLATION_LEVEL_DEFAULT,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_COMMITTED,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_READ_UNCOMMITTED,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_REPEATABLE_READ,
    FABRIC_TRANSACTION_ISOLATION_LEVEL_SERIALIZABLE, FABRIC_TRANSACTION_ISOLATION_LEVEL_SNAPSHOT,
};

#[derive(Default)]
pub struct ReplicatorSettings {
    pub flags: u32,
    pub retry_interval_milliseconds: u32,
    pub batch_acknowledgement_interval_milliseconds: u32,
    pub replicator_address: crate::WString,
    pub require_service_ack: bool,
    pub initial_replication_queue_size: u32,
    pub max_replication_queue_size: u32,
    pub initial_copy_queue_size: u32,
    pub max_copy_queue_size: u32,
    //pub SecurityCredentials: *const FABRIC_SECURITY_CREDENTIALS,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl ReplicatorSettings {
    pub fn get_raw(&self) -> FABRIC_REPLICATOR_SETTINGS {
        FABRIC_REPLICATOR_SETTINGS {
            Flags: self.flags,
            RetryIntervalMilliseconds: self.retry_interval_milliseconds,
            BatchAcknowledgementIntervalMilliseconds: self
                .batch_acknowledgement_interval_milliseconds,
            ReplicatorAddress: PCWSTR::from_raw(self.replicator_address.as_ptr()),
            RequireServiceAck: self.require_service_ack,
            InitialReplicationQueueSize: self.initial_replication_queue_size,
            MaxReplicationQueueSize: self.max_replication_queue_size,
            InitialCopyQueueSize: self.initial_copy_queue_size,
            MaxCopyQueueSize: self.max_copy_queue_size,
            SecurityCredentials: std::ptr::null(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub db_folder_path: crate::WString,
    pub log_file_size_in_kb: i32,
    pub log_buffer_size_in_kb: i32,
    pub max_cursors: i32,
    pub max_ver_pages: i32,
    pub max_async_commit_delay_in_milliseconds: i32,
    // pub Reserved: *mut ::core::ffi::c_void,
}

impl EseLocalStoreSettings {
    pub fn get_raw(&self) -> FABRIC_ESE_LOCAL_STORE_SETTINGS {
        FABRIC_ESE_LOCAL_STORE_SETTINGS {
            DbFolderPath: crate::PCWSTR::from_raw(self.db_folder_path.as_ptr()),
            LogFileSizeInKB: self.log_file_size_in_kb,
            LogBufferSizeInKB: self.log_buffer_size_in_kb,
            MaxCursors: self.max_cursors,
            MaxVerPages: self.max_ver_pages,
            MaxAsyncCommitDelayInMilliseconds: self.max_async_commit_delay_in_milliseconds,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_TRANSACTION_ISOLATION_LEVEL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
