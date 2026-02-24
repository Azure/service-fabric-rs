// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::ffi::c_void;

use crate::{PCWSTR, WString};
use mssf_com::{
    FabricRuntime::{
        IFabricKeyValueStoreReplica8, IFabricStoreEventHandler, IFabricStoreEventHandler_Impl,
    },
    FabricTypes::{FABRIC_ESE_LOCAL_STORE_SETTINGS, FABRIC_LOCAL_STORE_KIND},
};
use windows_core::implement;

use crate::types::{EseLocalStoreSettings, LocalStoreKind, ReplicatorSettings};

#[implement(IFabricStoreEventHandler)]
pub struct DummyStoreEventHandler {}

impl IFabricStoreEventHandler_Impl for DummyStoreEventHandler_Impl {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "debug", ret)
    )]
    fn OnDataLoss(&self) {}
}

pub fn create_com_key_value_store_replica(
    storename: &WString,
    partitionid: crate::GUID,
    replicaid: i64,
    replicatorsettings: &ReplicatorSettings,
    localstorekind: LocalStoreKind,
    localstoresettings: Option<&EseLocalStoreSettings>,
    storeeventhandler: &IFabricStoreEventHandler,
) -> crate::Result<IFabricKeyValueStoreReplica8> {
    let kind: FABRIC_LOCAL_STORE_KIND = localstorekind.into();
    let local_settings: Option<FABRIC_ESE_LOCAL_STORE_SETTINGS> =
        localstoresettings.map(|x| x.get_raw());

    let local_settings_ptr = match local_settings {
        Some(x) => &x,
        None => std::ptr::null(),
    };
    crate::API_TABLE
        .fabric_create_key_value_store_replica::<IFabricKeyValueStoreReplica8>(
            PCWSTR::from_raw(storename.as_ptr()),
            partitionid,
            replicaid,
            &replicatorsettings.get_raw(),
            kind,
            local_settings_ptr as *const c_void,
            Some(storeeventhandler),
        )
        .map_err(crate::Error::from)
}
