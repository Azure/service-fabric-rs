// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Safe types for Azure infrastructure tasks tracked by Service Fabric Repair Manager.

use mssf_com::FabricClient::IFabricGetRepairTaskListResult;
use mssf_com::FabricTypes::{
    FABRIC_REPAIR_IMPACT_KIND, FABRIC_REPAIR_NODE_IMPACT, FABRIC_REPAIR_NODE_IMPACT_LEVEL,
    FABRIC_REPAIR_NODE_IMPACT_LIST, FABRIC_REPAIR_SCOPE_IDENTIFIER,
    FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND, FABRIC_REPAIR_TARGET_KIND, FABRIC_REPAIR_TASK,
    FABRIC_REPAIR_TASK_QUERY_DESCRIPTION, FABRIC_REPAIR_TASK_STATE,
    FABRIC_REPAIR_TASK_STATE_FILTER, FABRIC_STRING_LIST,
};

use crate::{
    PCWSTR, WString,
    mem::{BoxPool, GetRawWithBoxPool},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairTaskType {
    PlatformUpdate,
    PlatformMaintenance,
    TenantUpdate,
    TenantMaintenance,
}

impl std::fmt::Display for RepairTaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::PlatformUpdate => "PlatformUpdate",
            Self::PlatformMaintenance => "PlatformMaintenance",
            Self::TenantUpdate => "TenantUpdate",
            Self::TenantMaintenance => "TenantMaintenance",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairTaskState {
    Preparing,
    Approved,
    Executing,
    Restoring,
}

impl std::fmt::Display for RepairTaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preparing => write!(f, "Preparing"),
            Self::Approved => write!(f, "Approved"),
            Self::Executing => write!(f, "Executing"),
            Self::Restoring => write!(f, "Restoring"),
        }
    }
}

impl TryFrom<FABRIC_REPAIR_TASK_STATE> for RepairTaskState {
    type Error = ();

    fn try_from(value: FABRIC_REPAIR_TASK_STATE) -> Result<Self, Self::Error> {
        match value {
            FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_PREPARING => Ok(Self::Preparing),
            FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_APPROVED => Ok(Self::Approved),
            FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_EXECUTING => Ok(Self::Executing),
            FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_RESTORING => Ok(Self::Restoring),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairNodeImpactLevel {
    None,
    Restart,
    RemoveData,
    RemoveNode,
    Pause,
    Invalid,
}

impl std::fmt::Display for RepairNodeImpactLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::None => "None",
            Self::Restart => "Restart",
            Self::RemoveData => "RemoveData",
            Self::RemoveNode => "RemoveNode",
            Self::Pause => "Pause",
            Self::Invalid => "Invalid",
        };
        write!(f, "{value}")
    }
}

