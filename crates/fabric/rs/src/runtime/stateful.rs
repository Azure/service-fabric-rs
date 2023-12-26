use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use tokio::{runtime::Handle, sync::Mutex};
use windows::core::implement;
use windows_core::{AsImpl, ComInterface, Error, HSTRING, PCWSTR};

use fabric_base::{
    FabricCommon::{
        FabricRuntime::{
            IFabricPrimaryReplicator, IFabricPrimaryReplicator_Impl, IFabricReplicator,
            IFabricReplicator_Impl, IFabricStatefulServiceFactory,
            IFabricStatefulServiceFactory_Impl, IFabricStatefulServicePartition,
            IFabricStatefulServiceReplica, IFabricStatefulServiceReplica_Impl,
        },
        IFabricAsyncOperationContext, IFabricAsyncOperationContext_Impl, IFabricStringResult,
    },
    FABRIC_EPOCH, FABRIC_REPLICA_INFORMATION, FABRIC_REPLICA_OPEN_MODE,
    FABRIC_REPLICA_OPEN_MODE_EXISTING, FABRIC_REPLICA_OPEN_MODE_INVALID,
    FABRIC_REPLICA_OPEN_MODE_NEW, FABRIC_REPLICA_ROLE, FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
    FABRIC_REPLICA_ROLE_IDLE_SECONDARY, FABRIC_REPLICA_ROLE_NONE, FABRIC_REPLICA_ROLE_PRIMARY,
    FABRIC_REPLICA_SET_CONFIGURATION, FABRIC_REPLICA_SET_QUORUM_ALL,
    FABRIC_REPLICA_SET_QUORUM_INVALID, FABRIC_REPLICA_SET_QUORUM_MODE,
    FABRIC_REPLICA_SET_WRITE_QUORUM, FABRIC_REPLICA_STATUS, FABRIC_REPLICA_STATUS_DOWN,
    FABRIC_REPLICA_STATUS_INVALID, FABRIC_REPLICA_STATUS_UP,
};

use crate::{runtime::BridgeContext, StringResult};

pub trait StatefulServiceFactory {
    fn create_replica(
        &self,
        servicetypename: &HSTRING,
        servicename: &HSTRING,
        initializationdata: &[u8],
        partitionid: &::windows::core::GUID,
        replicaid: i64,
    ) -> Result<Box<dyn StatefulServiceReplica + Send>, Error>;
}

#[implement(IFabricStatefulServiceFactory)]
pub struct StatefulServiceFactoryBridge {
    inner: Box<dyn StatefulServiceFactory>,
    rt: Handle,
}

impl StatefulServiceFactoryBridge {
    pub fn create(
        factory: Box<dyn StatefulServiceFactory>,
        rt: Handle,
    ) -> StatefulServiceFactoryBridge {
        StatefulServiceFactoryBridge { inner: factory, rt }
    }
}

impl IFabricStatefulServiceFactory_Impl for StatefulServiceFactoryBridge {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn CreateReplica(
        &self,
        servicetypename: &::windows_core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows_core::GUID,
        replicaid: i64,
    ) -> ::windows_core::Result<IFabricStatefulServiceReplica> {
        info!("StatelessServiceFactoryBridge::CreateInstance");
        let p_servicename = ::windows_core::PCWSTR::from_raw(servicename);
        let h_servicename = HSTRING::from_wide(unsafe { p_servicename.as_wide() }).unwrap();
        let h_servicetypename = HSTRING::from_wide(unsafe { servicetypename.as_wide() }).unwrap();
        let data = unsafe {
            std::slice::from_raw_parts(initializationdata, initializationdatalength as usize)
        };

        let replica = self.inner.create_replica(
            &h_servicetypename,
            &h_servicename,
            data,
            partitionid,
            replicaid,
        )?;
        let rt = self.rt.clone();
        let replica_bridge = IFabricStatefulReplicaBridge::create(replica, rt);
        Ok(replica_bridge.into())
    }
}

pub enum OpenMode {
    Invald,
    Existing,
    New,
}

