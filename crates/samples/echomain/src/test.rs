// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_core::{
    ErrorCode, GUID, WString,
    client::{
        FabricClient, GatewayInformationResult, PropertyManagementClient, ServiceNotification,
        svc_mgmt_client::{
            PartitionKeyType, ResolvedServiceEndpoint, ResolvedServicePartitionInfo,
            ServiceEndpointRole, ServicePartitionKind,
        },
    },
    sync::NONE_CANCEL_TOKEN,
    types::{
        QueryServiceReplicaStatus, RemoveReplicaDescription, ServiceNotificationFilterDescription,
        ServiceNotificationFilterFlags, ServicePartitionInformation,
        ServicePartitionQueryDescription, ServicePartitionQueryResult, ServicePartitionStatus,
        ServiceReplicaQueryDescription, ServiceReplicaQueryResult, SingletonPartitionInfomation,
        StatelessServiceInstanceQueryResult, StatelessServicePartitionQueryResult, Uri,
    },
};

static ECHO_SVC_URI: &str = "fabric:/EchoApp/EchoAppService";
static MAX_RETRY_COUNT: i32 = 5;
static RETRY_DURATION_SHORT: Duration = Duration::from_secs(1);

// Test client for echo server.
pub struct EchoTestClient {
    fc: FabricClient,
    service_uri: WString,
    timeout: Duration,
}

impl EchoTestClient {
    pub fn new(fc: FabricClient) -> Self {
        Self {
            fc,
            service_uri: WString::from(ECHO_SVC_URI),
            timeout: Duration::from_secs(1),
        }
    }

    pub async fn get_partition(
        &self,
    ) -> (
        StatelessServicePartitionQueryResult,
        SingletonPartitionInfomation,
    ) {
        let qc = self.fc.get_query_manager();
        let desc = ServicePartitionQueryDescription {
            service_name: self.service_uri.clone(),
            partition_id_filter: None,
        };
        let list = qc
            .get_partition_list(&desc, self.timeout, NONE_CANCEL_TOKEN)
            .await
            .unwrap();
        // there is only one partition
        let p = list.iter().next().unwrap();
        let stateless = match p {
            ServicePartitionQueryResult::Stateless(s) => s,
            _ => panic!("not stateless"),
        };
        let info = stateless.clone().partition_information;
        let single = match info {
            ServicePartitionInformation::Singleton(s) => s,
            _ => panic!("not singleton"),
        };
        (stateless, single)
    }

    pub async fn get_replica(
        &self,
        partition_id: GUID,
    ) -> mssf_core::Result<StatelessServiceInstanceQueryResult> {
        let qc = self.fc.get_query_manager();
        let desc = ServiceReplicaQueryDescription {
            partition_id,
            replica_id_or_instance_id_filter: None,
        };
        let replicas = qc
            .get_replica_list(&desc, self.timeout, NONE_CANCEL_TOKEN)
            .await?;
        let replica_op = replicas.iter().next(); // only one replica
        match replica_op {
            Some(replica) => Ok(match replica {
                ServiceReplicaQueryResult::Stateless(s) => s,
                _ => panic!("not stateless"),
            }),
            // replica might be restarting
            None => Err(ErrorCode::E_FAIL.into()),
        }
    }

    pub async fn resolve(&self) -> (ResolvedServicePartitionInfo, ResolvedServiceEndpoint) {
        let mgmt = self.fc.get_service_manager();
        let resolved_partition = mgmt
            .resolve_service_partition(
                &self.service_uri,
                &PartitionKeyType::None,
                None,
                self.timeout,
                NONE_CANCEL_TOKEN,
            )
            .await
            .expect("resolve failed");
        let info = resolved_partition.get_info();
        let endpoints = resolved_partition
            .get_endpoint_list()
            .iter()
            .collect::<Vec<_>>();
        // only has 1 instance
        assert_eq!(endpoints.len(), 1);
        (info, endpoints.first().unwrap().clone())
    }
}

