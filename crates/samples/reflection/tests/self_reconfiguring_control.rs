// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::collections::HashSet;
use std::time::Duration;

use mssf_core::WString;
use mssf_core::types::{
    DeleteServiceDescription, PartitionSchemeDescription, QueryServiceReplicaStatus,
    SelfReconfiguringServiceDescription, ServiceDescription, ServicePackageActivationMode,
    ServicePartitionQueryDescription, ServicePartitionQueryResultItem, ServicePartitionStatus,
    ServiceReplicaQueryDescription, ServiceReplicaQueryResultItem, Uri,
};
use prost::Message;
use samples_reflection::SELF_RECONFIGURING_SERVICE_TYPE_NAME;
use samples_reflection::control::ReplicaInitData;
use samples_reflection::grpc::ReflectionUrl;
use samples_reflection::grpc_control::proto::approval_event::Details;
use samples_reflection::grpc_control::proto::{
    ApprovalEvent, ApprovalKind, ConfigurationRequestId, SelfReconfiguringActivationState,
};
use samples_reflection::test_cluster::{Cluster, discover_partition_id, fabric_client};
use uuid::Uuid;

const APP_NAME: &str = "fabric:/ReflectionApp";
const INSTANCE_COUNT: usize = 3;
const SF_TIMEOUT: Duration = Duration::from_secs(30);

fn assert_request_id(request_id: Option<&ConfigurationRequestId>) {
    let request_id = request_id.expect("configuration callback must include request_id");
    assert!(request_id.generation_number > 0);
    assert!(request_id.sequence_number > 0);
}

fn validate_configuration_event(
    event: &ApprovalEvent,
    require_activated: bool,
) -> Option<HashSet<i64>> {
    match event.details.as_ref() {
        Some(Details::ConfigurationRequest(request)) => {
            assert_request_id(request.request_id.as_ref());
            None
        }
        Some(Details::ConfigurationChangeRequest(change)) => {
            assert_request_id(change.request_id.as_ref());
            assert_eq!(change.instances.len(), INSTANCE_COUNT);
            if require_activated {
                assert!(change.instances.iter().all(|instance| {
                    instance.requested_activation_state
                        == SelfReconfiguringActivationState::Activated as i32
                }));
            }
            Some(
                change
                    .instances
                    .iter()
                    .map(|instance| instance.instance_id)
                    .collect(),
            )
        }
        None => panic!("configuration callback must include typed details"),
    }
}