impl From<FABRIC_REPAIR_NODE_IMPACT_LEVEL> for RepairNodeImpactLevel {
    fn from(value: FABRIC_REPAIR_NODE_IMPACT_LEVEL) -> Self {
        match value {
            FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_NONE => Self::None,
            FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_RESTART => {
                Self::Restart
            }
            FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_DATA => {
                Self::RemoveData
            }
            FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_REMOVE_NODE => {
                Self::RemoveNode
            }
            FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_PAUSE => Self::Pause,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairNodeImpact {
    pub node_name: WString,
    pub level: RepairNodeImpactLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairTask {
    pub task_id: WString,
    pub repair_type: RepairTaskType,
    pub state: RepairTaskState,
    pub upgrade_domain: Option<String>,
    pub target_nodes: Vec<WString>,
    pub node_impacts: Vec<RepairNodeImpact>,
}

pub(crate) struct RepairTaskQueryDescription {
    task_id_filter: WString,
    state_filter: u32,
}

impl RepairTaskQueryDescription {
    pub(crate) fn in_flight() -> Self {
        let state_filter =
            FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_PREPARING.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_APPROVED.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_EXECUTING.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_RESTORING.0;
        Self {
            //Infra service tasks are prefixed with "Azure" in the task ID.
            //Since repair manager stores other, unrelated tasks, we filter them out here
            task_id_filter: WString::from("Azure/"),
            state_filter: state_filter as u32,
        }
    }
}

impl GetRawWithBoxPool<FABRIC_REPAIR_TASK_QUERY_DESCRIPTION> for RepairTaskQueryDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_REPAIR_TASK_QUERY_DESCRIPTION {
        let scope = pool.push(Box::new(FABRIC_REPAIR_SCOPE_IDENTIFIER {
            Kind: FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND::FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_CLUSTER,
            Value: std::ptr::null_mut(),
        }));
        FABRIC_REPAIR_TASK_QUERY_DESCRIPTION {
            Scope: scope.cast_mut(),
            TaskIdFilter: self.task_id_filter.as_pcwstr(),
            StateFilter: self.state_filter,
            ExecutorFilter: PCWSTR::null(),
            Reserved: std::ptr::null_mut(),
        }
    }
}

impl RepairTask {
    pub(crate) fn from_result(result: &IFabricGetRepairTaskListResult) -> Vec<Self> {
        // SAFETY: all pointers are owned by `result` and are consumed before it is dropped.
        unsafe {
            let Some(list) = result.get_Tasks().as_ref() else {
                return Vec::new();
            };
            raw_slice(list.Items, list.Count)
                .iter()
                .filter_map(Self::from_raw)
                .collect()
        }
    }

    fn from_raw(task: &FABRIC_REPAIR_TASK) -> Option<Self> {
        let state = RepairTaskState::try_from(task.State).ok()?;
        let task_id = WString::from(task.TaskId);
        let (repair_type, upgrade_domain) = parse_repair_task_id(&task_id.to_string_lossy())?;

        Some(Self {
            task_id,
            repair_type,
            state,
            upgrade_domain,
            target_nodes: repair_target_nodes(task),
            node_impacts: repair_node_impacts(task),
        })
    }
}

fn parse_repair_task_id(task_id: &str) -> Option<(RepairTaskType, Option<String>)> {
    // task ids we filter have the following format Azure/{TaskType}/{JobId}/{UD}/{Incarnation}
    let mut parts = task_id.split('/');
    if !parts.next()?.eq_ignore_ascii_case("Azure") {
        return None;
    }
    let repair_type = match parts.next()? {
        "PlatformUpdate" => RepairTaskType::PlatformUpdate,
        "PlatformMaintenance" => RepairTaskType::PlatformMaintenance,
        "TenantUpdate" => RepairTaskType::TenantUpdate,
        "TenantMaintenance" => RepairTaskType::TenantMaintenance,
        _ => return None,
    };
    parts.next()?;
    let upgrade_domain = parts
        .next()
        .filter(|value| *value != "-")
        .map(str::to_string);
    Some((repair_type, upgrade_domain))
}

fn repair_target_nodes(task: &FABRIC_REPAIR_TASK) -> Vec<WString> {
    unsafe {
        let Some(target) = task.Target.as_ref() else {
            return Vec::new();
        };
        if target.Kind != FABRIC_REPAIR_TARGET_KIND::FABRIC_REPAIR_TARGET_KIND_NODE {
            return Vec::new();
        }
        let Some(nodes) = (target.Value as *const FABRIC_STRING_LIST).as_ref() else {
            return Vec::new();
        };
        raw_slice(nodes.Items, nodes.Count)
            .iter()
            .map(|node| WString::from(*node))
            .collect()
    }
}

fn repair_node_impacts(task: &FABRIC_REPAIR_TASK) -> Vec<RepairNodeImpact> {
    unsafe {
        let Some(impact) = task.Impact.as_ref() else {
            return Vec::new();
        };
        if impact.Kind != FABRIC_REPAIR_IMPACT_KIND::FABRIC_REPAIR_IMPACT_KIND_NODE {
            return Vec::new();
        }
        let Some(nodes) = (impact.Value as *const FABRIC_REPAIR_NODE_IMPACT_LIST).as_ref() else {
            return Vec::new();
        };
        raw_slice(nodes.Items, nodes.Count)
            .iter()
            .map(|node: &FABRIC_REPAIR_NODE_IMPACT| RepairNodeImpact {
                node_name: WString::from(node.NodeName),
                level: node.ImpactLevel.into(),
            })
            .collect()
    }
}

unsafe fn raw_slice<'a, T>(items: *const T, count: u32) -> &'a [T] {
    if items.is_null() || count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(items, count as usize) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mssf_com::FabricTypes::{
        FABRIC_REPAIR_IMPACT_DESCRIPTION, FABRIC_REPAIR_TARGET_DESCRIPTION,
        FABRIC_REPAIR_TASK_STATE, FABRIC_STRING_LIST,
    };

    #[test]
    fn in_flight_task_query_marshals_expected_filters() {
        let query = RepairTaskQueryDescription::in_flight();
        let mut pool = BoxPool::new();
        let raw = query.get_raw_with_pool(&mut pool);
        let scope = unsafe { raw.Scope.as_ref().unwrap() };

        assert_eq!(
            scope.Kind,
            FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND::FABRIC_REPAIR_SCOPE_IDENTIFIER_KIND_CLUSTER
        );
        assert_eq!(WString::from(raw.TaskIdFilter).to_string_lossy(), "Azure/");
        assert_eq!(
            raw.StateFilter,
            (FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_PREPARING.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_APPROVED.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_EXECUTING.0
                | FABRIC_REPAIR_TASK_STATE_FILTER::FABRIC_REPAIR_TASK_STATE_FILTER_RESTORING.0)
                as u32
        );
        assert!(raw.ExecutorFilter.is_null());
    }

    #[test]
    fn repair_task_filter_rejects_terminal_and_unknown_tasks() {
        let completed_id = WString::from("Azure/TenantUpdate/job/4/100");
        let completed = FABRIC_REPAIR_TASK {
            TaskId: completed_id.as_pcwstr(),
            State: FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_COMPLETED,
            ..Default::default()
        };
        let unknown_id = WString::from("Azure/Unknown/job/4/100");
        let unknown = FABRIC_REPAIR_TASK {
            TaskId: unknown_id.as_pcwstr(),
            State: FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_EXECUTING,
            ..Default::default()
        };

        assert!(RepairTask::from_raw(&completed).is_none());
        assert!(RepairTask::from_raw(&unknown).is_none());
    }

    #[test]
    fn repair_task_parser_accepts_all_parallel_job_types() {
        let cases = [
            ("PlatformUpdate", RepairTaskType::PlatformUpdate),
            ("PlatformMaintenance", RepairTaskType::PlatformMaintenance),
            ("TenantUpdate", RepairTaskType::TenantUpdate),
            ("TenantMaintenance", RepairTaskType::TenantMaintenance),
        ];

        for (task_type, expected) in cases {
            let task_id = format!("Azure/{task_type}/job/2/100");
            assert_eq!(
                parse_repair_task_id(&task_id),
                Some((expected, Some("2".to_string())))
            );
        }
    }

    #[test]
    fn repair_task_filter_returns_in_flight_update_with_nodes() {
        let task_id = WString::from("Azure/PlatformUpdate/job/2/100");
        let target_node = WString::from("_CtrlNode_2");
        let target_node_ptrs = [target_node.as_pcwstr()];
        let target_list = FABRIC_STRING_LIST {
            Count: 1,
            Items: target_node_ptrs.as_ptr(),
        };
        let target = FABRIC_REPAIR_TARGET_DESCRIPTION {
            Kind: FABRIC_REPAIR_TARGET_KIND::FABRIC_REPAIR_TARGET_KIND_NODE,
            Value: std::ptr::addr_of!(target_list).cast_mut().cast(),
        };
        let impact_node = FABRIC_REPAIR_NODE_IMPACT {
            NodeName: target_node.as_pcwstr(),
            ImpactLevel: FABRIC_REPAIR_NODE_IMPACT_LEVEL::FABRIC_REPAIR_NODE_IMPACT_LEVEL_RESTART,
            Reserved: std::ptr::null_mut(),
        };
        let impact_nodes = [impact_node];
        let impact_list = FABRIC_REPAIR_NODE_IMPACT_LIST {
            Count: 1,
            Items: impact_nodes.as_ptr(),
        };
        let impact = FABRIC_REPAIR_IMPACT_DESCRIPTION {
            Kind: FABRIC_REPAIR_IMPACT_KIND::FABRIC_REPAIR_IMPACT_KIND_NODE,
            Value: std::ptr::addr_of!(impact_list).cast_mut().cast(),
        };
        let task = FABRIC_REPAIR_TASK {
            TaskId: task_id.as_pcwstr(),
            State: FABRIC_REPAIR_TASK_STATE::FABRIC_REPAIR_TASK_STATE_PREPARING,
            Target: std::ptr::addr_of!(target),
            Impact: std::ptr::addr_of!(impact).cast_mut(),
            ..Default::default()
        };

        let task = RepairTask::from_raw(&task).unwrap();
        assert_eq!(
            task.task_id.to_string_lossy(),
            "Azure/PlatformUpdate/job/2/100"
        );
        assert_eq!(task.repair_type, RepairTaskType::PlatformUpdate);
        assert_eq!(task.state, RepairTaskState::Preparing);
        assert_eq!(task.upgrade_domain.as_deref(), Some("2"));
        assert_eq!(task.target_nodes[0].to_string_lossy(), "_CtrlNode_2");
        assert_eq!(task.node_impacts[0].level, RepairNodeImpactLevel::Restart);
    }
}
