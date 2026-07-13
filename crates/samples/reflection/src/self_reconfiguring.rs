// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::sync::{Arc, Mutex};

use mssf_core::runtime::executor::BoxedCancelToken;
use mssf_core::runtime::{
    ISelfReconfiguringServiceFactory, ISelfReconfiguringServiceInstance,
    ISelfReconfiguringServicePartition,
};
use mssf_core::types::{
    AUTO_SEQUENCE_NUMBER, HealthInformation, HealthState, InstanceChangeRequest,
    InstanceInformation, SelfReconfiguringConfigurationChangeRequest,
    SelfReconfiguringConfigurationReport, SelfReconfiguringConfigurationReportId,
    SelfReconfiguringConfigurationRequest, SelfReconfiguringConfigurationRequestId,
    SelfReconfiguringInstanceActivationState, SelfReconfiguringOpenMode,
};
use mssf_core::{ErrorCode, WString, async_trait};
use mssf_util::tokio::TokioExecutor;
use tracing::info;

use crate::control::{
    Approval, ApprovalDetails, ControlMode, Decision, ReplicaController, decode_init_data,
    make_controller,
};
use crate::grpc::{ReflectionUrl, ReplicaRegistry};

pub struct SelfReconfiguringFactory {
    executor: TokioExecutor,
    registry: ReplicaRegistry,
    grpc_hostname: String,
    grpc_port: u16,
}

impl SelfReconfiguringFactory {
    pub fn new(
        executor: TokioExecutor,
        registry: ReplicaRegistry,
        grpc_hostname: String,
        grpc_port: u16,
    ) -> Self {
        Self {
            executor,
            registry,
            grpc_hostname,
            grpc_port,
        }
    }
}

impl ISelfReconfiguringServiceFactory for SelfReconfiguringFactory {
    fn create_instance(
        &self,
        service_type_name: WString,
        service_name: mssf_core::types::Uri,
        initialization_data: &[u8],
        partition_id: mssf_core::GUID,
        instance_id: i64,
    ) -> mssf_core::Result<Box<dyn ISelfReconfiguringServiceInstance>> {
        let init = decode_init_data(initialization_data);
        let mode = ControlMode::from_init_data(&init);
        let controller = make_controller(mode);

        info!(
            type_name = %service_type_name,
            service = %service_name,
            partition = ?partition_id,
            instance = instance_id,
            ?mode,
            "creating self-reconfiguring instance"
        );

        if controller.is_controllable() {
            self.registry
                .add_controller(partition_id, instance_id, controller.clone());
        } else {
            self.registry.add(partition_id, instance_id);
        }

        Ok(Box::new(SelfReconfiguringInstance::new(
            partition_id,
            instance_id,
            self.executor.clone(),
            self.registry.clone(),
            controller,
            self.grpc_hostname.clone(),
            self.grpc_port,
        )))
    }
}

struct InstanceState {
    request_id: SelfReconfiguringConfigurationRequestId,
    is_leader: bool,
    last_seen: Vec<InstanceInformation>,
    partition: Option<Arc<dyn ISelfReconfiguringServicePartition>>,
}

pub struct SelfReconfiguringInstance {
    partition_id: mssf_core::GUID,
    instance_id: i64,
    executor: TokioExecutor,
    registry: ReplicaRegistry,
    controller: Arc<dyn ReplicaController>,
    grpc_hostname: String,
    grpc_port: u16,
    state: Mutex<InstanceState>,
}

impl SelfReconfiguringInstance {
    fn new(
        partition_id: mssf_core::GUID,
        instance_id: i64,
        executor: TokioExecutor,
        registry: ReplicaRegistry,
        controller: Arc<dyn ReplicaController>,
        grpc_hostname: String,
        grpc_port: u16,
    ) -> Self {
        Self {
            partition_id,
            instance_id,
            executor,
            registry,
            controller,
            grpc_hostname,
            grpc_port,
            state: Mutex::new(InstanceState {
                request_id: SelfReconfiguringConfigurationRequestId {
                    generation_number: 0,
                    sequence_number: 0,
                },
                is_leader: false,
                last_seen: Vec::new(),
                partition: None,
            }),
        }
    }

    fn await_sync(
        &self,
        gate: Approval,
        details: Option<ApprovalDetails>,
    ) -> mssf_core::Result<()> {
        let controller = self.controller.clone();
        match self.executor.block_on_any(async move {
            controller.await_approval_with_details(gate, details).await
        }) {
            Decision::Proceed => Ok(()),
            Decision::Fail(err) => Err(err),
        }
    }

    fn remove_from_registry(&self) {
        self.registry.remove(self.partition_id, self.instance_id);
    }
}

enum RequestIdCheck {
    Proceed,
    AlreadyProcessed,
}

fn validate_request_id(
    stored: &SelfReconfiguringConfigurationRequestId,
    incoming: &SelfReconfiguringConfigurationRequestId,
) -> mssf_core::Result<RequestIdCheck> {
    if incoming.generation_number <= 0 || incoming.sequence_number <= 0 {
        return Err(ErrorCode::E_INVALIDARG.into());
    }
    if stored.generation_number != 0 && incoming.generation_number < stored.generation_number {
        return Err(ErrorCode::E_INVALIDARG.into());
    }
    if stored.sequence_number != 0 && incoming.sequence_number < stored.sequence_number {
        return Err(ErrorCode::E_INVALIDARG.into());
    }
    if stored.sequence_number != 0 && incoming.sequence_number == stored.sequence_number {
        return Ok(RequestIdCheck::AlreadyProcessed);
    }
    Ok(RequestIdCheck::Proceed)
}

