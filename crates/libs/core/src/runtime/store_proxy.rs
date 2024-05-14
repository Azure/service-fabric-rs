// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricCommon::FabricRuntime::{
        IFabricKeyValueStoreItemResult, IFabricKeyValueStoreReplica2, IFabricTransaction,
    },
    FABRIC_KEY_VALUE_STORE_ITEM, FABRIC_KEY_VALUE_STORE_ITEM_METADATA,
};
use tracing::info;
use windows_core::PCWSTR;

use super::store_types::TransactionIsolationLevel;

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

    pub fn create_transaction(&self) -> ::windows_core::Result<TransactionProxy> {
        let tx = unsafe { self.com_impl.CreateTransaction() }?;
        Ok(TransactionProxy { com_impl: tx })
    }

    pub fn add(
        &self,
        tx: &TransactionProxy,
        key: &[u16],
        value: &[u8],
    ) -> ::windows_core::Result<()> {
        unsafe {
            self.com_impl
                .Add(&tx.com_impl, PCWSTR::from_raw(key.as_ptr()), value)
        }
    }

    pub fn get(
        &self,
        tx: &TransactionProxy,
        key: &[u16],
    ) -> ::windows_core::Result<KVStoreItemProxy> {
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
    ) -> ::windows_core::Result<()> {
        unsafe {
            self.com_impl.Remove(
                &tx.com_impl,
                PCWSTR::from_raw(key.as_ptr()),
                checksequencenumber,
            )
        }
    }
}

impl TransactionProxy {
    pub fn get_id(&self) -> &::windows_core::GUID {
        unsafe { self.com_impl.get_Id().as_ref().unwrap() }
    }

    pub fn get_isolation_level(&self) -> TransactionIsolationLevel {
        unsafe { self.com_impl.get_IsolationLevel().into() }
    }

    pub async fn commit(&self, timeoutmilliseconds: u32) -> ::windows_core::Result<i64> {
        info!("TransactionProxy::commit");
        // replicator address
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com_impl.EndCommit(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });

        let _ = unsafe { self.com_impl.BeginCommit(timeoutmilliseconds, &callback)? };
        rx.await.unwrap()
    }

    pub fn rollback(&self) {
        unsafe { self.com_impl.Rollback() };
    }
}