impl From<FABRIC_REPLICA_OPEN_MODE> for OpenMode {
    fn from(e: FABRIC_REPLICA_OPEN_MODE) -> Self {
        match e {
            FABRIC_REPLICA_OPEN_MODE_EXISTING => OpenMode::Existing,
            FABRIC_REPLICA_OPEN_MODE_NEW => OpenMode::New,
            _ => OpenMode::Invald,
        }
    }
}
impl From<OpenMode> for FABRIC_REPLICA_OPEN_MODE {
    fn from(val: OpenMode) -> Self {
        match val {
            OpenMode::Invald => FABRIC_REPLICA_OPEN_MODE_INVALID,
            OpenMode::Existing => FABRIC_REPLICA_OPEN_MODE_EXISTING,
            OpenMode::New => FABRIC_REPLICA_OPEN_MODE_NEW,
        }
    }
}

// safe service instance
#[async_trait]
pub trait StatefulServiceReplica {
    // Note that open returns PrimaryReplicator instead of Replicator.
    // The replicator that gives to SF has to implement primary replicator all the time.
    async fn open(
        &self,
        openmode: OpenMode,
        partition: &StatefulServicePartition,
    ) -> windows::core::Result<Box<dyn PrimaryReplicator + Send>>;
    async fn change_role(&self, newrole: Role) -> ::windows_core::Result<HSTRING>; // replica address
    async fn close(&self) -> windows::core::Result<()>;
    fn abort(&self);
}

pub struct StatefulServicePartition {
    _com_impl: IFabricStatefulServicePartition,
}

impl StatefulServicePartition {
    pub fn get_com(&self) -> &IFabricStatefulServicePartition {
        &self._com_impl
    }
}