fn build_report(
    request_id: SelfReconfiguringConfigurationRequestId,
    instances: Vec<InstanceInformation>,
) -> SelfReconfiguringConfigurationReport {
    SelfReconfiguringConfigurationReport {
        request_id,
        report_id: SelfReconfiguringConfigurationReportId {
            sequence_number: request_id.sequence_number,
        },
        instances,
    }
}

fn elect_leader(instances: &[InstanceChangeRequest]) -> Option<i64> {
    instances
        .iter()
        .filter(|instance| {
            instance.requested_activation_state
                != SelfReconfiguringInstanceActivationState::Deactivated
        })
        .map(|instance| instance.instance_id)
        .min()
}

#[async_trait]
impl ISelfReconfiguringServiceInstance for SelfReconfiguringInstance {
    async fn open(
        &self,
        partition: Arc<dyn ISelfReconfiguringServicePartition>,
        open_mode: SelfReconfiguringOpenMode,
        _cancellation_token: BoxedCancelToken,
    ) -> mssf_core::Result<WString> {
        info!(instance = self.instance_id, ?open_mode, "open");
        self.state.lock().unwrap().partition = Some(partition.clone());
        partition.report_instance_health(&HealthInformation {
            source_id: WString::from("ReflectionSelfReconfig"),
            property: WString::from("Open"),
            time_to_live_seconds: u32::MAX,
            state: HealthState::Ok,
            description: WString::from("Reflection self-reconfiguring instance opened."),
            sequence_number: AUTO_SEQUENCE_NUMBER,
            remove_when_expired: false,
        })?;

        Ok(WString::from(
            ReflectionUrl::new(
                &self.grpc_hostname,
                self.grpc_port,
                self.partition_id,
                self.instance_id,
            )
            .to_url_string(),
        ))
    }

    fn request_configuration(
        &self,
        request: SelfReconfiguringConfigurationRequest,
    ) -> mssf_core::Result<()> {
        info!(instance = self.instance_id, "request_configuration");
        self.await_sync(
            Approval::RequestConfiguration,
            Some(ApprovalDetails::ConfigurationRequest(request)),
        )?;

        let (report, partition) = {
            let mut state = self.state.lock().unwrap();
            match validate_request_id(&state.request_id, &request.request_id)? {
                RequestIdCheck::AlreadyProcessed => return Ok(()),
                RequestIdCheck::Proceed => {}
            }
            state.request_id = request.request_id;
            if !state.is_leader {
                return Ok(());
            }
            (
                build_report(request.request_id, state.last_seen.clone()),
                state.partition.clone(),
            )
        };

        partition
            .ok_or_else(|| mssf_core::Error::from(ErrorCode::E_POINTER))?
            .report_configuration(&report)
    }

    fn request_configuration_change(
        &self,
        change: SelfReconfiguringConfigurationChangeRequest,
    ) -> mssf_core::Result<()> {
        info!(
            instance = self.instance_id,
            count = change.instances.len(),
            "request_configuration_change"
        );
        self.await_sync(
            Approval::RequestConfigurationChange,
            Some(ApprovalDetails::ConfigurationChange(change.clone())),
        )?;

        let (report, partition) = {
            let mut state = self.state.lock().unwrap();
            match validate_request_id(&state.request_id, &change.request_id)? {
                RequestIdCheck::AlreadyProcessed => return Ok(()),
                RequestIdCheck::Proceed => {}
            }
            if change.instances.is_empty() {
                return Err(ErrorCode::E_INVALIDARG.into());
            }

            state.request_id = change.request_id;
            state.last_seen = change
                .instances
                .iter()
                .map(|instance| InstanceInformation {
                    instance_id: instance.instance_id,
                    role: instance.requested_role,
                    activation_state: instance.requested_activation_state,
                })
                .collect();

            let leader = elect_leader(&change.instances);
            state.is_leader = leader == Some(self.instance_id);
            if !state.is_leader {
                return Ok(());
            }

            (
                build_report(change.request_id, state.last_seen.clone()),
                state.partition.clone(),
            )
        };

        partition
            .ok_or_else(|| mssf_core::Error::from(ErrorCode::E_POINTER))?
            .report_configuration(&report)
    }

    async fn close(&self, _cancellation_token: BoxedCancelToken) -> mssf_core::Result<()> {
        info!(instance = self.instance_id, "close");
        match self.controller.await_approval(Approval::Close).await {
            Decision::Proceed => {
                self.remove_from_registry();
                Ok(())
            }
            Decision::Fail(err) => Err(err),
        }
    }

    fn abort(&self) {
        info!(instance = self.instance_id, "abort");
        let controller = self.controller.clone();
        self.executor.block_on_any(async move {
            let _ = controller.await_approval(Approval::Abort).await;
        });
        self.remove_from_registry();
    }
}
