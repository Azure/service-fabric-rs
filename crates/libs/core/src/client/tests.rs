// contains tests for generated fabric client

use std::time::Duration;

use mssf_com::{FABRIC_E_SERVICE_DOES_NOT_EXIST, FABRIC_NODE_QUERY_DESCRIPTION};
use windows_core::HSTRING;

use crate::client::{
    query_client::{NodeQueryDescription, NodeStatusFilter},
    svc_mgmt_client::PartitionKeyType,
    FabricClient,
};

use super::gen::query::IFabricQueryClient10Wrap;

#[test]
fn test_cluster_sync() {
    let c = IFabricQueryClient10Wrap::new();

    let queryDescription = FABRIC_NODE_QUERY_DESCRIPTION {
        NodeNameFilter: windows_core::PCWSTR(std::ptr::null()),
        Reserved: std::ptr::null_mut(),
    };
    let future = c.GetNodeList(&queryDescription, 1000);

    let nodes = future.blocking_recv().unwrap();
    let list = unsafe { nodes.get_NodeList() };
    let node_count = unsafe { (*list).Count };
    assert_ne!(node_count, 0);
}

#[tokio::test]
async fn test_fabric_client() {
    let c = FabricClient::new();
    let qc = c.get_query_manager();
    let timeout = Duration::from_secs(1);
    {
        let desc = NodeQueryDescription {
            node_status_filter: NodeStatusFilter::Up,
            ..Default::default()
        };
        let list = qc.get_node_list(&desc, timeout).await.unwrap();

        let v = list.iter().collect::<Vec<_>>();
        assert_ne!(v.len(), 0);
        for n in v {
            println!("Node: {:?}", n)
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
                                == windows_core::HRESULT(mssf_com::FABRIC_E_SERVICE_OFFLINE.0)
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