// Requires app to be deployed on onebox.
// Uses fabric client to perform various actions to the app.
#[tokio::test]
async fn test_fabric_client() {
    // channel for service notification
    let (sn_tx, mut sn_rx) = tokio::sync::mpsc::channel::<ServiceNotification>(1);
    // channel for client connection notification
    let (cc_tx, mut cc_rx) = tokio::sync::mpsc::channel::<GatewayInformationResult>(1);
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .with_on_service_notification(move |notification| {
            sn_tx
                .blocking_send(notification.clone())
                .expect("cannot send notification");
            Ok(())
        })
        .with_on_client_connect(move |gw| {
            cc_tx.blocking_send(gw.clone()).expect("cannot send");
            Ok(())
        })
        .with_on_client_disconnect(move |_| {
            // This is not invoked in this test. FabricClient does not invoke this on drop.
            panic!("client disconnected");
        })
        .with_client_role(mssf_core::types::ClientRole::Unknown)
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();

    let ec = EchoTestClient::new(fc.clone());

    let timeout = Duration::from_secs(1);
    let service_uri = WString::from(ECHO_SVC_URI);

    // Get partition info
    let (stateless, single) = ec.get_partition().await;
    assert_eq!(stateless.instance_count, 1);
    assert_eq!(stateless.partition_status, ServicePartitionStatus::Ready);
    // For some reason the state is unknown
    // assert_eq!(stateless.health_state, HealthState::Ok);
    assert_ne!(single.id, GUID::zeroed());

    // Connection event notification should be received since we already sent a request.
    let gw = cc_rx.try_recv().expect("notification not present");
    assert!(!gw.node_name.is_empty());

    // Get replica info
    let stateless_replica = ec.get_replica(single.id).await.unwrap();

    // TODO: health is unknown
    // assert_eq!(stateless.aggregated_health_state, HealthState::Ok);
    assert_eq!(
        stateless_replica.replica_status,
        QueryServiceReplicaStatus::Ready
    );
    assert_ne!(stateless_replica.node_name, WString::new());

    let mgmt = fc.get_service_manager();
    // register service notification filter
    let filter_handle = {
        let desc = ServiceNotificationFilterDescription {
            name: service_uri.clone(),
            flags: ServiceNotificationFilterFlags::NamePrefix,
        };
        // register takes more than 1 sec.
        mgmt.register_service_notification_filter(&desc, Duration::from_secs(10), NONE_CANCEL_TOKEN)
            .await
            .unwrap()
    };

    // try resolve the app
    let (info, endpoint) = ec.resolve().await;
    assert_eq!(info.partition_key_type, PartitionKeyType::None);
    assert_eq!(info.service_name, service_uri);
    assert_eq!(info.service_partition_kind, ServicePartitionKind::Singleton);
    assert_eq!(endpoint.role, ServiceEndpointRole::Stateless);

    // Restart the stateless instance by removing it.
    {
        let desc = RemoveReplicaDescription {
            node_name: stateless_replica.node_name,
            partition_id: single.id,
            replica_or_instance_id: stateless_replica.instance_id,
        };
        mgmt.remove_replica(&desc, timeout, NONE_CANCEL_TOKEN)
            .await
            .expect("Failed to remove replica");
    }

    // replica id should be changed eventually.
    let mut count = 0;
    loop {
        let res = ec.get_replica(single.id).await;
        if res.is_err() {
            continue; // replica might be down.
        }
        let replica2 = res.unwrap();
        if replica2.instance_id != stateless_replica.instance_id {
            break;
        } else {
            if count > MAX_RETRY_COUNT {
                panic!(
                    "replica id not changed after retry. original {}, new {}",
                    stateless_replica.instance_id, replica2.instance_id
                );
            }
            // replica has not changed yet.
            tokio::time::sleep(RETRY_DURATION_SHORT).await;
        }
        count += 1;
    }

    // check service notification is invoked because service addr is changed for
    // replica removal and recreation.
    for i in 0..MAX_RETRY_COUNT {
        match sn_rx.try_recv() {
            Ok(sn) => {
                assert_eq!(sn.partition_id, single.id);
                break;
            }
            Err(e) => {
                if e == tokio::sync::mpsc::error::TryRecvError::Disconnected {
                    panic!("channnel should not be closed");
                }
                if i == MAX_RETRY_COUNT {
                    panic!("notification not received");
                }
                tokio::time::sleep(RETRY_DURATION_SHORT).await;
            }
        };
    }

    // unregisters the notification
    mgmt.unregister_service_notification_filter(filter_handle, timeout, NONE_CANCEL_TOKEN)
        .await
        .unwrap();
}

