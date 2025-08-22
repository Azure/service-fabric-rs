// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// contains tests for generated fabric client

use std::time::Duration;

use crate::{WString, client::FabricClient, sync::SimpleCancelToken};
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
        paging_status = list.get_paging_status();
        let v = list.iter().collect::<Vec<_>>();
        assert_ne!(v.len(), 0);
        for n in v {
            println!("Node: {n:?}")
        }
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
        let v = list.iter().collect::<Vec<_>>();
        for n in v {
            println!("More Node: {n:?}")
        }
    }

    // get node but cancel
    {
        let desc = NodeQueryDescription {
            ..Default::default()
        };
        let token = SimpleCancelToken::new_boxed();
        let list = qc.get_node_list(&desc, timeout, Some(token.clone()));
        token.cancel();
        let err = list.await.expect_err("request should be cancelled");
        assert_eq!(err, ErrorCode::E_ABORT.into());
    }

    let smgr = c.get_service_manager();
    // test resolve echo app
    {
        let res = smgr
            .resolve_service_partition(
                &WString::from("fabric:/EchoApp/EchoAppService"),
                &PartitionKeyType::None,
                None,
                timeout,
                None,
            )
            .await;
        match res {
            Ok(ptt) => {
                let info = ptt.get_info();
                println!("Info: {info:?}");
                let list = ptt.get_endpoint_list();
                let v = list.iter().collect::<Vec<_>>();
                println!("Endpoints: {v:?}");
            }
            Err(e) => {
                // If the app is not provisioned we validate the error.
                if cfg!(unix) {
                    // In linux ci the app is not healthy from day one.
                    // FABRIC_E_SERVICE_OFFLINE is the expected result.
                    // TODO: Investigate the ci.
                    assert!(
                        e.0 == crate::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0)
                            || e.0
                                == crate::HRESULT(
                                    mssf_com::FabricTypes::FABRIC_E_SERVICE_OFFLINE.0
                                )
                    );
                } else {
                    assert_eq!(e.0, crate::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0));
                    println!("EchoApp not provisioned. Skip validate.")
                }
            }
        }
    }
}
