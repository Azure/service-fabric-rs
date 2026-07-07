// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Self-reconfiguring service instance - equivalent of a "replica" in the traditional
//! stateful service model, but instead its membership and configuration is entirely managed
//! by the implementation itself and not rely on Service Fabric's built-in failover
//! management. It would still listen to the configuration change requests from Service
//! Fabric and report the current configuration back to Service Fabric.

use std::sync::{Arc, Mutex};

use mssf_core::runtime::executor::BoxedCancelToken;
use mssf_core::runtime::{ISelfReconfiguringServiceInstance, ISelfReconfiguringServicePartition};
use mssf_core::types::{
    AUTO_SEQUENCE_NUMBER, HealthInformation, HealthState, InstanceChangeRequest,
    InstanceInformation, SelfReconfiguringConfigurationChangeRequest,
    SelfReconfiguringConfigurationReport, SelfReconfiguringConfigurationReportId,
    SelfReconfiguringConfigurationRequest, SelfReconfiguringConfigurationRequestId,
    SelfReconfiguringInstanceActivationState, SelfReconfiguringOpenMode,
};
use mssf_core::{ErrorCode, WString, async_trait};
use tracing::info;

/// Mutable state of an instance, guarded by a mutex because the configuration
/// callbacks are synchronous and mutate it concurrently with `open`.
struct InstanceState {
    /// Highest request id seen so far; `{0, 0}` means none yet.
    request_id: SelfReconfiguringConfigurationRequestId,
    /// Whether this instance is the leader, which is the only instance that
    /// reports configuration back to Service Fabric.
    is_leader: bool,
    /// The last configuration the instance was told about, reported back when
    /// Service Fabric asks for the current configuration.
    last_seen: Vec<InstanceInformation>,
    partition: Option<Arc<dyn ISelfReconfiguringServicePartition>>,
}

pub struct SelfReconfigInstance {
    instance_id: i64,
    state: Mutex<InstanceState>,
}

impl SelfReconfigInstance {
    pub fn new(instance_id: i64) -> Self {
        Self {
            instance_id,
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
}

/// Outcome of validating an incoming request id against the highest one seen.
enum RequestIdCheck {
    /// Proceed with processing the request.
    Proceed,
    /// The request was already processed (same sequence number); report success
    /// without doing any work.
    AlreadyProcessed,
}

/// Validates a request id, mirroring the C++ sample: generation and sequence
/// numbers must be positive and non-decreasing relative to the highest seen.
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

/// Builds a configuration report whose report id reuses the request's sequence
/// number (a shortcut from the reference sample, which has no way to generate
/// its own report ids).
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

/// Dummy leader election: pick the instance with the lowest instance id, unless it is deactivated,
/// in which case pick the next lowest.
fn elect_leader(instances: &[InstanceChangeRequest]) -> Option<i64> {
    let mut leader_index = 0usize;
    let mut leader_id = instances[0].instance_id;
    for (i, inst) in instances.iter().enumerate() {
        if leader_id > inst.instance_id {
            leader_id = inst.instance_id;
            leader_index = i;
        }
    }

    let deactivated = SelfReconfiguringInstanceActivationState::Deactivated;
    if instances[leader_index].requested_activation_state == deactivated {
        for (i, inst) in instances.iter().enumerate() {
            if i != leader_index && inst.requested_activation_state != deactivated {
                return Some(inst.instance_id);
            }
        }
        return None;
    }
    Some(leader_id)
}

#[async_trait]
impl ISelfReconfiguringServiceInstance for SelfReconfigInstance {
    async fn open(
        &self,
        partition: Arc<dyn ISelfReconfiguringServicePartition>,
        open_mode: SelfReconfiguringOpenMode,
        _cancellation_token: BoxedCancelToken,
    ) -> mssf_core::Result<WString> {
        info!("open: instance={} mode={open_mode:?}", self.instance_id);
        {
            let mut st = self.state.lock().unwrap();
            st.partition = Some(partition.clone());
        }

        let info = HealthInformation {
            source_id: WString::from("SelfReconfigOpen"),
            property: WString::from("Open"),
            time_to_live_seconds: u32::MAX,
            state: HealthState::Ok,
            description: WString::from("Self reconfiguring test service opened."),
            sequence_number: AUTO_SEQUENCE_NUMBER,
            remove_when_expired: false,
        };
        partition.report_instance_health(&info)?;

        Ok(WString::from(format!("address_{}", self.instance_id)))
    }

    fn request_configuration(
        &self,
        request: SelfReconfiguringConfigurationRequest,
    ) -> mssf_core::Result<()> {
        info!("request_configuration: instance={}", self.instance_id);
        let (report, partition) = {
            let mut st = self.state.lock().unwrap();
            match validate_request_id(&st.request_id, &request.request_id)? {
                RequestIdCheck::AlreadyProcessed => return Ok(()),
                RequestIdCheck::Proceed => {}
            }
            st.request_id = request.request_id;
            if !st.is_leader {
                return Ok(());
            }
            let report = build_report(request.request_id, st.last_seen.clone());
            (report, st.partition.clone())
        };

        match partition {
            Some(p) => p.report_configuration(&report),
            None => Err(ErrorCode::E_POINTER.into()),
        }
    }

    fn request_configuration_change(
        &self,
        change: SelfReconfiguringConfigurationChangeRequest,
    ) -> mssf_core::Result<()> {
        info!(
            "request_configuration_change: instance={} instances={}",
            self.instance_id,
            change.instances.len()
        );
        let (report, partition) = {
            let mut st = self.state.lock().unwrap();
            match validate_request_id(&st.request_id, &change.request_id)? {
                RequestIdCheck::AlreadyProcessed => return Ok(()),
                RequestIdCheck::Proceed => {}
            }
            if change.instances.is_empty() {
                return Err(ErrorCode::E_INVALIDARG.into());
            }

            // Record the requested configuration so it can be reported back when
            // Service Fabric later calls request_configuration.
            st.last_seen = change
                .instances
                .iter()
                .map(|i| InstanceInformation {
                    instance_id: i.instance_id,
                    role: i.requested_role,
                    activation_state: i.requested_activation_state,
                })
                .collect();

            let leader_id = match elect_leader(&change.instances) {
                Some(id) => id,
                None => {
                    info!("all instances deactivated; no leader elected");
                    return Ok(());
                }
            };

            if self.instance_id != leader_id {
                st.is_leader = false;
                return Ok(());
            }
            st.is_leader = true;
            st.request_id = change.request_id;

            let report = build_report(change.request_id, st.last_seen.clone());
            (report, st.partition.clone())
        };

        match partition {
            Some(p) => p.report_configuration(&report),
            None => Err(ErrorCode::E_POINTER.into()),
        }
    }

    async fn close(&self, _cancellation_token: BoxedCancelToken) -> mssf_core::Result<()> {
        info!("close: instance={}", self.instance_id);
        Ok(())
    }

    fn abort(&self) {
        info!("abort: instance={}", self.instance_id);
    }
}