async fn delete_property_if_exist(
    pc: &PropertyManagementClient,
    svc_uri: &Uri,
    property_name: &WString,
    timeout: Duration,
) {
    match pc
        .get_property_metadata(svc_uri, property_name, timeout, NONE_CANCEL_TOKEN)
        .await
    {
        Ok(_) => {
            // Property already exists, remove it.
            pc.delete_property(svc_uri, property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
        }
        Err(e) => {
            if e.try_as_fabric_error_code().unwrap() == ErrorCode::FABRIC_E_PROPERTY_DOES_NOT_EXIST
            {
                // Property does not exist, continue.
            } else {
                panic!("unexpected error: {e}");
            }
        }
    };
}

#[tokio::test]
async fn test_property_client() {
    let fc = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();

    let pc = fc.get_property_manager();

    let app_uri = Uri::from("fabric:/EchoApp");
    let svc_uri = Uri::from("fabric:/EchoApp/EchoAppService");
    let timeout = Duration::from_secs(5);
    // If app is deployed, the name should be present.
    {
        let exist = pc
            .name_exists(&app_uri, timeout, NONE_CANCEL_TOKEN)
            .await
            .unwrap();
        assert!(exist);
        let sub_names = pc
            .enumerate_sub_names(&app_uri, None, false, timeout, NONE_CANCEL_TOKEN)
            .await
            .unwrap();
        assert_eq!(
            sub_names.get_enumeration_status(),
            mssf_core::types::EnumerationStatus::ConsistentFinished
        );
        let names = sub_names.get_names().unwrap();
        assert!(names.contains(&svc_uri));
    }
    {
        let exist = pc
            .name_exists(&svc_uri, timeout, NONE_CANCEL_TOKEN)
            .await
            .unwrap();
        assert!(exist);
    }
    // create new property test
    // Clean up previous runs.
    for property_name in [
        WString::from("test_property_binary"),
        WString::from("test_property_int64"),
        WString::from("test_property_double"),
        WString::from("test_property_wstring"),
        WString::from("test_property_guid"),
    ] {
        delete_property_if_exist(pc, &svc_uri, &property_name, timeout).await;

        // Create a binary property and read it back.
        {
            let property_name = WString::from("test_property_binary");
            let value = WString::from("test_binary_value");
            pc.put_property_binary(
                &svc_uri,
                &property_name,
                value.to_string_lossy().as_bytes(),
                timeout,
                NONE_CANCEL_TOKEN,
            )
            .await
            .unwrap();
            let meta = pc
                .get_property_metadata(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            assert_eq!(meta.get_metadata().unwrap().name, svc_uri);
            let value_result = pc
                .get_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let (meta2, data) = value_result.get_named_property();
            assert_eq!(meta2.name, svc_uri);
            assert_eq!(meta2.value_size, data.len() as i32);
            assert_eq!(data, value.to_string_lossy().as_bytes());

            let data2 = value_result.get_value_as_binary().unwrap();
            assert_eq!(data2, value.to_string_lossy().as_bytes());
            assert_eq!(
                value_result
                    .get_value_as_double()
                    .err()
                    .unwrap()
                    .try_as_fabric_error_code()
                    .unwrap(),
                ErrorCode::E_INVALIDARG
            );
        }

        // Create an int64 property and read it back.
        {
            let property_name = WString::from("test_property_int64");
            let value = 1234567890_i64;
            pc.put_property_int64(&svc_uri, &property_name, value, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let meta = pc
                .get_property_metadata(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            assert_eq!(meta.get_metadata().unwrap().name, svc_uri);
            let value_result = pc
                .get_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let (meta2, data) = value_result.get_named_property();
            assert_eq!(meta2.name, svc_uri);
            assert_eq!(meta2.value_size, 8);
            assert_eq!(data.len(), 8);
            assert_eq!(value_result.get_value_as_int64().unwrap(), value);
            pc.delete_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
        }
        // create a double property and read it back.
        {
            let property_name = WString::from("test_property_double");
            let value = 1234.5678_f64;
            pc.put_property_double(&svc_uri, &property_name, value, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let value_result = pc
                .get_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let (meta2, data) = value_result.get_named_property();
            assert_eq!(meta2.name, svc_uri);
            assert_eq!(meta2.value_size, 8);
            assert_eq!(data.len(), 8);
            assert_eq!(value_result.get_value_as_double().unwrap(), value);
        }
        // Create a wstring property and read it back.
        {
            let property_name = WString::from("test_property_wstring");
            let value = WString::from("test_value_wstring");
            pc.put_property_wstring(&svc_uri, &property_name, &value, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let value_result = pc
                .get_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let (meta2, data) = value_result.get_named_property();
            assert_eq!(meta2.name, svc_uri);
            assert_eq!(meta2.value_size, (value.len() + 1) as i32 * 2); // +1 for null terminator
            assert_eq!(meta2.value_size, data.len() as i32);

            let data2 = value_result.get_value_as_wstring().unwrap();
            assert_eq!(data2, value);
        }
        // Create a guid property and read it back.
        {
            let property_name = WString::from("test_property_guid");
            let value = GUID::from_values(
                0x12345678,
                0x1234,
                0x5678,
                [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef],
            );
            pc.put_property_guid(&svc_uri, &property_name, &value, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let value_result = pc
                .get_property(&svc_uri, &property_name, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            let (meta2, data) = value_result.get_named_property();
            assert_eq!(meta2.name, svc_uri);
            assert_eq!(meta2.value_size, 16);
            assert_eq!(data.len(), 16);
            assert_eq!(value_result.get_value_as_guid().unwrap(), value);
        }

        // Create a non-existent name
        {
            let app_name2 = Uri::from("fabric:/EchoAppNonExistent");
            let svc_name2 = Uri::from("fabric:/EchoAppNonExistent/EchoAppServiceNonExistent");
            let property_name = WString::from("test_property_wstring");
            let value = WString::from("test_value_wstring2");

            let svc_name2_cp = svc_name2.clone();
            let pc_cp = pc.clone();
            if tokio::spawn(async move {
                pc_cp
                    .name_exists(&svc_name2_cp, timeout, NONE_CANCEL_TOKEN)
                    .await
                    .unwrap()
            })
            .await
            .unwrap()
            {
                delete_property_if_exist(pc, &svc_name2, &property_name, timeout).await;
                // If the name exists, delete it.
                let pc_cp = pc.clone();
                let svc_name2_cp = svc_name2.clone();
                tokio::spawn(async move {
                    pc_cp
                        .delete_name(&svc_name2_cp, timeout, NONE_CANCEL_TOKEN)
                        .await
                        .unwrap()
                })
                .await
                .unwrap();
            }
            if pc
                .name_exists(&app_name2, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap()
            {
                // If the name exists, delete it.
                pc.delete_name(&app_name2, timeout, NONE_CANCEL_TOKEN)
                    .await
                    .unwrap();
            }
            // Create a new name.
            let pc_cp = pc.clone();
            let svc_name2_cp = svc_name2.clone();
            tokio::spawn(async move {
                pc_cp
                    .create_name(&svc_name2_cp, timeout, NONE_CANCEL_TOKEN)
                    .await
                    .unwrap()
            })
            .await
            .unwrap();
            // Check if the name exists.
            let exist = pc
                .name_exists(&svc_name2, timeout, NONE_CANCEL_TOKEN)
                .await
                .unwrap();
            assert!(exist);
            // create a property under that name.
            let pc_cp = pc.clone();
            let svc_name2_cp = svc_name2.clone();
            let property_name_cp = property_name.clone();
            let value_cp = value.clone();
            tokio::spawn(async move {
                pc_cp
                    .put_property_wstring(
                        &svc_name2_cp,
                        &property_name_cp,
                        &value_cp,
                        timeout,
                        NONE_CANCEL_TOKEN,
                    )
                    .await
                    .unwrap()
            })
            .await
            .unwrap();
            let pc_cp = pc.clone();
            let res = tokio::spawn(async move {
                pc_cp
                    .get_property(&svc_name2, &property_name, timeout, NONE_CANCEL_TOKEN)
                    .await
                    .unwrap()
            })
            .await
            .unwrap();
            assert_eq!(res.get_value_as_wstring().unwrap(), value);
        }
    }
}
