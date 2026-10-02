// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Safe types for Azure infrastructure jobs coordinated by Service Fabric.

use serde_json::Value;

use crate::{Error, ErrorCode, HRESULT, WString, types::Uri};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfrastructureUpgradeType {
    PlatformUpdate,
    PlatformRepair,
    PlatformMaintenance,
    TenantUpdate,
    TenantMaintenance,
}

impl std::fmt::Display for InfrastructureUpgradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlatformUpdate => write!(f, "PlatformUpdate"),
            Self::PlatformRepair => write!(f, "PlatformRepair"),
            Self::PlatformMaintenance => write!(f, "PlatformMaintenance"),
            Self::TenantUpdate => write!(f, "TenantUpdate"),
            Self::TenantMaintenance => write!(f, "TenantMaintenance"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfrastructureUpgradeJobStatus {
    Unknown,
    Pending,
    Executing,
    Alerted,
    Cancelled,
    Completed,
    Failed,
    Suspended,
}

impl TryFrom<&str> for InfrastructureUpgradeJobStatus {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Pending" => Ok(Self::Pending),
            "Executing" => Ok(Self::Executing),
            "Alerted" => Ok(Self::Alerted),
            "Cancelled" => Ok(Self::Cancelled),
            "Completed" => Ok(Self::Completed),
            "Failed" => Ok(Self::Failed),
            "Suspended" => Ok(Self::Suspended),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for InfrastructureUpgradeJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Unknown => "Unknown",
            Self::Pending => "Pending",
            Self::Executing => "Executing",
            Self::Alerted => "Alerted",
            Self::Cancelled => "Cancelled",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Suspended => "Suspended",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfrastructureUpgradeJobPhase {
    WaitingForApproval,
    Executing,
    WaitingForHealthCheck,
}

impl std::fmt::Display for InfrastructureUpgradeJobPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WaitingForApproval => write!(f, "WaitingForApproval"),
            Self::Executing => write!(f, "Executing"),
            Self::WaitingForHealthCheck => write!(f, "WaitingForHealthCheck"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfrastructureUpgradeJob {
    pub service_name: Uri,
    pub job_id: String,
    pub upgrade_type: InfrastructureUpgradeType,
    pub status: InfrastructureUpgradeJobStatus, //current state of the infrastructure upgrade job
    pub phase: Option<InfrastructureUpgradeJobPhase>, //current coordination step within an active infra upgrade job
    pub upgrade_domain: Option<String>,
    pub impacted_nodes: Vec<WString>, //note these will be in _NodeType_<base10 number> format.
}

impl InfrastructureUpgradeJob {
    pub(crate) fn from_query_response(
        service_name: &Uri,
        response: &str,
    ) -> crate::Result<Vec<Self>> {
        let state: Value = serde_json::from_str(response)
            .map_err(|error| invalid_query_response(format!("invalid JSON: {error}")))?;
        //try to get the mode of the infra upgrade coordinator, if unavailable, try to infer from presence of jobs array
        let mode = state
            .get("Mode")
            .and_then(Value::as_str)
            .unwrap_or_else(|| {
                if state.get("Jobs").and_then(Value::as_array).is_some() {
                    "Parallel"
                } else {
                    "Serial"
                }
            });
        match mode {
            "Serial" => Ok(Self::from_serial_state(service_name, &state)?
                .into_iter()
                .collect()),
            "Parallel" => Self::from_parallel_state(service_name, &state),
            mode => Err(invalid_query_response(format!(
                "unsupported coordinator mode '{mode}'"
            ))),
        }
    }

    fn from_serial_state(service_name: &Uri, state: &Value) -> crate::Result<Option<Self>> {
        //serial upgrade jobs seems to be a legacy option with a different json response. including it for completeness.
        let last_known_job_state = state
            .get("LastKnownJobState")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                invalid_query_response("missing serial LastKnownJobState".to_string())
            })?;
        let notification = match state.get("ManagementNotification") {
            Some(Value::Object(notification)) => notification,
            Some(Value::Null) | None if matches!(last_known_job_state, "Unknown" | "Idle") => {
                return Ok(None);
            }
            _ => {
                return Err(invalid_query_response(
                    "missing or invalid serial ManagementNotification".to_string(),
                ));
            }
        };
        let detailed_status = notification
            .get("ActiveJobDetailedStatus")
            .and_then(Value::as_str);
        let coordinator_phase = match last_known_job_state {
            "WaitingForApproval" => Some(InfrastructureUpgradeJobPhase::WaitingForApproval),
            "Executing" => Some(InfrastructureUpgradeJobPhase::Executing),
            "WaitingForHealthCheck" => Some(InfrastructureUpgradeJobPhase::WaitingForHealthCheck),
            "Unknown" | "Idle" => None,
            state => {
                return Err(invalid_query_response(format!(
                    "unsupported serial LastKnownJobState '{state}'"
                )));
            }
        };
        let status = match detailed_status
            .and_then(|value| InfrastructureUpgradeJobStatus::try_from(value).ok())
        {
            Some(status) => status,
            None => match last_known_job_state {
                "Unknown" => InfrastructureUpgradeJobStatus::Unknown,
                "Idle" => InfrastructureUpgradeJobStatus::Pending,
                _ => InfrastructureUpgradeJobStatus::Executing,
            },
        };
        let phase = (status == InfrastructureUpgradeJobStatus::Executing)
            .then_some(coordinator_phase)
            .flatten();
        let active_job_type = notification
            .get("ActiveJobType")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_query_response("missing serial ActiveJobType".to_string()))?;
        let upgrade_type = match active_job_type {
            "PlatformUpdateJob" => InfrastructureUpgradeType::PlatformUpdate,
            "PlatformRepairJob" => InfrastructureUpgradeType::PlatformRepair,
            "PlatformMaintenanceJob" => InfrastructureUpgradeType::PlatformMaintenance,
            "DeploymentUpdateJob" => InfrastructureUpgradeType::TenantUpdate,
            "DeploymentMaintenanceJob" => InfrastructureUpgradeType::TenantMaintenance,
            _ => return Ok(None),
        };
        let job_id = notification
            .get("ActiveJobId")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| invalid_query_response("missing serial ActiveJobId".to_string()))?
            .to_string();
        let impacted_nodes = state
            .pointer("/LastKnownTask/TaskDescription")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|task| task.get("NodeName").and_then(Value::as_str))
            .map(WString::from)
            .collect();

        Ok(Some(Self {
            service_name: service_name.clone(),
            job_id,
            upgrade_type,
            status,
            phase,
            upgrade_domain: json_scalar_to_string(notification.get("ActiveJobStepTargetUD")),
            impacted_nodes,
        }))
    }

    fn from_parallel_state(service_name: &Uri, state: &Value) -> crate::Result<Vec<Self>> {
        let jobs = state
            .get("Jobs")
            .and_then(Value::as_array)
            .ok_or_else(|| invalid_query_response("missing parallel Jobs array".to_string()))?;
        jobs.iter()
            .map(|job| -> crate::Result<Option<Self>> {
                let Some(status_value) = job.get("JobStatus").and_then(Value::as_str) else {
                    return Err(invalid_query_response(
                        "missing parallel JobStatus".to_string(),
                    ));
                };
                let Ok(status) = InfrastructureUpgradeJobStatus::try_from(status_value) else {
                    return Ok(None);
                };
                let impact_action =
                    job.get("ImpactAction")
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            invalid_query_response("missing parallel ImpactAction".to_string())
                        })?;
                let upgrade_type = match impact_action {
                    "PlatformUpdate" => InfrastructureUpgradeType::PlatformUpdate,
                    "PlatformMaintenance" => InfrastructureUpgradeType::PlatformMaintenance,
                    "TenantUpdate" => InfrastructureUpgradeType::TenantUpdate,
                    "TenantMaintenance" => InfrastructureUpgradeType::TenantMaintenance,
                    _ => return Ok(None),
                };
                let phase = if status == InfrastructureUpgradeJobStatus::Executing {
                    match (
                        job.get("ImpactStep").and_then(Value::as_str),
                        job.get("AcknowledgementStatus").and_then(Value::as_str),
                    ) {
                        (Some("ImpactStart"), Some("WaitingForAcknowledgement")) => {
                            Some(InfrastructureUpgradeJobPhase::WaitingForApproval)
                        }
                        (Some("ImpactStart"), Some("Acknowledged" | "Timedout")) => {
                            Some(InfrastructureUpgradeJobPhase::Executing)
                        }
                        (Some("ImpactEnd"), Some("WaitingForAcknowledgement")) => {
                            Some(InfrastructureUpgradeJobPhase::WaitingForHealthCheck)
                        }
                        _ => None,
                    }
                } else {
                    None
                };
                let job_id = job
                    .get("Id")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| invalid_query_response("missing parallel job Id".to_string()))?
                    .to_string();
                let impacted_nodes = job
                    .get("CurrentlyImpactedRoleInstances")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|instance| instance.get("Name").and_then(Value::as_str))
                    .map(WString::from)
                    .collect();

                Ok(Some(Self {
                    service_name: service_name.clone(),
                    job_id,
                    upgrade_type,
                    status,
                    phase,
                    upgrade_domain: json_scalar_to_string(job.get("CurrentUD")),
                    impacted_nodes,
                }))
            })
            .filter_map(|result| result.transpose())
            .collect()
    }
}