impl From<&IFabricStatefulServicePartition> for StatefulServicePartition {
    fn from(e: &IFabricStatefulServicePartition) -> Self {
        StatefulServicePartition {
            _com_impl: e.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Epoch {
    pub data_loss_number: i64,
    pub configuration_number: i64,
}

impl From<&FABRIC_EPOCH> for Epoch {
    fn from(e: &FABRIC_EPOCH) -> Self {
        Epoch {
            data_loss_number: e.DataLossNumber,
            configuration_number: e.ConfigurationNumber,
        }
    }
}

impl From<Epoch> for FABRIC_EPOCH {
    fn from(val: Epoch) -> Self {
        FABRIC_EPOCH {
            DataLossNumber: val.data_loss_number,
            ConfigurationNumber: val.configuration_number,
            Reserved: std::ptr::null_mut(),
        }
    }
}

#[derive(Clone)]
pub enum Role {
    ActiveSecondary,
    IdleSecondary,
    None,
    Primary,
    Unknown,
}

impl From<FABRIC_REPLICA_ROLE> for Role {
    fn from(r: FABRIC_REPLICA_ROLE) -> Self {
        match r {
            FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY => Role::ActiveSecondary,
            FABRIC_REPLICA_ROLE_IDLE_SECONDARY => Role::IdleSecondary,
            FABRIC_REPLICA_ROLE_NONE => Role::None,
            FABRIC_REPLICA_ROLE_PRIMARY => Role::Primary,
            _ => Role::Unknown,
        }
    }
}

impl From<Role> for FABRIC_REPLICA_ROLE {
    fn from(val: Role) -> Self {
        match val {
            Role::ActiveSecondary => FABRIC_REPLICA_ROLE_ACTIVE_SECONDARY,
            Role::IdleSecondary => FABRIC_REPLICA_ROLE_IDLE_SECONDARY,
            Role::None => FABRIC_REPLICA_ROLE_NONE,
            Role::Primary => FABRIC_REPLICA_ROLE_PRIMARY,
            Role::Unknown => FABRIC_REPLICA_ROLE_NONE,
        }
    }
}

#[async_trait]
pub trait Replicator {
    async fn open(&self) -> ::windows_core::Result<HSTRING>; // replicator address
    async fn close(&self) -> ::windows_core::Result<()>;
    async fn change_role(&self, epoch: &Epoch, role: &Role) -> ::windows_core::Result<()>;
    async fn update_epoch(&self, epoch: &Epoch) -> ::windows_core::Result<()>;
    fn get_current_progress(&self) -> ::windows_core::Result<i64>;
    fn get_catch_up_capability(&self) -> ::windows_core::Result<i64>;
    fn abort(&self);
}

#[derive(Clone)]
pub enum ReplicaStatus {
    Invalid,
    Down,
    Up,
}

impl From<ReplicaStatus> for FABRIC_REPLICA_STATUS {
    fn from(val: ReplicaStatus) -> Self {
        match val {
            ReplicaStatus::Invalid => FABRIC_REPLICA_STATUS_INVALID,
            ReplicaStatus::Down => FABRIC_REPLICA_STATUS_DOWN,
            ReplicaStatus::Up => FABRIC_REPLICA_STATUS_UP,
        }
    }
}

impl From<FABRIC_REPLICA_STATUS> for ReplicaStatus {
    fn from(r: FABRIC_REPLICA_STATUS) -> Self {
        match r {
            FABRIC_REPLICA_STATUS_INVALID => ReplicaStatus::Invalid,
            FABRIC_REPLICA_STATUS_DOWN => ReplicaStatus::Down,
            FABRIC_REPLICA_STATUS_UP => ReplicaStatus::Up,
            _ => ReplicaStatus::Invalid,
        }
    }
}

pub struct ReplicaSetConfig {
    //pub ReplicaCount: u32,
    //pub Replicas: *const FABRIC_REPLICA_INFORMATION,
    pub Replicas: Vec<ReplicaInfo>,
    pub WriteQuorum: u32,
    // pub Reserved: *mut ::core::ffi::c_void,
    replica_raw_cache: Vec<FABRIC_REPLICA_INFORMATION>,
}

impl From<&FABRIC_REPLICA_SET_CONFIGURATION> for ReplicaSetConfig {
    fn from(r: &FABRIC_REPLICA_SET_CONFIGURATION) -> Self {
        let mut res = ReplicaSetConfig {
            Replicas: vec![],
            WriteQuorum: r.WriteQuorum,
            replica_raw_cache: vec![],
        };
        // fill the vec
        for i in 0..r.ReplicaCount {
            let replica = unsafe { r.Replicas.offset(i as isize) };
            let replica_ref = unsafe { replica.as_ref().unwrap() };
            res.Replicas.push(ReplicaInfo::from(replica_ref))
        }
        res.fill_cache_copy();
        res
    }
}

impl ReplicaSetConfig {
    pub fn get_raw(&self) -> FABRIC_REPLICA_SET_CONFIGURATION {
        FABRIC_REPLICA_SET_CONFIGURATION {
            ReplicaCount: self.replica_raw_cache.len() as u32,
            Replicas: self.replica_raw_cache.as_ptr(),
            WriteQuorum: self.WriteQuorum,
            Reserved: std::ptr::null_mut(),
        }
    }

    fn fill_cache_copy(&mut self) {
        self.replica_raw_cache.clear();
        for replica in &self.Replicas {
            self.replica_raw_cache.push(replica.get_raw());
        }
    }
}

pub struct ReplicaInfo {
    pub Id: i64,
    pub Role: Role,
    pub Status: ReplicaStatus,
    pub ReplicatorAddress: ::windows_core::HSTRING,
    pub CurrentProgress: i64,
    pub CatchUpCapability: i64,
    //pub Reserved: *mut ::core::ffi::c_void,
}

impl From<&FABRIC_REPLICA_INFORMATION> for ReplicaInfo {
    fn from(r: &FABRIC_REPLICA_INFORMATION) -> Self {
        ReplicaInfo {
            Id: r.Id,
            Role: r.Role.into(),
            Status: r.Status.into(),
            ReplicatorAddress: HSTRING::from_wide(unsafe { r.ReplicatorAddress.as_wide() })
                .unwrap(),
            CurrentProgress: r.CurrentProgress,
            CatchUpCapability: r.CatchUpCapability,
        }
    }
}

impl ReplicaInfo {
    pub fn get_raw(&self) -> FABRIC_REPLICA_INFORMATION {
        FABRIC_REPLICA_INFORMATION {
            Id: self.Id,
            Role: self.Role.clone().into(),
            Status: self.Status.clone().into(),
            ReplicatorAddress: PCWSTR::from_raw(self.ReplicatorAddress.as_ptr()),
            CurrentProgress: self.CurrentProgress,
            CatchUpCapability: self.CatchUpCapability,
            Reserved: std::ptr::null_mut(),
        }
    }
}

// FABRIC_REPLICA_SET_QUORUM_MODE
pub enum ReplicaSetQuarumMode {
    All,
    Invalid,
    Write,
}

impl From<FABRIC_REPLICA_SET_QUORUM_MODE> for ReplicaSetQuarumMode {
    fn from(r: FABRIC_REPLICA_SET_QUORUM_MODE) -> Self {
        match r {
            FABRIC_REPLICA_SET_QUORUM_ALL => ReplicaSetQuarumMode::All,
            FABRIC_REPLICA_SET_QUORUM_INVALID => ReplicaSetQuarumMode::Invalid,
            FABRIC_REPLICA_SET_WRITE_QUORUM => ReplicaSetQuarumMode::Write,
            _ => ReplicaSetQuarumMode::Invalid,
        }
    }
}

impl From<ReplicaSetQuarumMode> for FABRIC_REPLICA_SET_QUORUM_MODE {
    fn from(val: ReplicaSetQuarumMode) -> Self {
        match val {
            ReplicaSetQuarumMode::All => FABRIC_REPLICA_SET_QUORUM_ALL,
            ReplicaSetQuarumMode::Invalid => FABRIC_REPLICA_SET_QUORUM_INVALID,
            ReplicaSetQuarumMode::Write => FABRIC_REPLICA_SET_WRITE_QUORUM,
        }
    }
}

#[async_trait]
pub trait PrimaryReplicator: Replicator {
    async fn on_data_loss(&self) -> ::windows_core::Result<u8>;
    fn update_catch_up_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
        previousconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()>;
    async fn wait_for_catch_up_quorum(
        &self,
        catchupmode: ReplicaSetQuarumMode,
    ) -> ::windows_core::Result<()>;
    fn update_current_replica_set_configuration(
        &self,
        currentconfiguration: &ReplicaSetConfig,
    ) -> ::windows_core::Result<()>;
    async fn build_replica(&self, replica: &ReplicaInfo) -> ::windows_core::Result<()>;
    fn remove_replica(&self, replicaid: i64) -> ::windows_core::Result<()>;
}

// bridges from rs into com

// bridge from safe service instance to com
#[implement(IFabricReplicator)]

pub struct IFabricReplicatorBridge {
    inner: Arc<Mutex<Box<dyn Replicator + Send>>>,
    rt: Handle,
}

impl IFabricReplicatorBridge {
    pub fn create(rplctr: Box<dyn Replicator + Send>, rt: Handle) -> IFabricReplicatorBridge {
        IFabricReplicatorBridge {
            inner: Arc::new(Mutex::new(rplctr)),
            rt,
        }
    }

    fn create_from_primary_replicator(
        replicator: Arc<Mutex<Box<dyn Replicator + Send>>>,
        rt: Handle,
    ) -> IFabricReplicatorBridge {
        IFabricReplicatorBridge {
            inner: replicator,
            rt,
        }
    }
}

impl IFabricReplicator_Impl for IFabricReplicatorBridge {
    fn BeginOpen(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginOpen");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.open().await;
            let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricReplicatorBridge::EndOpen");
        let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> =
            unsafe { context.unwrap().as_impl() };

        let content = ctx_bridge.consume_content()?;
        Ok(StringResult::new(content).into())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginChangeRole");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();

        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };
        let role2: Role = role.into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.change_role(&epoch2, &role2).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::EndChangeRole");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginUpdateEpoch");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();
        let epoch2: Epoch = unsafe { epoch.as_ref().unwrap().into() };

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.update_epoch(&epoch2).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::BeginUpdateEpoch");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricReplicatorBridge::BeginClose");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.close().await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricReplicatorBridge::EndClose");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn Abort(&self) {
        info!("IFabricReplicatorBridge::Abort");
        self.inner.blocking_lock().abort();
    }

    fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        info!("IFabricReplicatorBridge::GetCurrentProgress");
        self.inner.blocking_lock().get_current_progress()
    }

    fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        info!("IFabricReplicatorBridge::GetCatchUpCapability");
        self.inner.blocking_lock().get_catch_up_capability()
    }
}

// primary replicator bridge
#[implement(IFabricPrimaryReplicator)]
pub struct IFabricPrimaryReplicatorBridge {
    inner: Arc<Mutex<Box<dyn PrimaryReplicator + Send>>>,
    rt: Handle,
    rplctr: IFabricReplicatorBridge,
}

impl IFabricPrimaryReplicatorBridge {
    pub fn create(
        rplctr: Box<dyn PrimaryReplicator + Send>,
        rt: Handle,
    ) -> IFabricPrimaryReplicatorBridge {
        let inner = Arc::new(Mutex::new(rplctr));

        // hack to construct a replicator bridge.
        let raw: *const Mutex<Box<dyn PrimaryReplicator + Send>> = Arc::into_raw(inner.clone());
        let raw: *const Mutex<Box<dyn Replicator + Send>> = raw.cast();

        let rpl_cast = unsafe { Arc::from_raw(raw) };
        // SAFETY: This is safe because the pointer orignally came from an Arc
        // with the same size and alignment since we've checked (via Any) that
        // the object within is the type being casted to.

        let replicator_bridge =
            IFabricReplicatorBridge::create_from_primary_replicator(rpl_cast, rt.clone());

        IFabricPrimaryReplicatorBridge {
            inner,
            rt,
            rplctr: replicator_bridge,
        }
    }
}

