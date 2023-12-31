use std::{cell::Cell, sync::Mutex};

use async_trait::async_trait;
use fabric_base::{
    FabricCommon::FabricRuntime::{
        IFabricKeyValueStoreReplica2, IFabricStatefulServiceReplica, IFabricStoreEventHandler,
    },
    FABRIC_REPLICATOR_ADDRESS,
};
use fabric_rs::runtime::{
    stateful::{
        PrimaryReplicator, StatefulServiceFactory, StatefulServicePartition, StatefulServiceReplica,
    },
    stateful_proxy::StatefulServiceReplicaProxy,
    stateful_types::{OpenMode, Role},
    store::{create_com_key_value_store_replica, DummyStoreEventHandler},
    store_proxy::KVStoreProxy,
    store_types::ReplicatorSettings,
};
use log::info;
use tokio::{
    runtime::Handle,
    select,
    sync::oneshot::{self, Sender},
};
use windows_core::{ComInterface, Error, HSTRING};

pub struct Factory {
    replication_port: u32,
    rt: Handle,
}

impl Factory {
    pub fn create(replication_port: u32, rt: Handle) -> Factory {
        Factory {
            replication_port,
            rt,
        }
    }
}

fn get_addr(port: u32, hostname: HSTRING) -> String {
    let mut addr = String::new();
    addr.push_str(&hostname.to_string());
    addr.push(':');
    addr.push_str(&port.to_string());
    addr
}

impl StatefulServiceFactory<Replica> for Factory {
    fn create_replica(
        &self,
        servicetypename: &windows_core::HSTRING,
        servicename: &windows_core::HSTRING,
        initializationdata: &[u8],
        partitionid: &windows::core::GUID,
        replicaid: i64,
    ) -> Result<Replica, Error> {
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
            fabric_rs::runtime::store_types::LocalStoreKind::Ese,
            None,
            &handler,
        )?;
        let kv_replica: IFabricStatefulServiceReplica = kv.clone().cast().unwrap();
        let proxy = StatefulServiceReplicaProxy::new(kv_replica);

        let svc = Service::new(kv, self.rt.clone());

        let replica = Replica::new(proxy, svc);
        Ok(replica)
    }
}

pub struct Replica {
    kv: StatefulServiceReplicaProxy,
    svc: Service,
}

impl Replica {
    pub fn new(kv: StatefulServiceReplicaProxy, svc: Service) -> Replica {
        Replica { kv, svc }
    }
}

// The serving of the database.
pub struct Service {
    kvproxy: KVStoreProxy,
    rt: Handle,
    tx: Mutex<Cell<Option<Sender<()>>>>,
}

impl Service {
    pub fn new(com: IFabricKeyValueStoreReplica2, rt: Handle) -> Service {
        Service {
            kvproxy: KVStoreProxy::new(com),
            rt,
            tx: Mutex::new(Cell::new(None)),
        }
    }

    pub fn start_loop(&self) {
        let (tx, mut rx) = oneshot::channel::<()>();
        let kv = self.kvproxy.clone();
        self.stop();
        self.tx.lock().unwrap().set(Some(tx));
        self.rt.spawn(async move {
            let mut counter = 0;
            loop {
                info!("Service::run_single: {}", counter);
                let res = Self::run_single(&kv).await;
                match res {
                    Ok(_) => info!("run_single success"),
                    Err(e) => info!("run_single error : {}", e),
                }
                counter += 1;
                // sleep or stop
                select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                        continue;
                    }
                    _ = &mut rx =>{
                        info!("Service::loop stopped from rx");
                        break;
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        let mut op = self.tx.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().send(()).unwrap()
        }
    }

    async fn run_single(kv: &KVStoreProxy) -> windows_core::Result<()> {
        // add kv
        let seq;
        {
            let tx = kv.create_transaction()?;
            let key = HSTRING::from("mykey");
            let value = String::from("myvalue");
            kv.add(&tx, key.as_wide(), value.as_bytes())?;
            seq = tx.commit(1000).await?;
        }

        // remove kv
        {
            let tx = kv.create_transaction()?;
            let key = HSTRING::from("mykey");
            kv.remove(&tx, key.as_wide(), seq)?;
            let _ = tx.commit(1000).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl StatefulServiceReplica for Replica {
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<Box<dyn PrimaryReplicator>> {
        // should be primary replicator
        info!("Replica::open {:?}", openmode);
        self.kv.open(openmode, partition).await
    }
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING> {
        info!("Replica::change_role {:?}", newrole);
        let addr = self.kv.change_role(newrole.clone()).await?;
        if newrole == Role::Primary {
            self.svc.start_loop();
        }
        Ok(addr)
    }
    async fn close(&self) -> windows::core::Result<()> {
        info!("Replica::close");
        self.svc.stop();
        self.kv.close().await
    }
    fn abort(&self) {
        info!("Replica::abort");
        self.svc.stop();
        self.kv.abort();
    }
}
