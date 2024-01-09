use std::{cell::Cell, sync::Mutex};
use std::thread::JoinHandle;
use async_trait::async_trait;
use fabric_base::{
    FabricCommon::FabricRuntime::{
        IFabricKeyValueStoreReplica2, IFabricStatefulServiceReplica, IFabricStoreEventHandler,
    },
    FABRIC_REPLICATOR_ADDRESS,
};
use fabric_rs::runtime::{
    executor::{DefaultExecutor, Executor},
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
    select,
    sync::oneshot::{self, Sender},
};
use windows_core::{ComInterface, Error, HSTRING};
mod app;
mod echo;

pub struct Factory {
    replication_port: u32,
    rt: DefaultExecutor,
}

impl Factory {
    pub fn create(replication_port: u32, rt: DefaultExecutor) -> Factory {
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

//#[derive(Debug)]
//#[implement(StatefulServiceReplica)]
/*pub struct AppInstance {
    port_: u32,
    hostname_: HSTRING,
    //role_ : Cell<fabric_base::FABRIC_REPLICA_ROLE>,
    //replicator_ : Cell<Option<Box<dyn PrimaryReplicator>>>,
}

impl AppInstance {
    pub fn new(port: u32, hostname: HSTRING) -> AppInstance {
        AppInstance {
            port_: port,
            hostname_: hostname,
            //role_ : Cell::from(fabric_base::FABRIC_REPLICA_ROLE_UNKNOWN),
            //replicator_ : Cell::from(None),
        }
    }
}

impl StatefulServiceReplica for AppInstance {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn open<'life0,'life1,'async_trait>(&'life0 self,openmode:OpenMode,partition: &'life1 StatefulServicePartition,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = windows::core::Result<Box<dyn PrimaryReplicator> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn change_role<'life0,'async_trait>(&'life0 self,newrole:Role) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output =  ::windows_core::Result<HSTRING> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn close<'life0,'async_trait>(&'life0 self) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = windows::core::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn abort(&self) {
        todo!()
    }
}*/

//use struct AppInstance from mod echo

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
        /*let kv = create_com_key_value_store_replica(
            &HSTRING::from("mystorename"),
            *partitionid,
            replicaid,
            &settings,
            fabric_rs::runtime::store_types::LocalStoreKind::Ese,
            None,
            &handler,
        )?;
        let kv_replica: IFabricStatefulServiceReplica = kv.clone().cast().unwrap();*/
        let port_copy = self.replication_port;
        let hostname_copy = "localhost".into();
        let instance = app::AppInstance::new(port_copy, hostname_copy);
        let kv_replica : IFabricStatefulServiceReplica = instance.into();
        /*let port_copy = self.replication_port;
        let hostname_copy = "localhost".into();
        let instance = AppInstance::new(port_copy, hostname_copy);
        // case AppInstance to IFabricStatefulServiceReplica
        let kv_replica: IFabricStatefulServiceReplica =  AppInstance::new(port_copy, hostname_copy);
        //let kv_replica : IFabricStatefulServiceReplica = instance;

        //let kv_replica: IFabricStatefulServiceReplica = AppInstance::new(port_copy, hostname_copy);*/
        let proxy: StatefulServiceReplicaProxy = StatefulServiceReplicaProxy::new(kv_replica);

        let hostname_copy_2 = "localhost".into();
        let svc = Service::new(port_copy, hostname_copy_2);

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
    port_: u32,
    hostname_: HSTRING,
    //th_: Cell<Option<JoinHandle<Result<(), Error>>>>,
    tx_: Mutex<Cell<Option<Sender<()>>>>,
}

impl Service {
    pub fn new(port: u32, hostname: HSTRING) -> Service {
        Service {
            port_: port,
            hostname_: hostname,
            tx_: Mutex::new(Cell::new(None)),
            //th_: Cell::from(None),
        }
    }

    pub fn start_loop(&self) {
        let (tx, mut rx) = oneshot::channel::<()>();
        self.stop();
        self.tx_.lock().unwrap().set(Some(tx));
        
        
        let port_copy = self.port_;
        let hostname_copy = self.hostname_.clone();

        let th = std::thread::spawn(move || echo::start_echo(rx, port_copy, hostname_copy));
        //self.th_.set(Some(th));
    }

    pub fn stop(&self) {
        let mut op = self.tx_.lock().unwrap().take();
        if op.is_some() {
            op.take().unwrap().send(()).unwrap()
        }
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
            info!("primary {:?}", self.svc.port_);
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
