// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use mssf_core::runtime::IStatefulServicePartition;
use mssf_core::types::{ReplicaRole, ServicePartitionAccessStatus};

use crate::control::ReplicaController;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{
    GetReplicasRequest, GetReplicasResponse, HelloReply, HelloRequest, ReplicaInfo, WriteReply,
    WriteRequest,
};

/// Trailer/header name the SF Rust SDK uses to carry the
/// failover signal back to the client. Mirrors the constant in
/// `mssf_util::tonic` (kept as a `&str` here so this crate
/// doesn't have to depend on `http` at the proto-include site).
pub const MSSF_STATUS_TRAILER: &str = "mssf-status";

/// Request metadata header the client must set so a role-gated
/// RPC can identify which partition it's talking to. Required
/// because a single process can host multiple partitions of the
/// same service and the tonic handler has no other way to tell
/// them apart.
pub const MSSF_PARTITION_ID_HEADER: &str = "mssf-partition-id";

fn replica_role_to_proto(role: ReplicaRole) -> i32 {
    match role {
        ReplicaRole::None => hello_world::ReplicaRole::None.into(),
        ReplicaRole::Primary => hello_world::ReplicaRole::Primary.into(),
        ReplicaRole::IdleSecondary => hello_world::ReplicaRole::IdleSecondary.into(),
        ReplicaRole::ActiveSecondary => hello_world::ReplicaRole::ActiveSecondary.into(),
        ReplicaRole::IdleAuxiliary => hello_world::ReplicaRole::IdleAuxiliary.into(),
        ReplicaRole::ActiveAuxiliary => hello_world::ReplicaRole::ActiveAuxiliary.into(),
        ReplicaRole::PrimaryAuxiliary => hello_world::ReplicaRole::PrimaryAuxiliary.into(),
        _ => hello_world::ReplicaRole::Unknown.into(),
    }
}

/// Entry tracking a replica in the registry.
#[derive(Debug, Clone)]
pub struct ReplicaEntry {
    pub partition_id: mssf_core::GUID,
    pub replica_id: i64,
    pub role: ReplicaRole,
    /// `None` for `NoopController` replicas (production path); `Some` for
    /// `GrpcController` replicas (test-driven). gRPC handlers that need
    /// the controller use [`ReplicaRegistry::get_controller`].
    pub controller: Option<Arc<dyn ReplicaController>>,
}

/// Per-partition state. Bundles the list of replicas hosted in
/// this process plus the live `IStatefulServicePartition`
/// handle (when one of those replicas has been opened by SF).
/// The handle is what role-gated RPCs (`Write`) consult via
/// `partition.get_write_status()`.
#[derive(Default)]
struct PartitionState {
    replicas: Vec<ReplicaEntry>,
    /// Set by `bind_partition` (called from `Replica::open`),
    /// cleared by `unbind_partition` (called from
    /// `Replica::close` / `Replica::abort`). `None` means "no
    /// live replica is currently open for this partition".
    partition: Option<Arc<dyn IStatefulServicePartition>>,
}

/// Shared state between gRPC server and Service Fabric service factory.
/// Keyed by partition_id.
#[derive(Clone, Default)]
pub struct ReplicaRegistry {
    inner: Arc<Mutex<HashMap<mssf_core::GUID, PartitionState>>>,
}

impl std::fmt::Debug for ReplicaRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplicaRegistry").finish_non_exhaustive()
    }
}

