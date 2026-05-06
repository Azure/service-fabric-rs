// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! gRPC `ReplicaControl` server implementation.
//!
//! See `docs/design/ReflectionReplicaTestControl.md` §4 (proto), §7
//! (transport), and §8 (RPC error model).

use std::time::Duration;

use tonic::{Request, Response, Status};

use crate::control::{Approval, ApprovalKindFilter, ApproveResult, Decision};
use crate::grpc::ReplicaRegistry;

pub mod proto {
    tonic::include_proto!("reflection.control.v1");
}

use proto::approve_request::Decision as ApproveDecisionOneof;
use proto::list_pending_request::ReplicaFilter;
use proto::replica_control_server::{ReplicaControl, ReplicaControlServer};
use proto::{
    ApprovalEvent, ApprovalKind, ApproveRequest, DetachAllResponse, Empty, ListPendingRequest,
    ListPendingResponse, ReplicaRef, ReplicaRole as ProtoReplicaRole, WaitForApprovalRequest,
};

/// Default wait timeout when `WaitForApprovalRequest.timeout_ms` is 0.
/// Capped at [`MAX_WAIT_TIMEOUT`] to bound server-side resource usage.
pub const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_WAIT_TIMEOUT: Duration = Duration::from_secs(300);

/// Base port for the `ReplicaControl` gRPC server. Per-node port is
/// `BASE + node_index(Fabric_NodeName)`. Picked outside both onebox
/// `<ApplicationEndpoints>` ranges (Linux 22001–27000, Windows
/// 30001–35000) and outside Linux/Windows ephemeral ranges.
pub const REFLECTION_CONTROL_BASE_PORT: u16 = 28_000;

/// 0-based onebox node index parsed from `Fabric_NodeName`. Onebox
/// node names differ by platform; both arms produce 0..=4 for the
/// current 5-node topology.
pub fn node_index(node_name: &str) -> u16 {
    #[cfg(target_os = "windows")]
    {
        // Windows onebox: "_Node_3" -> "3" -> 3
        node_name
            .rsplit('_')
            .next()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or_else(|| panic!("unexpected Windows onebox node name: {node_name}"))
    }
    #[cfg(target_os = "linux")]
    {
        // Linux onebox: "N0030" -> 30 -> 30/10 - 1 = 2
        let digits = node_name.trim_start_matches('N');
        let n: u16 = digits
            .parse()
            .unwrap_or_else(|_| panic!("unexpected Linux onebox node name: {node_name}"));
        assert!(
            n >= 10 && n.is_multiple_of(10),
            "Linux onebox node names are expected to be N00<idx>0, got: {node_name}"
        );
        n / 10 - 1
    }
}

pub fn control_port_for_node(node_name: &str) -> u16 {
    REFLECTION_CONTROL_BASE_PORT
        .checked_add(node_index(node_name))
        .expect("node index too large for port range")
}

#[derive(Debug)]
pub struct ReplicaControlImpl {
    registry: ReplicaRegistry,
}

impl ReplicaControlImpl {
    pub fn new(registry: ReplicaRegistry) -> Self {
        Self { registry }
    }
}

pub fn replica_control_server(
    registry: ReplicaRegistry,
) -> ReplicaControlServer<ReplicaControlImpl> {
    ReplicaControlServer::new(ReplicaControlImpl::new(registry))
}

// ----------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------

fn parse_replica_ref(r: Option<&ReplicaRef>) -> Result<(mssf_core::GUID, i64), Status> {
    let r = r.ok_or_else(|| Status::invalid_argument("missing target"))?;
    if r.partition_id.is_empty() {
        return Err(Status::invalid_argument("missing target.partition_id"));
    }
    let parsed = uuid::Uuid::parse_str(&r.partition_id)
        .map_err(|e| Status::invalid_argument(format!("invalid partition_id: {e}")))?;
    Ok((mssf_core::GUID::from_u128(parsed.as_u128()), r.replica_id))
}

fn approval_kind_filter_from_proto(k: i32) -> Result<Option<ApprovalKindFilter>, Status> {
    let kind = ApprovalKind::try_from(k)
        .map_err(|_| Status::invalid_argument(format!("unknown ApprovalKind: {k}")))?;
    Ok(match kind {
        ApprovalKind::ApprovalUnspecified => None,
        ApprovalKind::ApprovalOpen => Some(ApprovalKindFilter::Open),
        ApprovalKind::ApprovalChangeRole => Some(ApprovalKindFilter::ChangeRole),
        ApprovalKind::ApprovalClose => Some(ApprovalKindFilter::Close),
        ApprovalKind::ApprovalAbort => Some(ApprovalKindFilter::Abort),
    })
}