// TODO: this impl has duplicate code with replicator bridge
impl IFabricReplicator_Impl for IFabricPrimaryReplicatorBridge {
    fn BeginOpen(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginOpen(callback)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        self.rplctr.EndOpen(context)
    }

    fn BeginChangeRole(
        &self,
        epoch: *const FABRIC_EPOCH,
        role: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginChangeRole(epoch, role, callback)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndChangeRole(context)
    }

    fn BeginUpdateEpoch(
        &self,
        epoch: *const FABRIC_EPOCH,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginUpdateEpoch(epoch, callback)
    }

    fn EndUpdateEpoch(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndUpdateEpoch(context)
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        self.rplctr.BeginClose(callback)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        self.rplctr.EndClose(context)
    }

    fn Abort(&self) {
        self.rplctr.Abort()
    }

    fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        self.rplctr.GetCurrentProgress()
    }

    fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        self.rplctr.GetCatchUpCapability()
    }
}

impl IFabricPrimaryReplicator_Impl for IFabricPrimaryReplicatorBridge {
    fn BeginOnDataLoss(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricPrimaryReplicatorBridge::BeginOnDataLoss");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.on_data_loss().await;
            let ctx_bridge: &BridgeContext<Result<u8, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOnDataLoss(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<u8> {
        info!("IFabricPrimaryReplicatorBridge::EndOnDataLoss");
        let ctx_bridge: &BridgeContext<Result<u8, Error>> = unsafe { context.unwrap().as_impl() };

        ctx_bridge.consume_content()
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::UpdateCatchUpReplicaSetConfiguration");
        let cc = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref().unwrap() });
        let pc = ReplicaSetConfig::from(unsafe { previousconfiguration.as_ref().unwrap() });
        self.inner
            .blocking_lock()
            .update_catch_up_replica_set_configuration(&cc, &pc)
    }