impl ReplicaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, partition_id: mssf_core::GUID, replica_id: i64) {
        self.add_with_controller(partition_id, replica_id, None);
    }

    /// Register a controllable replica. Only called for `GrpcController`
    /// replicas (i.e., when the decoded `ReplicaInitData` had `control = true`).
    pub fn add_controller(
        &self,
        partition_id: mssf_core::GUID,
        replica_id: i64,
        controller: Arc<dyn ReplicaController>,
    ) {
        self.add_with_controller(partition_id, replica_id, Some(controller));
    }

    fn add_with_controller(
        &self,
        partition_id: mssf_core::GUID,
        replica_id: i64,
        controller: Option<Arc<dyn ReplicaController>>,
    ) {
        let mut map = self.inner.lock().unwrap();
        let state = map.entry(partition_id).or_default();
        // SF can call Factory::create_replica again with the same
        // (partition_id, replica_id) after a failed Open or
        // change_role (the failed replica is dropped, a fresh one
        // is constructed in its place — see fail_open / fail_change_role
        // tests). Replace any pre-existing entry so the registry
        // always points at the live replica's controller; otherwise
        // ListPending and get_controller would dispatch to a stale
        // controller whose pending slot has been drained.
        let new_entry = ReplicaEntry {
            partition_id,
            replica_id,
            role: ReplicaRole::Unknown,
            controller,
        };
        if let Some(existing) = state
            .replicas
            .iter_mut()
            .find(|e| e.replica_id == replica_id)
        {
            *existing = new_entry;
        } else {
            state.replicas.push(new_entry);
        }
    }

    /// Bind the live `IStatefulServicePartition` handle for a
    /// partition. Called from `Replica::open` once SF has handed
    /// us the partition; only one replica per partition is
    /// opened on this node at a time so this overwrites any
    /// stale prior binding (defensive — should not normally
    /// happen).
    pub fn bind_partition(
        &self,
        partition_id: mssf_core::GUID,
        partition: Arc<dyn IStatefulServicePartition>,
    ) {
        let mut map = self.inner.lock().unwrap();
        map.entry(partition_id).or_default().partition = Some(partition);
    }

    /// Clear the partition handle for `partition_id`. Called
    /// from `Replica::close` and `Replica::abort`. If the
    /// partition state is then empty (no replicas and no
    /// partition), the entry is dropped to keep the map tidy.
    pub fn unbind_partition(&self, partition_id: mssf_core::GUID) {
        let mut map = self.inner.lock().unwrap();
        if let Some(state) = map.get_mut(&partition_id) {
            state.partition = None;
            if state.replicas.is_empty() {
                map.remove(&partition_id);
            }
        }
    }

    /// Look up the currently bound partition handle. Returns
    /// `None` if no replica is currently open for this
    /// partition on this node.
    pub fn get_partition(
        &self,
        partition_id: mssf_core::GUID,
    ) -> Option<Arc<dyn IStatefulServicePartition>> {
        self.inner
            .lock()
            .unwrap()
            .get(&partition_id)?
            .partition
            .clone()
    }

    /// Look up a replica's controller (if any).
    pub fn get_controller(
        &self,
        partition_id: mssf_core::GUID,
        replica_id: i64,
    ) -> Option<Arc<dyn ReplicaController>> {
        self.inner
            .lock()
            .unwrap()
            .get(&partition_id)?
            .replicas
            .iter()
            .find(|e| e.replica_id == replica_id)?
            .controller
            .clone()
    }

    /// Snapshot of all registered entries (used by ListPending).
    pub fn snapshot(&self) -> Vec<ReplicaEntry> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .flat_map(|s| s.replicas.iter().cloned())
            .collect()
    }

    pub fn update_role(&self, partition_id: mssf_core::GUID, replica_id: i64, role: ReplicaRole) {
        let mut map = self.inner.lock().unwrap();
        if let Some(entry) = map.get_mut(&partition_id).and_then(|state| {
            state
                .replicas
                .iter_mut()
                .find(|e| e.replica_id == replica_id)
        }) {
            entry.role = role;
        }
    }

    pub fn remove(&self, partition_id: mssf_core::GUID, replica_id: i64) {
        let mut map = self.inner.lock().unwrap();
        if let Some(state) = map.get_mut(&partition_id) {
            state.replicas.retain(|e| e.replica_id != replica_id);
            if state.replicas.is_empty() && state.partition.is_none() {
                map.remove(&partition_id);
            }
        }
    }

    fn get_all(&self) -> Vec<ReplicaEntry> {
        self.snapshot()
    }

    fn get_by_partition(&self, partition_id: mssf_core::GUID) -> Vec<ReplicaEntry> {
        self.inner
            .lock()
            .unwrap()
            .get(&partition_id)
            .map(|s| s.replicas.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct MyGreeter {
    registry: ReplicaRegistry,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn get_replicas(
        &self,
        request: Request<GetReplicasRequest>,
    ) -> Result<Response<GetReplicasResponse>, Status> {
        let req = request.into_inner();
        let entries = if req.partition_id.is_empty() {
            self.registry.get_all()
        } else {
            let parsed = uuid::Uuid::parse_str(&req.partition_id)
                .map_err(|_| Status::invalid_argument("invalid partition_id format"))?;
            let guid = mssf_core::GUID::from_u128(parsed.as_u128());
            self.registry.get_by_partition(guid)
        };

        let replicas = entries
            .into_iter()
            .map(|e| ReplicaInfo {
                partition_id: format!("{:?}", e.partition_id),
                replica_id: e.replica_id,
                role: replica_role_to_proto(e.role),
            })
            .collect();

        Ok(Response::new(GetReplicasResponse { replicas }))
    }

    async fn write(&self, request: Request<WriteRequest>) -> Result<Response<WriteReply>, Status> {
        // Identify the target partition from the required
        // `mssf-partition-id` request metadata header. Missing
        // or malformed -> invalid_argument; an absent partition
        // (process has never opened this partition or it has
        // closed) -> unavailable so the client retries via a
        // resolve.
        let partition_id = parse_partition_id_header(&request)?;
        let partition = self.registry.get_partition(partition_id).ok_or_else(|| {
            Status::unavailable(format!(
                "partition {partition_id:?} is not currently hosted on this node"
            ))
        })?;
        let write_status = partition
            .get_write_status()
            .map_err(|e| Status::internal(format!("get_write_status failed: {e:?}")))?;
        match write_status {
            ServicePartitionAccessStatus::Granted => {
                let replica_id = self
                    .registry
                    .get_by_partition(partition_id)
                    .first()
                    .map(|e| e.replica_id)
                    .unwrap_or(0);
                let payload = request.into_inner().payload;
                tracing::info!(
                    partition = ?partition_id,
                    replica_id,
                    payload_len = payload.len(),
                    "Write accepted on primary"
                );
                Ok(Response::new(WriteReply {
                    acked_by: format!("{:?}/{}", partition_id, replica_id),
                }))
            }
            ServicePartitionAccessStatus::NotPrimary => Err(status_with_mssf(
                tonic::Code::Unavailable,
                "not primary",
                "not-primary",
            )),
            ServicePartitionAccessStatus::ReconfigurationPending => Err(status_with_mssf(
                tonic::Code::Unavailable,
                "reconfiguration pending",
                "reconfiguration-pending",
            )),
            // Transient on the same primary; the client should
            // retry against this same channel without rebuilding.
            // No mssf-status metadata.
            ServicePartitionAccessStatus::NoWriteQuorum => {
                Err(Status::unavailable("no write quorum"))
            }
            ServicePartitionAccessStatus::Invalid => {
                Err(Status::internal("partition reported Invalid write status"))
            }
        }
    }
}

/// Parse and validate the `mssf-partition-id` request metadata
/// header. Returns `invalid_argument` on missing / non-ascii /
/// non-GUID values.
fn parse_partition_id_header<T>(request: &Request<T>) -> Result<mssf_core::GUID, Status> {
    let raw = request
        .metadata()
        .get(MSSF_PARTITION_ID_HEADER)
        .ok_or_else(|| {
            Status::invalid_argument(format!(
                "missing required `{MSSF_PARTITION_ID_HEADER}` metadata header"
            ))
        })?
        .to_str()
        .map_err(|_| {
            Status::invalid_argument(format!("`{MSSF_PARTITION_ID_HEADER}` must be ASCII"))
        })?;
    let uuid = uuid::Uuid::parse_str(raw).map_err(|e| {
        Status::invalid_argument(format!(
            "`{MSSF_PARTITION_ID_HEADER}` is not a valid UUID: {e}"
        ))
    })?;
    Ok(mssf_core::GUID::from_u128(uuid.as_u128()))
}

/// Build an error `Status` carrying the `mssf-status` metadata
/// entry that triggers a channel rebuild in
/// [`mssf_util::tonic::ResolveStatusMiddleware`].
fn status_with_mssf(code: tonic::Code, message: &str, mssf_status: &str) -> Status {
    let mut md = tonic::metadata::MetadataMap::new();
    md.insert(
        MSSF_STATUS_TRAILER,
        mssf_status.parse().expect("valid ascii"),
    );
    Status::with_metadata(code, message.to_string(), md)
}

pub fn greeter_server(registry: ReplicaRegistry) -> GreeterServer<MyGreeter> {
    GreeterServer::new(MyGreeter { registry })
}

// ===== Client-side helpers (Phase 2) =====================
//
// These wrap `mssf_util::tonic` so a caller building a
// `GreeterClient` against `fabric:/.../ReflectionApp/<svc>`
// doesn't have to repeat the resolver/selector/channel wiring.
// The selector picks the StatefulPrimary endpoint and parses
// its `ReflectionUrl`-encoded address into a TCP `DialTarget`;
// the channel attaches `ResolveStatusMiddleware` to watch for
// the `mssf-status` failover signal.

use mssf_core::client::FabricClient;
use mssf_core::client::svc_mgmt_client::{
    PartitionKeyType, ResolvedServicePartition, ServiceEndpointRole,
};
use mssf_util::tonic::{
    DialTarget, FabricTargetResolverBuilder, SelectError, TargetChannel, TargetChannelBuilder,
};

/// Selector that returns the dial target for the
/// `StatefulPrimary` endpoint of a singleton partition. Used by
/// [`build_primary_channel`]; exposed as a standalone fn for
/// callers that want to share the parse-and-select logic with a
/// custom resolver.
pub fn primary_selector()
-> impl Fn(&ResolvedServicePartition) -> Result<DialTarget, SelectError> + Send + Sync + 'static {
    |rsp| {
        let ep = rsp
            .endpoints
            .iter()
            .find(|e| e.role == ServiceEndpointRole::StatefulPrimary)
            .ok_or(SelectError::NoMatch)?;
        let url = ReflectionUrl::parse(&ep.address.to_string())
            .map_err(|e| SelectError::Fatal(e.into()))?;
        let host = url
            .base_url
            .host_str()
            .ok_or_else(|| SelectError::Fatal("ReflectionUrl missing host".into()))?
            .to_string();
        let port = url
            .base_url
            .port()
            .ok_or_else(|| SelectError::Fatal("ReflectionUrl missing port".into()))?;
        Ok(DialTarget { host, port })
    }
}

/// Build a failover-aware `tonic::Channel` for the reflection
/// sample's `Greeter` service. Resolves `service_uri` (a
/// `fabric:/...` URI) via SF naming, dials the current primary,
/// and rebuilds the inner channel when the server attaches an
/// `mssf-status` metadata entry (via [`MyGreeter::write`] on a
/// non-primary).
///
/// Pure construction — no IO until the first RPC.
pub fn build_primary_channel(fc: FabricClient, service_uri: &str) -> TargetChannel {
    let uri = mssf_core::types::Uri::from(service_uri);
    let resolver = FabricTargetResolverBuilder::new(fc)
        .service_uri(uri)
        .partition_key(PartitionKeyType::None)
        .target_selector(primary_selector())
        .build();
    TargetChannelBuilder::new()
        .resolver(resolver)
        .trailer_header(MSSF_STATUS_TRAILER)
        .build()
}

/// A parsed reflection service URL containing the gRPC base address,
/// partition ID, and replica ID.
#[derive(Debug, Clone)]
pub struct ReflectionUrl {
    pub base_url: url::Url,
    pub partition_id: mssf_core::GUID,
    pub replica_id: i64,
}

impl ReflectionUrl {
    /// Build a reflection URL from components.
    pub fn new(hostname: &str, port: u16, partition_id: mssf_core::GUID, replica_id: i64) -> Self {
        let base_url = url::Url::parse(&format!("http://{}:{}", hostname, port))
            .expect("failed to parse gRPC URL");
        Self {
            base_url,
            partition_id,
            replica_id,
        }
    }

    /// Serialize to a URL string with query params.
    pub fn to_url_string(&self) -> String {
        let mut url = self.base_url.clone();
        url.query_pairs_mut()
            .append_pair("partitionId", &format!("{:?}", self.partition_id))
            .append_pair("replicaId", &self.replica_id.to_string());
        url.into()
    }

    /// Parse a reflection URL string back into components.
    pub fn parse(s: &str) -> Result<Self, String> {
        let url = url::Url::parse(s).map_err(|e| format!("invalid URL: {e}"))?;
        let mut partition_id = None;
        let mut replica_id = None;
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "partitionId" => {
                    let uuid = uuid::Uuid::parse_str(&value)
                        .map_err(|e| format!("invalid partitionId: {e}"))?;
                    partition_id = Some(mssf_core::GUID::from_u128(uuid.as_u128()));
                }
                "replicaId" => {
                    replica_id = Some(
                        value
                            .parse::<i64>()
                            .map_err(|e| format!("invalid replicaId: {e}"))?,
                    );
                }
                _ => {}
            }
        }
        let mut base_url = url.clone();
        base_url.set_query(None);
        Ok(Self {
            base_url,
            partition_id: partition_id.ok_or("missing partitionId")?,
            replica_id: replica_id.ok_or("missing replicaId")?,
        })
    }

    /// Get the base URL string (without query params) for gRPC connection.
    pub fn grpc_connect_url(&self) -> String {
        let mut url = self.base_url.clone();
        url.set_query(None);
        url.into()
    }
}
