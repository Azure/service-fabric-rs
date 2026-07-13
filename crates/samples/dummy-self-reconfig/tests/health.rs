// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    WString,
    client::FabricClient,
    types::{
        ApplicationHealthQueryDescription, HealthState, PartitionHealthQueryDescription,
        ReplicaHealthState, ServiceHealthQueryDescription, Uri,
    },
};

const APP_URI: &str = "fabric:/DummySelfReconfigApp";
const SERVICE_URI: &str = "fabric:/DummySelfReconfigApp/DummySelfReconfigService";
const INSTANCE_COUNT: usize = 3;
const MAX_ATTEMPTS: usize = 30;
const RETRY_DELAY: Duration = Duration::from_secs(2);

async fn verify_health(client: &FabricClient) -> Result<(), String> {
    let health = client.get_health_manager();
    let timeout = Duration::from_secs(10);

    let application = health
        .get_application_health(
            &ApplicationHealthQueryDescription {
                application_name: Uri::from(APP_URI),
                ..Default::default()
            },
            timeout,
            None,
        )
        .await
        .map_err(|err| format!("application health query failed: {err}"))?;
    if application.aggregated_health_state != HealthState::Ok {
        return Err(format!(
            "application health is {:?}",
            application.aggregated_health_state
        ));
    }
    if application.service_health_states.len() != 1 {
        return Err(format!(
            "expected one service, found {}",
            application.service_health_states.len()
        ));
    }

    let service_state = &application.service_health_states[0];
    if service_state.service_name != Uri::from(SERVICE_URI)
        || service_state.aggregated_health_state != HealthState::Ok
    {
        return Err(format!("service is not healthy: {service_state:?}"));
    }

    let service = health
        .get_service_health(
            &ServiceHealthQueryDescription {
                service_name: Uri::from(SERVICE_URI),
                ..Default::default()
            },
            timeout,
            None,
        )
        .await
        .map_err(|err| format!("service health query failed: {err}"))?;
    if service.aggregated_health_state != HealthState::Ok
        || service.partition_health_states.len() != 1
    {
        return Err(format!(
            "service health is {:?} with {} partitions",
            service.aggregated_health_state,
            service.partition_health_states.len()
        ));
    }

    let partition_state = &service.partition_health_states[0];
    if partition_state.aggregated_health_state != HealthState::Ok {
        return Err(format!(
            "partition health is {:?}",
            partition_state.aggregated_health_state
        ));
    }

    let partition = health
        .get_partition_health(
            &PartitionHealthQueryDescription {
                partition_id: partition_state.partition_id,
                ..Default::default()
            },
            timeout,
            None,
        )
        .await
        .map_err(|err| format!("partition health query failed: {err}"))?;
    if partition.aggregated_health_state != HealthState::Ok
        || partition.replica_health_states.len() != INSTANCE_COUNT
        || !partition.replica_health_states.iter().all(|instance| {
            matches!(instance, ReplicaHealthState::SelfReconfiguring { .. })
                && instance.get_aggregated_health_state() == HealthState::Ok
        })
    {
        return Err(format!(
            "partition health is {:?}; expected {INSTANCE_COUNT} healthy self-reconfiguring \
             instances, found {:?}",
            partition.aggregated_health_state, partition.replica_health_states
        ));
    }

    Ok(())
}

#[tokio::test]
#[test_log::test]
async fn test_application_service_and_instances_are_healthy() {
    let client = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();

    let mut last_error = String::new();
    for _ in 0..MAX_ATTEMPTS {
        match verify_health(&client).await {
            Ok(()) => return,
            Err(err) => last_error = err,
        }
        tokio::time::sleep(RETRY_DELAY).await;
    }

    panic!("dummy self-reconfiguring service did not become healthy: {last_error}");
}