fn approval_to_proto(gate: Approval) -> (ApprovalKind, ProtoReplicaRole) {
    match gate {
        Approval::Open => (ApprovalKind::ApprovalOpen, ProtoReplicaRole::Unknown),
        Approval::Close => (ApprovalKind::ApprovalClose, ProtoReplicaRole::Unknown),
        Approval::Abort => (ApprovalKind::ApprovalAbort, ProtoReplicaRole::Unknown),
        Approval::ChangeRole(role) => (
            ApprovalKind::ApprovalChangeRole,
            replica_role_to_proto(role),
        ),
    }
}

fn replica_role_to_proto(role: mssf_core::types::ReplicaRole) -> ProtoReplicaRole {
    use mssf_core::types::ReplicaRole as R;
    match role {
        R::None => ProtoReplicaRole::None,
        R::Primary => ProtoReplicaRole::Primary,
        R::IdleSecondary => ProtoReplicaRole::IdleSecondary,
        R::ActiveSecondary => ProtoReplicaRole::ActiveSecondary,
        R::IdleAuxiliary => ProtoReplicaRole::IdleAuxiliary,
        R::ActiveAuxiliary => ProtoReplicaRole::ActiveAuxiliary,
        R::PrimaryAuxiliary => ProtoReplicaRole::PrimaryAuxiliary,
        _ => ProtoReplicaRole::Unknown,
    }
}

fn build_approval_event(
    partition_id: mssf_core::GUID,
    replica_id: i64,
    gate_id: uuid::Uuid,
    gate: Approval,
) -> ApprovalEvent {
    let (kind, new_role) = approval_to_proto(gate);
    ApprovalEvent {
        target: Some(ReplicaRef {
            partition_id: format!("{partition_id:?}"),
            replica_id,
        }),
        kind: kind as i32,
        new_role: new_role as i32,
        gate_id: gate_id.to_string(),
    }
}

fn clamp_timeout(timeout_ms: u32) -> Duration {
    if timeout_ms == 0 {
        DEFAULT_WAIT_TIMEOUT
    } else {
        let requested = Duration::from_millis(timeout_ms as u64);
        std::cmp::min(requested, MAX_WAIT_TIMEOUT)
    }
}

// ----------------------------------------------------------------------
// gRPC handlers
// ----------------------------------------------------------------------

#[tonic::async_trait]
impl ReplicaControl for ReplicaControlImpl {
    async fn wait_for_approval(
        &self,
        request: Request<WaitForApprovalRequest>,
    ) -> Result<Response<ApprovalEvent>, Status> {
        let req = request.into_inner();
        let (partition_id, replica_id) = parse_replica_ref(req.target.as_ref())?;
        let expected = approval_kind_filter_from_proto(req.expected)?;
        let timeout = clamp_timeout(req.timeout_ms);

        let controller = self
            .registry
            .get_controller(partition_id, replica_id)
            .ok_or_else(|| Status::not_found("replica not registered (or not controllable)"))?;

        // Downcast to the concrete GrpcController so we can call its
        // wait_for_approval. The trait only exposes await_approval; the
        // inspection method is a concrete-type concern.
        let grpc = controller
            .as_any()
            .downcast_ref::<crate::control::GrpcController>()
            .ok_or_else(|| Status::internal("registered controller is not a GrpcController"))?;

        match tokio::time::timeout(timeout, grpc.wait_for_approval(expected)).await {
            Ok((gate_id, gate)) => Ok(Response::new(build_approval_event(
                partition_id,
                replica_id,
                gate_id,
                gate,
            ))),
            Err(_) => Err(Status::deadline_exceeded(format!(
                "WaitForApproval timed out after {} ms",
                timeout.as_millis(),
            ))),
        }
    }

    async fn approve(&self, request: Request<ApproveRequest>) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let (partition_id, replica_id) = parse_replica_ref(req.target.as_ref())?;
        let gate_id = uuid::Uuid::parse_str(&req.gate_id)
            .map_err(|e| Status::invalid_argument(format!("invalid gate_id: {e}")))?;

        let controller = self
            .registry
            .get_controller(partition_id, replica_id)
            .ok_or_else(|| Status::not_found("replica not registered (or already removed)"))?;
        let grpc = controller
            .as_any()
            .downcast_ref::<crate::control::GrpcController>()
            .ok_or_else(|| Status::internal("registered controller is not a GrpcController"))?;

        // Peek the pending kind first so we can validate Abort
        // gates reject fail_message decisions cleanly (per §8 RPC
        // error model).
        let pending_kind = grpc.peek_pending().map(|(_, gate)| gate);

        let decision = match req.decision {
            Some(ApproveDecisionOneof::Proceed(_)) | None => Decision::Proceed,
            Some(ApproveDecisionOneof::FailMessage(msg)) => {
                if matches!(pending_kind, Some(Approval::Abort)) {
                    return Err(Status::invalid_argument(
                        "fail_message is not allowed for APPROVAL_ABORT (SF abort cannot fail)",
                    ));
                }
                // mssf_core::Error only wraps an HRESULT; the
                // message is logged server-side for diagnostics but
                // SF itself only sees the code.
                tracing::info!("Approve(FailMessage): {} (returned to SF as E_FAIL)", msg);
                Decision::Fail(mssf_core::ErrorCode::E_FAIL.into())
            }
        };