async fn wait_for_instances_ready(
    service_name: &Uri,
    partition_id: mssf_core::GUID,
) -> HashSet<i64> {
    let query = fabric_client().get_query_manager();
    let deadline = std::time::Instant::now() + Duration::from_secs(30);

    loop {
        let partition_desc = ServicePartitionQueryDescription {
            service_name: service_name.clone(),
            partition_id_filter: Some(partition_id),
        };
        let partition = query
            .get_partition_list(&partition_desc, SF_TIMEOUT, None)
            .await
            .ok()
            .and_then(|list| list.service_partitions.into_iter().next());

        if let Some(ServicePartitionQueryResultItem::SelfReconfiguring(partition)) = partition
            && partition.partition_status == ServicePartitionStatus::Ready
            && partition.instance_count == INSTANCE_COUNT as u32
        {
            let replica_desc = ServiceReplicaQueryDescription {
                partition_id,
                replica_id_or_instance_id_filter: None,
            };
            if let Ok(instances) = query
                .get_replica_list(&replica_desc, SF_TIMEOUT, None)
                .await
            {
                let ready = instances
                    .service_replicas
                    .into_iter()
                    .filter_map(|instance| match instance {
                        ServiceReplicaQueryResultItem::SelfReconfiguring(instance)
                            if instance.replica_status == QueryServiceReplicaStatus::Ready =>
                        {
                            Some(instance.instance_id)
                        }
                        _ => None,
                    })
                    .collect::<HashSet<_>>();
                if ready.len() == INSTANCE_COUNT {
                    return ready;
                }
            }
        }

        assert!(
            std::time::Instant::now() < deadline,
            "self-reconfiguring service did not reach {INSTANCE_COUNT} ready instances"
        );
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

async fn wait_for_instance_endpoints(partition_id: mssf_core::GUID) -> (HashSet<i64>, Vec<String>) {
    let query = fabric_client().get_query_manager();
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    let description = ServiceReplicaQueryDescription {
        partition_id,
        replica_id_or_instance_id_filter: None,
    };

    loop {
        if let Ok(instances) = query.get_replica_list(&description, SF_TIMEOUT, None).await {
            let endpoints = instances
                .service_replicas
                .into_iter()
                .filter_map(|instance| match instance {
                    ServiceReplicaQueryResultItem::SelfReconfiguring(instance)
                        if !instance.replica_address.is_empty() =>
                    {
                        let url = ReflectionUrl::parse(&instance.replica_address.to_string())
                            .expect("self-reconfiguring reflection URL")
                            .grpc_connect_url();
                        Some((instance.instance_id, url))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            if endpoints.len() == INSTANCE_COUNT {
                return (
                    endpoints.iter().map(|(id, _)| *id).collect(),
                    endpoints.into_iter().map(|(_, url)| url).collect(),
                );
            }
        }

        assert!(
            std::time::Instant::now() < deadline,
            "self-reconfiguring instances did not publish control endpoints"
        );
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn approve_self_reconfiguring_lifecycle() {
    let suffix = Uuid::new_v4().simple().to_string();
    let service_name = Uri::from(format!("{APP_NAME}/SelfReconfig_{suffix}").as_str());
    let initialization_data = ReplicaInitData { control: true }.encode_to_vec();
    let description = ServiceDescription::SelfReconfiguring(
        SelfReconfiguringServiceDescription::new(
            Uri::from(APP_NAME),
            service_name.clone(),
            WString::from(SELF_RECONFIGURING_SERVICE_TYPE_NAME),
            PartitionSchemeDescription::Singleton,
        )
        .with_initialization_data(initialization_data)
        .with_instance_count(INSTANCE_COUNT as i32)
        .with_min_instance_count(INSTANCE_COUNT as i32)
        .with_service_activation_mode(ServicePackageActivationMode::ExclusiveProcess),
    );

    fabric_client()
        .get_service_manager()
        .create_service(&description, SF_TIMEOUT, None)
        .await
        .expect("create self-reconfiguring service");

    let partition_id_text = discover_partition_id(fabric_client(), &service_name).await;
    let partition_uuid = Uuid::parse_str(&partition_id_text).expect("partition GUID");
    let partition_id = mssf_core::GUID::from_u128(partition_uuid.as_u128());
    let (opened, control_urls) = wait_for_instance_endpoints(partition_id).await;
    let mut cluster = Cluster::with_urls(control_urls);
    let mut driver = cluster.partition_driver(partition_id_text);

    let mut configured = HashSet::new();
    let mut configured_members = None;

    while configured.len() < INSTANCE_COUNT {
        let (node, event) = driver.wait_next().await;
        let target = event.target.clone().expect("approval target");
        let kind = ApprovalKind::try_from(event.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);

        match kind {
            ApprovalKind::ApprovalRequestConfiguration => {
                validate_configuration_event(&event, true);
            }
            ApprovalKind::ApprovalRequestConfigurationChange => {
                let members = validate_configuration_event(&event, true)
                    .expect("configuration-change members");
                if let Some(previous) = configured_members.as_ref() {
                    assert_eq!(previous, &members);
                }
                configured_members = Some(members);
                configured.insert(target.replica_id);
            }
            other => panic!("unexpected activation callback: {other:?}"),
        }

        driver
            .approve_proceed(node, target, event.gate_id.clone())
            .await;
    }

    assert_eq!(opened, configured);
    assert_eq!(
        configured_members.expect("configuration-change members"),
        opened
    );
    assert_eq!(
        wait_for_instances_ready(&service_name, partition_id).await,
        opened
    );

    let delete_handle = {
        let manager = fabric_client().get_service_manager().clone();
        let description = DeleteServiceDescription::new(service_name.clone());
        tokio::spawn(async move {
            manager
                .delete_service2(&description, SF_TIMEOUT, None)
                .await
        })
    };

    let mut terminated = HashSet::new();
    while terminated.len() < INSTANCE_COUNT {
        let (node, event) = driver.wait_next().await;
        let target = event.target.clone().expect("approval target");
        let kind = ApprovalKind::try_from(event.kind).unwrap_or(ApprovalKind::ApprovalUnspecified);

        match kind {
            ApprovalKind::ApprovalRequestConfiguration
            | ApprovalKind::ApprovalRequestConfigurationChange => {
                validate_configuration_event(&event, false);
            }
            ApprovalKind::ApprovalClose | ApprovalKind::ApprovalAbort => {
                terminated.insert(target.replica_id);
            }
            other => panic!("unexpected teardown callback: {other:?}"),
        }

        driver
            .approve_proceed(node, target, event.gate_id.clone())
            .await;
    }

    delete_handle
        .await
        .expect("delete task panicked")
        .expect("delete self-reconfiguring service");
    assert_eq!(terminated, opened);
}
