// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Integration tests covering partition / replica queries and
//! service create / update / delete operations against a deployed
//! `ReflectionApp`. Requires the app to be deployed on the local
//! onebox (see top-level test instructions).

use std::time::Duration;

use mssf_core::{
    GUID, WString,
    client::{
        FabricClient,
        svc_mgmt_client::{PartitionKeyType, ResolvedServicePartition},
    },
    types::{
        DeployedServiceReplicaDetailQueryResultValue, NamedPartitionSchemeDescription,
        QueryServiceReplicaStatus, ServiceNotificationFilterDescription,
        ServiceNotificationFilterFlags, ServicePartitionAccessStatus, ServicePartitionStatus, Uri,
    },
};

use samples_reflection::test_admin::{
    SVC_URI, TestClient, TestCreateUpdateClient, TestPartitionReplicaLayout,
};

// Requires app to be deployed on onebox.
// Uses fabric client to perform various actions for this service.
#[tokio::test]
#[test_log::test]
async fn test_partition_info() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let tc = TestClient::new(fc.clone());
    let timeout = Duration::from_secs(1);

    let (stateful, single) = tc.get_partition().await.unwrap();

    // TODO: not sure why state is unknown.
    // assert_eq!(stateful.health_state, HealthState::Ok);
    assert_eq!(stateful.partition_status, ServicePartitionStatus::Ready);
    assert_eq!(stateful.target_replica_set_size, 3);
    assert_eq!(stateful.min_replica_set_size, 2);
    assert_eq!(stateful.auxiliary_replica_count, 1);
    assert_ne!(single.id, GUID::zeroed());

    // test get replica info
    let (p, _, _) = tc.get_replicas(single.id).await.unwrap();
    assert_eq!(p.replica_status, QueryServiceReplicaStatus::Ready);
    assert_ne!(p.node_name, WString::new());

    let mgmt = fc.get_service_manager();

    // Test registering service notification filter with different names and flags.
    // It turns out the SF accepts any fabric uri and does not validate the existence of app or services.
    enum ExpectedResult {
        Success,
        Fail,
    }
    let test_table = vec![
        // Non-existing app with none existing servie should succeed.
        (
            "fabric:/NonExistenceApp/NonExistingService",
            ServiceNotificationFilterFlags::NamePrefix,
            ExpectedResult::Success,
        ),
        // Non-existing service for existing app should succeed, since service can be created later.
        (
            "fabric:/ReflectionApp/NonExistingService",
            ServiceNotificationFilterFlags::NamePrefix,
            ExpectedResult::Success,
        ),
        (
            "fabric:/ReflectionApp/NonExistingService",
            ServiceNotificationFilterFlags::None,
            ExpectedResult::Success,
        ),
        // App prefix should succeed.
        (
            "fabric:/ReflectionApp",
            ServiceNotificationFilterFlags::NamePrefix,
            ExpectedResult::Success,
        ),
        // App non-prefix should succeed.
        (
            "fabric:/ReflectionApp",
            ServiceNotificationFilterFlags::PrimaryOnly,
            ExpectedResult::Success,
        ),
        // Invalid app name should succeed.
        (
            "fabric:/Invalid!*App()",
            ServiceNotificationFilterFlags::NamePrefix,
            ExpectedResult::Success,
        ),
        // Not a fabric uri should fail
        (
            "InvalidUri",
            ServiceNotificationFilterFlags::NamePrefix,
            ExpectedResult::Fail,
        ),
    ];

    for (service_name, flags, expected) in test_table {
        let desc = ServiceNotificationFilterDescription {
            name: Uri::from(service_name),
            flags,
        };
        let res = mgmt
            .register_service_notification_filter(&desc, Duration::from_secs(5), None)
            .await;
        match expected {
            ExpectedResult::Success => {
                let filter_handle = res.unwrap_or_else(|_| {
                    panic!("registering filter for {service_name} should succeed")
                });
                mgmt.unregister_service_notification_filter(filter_handle, timeout, None)
                    .await
                    .unwrap();
            }
            ExpectedResult::Fail => {
                res.expect_err(&format!(
                    "registering filter for {service_name} should fail"
                ));
            }
        }
    }

    // register service notification filter
    let filter_handle = {
        let desc = ServiceNotificationFilterDescription {
            name: Uri::from(SVC_URI),
            flags: ServiceNotificationFilterFlags::NamePrefix,
        };
        // register takes more than 1 sec.
        mgmt.register_service_notification_filter(&desc, Duration::from_secs(10), None)
            .await
            .unwrap()
    };

    // resolve the service
    let (p_endpoint, _, _) = tc.resolve().await.unwrap();

    // Test gRPC connection to the service
    {
        use samples_reflection::grpc::ReflectionUrl;
        use samples_reflection::grpc::hello_world::greeter_client::GreeterClient;
        use samples_reflection::grpc::hello_world::{
            GetReplicasRequest, HelloRequest, ReplicaRole as ProtoRole,
        };

        // The endpoint address is the gRPC URL with query params
        let grpc_url = p_endpoint.address.to_string();
        let reflection_url =
            ReflectionUrl::parse(&grpc_url).expect("failed to parse reflection URL");
        assert_eq!(
            reflection_url.partition_id, single.id,
            "partition ID in endpoint URL should match SF partition"
        );
        let base_url = reflection_url.grpc_connect_url();

        let mut client = GreeterClient::connect(base_url.clone())
            .await
            .expect("failed to connect to gRPC server");

        // Test SayHello
        let response = client
            .say_hello(HelloRequest {
                name: "ServiceFabric".into(),
            })
            .await
            .expect("SayHello failed");
        assert_eq!(response.into_inner().message, "Hello ServiceFabric!");
        tracing::info!("SayHello succeeded");

        // Test GetReplicas - get all
        let response = client
            .get_replicas(GetReplicasRequest {
                partition_id: String::new(),
            })
            .await
            .expect("GetReplicas failed");
        let replicas = response.into_inner().replicas;
        tracing::info!("GetReplicas returned {} replicas", replicas.len());
        assert!(!replicas.is_empty(), "expected at least one replica");

        // Test GetReplicas - filter by partition ID
        let partition_id_str = format!("{:?}", single.id);
        let response = client
            .get_replicas(GetReplicasRequest {
                partition_id: partition_id_str,
            })
            .await
            .expect("GetReplicas with partition filter failed");
        let filtered = response.into_inner().replicas;
        tracing::info!(
            "GetReplicas (filtered) returned {} replicas",
            filtered.len()
        );
        assert!(
            !filtered.is_empty(),
            "expected at least one replica for partition"
        );
        // The primary endpoint was resolved, so there must be exactly one primary replica
        let primary_count = filtered
            .iter()
            .filter(|r| r.role == ProtoRole::Primary as i32)
            .count();
        assert_eq!(
            primary_count, 1,
            "expected exactly one primary replica in partition"
        );
    }

    // restart primary
    tc.restart_primary_wait_for_replica_id_change(single.id)
        .await;

    // resolve again the primary addr should change
    {
        let mut count = 0;
        loop {
            let res = tc.resolve().await;
            let p2_endpoint = match res {
                Ok((p2, _, _)) => p2,
                Err(_) => {
                    // not yet ready
                    count += 1;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            if p2_endpoint.address != p_endpoint.address {
                tracing::info!("addr updated after {count} retries");
                break;
            } else {
                // addr update might be slow.
                // This typically takes 8 seconds which includes service boot time.
                if count > 30 {
                    panic!("addr for primary is not changed {}", p2_endpoint.address);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            count += 1;
        }
    }
    // unregisters the notification
    mgmt.unregister_service_notification_filter(filter_handle, timeout, None)
        .await
        .unwrap();

    // resolve new addr
    let (p_endpoint, _, _) = tc.resolve().await.unwrap();
    // restart primary again and use brute force resolve.
    // This gives a comparison of perf of the notification based resolve and the brute force way.
    tc.restart_primary_wait_for_replica_id_change(single.id)
        .await;
    {
        let mut count = 0;
        let mut prev: Option<ResolvedServicePartition> = None;
        loop {
            let res = tc.resolve_with_prev(prev.as_ref()).await;
            let p2_endpoint_res = match res {
                Ok(p_res) => {
                    // save the prev result
                    prev = Some(p_res.clone());
                    tc.convert_resolve_results(p_res).map(|(p, _, _)| p)
                }
                Err(e) => Err(e),
            };

            match p2_endpoint_res {
                Ok(p2_endpoint) => {
                    if p2_endpoint.address != p_endpoint.address {
                        tracing::info!("addr updated after {count} retries");
                        break;
                    } else {
                        // addr update might be slow.
                        // This typically takes 8 seconds which includes service boot time.
                        if count > 30 {
                            panic!("addr for primary is not changed {}", p2_endpoint.address);
                        }
                    }
                }
                Err(_) => {
                    // retry in next loop
                    if count > 30 {
                        panic!("retry max limit reached");
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    }

    // Test get partition load
    let partition_load_info = tc.get_partition_loads(single.id).await.unwrap();
    let primary_loads = partition_load_info
        .primary_load_metric_reports
        .iter()
        .collect::<Vec<_>>();
    tracing::info!("Primary metric loads: {primary_loads:?}");
    let secondary_loads = partition_load_info
        .secondary_load_metric_reports
        .iter()
        .collect::<Vec<_>>();
    tracing::info!("Secondary metric loads: {secondary_loads:?}");

    // test get deployed service info after failover
    let (p, s1, _) = tc.get_replicas(single.id).await.unwrap();
    let deployed_replica_detail = tc
        .get_deployed_replica_detail(&p.node_name, single.id, p.replica_id)
        .await
        .expect("get deployed replica detail failed");
    let result = match deployed_replica_detail.value {
        DeployedServiceReplicaDetailQueryResultValue::Stateful(s) => s,
        _ => panic!("not stateful"),
    };
    // check write ad read status of primary is GRANTED
    assert_eq!(result.write_status, ServicePartitionAccessStatus::Granted);
    assert_eq!(result.read_status, ServicePartitionAccessStatus::Granted);

    // check write and read status of secondary is NOT_PRIMARY
    let deployed_replica_detail = tc
        .get_deployed_replica_detail(&s1.node_name, single.id, s1.replica_id)
        .await
        .expect("get deployed replica detail failed");
    let result = match deployed_replica_detail.value {
        DeployedServiceReplicaDetailQueryResultValue::Stateful(s) => s,
        _ => panic!("not stateful"),
    };
    assert_eq!(
        result.write_status,
        ServicePartitionAccessStatus::NotPrimary
    );
    assert_eq!(result.read_status, ServicePartitionAccessStatus::NotPrimary);
}

async fn test_service_create_delete(
    fc: &FabricClient,
    partition_scheme: &mssf_core::types::PartitionSchemeDescription,
    service_name: &Uri,
) {
    // TODO: get service first
    let tc = TestCreateUpdateClient::new(fc.clone());
    // create service
    tc.create_service(
        service_name,
        partition_scheme,
        TestPartitionReplicaLayout::Target1Min1,
    )
    .await;

    let key_type = match partition_scheme {
        mssf_core::types::PartitionSchemeDescription::Singleton => PartitionKeyType::None,
        mssf_core::types::PartitionSchemeDescription::Named(names) => {
            PartitionKeyType::String(names.get_ref().first().unwrap().clone())
        }
        mssf_core::types::PartitionSchemeDescription::Int64Range(data) => {
            PartitionKeyType::Int64(data.as_raw().LowKey)
        }
        mssf_core::types::PartitionSchemeDescription::Invalid => panic!("invalid partition scheme"),
    };

    // resolve until the service is ready
    let addrs = tc.resolve_service(service_name, key_type).await;
    tracing::info!("resolved service {addrs:?}");

    // delete service
    tc.delete_service(service_name).await;
}

#[tokio::test]
#[test_log::test]
async fn test_service_curd_singleton() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Singleton;
    let service_name = Uri::from("fabric:/ReflectionApp/CurdTestServiceSingleton");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
#[test_log::test]
async fn test_service_curd_named() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Named(
        NamedPartitionSchemeDescription::new(vec![WString::from("test")]),
    );
    let service_name = Uri::from("fabric:/ReflectionApp/CurdTestServiceNamed");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
#[test_log::test]
async fn test_service_curd_range() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Int64Range(
        mssf_core::types::UniformIn64PartitionSchemeDescription::new(1, 10, 100),
    );
    let service_name = Uri::from("fabric:/ReflectionApp/CurdTestServiceRange");
    test_service_create_delete(&fc, &partition_scheme, &service_name).await;
}

#[tokio::test]
#[test_log::test]
async fn test_service_reparition() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let tc = TestCreateUpdateClient::new(fc.clone());
    // Note: SF does not support 0 named partitions.
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Named(
        NamedPartitionSchemeDescription::new(vec![WString::from("1")]),
    );
    let service_name = Uri::from("fabric:/ReflectionApp/RepartitionTest");

    // create service
    tc.create_service(
        &service_name,
        &partition_scheme,
        TestPartitionReplicaLayout::Target1Min1,
    )
    .await;

    // resolve until the service is ready
    {
        let key_type = PartitionKeyType::String(WString::from("1"));
        let addrs = tc.resolve_service(&service_name, key_type).await;
        tracing::info!("resolved service {addrs:?}");
    }

    // Add a partition 2
    {
        tracing::info!("adding partition 2: {service_name:?}");
        let names_to_add = vec![WString::from("2")];
        let names_to_remove = vec![];
        tc.update_service_named_partitions(&service_name, names_to_add, names_to_remove)
            .await;
    }

    {
        let key_type = PartitionKeyType::String(WString::from("2"));
        let addrs = tc.resolve_service(&service_name, key_type).await;
        tracing::info!("resolved service partition 2 {addrs:?}");
    }

    // remove parition 1
    {
        tracing::info!("removing partition 1: {service_name:?}");
        let names_to_add = vec![];
        let names_to_remove = vec![WString::from("1")];
        tc.update_service_named_partitions(&service_name, names_to_add, names_to_remove)
            .await;
    }

    // delete service
    tc.delete_service(&service_name).await;
}

#[tokio::test]
#[test_log::test]
async fn test_service_repartition_and_query() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let tc = TestCreateUpdateClient::new(fc.clone());
    // Note: SF does not support 0 named partitions.
    let partition_scheme = mssf_core::types::PartitionSchemeDescription::Named(
        NamedPartitionSchemeDescription::new(vec![WString::from("1")]),
    );
    let service_name = Uri::from("fabric:/ReflectionApp/RepartitionQueryTest");

    // create service
    tc.create_service(
        &service_name,
        &partition_scheme,
        TestPartitionReplicaLayout::Target1Min1,
    )
    .await;

    const MAX_RETRY: u32 = 60;

    // Wait for partition "1" to be ready
    tc.wait_for_named_partitions(&service_name, &["1"], MAX_RETRY)
        .await;

    // Add partition "2"
    tracing::info!("adding partition 2: {service_name:?}");
    tc.update_service_named_partitions(&service_name, vec![WString::from("2")], vec![])
        .await;

    // Wait for both partitions "1" and "2" to be ready
    tc.wait_for_named_partitions(&service_name, &["1", "2"], MAX_RETRY)
        .await;

    // Remove partition "1"
    tracing::info!("removing partition 1: {service_name:?}");
    tc.update_service_named_partitions(&service_name, vec![], vec![WString::from("1")])
        .await;

    // Wait for only partition "2" to remain
    tc.wait_for_named_partitions(&service_name, &["2"], MAX_RETRY)
        .await;

    // Run add-5/remove-1 cycle 5 times.
    // Each iteration starts with 1 partition and ends with 5 new ones,
    // then the next iteration removes 4 to get back to 1 before repeating.
    let mut current_base = 2u32; // the single partition we start with
    for round in 0..5 {
        let new_start = current_base + 1;
        let new_names: Vec<WString> = (new_start..new_start + 5)
            .map(|i| WString::from(i.to_string()))
            .collect();
        let new_name_strs: Vec<String> = new_names.iter().map(|n| n.to_string()).collect();

        // Add 5 partitions
        tracing::info!(
            "round {round}: adding partitions {new_start}..{}: {service_name:?}",
            new_start + 4
        );
        tc.update_service_named_partitions(&service_name, new_names, vec![])
            .await;

        // Remove the old single partition
        let remove_name = current_base.to_string();
        tracing::info!("round {round}: removing partition {remove_name}: {service_name:?}");
        tc.update_service_named_partitions(&service_name, vec![], vec![WString::from(remove_name)])
            .await;

        // Wait for the 5 new partitions
        let expected_strs: Vec<&str> = new_name_strs.iter().map(|s| s.as_str()).collect();
        tc.wait_for_named_partitions(&service_name, &expected_strs, MAX_RETRY)
            .await;

        // Remove 4 partitions to leave only the last one for the next round
        let to_remove: Vec<WString> = (new_start..new_start + 4)
            .map(|i| WString::from(i.to_string()))
            .collect();
        tracing::info!(
            "round {round}: removing partitions {new_start}..{}: {service_name:?}",
            new_start + 3
        );
        tc.update_service_named_partitions(&service_name, vec![], to_remove)
            .await;

        current_base = new_start + 4;
        let remaining = current_base.to_string();
        tc.wait_for_named_partitions(&service_name, &[remaining.as_str()], MAX_RETRY)
            .await;

        tracing::info!("round {round}: completed, remaining partition: {remaining}");
    }

    // Cleanup
    tc.delete_service(&service_name).await;
}
