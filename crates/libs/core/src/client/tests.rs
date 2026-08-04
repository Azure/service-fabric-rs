// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// contains tests for generated fabric client

use std::time::Duration;

use crate::{WString, client::FabricClient, sync::SimpleCancelToken, types::Uri};
use mssf_com::FabricTypes::FABRIC_E_SERVICE_DOES_NOT_EXIST;

use crate::{
    client::svc_mgmt_client::PartitionKeyType,
    error::ErrorCode,
    types::{NodeQueryDescription, NodeStatusFilter, PagedQueryDescription},
};

#[tokio::test]
async fn test_fabric_client() {
    let c = FabricClient::builder()
        .with_connection_strings(vec![WString::from("localhost:19000")])
        .build()
        .unwrap();
    let qc = c.get_query_manager();
    let timeout = Duration::from_secs(1);
    let paging_status;
    {
        let desc = NodeQueryDescription {
            node_status_filter: NodeStatusFilter::Up,
            paged_query: PagedQueryDescription {
                continuation_token: None,
                max_results: Some(2),
            },
            ..Default::default()
        };
        let qc_cp = qc.clone();
        let list = tokio::spawn(async move {
            // make sure api is Send.
            qc_cp.get_node_list(&desc, timeout, None).await
        })
        .await
        .unwrap()
        .unwrap();
        println!("Nodes: {list:?}");
        paging_status = list.paging_status;
        assert_ne!(list.nodes.len(), 0);
    }
    // get more nodes using paging
    {
        let desc = NodeQueryDescription {
            node_status_filter: NodeStatusFilter::Up,
            paged_query: PagedQueryDescription {
                continuation_token: paging_status.map(|x| x.continuation_token),
                max_results: Some(2),
            },
            ..Default::default()
        };
        let list = qc.get_node_list(&desc, timeout, None).await.unwrap();
        println!("Nodes: {list:?}");
    }

    // get node but cancel
    {
        let desc = NodeQueryDescription {
            ..Default::default()
        };
        let token = SimpleCancelToken::new_boxed();
        let list = qc.get_node_list(&desc, timeout, Some(token.clone()));
        token.cancel();
        // It is possible that the request is already success before cancel.
        if let Err(err) = list.await {
            assert_eq!(err, ErrorCode::E_ABORT.into());
        }
    }

    // test deployed application / deployed service package health.
    // There is no "list deployed applications" query yet, so this just
    // exercises the new HealthClient APIs against the first node using a
    // well-known app name; a not-found style error is expected and fine when
    // EchoApp isn't provisioned in this environment.
    {
        let hc = c.get_health_manager();
        let nodes = qc
            .get_node_list(&NodeQueryDescription::default(), timeout, None)
            .await
            .unwrap();
        if let Some(node) = nodes.nodes.first() {
            let app_desc = crate::types::DeployedApplicationHealthQueryDescription {
                application_name: Uri::from("fabric:/EchoApp"),
                node_name: node.name.clone(),
                ..Default::default()
            };
            match hc
                .get_deployed_application_health(&app_desc, timeout, None)
                .await
            {
                Ok(health) => {
                    println!("Deployed application health: {health:?}");
                    let pkg_desc = crate::types::DeployedServicePackageHealthQueryDescription {
                        application_name: Uri::from("fabric:/EchoApp"),
                        node_name: node.name.clone(),
                        service_manifest_name: WString::from("EchoServicePkg"),
                        ..Default::default()
                    };
                    match hc
                        .get_deployed_service_package_health(&pkg_desc, timeout, None)
                        .await
                    {
                        Ok(pkg_health) => {
                            println!("Deployed service package health: {pkg_health:?}")
                        }
                        Err(e) => println!(
                            "Deployed service package health not available (expected if the service manifest name doesn't match): {e:?}"
                        ),
                    }
                }
                Err(e) => println!(
                    "Deployed application health not available (expected if EchoApp isn't deployed on this node): {e:?}"
                ),
            }
        }
    }

    let smgr = c.get_service_manager();
    // test resolve echo app
    {
        let res = smgr
            .resolve_service_partition(
                &Uri::from("fabric:/EchoApp/EchoAppService"),
                &PartitionKeyType::None,
                None,
                timeout,
                None,
            )
            .await;
        match res {
            Ok(ptt) => {
                println!("Info: {ptt:?}");
                let endpoints = ptt.endpoints;
                println!("Endpoints: {endpoints:?}");
            }
            Err(e) => {
                // If the app is not provisioned we validate the error.
                if cfg!(unix) {
                    // In linux ci the app is not healthy from day one.
                    // FABRIC_E_SERVICE_OFFLINE is the expected result.
                    // TODO: Investigate the ci.
                    assert!(
                        e.code() == crate::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0)
                            || e.code()
                                == crate::HRESULT(
                                    mssf_com::FabricTypes::FABRIC_E_SERVICE_OFFLINE.0
                                )
                    );
                } else {
                    assert_eq!(e.code(), crate::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0));
                    println!("EchoApp not provisioned. Skip validate.")
                }
            }
        }
    }

    // Test property client with error
    {
        let pc = c.get_property_manager();
        // Create a name that is invalid to force error, and check the error message is propagated.
        {
            let err = pc
                .create_name(&Uri::from("fabric:/bad?x=1"), timeout, None)
                .await
                .unwrap_err();
            assert_eq!(
                err.try_as_fabric_error_code().unwrap(),
                ErrorCode::FABRIC_E_INVALID_NAME_URI
            );
            assert_eq!(
                err.to_string(),
                "FABRIC_E_INVALID_NAME_URI (-2147017794): The name 'fabric:/bad?x=1' is invalid: character '?' is not supported."
            );
        }
    }
}
