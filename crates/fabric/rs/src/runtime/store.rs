use std::ffi::c_void;

use fabric_base::{
    FabricCommon::FabricRuntime::{
        FabricCreateKeyValueStoreReplica, IFabricKeyValueStoreReplica2, IFabricStoreEventHandler,
        IFabricStoreEventHandler_Impl,
    },
    FABRIC_ESE_LOCAL_STORE_SETTINGS, FABRIC_LOCAL_STORE_KIND, FABRIC_LOCAL_STORE_KIND_ESE,
    FABRIC_LOCAL_STORE_KIND_INVALID, FABRIC_REPLICATOR_SETTINGS,
};
use log::info;
use windows::core::implement;
use windows_core::{ComInterface, Error, Interface, HSTRING, PCWSTR};

#[implement(IFabricStoreEventHandler)]
pub struct DummyStoreEventHandler {}

impl IFabricStoreEventHandler_Impl for DummyStoreEventHandler {
    fn OnDataLoss(&self) {
        info!("DummyStoreEventHandler::OnDataLoss");
    }
}

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

pub fn create_com_key_value_store_replica(
    storename: &HSTRING,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    replicatorsettings: &ReplicatorSettings,
    localstorekind: LocalStoreKind,
    localstoresettings: Option<&EseLocalStoreSettings>,
    storeeventhandler: &IFabricStoreEventHandler,
) -> Result<IFabricKeyValueStoreReplica2, Error> {
    let kind: FABRIC_LOCAL_STORE_KIND = localstorekind.into();
    let local_settings: Option<FABRIC_ESE_LOCAL_STORE_SETTINGS> =
        localstoresettings.map(|x| x.get_raw());

    let local_settings_ptr = match local_settings {
        Some(x) => &x,
        None => std::ptr::null(),
    };

    // let handler:IFabricStoreEventHandler = DummyStoreEventHandler{}.into();
    let raw = unsafe {
        FabricCreateKeyValueStoreReplica(
            &IFabricKeyValueStoreReplica2::IID,
            PCWSTR::from_raw(storename.as_ptr()),
            partitionid,
            replicaid,
            &replicatorsettings.get_raw(),
            kind,
            local_settings_ptr as *const c_void,
            storeeventhandler,
        )?
    };
    Ok(unsafe { IFabricKeyValueStoreReplica2::from_raw(raw) })
}
