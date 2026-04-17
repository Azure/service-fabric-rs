// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tonic::{Request, Response, Status};

use mssf_core::types::ReplicaRole;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{GetReplicasRequest, GetReplicasResponse, HelloReply, HelloRequest, ReplicaInfo};

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
}

/// Shared state between gRPC server and Service Fabric service factory.
/// Keyed by partition_id.
#[derive(Debug, Clone, Default)]
pub struct ReplicaRegistry {
    inner: Arc<Mutex<HashMap<mssf_core::GUID, Vec<ReplicaEntry>>>>,
}

impl ReplicaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, partition_id: mssf_core::GUID, replica_id: i64) {
        self.inner
            .lock()
            .unwrap()
            .entry(partition_id)
            .or_default()
            .push(ReplicaEntry {
                partition_id,
                replica_id,
                role: ReplicaRole::Unknown,
            });
    }

    pub fn update_role(&self, partition_id: mssf_core::GUID, replica_id: i64, role: ReplicaRole) {
        let mut map = self.inner.lock().unwrap();
        if let Some(entry) = map
            .get_mut(&partition_id)
            .and_then(|replicas| replicas.iter_mut().find(|e| e.replica_id == replica_id))
        {
            entry.role = role;
        }
    }

    pub fn remove(&self, partition_id: mssf_core::GUID, replica_id: i64) {
        let mut map = self.inner.lock().unwrap();
        if let Some(replicas) = map.get_mut(&partition_id) {
            replicas.retain(|e| e.replica_id != replica_id);
            if replicas.is_empty() {
                map.remove(&partition_id);
            }
        }
    }

    fn get_all(&self) -> Vec<ReplicaEntry> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .flatten()
            .cloned()
            .collect()
    }

    fn get_by_partition(&self, partition_id: mssf_core::GUID) -> Vec<ReplicaEntry> {
        self.inner
            .lock()
            .unwrap()
            .get(&partition_id)
            .cloned()
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
}

pub fn greeter_server(registry: ReplicaRegistry) -> GreeterServer<MyGreeter> {
    GreeterServer::new(MyGreeter { registry })
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
    #[cfg(test)]
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
    #[cfg(test)]
    pub fn grpc_connect_url(&self) -> String {
        let mut url = self.base_url.clone();
        url.set_query(None);
        url.into()
    }
}
