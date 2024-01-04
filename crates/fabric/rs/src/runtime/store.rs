use std::ffi::c_void;

use fabric_base::{
    FabricCommon::FabricRuntime::{
        FabricCreateKeyValueStoreReplica, IFabricKeyValueStoreReplica2, IFabricStoreEventHandler,
        IFabricStoreEventHandler_Impl,
    },
    FABRIC_ESE_LOCAL_STORE_SETTINGS, FABRIC_LOCAL_STORE_KIND,
};
use log::info;
use windows::core::implement;
use windows_core::{ComInterface, Error, Interface, HSTRING, PCWSTR};

use super::store_types::{EseLocalStoreSettings, LocalStoreKind, ReplicatorSettings};

#[implement(IFabricStoreEventHandler)]
pub struct DummyStoreEventHandler {}

impl IFabricStoreEventHandler_Impl for DummyStoreEventHandler {
    fn OnDataLoss(&self) {
        info!("DummyStoreEventHandler::OnDataLoss");
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

// This requires intensive mocking.
// #[cfg(test)]
// mod test {
//     use fabric_base::FabricCommon::FabricRuntime::{
//         IFabricStatefulServiceReplica, IFabricStoreEventHandler,
//     };
//     use windows_core::{ComInterface, GUID, HSTRING};

//     use crate::runtime::{
//         proxy::{KVStoreProxy, StatefulServiceReplicaProxy},
//         store::EseLocalStoreSettings,
//     };

//     use super::{create_com_key_value_store_replica, DummyStoreEventHandler, ReplicatorSettings};

//     // mock the partition

//     #[tokio::test]
//     async fn test_kvstore_local() {
//         let mut db_dir = std::env::temp_dir();
//         db_dir.push("sfkvtest");
//         // create db dir
//         std::fs::create_dir_all(&db_dir).unwrap();

//         let name = HSTRING::from("mykvstore");
//         let guid = GUID::new().unwrap();
//         let settings = ReplicatorSettings::default();
//         let local_settings = EseLocalStoreSettings {
//             DbFolderPath: HSTRING::from(db_dir.to_str().unwrap()),
//             ..Default::default()
//         };
//         let evHander: IFabricStoreEventHandler = DummyStoreEventHandler {}.into();
//         let kv = create_com_key_value_store_replica(
//             &name,
//             guid,
//             123,
//             &settings,
//             super::LocalStoreKind::Ese,
//             Some(&local_settings),
//             &evHander,
//         )
//         .unwrap();

//         let kv_replica: IFabricStatefulServiceReplica = kv.clone().cast().unwrap();
//         let _proxy = StatefulServiceReplicaProxy::new(kv_replica);

//         //proxy.open(OpenMode::New, partition);

//         let kv_proxy = KVStoreProxy::new(kv);
//         let tx = kv_proxy.create_transaction().unwrap();
//         kv_proxy
//             .add(
//                 &tx,
//                 HSTRING::from("mykey").as_wide(),
//                 String::from("myval").as_bytes(),
//             )
//             .unwrap();
//         // clean up
//         std::fs::remove_dir_all(&db_dir).unwrap()
//     }
// }