fn json_scalar_to_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(value)) => Some(value.clone()),
        Some(Value::Number(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn invalid_query_response(message: String) -> Error {
    Error::new(
        HRESULT::from(ErrorCode::E_INVALIDARG),
        Some(WString::from(message)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_query_maps_job_types_phases_and_nodes() {
        let response = r#"{
            "Mode": "Parallel",
            "Jobs": [
                {
                    "Id": "11111111-1111-1111-1111-111111111111",
                    "ImpactAction": "TenantUpdate",
                    "JobStatus": "Executing",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged",
                    "CurrentUD": 4,
                    "CurrentlyImpactedRoleInstances": [
                        { "Name": "_PSNode_7", "UD": "4" }
                    ]
                },
                {
                    "Id": "22222222-2222-2222-2222-222222222222",
                    "ImpactAction": "PlatformUpdate",
                    "JobStatus": "Completed",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged",
                    "CurrentlyImpactedRoleInstances": []
                },
                {
                    "Id": "33333333-3333-3333-3333-333333333333",
                    "ImpactAction": "PlatformMaintenance",
                    "JobStatus": "Executing",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged",
                    "CurrentUD": 3,
                    "CurrentlyImpactedRoleInstances": []
                },
                {
                    "Id": "44444444-4444-4444-4444-444444444444",
                    "ImpactAction": "TenantMaintenance",
                    "JobStatus": "Executing",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged",
                    "CurrentlyImpactedRoleInstances": []
                },
                {
                    "Id": "55555555-5555-5555-5555-555555555555",
                    "ImpactAction": "PlatformUpdate",
                    "JobStatus": "Executing",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged",
                    "CurrentlyImpactedRoleInstances": []
                },
                {
                    "Id": "66666666-6666-6666-6666-666666666666",
                    "ImpactAction": "Unknown",
                    "JobStatus": "Executing",
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged"
                },
                {
                    "Id": "77777777-7777-7777-7777-777777777777",
                    "ImpactAction": "PlatformUpdate",
                    "JobStatus": "Failed",
                    "CurrentlyImpactedRoleInstances": [
                        { "Name": "_PSNode_8", "UD": "5" }
                    ]
                },
                {
                    "Id": "88888888-8888-8888-8888-888888888888",
                    "ImpactAction": "TenantMaintenance",
                    "JobStatus": "Suspended",
                    "CurrentlyImpactedRoleInstances": [
                        { "Name": "_PSNode_9", "UD": "6" }
                    ]
                }
            ]
        }"#;

        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/PSNode"),
            response,
        )
        .unwrap();

        assert_eq!(jobs.len(), 7);
        assert_eq!(jobs[0].job_id, "11111111-1111-1111-1111-111111111111");
        assert_eq!(
            jobs[0].upgrade_type,
            InfrastructureUpgradeType::TenantUpdate
        );
        assert_eq!(
            jobs[0].phase,
            Some(InfrastructureUpgradeJobPhase::Executing)
        );
        assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Executing);
        assert_eq!(jobs[0].upgrade_domain.as_deref(), Some("4"));
        assert_eq!(jobs[0].impacted_nodes[0].to_string_lossy(), "_PSNode_7");
        assert_eq!(
            jobs.iter().map(|job| job.upgrade_type).collect::<Vec<_>>(),
            vec![
                InfrastructureUpgradeType::TenantUpdate,
                InfrastructureUpgradeType::PlatformUpdate,
                InfrastructureUpgradeType::PlatformMaintenance,
                InfrastructureUpgradeType::TenantMaintenance,
                InfrastructureUpgradeType::PlatformUpdate,
                InfrastructureUpgradeType::PlatformUpdate,
                InfrastructureUpgradeType::TenantMaintenance,
            ]
        );
        assert_eq!(jobs[1].status, InfrastructureUpgradeJobStatus::Completed);
        assert_eq!(jobs[1].phase, None);
        assert_eq!(jobs[5].status, InfrastructureUpgradeJobStatus::Failed);
        assert_eq!(jobs[5].phase, None);
        assert_eq!(jobs[6].status, InfrastructureUpgradeJobStatus::Suspended);
        assert_eq!(jobs[6].phase, None);
    }

    #[test]
    fn parallel_query_returns_all_known_job_statuses() {
        let cases = [
            ("Unknown", InfrastructureUpgradeJobStatus::Unknown),
            ("Pending", InfrastructureUpgradeJobStatus::Pending),
            ("Executing", InfrastructureUpgradeJobStatus::Executing),
            ("Alerted", InfrastructureUpgradeJobStatus::Alerted),
            ("Cancelled", InfrastructureUpgradeJobStatus::Cancelled),
            ("Completed", InfrastructureUpgradeJobStatus::Completed),
            ("Failed", InfrastructureUpgradeJobStatus::Failed),
            ("Suspended", InfrastructureUpgradeJobStatus::Suspended),
        ];
        let jobs = cases
            .iter()
            .map(|(status, _)| {
                serde_json::json!({
                    "Id": format!("job-{status}"),
                    "ImpactAction": "TenantUpdate",
                    "JobStatus": status,
                    "ImpactStep": "ImpactStart",
                    "AcknowledgementStatus": "Acknowledged"
                })
            })
            .collect::<Vec<_>>();
        let response = serde_json::json!({
            "Mode": "Parallel",
            "Jobs": jobs
        })
        .to_string();

        let parsed = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/PSNode"),
            &response,
        )
        .unwrap();

        assert_eq!(
            parsed.iter().map(|job| job.status).collect::<Vec<_>>(),
            cases
                .iter()
                .map(|(_, expected)| *expected)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn serial_query_returns_all_infrastructure_job_types() {
        let cases = [
            (
                "PlatformUpdateJob",
                InfrastructureUpgradeType::PlatformUpdate,
            ),
            (
                "PlatformRepairJob",
                InfrastructureUpgradeType::PlatformRepair,
            ),
            (
                "PlatformMaintenanceJob",
                InfrastructureUpgradeType::PlatformMaintenance,
            ),
            (
                "DeploymentUpdateJob",
                InfrastructureUpgradeType::TenantUpdate,
            ),
            (
                "DeploymentMaintenanceJob",
                InfrastructureUpgradeType::TenantMaintenance,
            ),
        ];

        for (active_job_type, expected) in cases {
            let response = serde_json::json!({
                "Mode": "Serial",
                "LastKnownJobState": "Executing",
                "ManagementNotification": {
                    "ActiveJobId": format!("job-{active_job_type}"),
                    "ActiveJobType": active_job_type,
                    "ActiveJobDetailedStatus": "WaitingForStartStepAcknowledgement",
                    "ActiveJobStepTargetUD": 2
                },
                "LastKnownTask": {
                    "TaskDescription": [{ "NodeName": "_CtrlNode_2" }]
                }
            })
            .to_string();

            let jobs = InfrastructureUpgradeJob::from_query_response(
                &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
                &response,
            )
            .unwrap();

            assert_eq!(jobs.len(), 1, "active job type: {active_job_type}");
            assert_eq!(jobs[0].job_id, format!("job-{active_job_type}"));
            assert_eq!(jobs[0].upgrade_type, expected);
            assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Executing);
            assert_eq!(
                jobs[0].phase,
                Some(InfrastructureUpgradeJobPhase::Executing)
            );
        }
    }

    #[test]
    fn serial_query_without_mode_uses_legacy_serial_format() {
        let response = r#"{
            "LastKnownJobState": "WaitingForApproval",
            "ManagementNotification": {
                "ActiveJobId": "legacy-job",
                "ActiveJobType": "PlatformUpdateJob",
                "ActiveJobDetailedStatus": "WaitingForStartStepAcknowledgement",
                "ActiveJobStepTargetUD": 2
            },
            "LastKnownTask": {
                "TaskDescription": [{ "NodeName": "_CtrlNode_2" }]
            }
        }"#;

        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
            response,
        )
        .unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].job_id, "legacy-job");
        assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Executing);
        assert_eq!(
            jobs[0].phase,
            Some(InfrastructureUpgradeJobPhase::WaitingForApproval)
        );
    }

    #[test]
    fn parallel_query_without_mode_is_inferred_from_jobs() {
        let response = r#"{
            "Jobs": [{
                "Id": "99999999-9999-9999-9999-999999999999",
                "ImpactAction": "TenantUpdate",
                "JobStatus": "Executing",
                "ImpactStep": "ImpactStart",
                "AcknowledgementStatus": "Acknowledged",
                "CurrentUD": 4,
                "CurrentlyImpactedRoleInstances": []
            }]
        }"#;

        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/PSNode"),
            response,
        )
        .unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].job_id, "99999999-9999-9999-9999-999999999999");
        assert_eq!(
            jobs[0].upgrade_type,
            InfrastructureUpgradeType::TenantUpdate
        );
        assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Executing);
    }

    #[test]
    fn serial_query_returns_all_known_job_statuses() {
        let cases = [
            ("Unknown", InfrastructureUpgradeJobStatus::Unknown),
            ("Pending", InfrastructureUpgradeJobStatus::Pending),
            ("Executing", InfrastructureUpgradeJobStatus::Executing),
            ("Alerted", InfrastructureUpgradeJobStatus::Alerted),
            ("Cancelled", InfrastructureUpgradeJobStatus::Cancelled),
            ("Completed", InfrastructureUpgradeJobStatus::Completed),
            ("Failed", InfrastructureUpgradeJobStatus::Failed),
            ("Suspended", InfrastructureUpgradeJobStatus::Suspended),
        ];

        for (status, expected_status) in cases {
            let last_known_job_state = if status == "Executing" {
                "Executing"
            } else {
                "Idle"
            };
            let response = serde_json::json!({
                "Mode": "Serial",
                "LastKnownJobState": last_known_job_state,
                "ManagementNotification": {
                    "ActiveJobId": format!("job-{status}"),
                    "ActiveJobType": "PlatformUpdateJob",
                    "ActiveJobDetailedStatus": status,
                    "ActiveJobStepTargetUD": 2
                },
                "LastKnownTask": {
                    "TaskDescription": [{ "NodeName": "_CtrlNode_2" }]
                }
            })
            .to_string();

            let jobs = InfrastructureUpgradeJob::from_query_response(
                &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
                &response,
            )
            .unwrap();

            assert_eq!(jobs.len(), 1, "job status: {status}");
            assert_eq!(jobs[0].status, expected_status, "job status: {status}");
            let expected_phase = match status {
                "Executing" => Some(InfrastructureUpgradeJobPhase::Executing),
                _ => None,
            };
            assert_eq!(jobs[0].phase, expected_phase, "job status: {status}");
        }
    }

    #[test]
    fn serial_idle_query_with_notification_returns_pending_job() {
        let response = r#"{
            "Mode": "Serial",
            "LastKnownJobState": "Idle",
            "ManagementNotification": {
                "ActiveJobId": "pending-job",
                "ActiveJobType": "PlatformUpdateJob",
                "ActiveJobDetailedStatus": "WaitingForStartStepAcknowledgement",
                "ActiveJobStepTargetUD": 2
            }
        }"#;

        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
            response,
        )
        .unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Pending);
        assert_eq!(jobs[0].phase, None);
    }

    #[test]
    fn serial_idle_query_without_notification_returns_no_job() {
        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
            r#"{
                "Mode": "Serial",
                "LastKnownJobState": "Idle",
                "ManagementNotification": null
            }"#,
        )
        .unwrap();

        assert!(jobs.is_empty());
    }

    #[test]
    fn malformed_parallel_query_returns_error() {
        let error = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/PSNode"),
            r#"{ "Mode": "Parallel" }"#,
        )
        .unwrap_err();

        assert!(error.to_string().contains("missing parallel Jobs array"));
    }

    #[test]
    fn parallel_executing_job_with_unknown_phase_is_reported() {
        let response = r#"{
            "Mode": "Parallel",
            "Jobs": [{
                "Id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
                "ImpactAction": "TenantUpdate",
                "JobStatus": "Executing",
                "ImpactStep": "Unknown",
                "AcknowledgementStatus": "Unknown"
            }]
        }"#;

        let jobs = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/PSNode"),
            response,
        )
        .unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].status, InfrastructureUpgradeJobStatus::Executing);
        assert_eq!(jobs[0].phase, None);
    }

    #[test]
    fn malformed_serial_query_returns_error() {
        let error = InfrastructureUpgradeJob::from_query_response(
            &Uri::from("fabric:/System/InfrastructureService/CtrlNode"),
            r#"{ "Mode": "Serial" }"#,
        )
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("missing serial LastKnownJobState")
        );
    }
}