    fn BeginWaitForCatchUpQuorum(
        &self,
        catchupmode: FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp
                .lock()
                .await
                .wait_for_catch_up_quorum(catchupmode.into())
                .await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndWaitForCatchUpQuorum(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::BeginWaitForCatchUpQuorum");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::UpdateCurrentReplicaSetConfiguration");
        let c = ReplicaSetConfig::from(unsafe { currentconfiguration.as_ref() }.unwrap());
        self.inner
            .blocking_lock()
            .update_current_replica_set_configuration(&c)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn BeginBuildReplica(
        &self,
        replica: *const FABRIC_REPLICA_INFORMATION,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricPrimaryReplicatorBridge::BeginBuildReplica");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();

        let r = ReplicaInfo::from(unsafe { replica.as_ref().unwrap() });

        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.build_replica(&r).await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndBuildReplica(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::EndBuildReplica");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()
    }

    fn RemoveReplica(&self, replicaid: i64) -> ::windows_core::Result<()> {
        info!("IFabricPrimaryReplicatorBridge::RemoveReplica");
        self.inner.blocking_lock().remove_replica(replicaid)
    }
}

// bridge for replica
// bridge from safe service instance to com
#[implement(IFabricStatefulServiceReplica)]

pub struct IFabricStatefulReplicaBridge {
    inner: Arc<Mutex<Box<dyn StatefulServiceReplica + Send>>>,
    rt: Handle,
}

impl IFabricStatefulReplicaBridge {
    pub fn create(
        rplctr: Box<dyn StatefulServiceReplica + Send>,
        rt: Handle,
    ) -> IFabricStatefulReplicaBridge {
        IFabricStatefulReplicaBridge {
            inner: Arc::new(Mutex::new(rplctr)),
            rt,
        }
    }
}

impl IFabricStatefulServiceReplica_Impl for IFabricStatefulReplicaBridge {
    fn BeginOpen(
        &self,
        openmode: FABRIC_REPLICA_OPEN_MODE,
        partition: ::core::option::Option<&IFabricStatefulServicePartition>,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginOpen");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let openmode2: OpenMode = openmode.into();
        let partition2: StatefulServicePartition = partition.unwrap().into();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<Box<dyn Replicator + Send>, Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.open(openmode2, &partition2).await;
            let ctx_bridge: &BridgeContext<Result<Box<dyn PrimaryReplicator + Send>, Error>> =
                unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            ctx_bridge.set_complete();
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndOpen(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricReplicator> {
        info!("IFabricStatefulReplicaBridge::EndOpen");
        let ctx_bridge: &BridgeContext<Result<Box<dyn PrimaryReplicator + Send>, Error>> =
            unsafe { context.unwrap().as_impl() };
        let rplctr = ctx_bridge.consume_content()?;

        // Replicator must impl primary replicator as well.
        let bridge: IFabricPrimaryReplicator =
            IFabricPrimaryReplicatorBridge::create(rplctr, self.rt.clone()).into();
        Ok(bridge.clone().cast::<IFabricReplicator>().unwrap())
    }

    fn BeginChangeRole(
        &self,
        newrole: FABRIC_REPLICA_ROLE,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginChangeRole");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let newrole2: Role = newrole.into();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<HSTRING, Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.change_role(newrole2).await;
            let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndChangeRole(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<IFabricStringResult> {
        info!("IFabricStatefulReplicaBridge::EndChangeRole");
        let ctx_bridge: &BridgeContext<Result<HSTRING, Error>> =
            unsafe { context.unwrap().as_impl() };
        let addr = ctx_bridge.consume_content()?;
        Ok(StringResult::new(addr).into())
    }

    fn BeginClose(
        &self,
        callback: ::core::option::Option<&super::IFabricAsyncOperationCallback>,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext> {
        info!("IFabricStatefulReplicaBridge::BeginClose");
        let inner_cp = self.inner.clone();
        let callback_cp = callback.unwrap().clone();

        let ctx: IFabricAsyncOperationContext =
            BridgeContext::<Result<(), Error>>::new(callback_cp).into();
        let ctx_cpy = ctx.clone();
        self.rt.spawn(async move {
            let ok = inner_cp.lock().await.close().await;
            let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { ctx_cpy.as_impl() };
            ctx_bridge.set_content(ok);
            let cb = ctx_bridge.Callback().unwrap();
            unsafe { cb.Invoke(&ctx_cpy) };
        });
        Ok(ctx)
    }

    fn EndClose(
        &self,
        context: ::core::option::Option<&super::IFabricAsyncOperationContext>,
    ) -> ::windows_core::Result<()> {
        info!("IFabricStatefulReplicaBridge::EndClose");
        let ctx_bridge: &BridgeContext<Result<(), Error>> = unsafe { context.unwrap().as_impl() };
        ctx_bridge.consume_content()?;
        Ok(())
    }

    fn Abort(&self) {
        self.inner.as_ref().blocking_lock().abort();
    }
}
