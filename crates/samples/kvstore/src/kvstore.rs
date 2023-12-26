use async_trait::async_trait;
use fabric_base::{
    FabricCommon::FabricRuntime::{IFabricStatefulServiceReplica, IFabricStoreEventHandler},
    FABRIC_REPLICATOR_ADDRESS,
};
use fabric_rs::runtime::{
    proxy::StatefulServiceReplicaProxy,
    stateful::{
        OpenMode, PrimaryReplicator, Role, StatefulServiceFactory, StatefulServicePartition,
        StatefulServiceReplica,
    },
    store::{create_com_key_value_store_replica, DummyStoreEventHandler, ReplicatorSettings},
};
use log::info;
use windows_core::{ComInterface, Error, HSTRING};

pub struct Factory {
    replication_port: u32,
}

impl Factory {
    pub fn create(replication_port: u32) -> Factory {
        Factory { replication_port }
    }
}

fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

impl StatefulServiceFactory for Factory {
    fn create_replica(
        &self,
        servicetypename: &windows_core::HSTRING,
        servicename: &windows_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &windows::core::GUID,
        replicaid: i64,
    ) -> Result<Box<dyn fabric_rs::runtime::stateful::StatefulServiceReplica + Send>, Error> {
        info!(
            "Factory::create_replica type {}, service {}, init data size {}",
            servicetypename,
            servicename,
            initializationdata.len()
        );
        let settings = ReplicatorSettings {
            Flags: FABRIC_REPLICATOR_ADDRESS.0 as u32,
            ReplicatorAddress: HSTRING::from(get_addr(self.replication_port, "localhost".into())),
            ..Default::default()
        };

        info!(
            "Factory::create_replica using address {}",
            settings.ReplicatorAddress
        );

        let handler: IFabricStoreEventHandler = DummyStoreEventHandler {}.into();
        let kv = create_com_key_value_store_replica(
            &HSTRING::from("mystorename"),
            *partitionid,
            replicaid,
            &settings,
            fabric_rs::runtime::store::LocalStoreKind::Ese,
            None,
            &handler,
        )?;
        let kv_replica: IFabricStatefulServiceReplica = kv.clone().cast().unwrap();
        let proxy = StatefulServiceReplicaProxy::new(kv_replica);

        let replica = Replica::new(proxy);
        Ok(Box::new(replica))
    }
}

pub struct Replica {
    kv: StatefulServiceReplicaProxy,
}

impl Replica {
    pub fn new(kv: StatefulServiceReplicaProxy) -> Replica {
        Replica { kv }
    }
}

#[async_trait]
impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<Box<dyn PrimaryReplicator + Send>> {
        // should be primary replicator
        info!("Replica::open");
        self.kv.open(openmode, partition).await
    }
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING> {
        info!("Replica::change_role");
        self.kv.change_role(newrole).await
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("Replica::close");
        self.kv.close().await
    }
    fn abort(&self) {
        info!("Replica::abort");
        self.kv.abort();
    }
}