        match grpc.approve(gate_id, decision) {
            ApproveResult::Released => Ok(Response::new(Empty {})),
            ApproveResult::SlotEmpty => Err(Status::failed_precondition(
                "gate already consumed (pending slot is empty)",
            )),
            ApproveResult::IdMismatch {
                pending_id,
                requested_id,
            } => Err(Status::failed_precondition(format!(
                "gate id mismatch: pending={pending_id}, requested={requested_id}"
            ))),
        }
    }

    async fn list_pending(
        &self,
        request: Request<ListPendingRequest>,
    ) -> Result<Response<ListPendingResponse>, Status> {
        let req = request.into_inner();

        let partition_filter = if req.partition_id.is_empty() {
            None
        } else {
            let parsed = uuid::Uuid::parse_str(&req.partition_id)
                .map_err(|e| Status::invalid_argument(format!("invalid partition_id: {e}")))?;
            Some(mssf_core::GUID::from_u128(parsed.as_u128()))
        };
        let replica_filter = req
            .replica_filter
            .map(|ReplicaFilter::SpecificReplicaId(id)| id);
        if replica_filter.is_some() && partition_filter.is_none() {
            return Err(Status::invalid_argument(
                "specific_replica_id requires partition_id (replica ids are not unique across partitions)",
            ));
        }

        let entries = self.registry.snapshot();
        let mut events = Vec::new();
        for entry in entries {
            if let Some(p) = partition_filter
                && entry.partition_id != p
            {
                continue;
            }
            if let Some(r) = replica_filter
                && entry.replica_id != r
            {
                continue;
            }
            let Some(controller) = entry.controller.as_ref() else {
                continue;
            };
            let Some(grpc) = controller
                .as_any()
                .downcast_ref::<crate::control::GrpcController>()
            else {
                continue;
            };
            if let Some((gate_id, gate)) = grpc.peek_pending() {
                events.push(build_approval_event(
                    entry.partition_id,
                    entry.replica_id,
                    gate_id,
                    gate,
                ));
            }
        }
        Ok(Response::new(ListPendingResponse { events }))
    }

    async fn detach(&self, request: Request<ReplicaRef>) -> Result<Response<Empty>, Status> {
        let r = request.into_inner();
        let (partition_id, replica_id) = parse_replica_ref(Some(&r))?;
        let controller = self
            .registry
            .get_controller(partition_id, replica_id)
            .ok_or_else(|| Status::not_found("replica not registered (or not controllable)"))?;
        let grpc = controller
            .as_any()
            .downcast_ref::<crate::control::GrpcController>()
            .ok_or_else(|| Status::internal("registered controller is not a GrpcController"))?;
        grpc.detach();
        Ok(Response::new(Empty {}))
    }

    async fn detach_all(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<DetachAllResponse>, Status> {
        let mut detached = 0u32;
        for entry in self.registry.snapshot() {
            let Some(controller) = entry.controller else {
                continue;
            };
            let Some(grpc) = controller
                .as_any()
                .downcast_ref::<crate::control::GrpcController>()
            else {
                continue;
            };
            if grpc.is_detached() {
                continue;
            }
            grpc.detach();
            detached += 1;
        }
        Ok(Response::new(DetachAllResponse { detached }))
    }
}

// ----------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn linux_node_index_parses_n00x0_format() {
        assert_eq!(node_index("N0010"), 0);
        assert_eq!(node_index("N0020"), 1);
        assert_eq!(node_index("N0030"), 2);
        assert_eq!(node_index("N0040"), 3);
        assert_eq!(node_index("N0050"), 4);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_node_index_parses_underscore_format() {
        assert_eq!(node_index("_Node_0"), 0);
        assert_eq!(node_index("_Node_1"), 1);
        assert_eq!(node_index("_Node_4"), 4);
    }

    #[test]
    fn control_port_offsets_from_base() {
        // Smoke-test the cfg-correct host's range maps to 28000..=28004.
        #[cfg(target_os = "linux")]
        for (name, expected) in [("N0010", 28000), ("N0030", 28002), ("N0050", 28004)] {
            assert_eq!(control_port_for_node(name), expected);
        }
        #[cfg(target_os = "windows")]
        for (name, expected) in [("_Node_0", 28000), ("_Node_2", 28002), ("_Node_4", 28004)] {
            assert_eq!(control_port_for_node(name), expected);
        }
    }

    #[test]
    fn clamp_timeout_zero_uses_default() {
        assert_eq!(clamp_timeout(0), DEFAULT_WAIT_TIMEOUT);
    }

    #[test]
    fn clamp_timeout_oversized_clamps_to_max() {
        assert_eq!(clamp_timeout(u32::MAX), MAX_WAIT_TIMEOUT);
    }

    #[test]
    fn clamp_timeout_in_range_passes_through() {
        assert_eq!(clamp_timeout(1500), Duration::from_millis(1500));
    }
}
