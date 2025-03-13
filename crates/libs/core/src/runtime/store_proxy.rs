// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::PCWSTR;
use mssf_com::{
    FabricRuntime::{
        IFabricKeyValueStoreItemResult, IFabricKeyValueStoreReplica2, IFabricTransaction,
    },
    FabricTypes::{FABRIC_KEY_VALUE_STORE_ITEM, FABRIC_KEY_VALUE_STORE_ITEM_METADATA},
};
use tracing::debug;

use crate::sync::{fabric_begin_end_proxy, CancellationToken};

use crate::types::TransactionIsolationLevel;

// wrapp for kv store
#[derive(Clone)]
pub struct KVStoreProxy {
    com_impl: IFabricKeyValueStoreReplica2,
}

pub struct TransactionProxy {
    com_impl: IFabricTransaction,
}

pub struct KVStoreItemProxy {
    com_impl: IFabricKeyValueStoreItemResult,
}

impl KVStoreItemProxy {
    pub fn key(&self) -> &[u16] {
        let item = self.get_item_inner();
        let meta = Self::get_meta_inner(item);
        unsafe { meta.Key.as_wide() }
    }

    pub fn val(&self) -> &[u8] {
        let item = self.get_item_inner();
        let meta = Self::get_meta_inner(item);
        unsafe { std::slice::from_raw_parts(item.Value, meta.ValueSizeInBytes as usize) }
    }

    fn get_item_inner(&self) -> &FABRIC_KEY_VALUE_STORE_ITEM {
        unsafe { self.com_impl.get_Item().as_ref().unwrap() }
    }
    fn get_meta_inner(item: &FABRIC_KEY_VALUE_STORE_ITEM) -> &FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
        unsafe { item.Metadata.as_ref().unwrap() }
    }
}

impl KVStoreProxy {
    pub fn new(com_impl: IFabricKeyValueStoreReplica2) -> KVStoreProxy {
        KVStoreProxy { com_impl }
    }

    pub fn create_transaction(&self) -> crate::Result<TransactionProxy> {
        let tx = unsafe { self.com_impl.CreateTransaction() }?;
        Ok(TransactionProxy { com_impl: tx })
    }

    pub fn add(&self, tx: &TransactionProxy, key: &[u16], value: &[u8]) -> crate::Result<()> {
        unsafe {
            self.com_impl
                .Add(&tx.com_impl, PCWSTR::from_raw(key.as_ptr()), value)
        }
        .map_err(crate::Error::from)
    }

    pub fn get(&self, tx: &TransactionProxy, key: &[u16]) -> crate::Result<KVStoreItemProxy> {
        let com = unsafe {
            self.com_impl
                .Get(&tx.com_impl, PCWSTR::from_raw(key.as_ptr()))
        }?;
        Ok(KVStoreItemProxy { com_impl: com })
    }

    // check sequence number is the lsn that last time the key got modified.
    // if lsn does not match the remove will error out.
    // specify 0 to ignore check.
    pub fn remove(
        &self,
        tx: &TransactionProxy,
        key: &[u16],
        checksequencenumber: i64,
    ) -> crate::Result<()> {
        unsafe {
            self.com_impl.Remove(
                &tx.com_impl,
                PCWSTR::from_raw(key.as_ptr()),
                checksequencenumber,
            )
        }
        .map_err(crate::Error::from)
    }
}

impl TransactionProxy {
    pub fn get_id(&self) -> &crate::GUID {
        unsafe { self.com_impl.get_Id().as_ref().unwrap() }
    }

    pub fn get_isolation_level(&self) -> TransactionIsolationLevel {
        unsafe { self.com_impl.get_IsolationLevel().into() }
    }

    pub async fn commit(
        &self,
        timeoutmilliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> crate::Result<i64> {
        debug!("TransactionProxy::commit");
        let com1 = &self.com_impl;
        let com2 = self.com_impl.clone();
        let rx = fabric_begin_end_proxy(
            move |callback| unsafe { com1.BeginCommit(timeoutmilliseconds, callback) },
            move |ctx| unsafe { com2.EndCommit(ctx) },
            cancellation_token,
        );
        rx.await?.map_err(crate::Error::from)
    }

    pub fn rollback(&self) {
        unsafe { self.com_impl.Rollback() };
    }
}
