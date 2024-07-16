// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// contains tests for generated fabric client

use std::time::Duration;

use mssf_com::FabricTypes::FABRIC_E_SERVICE_DOES_NOT_EXIST;
use windows_core::HSTRING;

use crate::{
    client::{svc_mgmt_client::PartitionKeyType, FabricClient},
    types::{NodeQueryDescription, NodeStatusFilter, PagedQueryDescription},
};

#[tokio::test]
async fn test_fabric_client() {
    let c = FabricClient::new();
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
        let list = qc.get_node_list(&desc, timeout).await.unwrap();
        paging_status = list.get_paging_status();
        let v = list.iter().collect::<Vec<_>>();
        assert_ne!(v.len(), 0);
        for n in v {
            println!("Node: {:?}", n)
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
        let list = qc.get_node_list(&desc, timeout).await.unwrap();
        let v = list.iter().collect::<Vec<_>>();
        for n in v {
            println!("More Node: {:?}", n)
        }
    }

    let smgr = c.get_service_manager();
    // test resolve echo app
    {
        let res = smgr
            .resolve_service_partition(
                &HSTRING::from("fabric:/EchoApp/EchoAppService"),
                &PartitionKeyType::None,
                None,
                timeout,
            )
            .await;
        match res {
            Ok(ptt) => {
                let info = ptt.get_info();
                println!("Info: {:?}", info);
                let list = ptt.get_endpoint_list();
                let v = list.iter().collect::<Vec<_>>();
                println!("Endpoints: {:?}", v);
            }
            Err(e) => {
                // If the app is not provisioned we validate the error.
                if cfg!(unix) {
                    // In linux ci the app is not healthy from day one.
                    // FABRIC_E_SERVICE_OFFLINE is the expected result.
                    // TODO: Investigate the ci.
                    assert!(
                        e.code() == windows_core::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0)
                            || e.code()
                                == windows_core::HRESULT(
                                    mssf_com::FabricTypes::FABRIC_E_SERVICE_OFFLINE.0
                                )
                    );
                } else {
                    assert_eq!(
                        e.code(),
                        windows_core::HRESULT(FABRIC_E_SERVICE_DOES_NOT_EXIST.0)
                    );
                    println!("EchoApp not provisioned. Skip validate.")
                }
            }
        }
    }
}
