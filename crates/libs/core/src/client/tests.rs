// contains tests for generated fabric client

use mssf_com::FABRIC_NODE_QUERY_DESCRIPTION;

use super::query::IFabricQueryClient10Wrap;

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
async fn test_get_node() {
    let c = IFabricQueryClient10Wrap::new();

    let handle = tokio::spawn(async move {
        let future;
        {
            let queryDescription = FABRIC_NODE_QUERY_DESCRIPTION {
                NodeNameFilter: windows_core::PCWSTR(std::ptr::null()),
                Reserved: std::ptr::null_mut(),
            };
            future = c.GetNodeList(&queryDescription, 1000);
        }
        let nodes = future.await.unwrap();
        let list = unsafe { nodes.get_NodeList() };
        let node_count = unsafe { (*list).Count };
        assert_ne!(node_count, 0);
    });

    handle.await.unwrap();
}
