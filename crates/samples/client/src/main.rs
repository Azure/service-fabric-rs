// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::IFabricQueryClient;
use mssf_com::FabricTypes::{FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_RESULT_ITEM};
use mssf_core::sync::wait::WaitableCallback;

fn main() -> mssf_core::Result<()> {
    println!("GetNodeCli");

    let c: IFabricQueryClient = mssf_core::API_TABLE
        .fabric_create_local_client3::<IFabricQueryClient>(None, None)
        .expect("cannot get localclient");

    let (token, callback) = WaitableCallback::channel();

    let querydescription = FABRIC_NODE_QUERY_DESCRIPTION::default();

    let ctx = unsafe {
        c.BeginGetNodeList(&querydescription, 1000, &callback)
            .expect("cannot get ctx")
    };

    // wait callback to be triggered
    token.wait();

    // note: there must be a variable to hold COM object, ortherwise it is released.
    // result.expect().get_NodeList() will give a released/garbage node description pointer.
    let result = unsafe { c.EndGetNodeList(&ctx) };
    let result_node_list = result.expect("endcall_failed");

    let nodes = unsafe { result_node_list.get_NodeList() };
    let node_count = unsafe { (*nodes).Count };
    let node_list = unsafe { (*nodes).Items };

    println!("node_count {node_count}");

    if !node_list.is_null() {
        let node: FABRIC_NODE_QUERY_RESULT_ITEM = unsafe { *node_list };
        println!(
            "node info: name: {}",
            mssf_core::WString::from(mssf_core::strings::WStringWrap::from(node.NodeName))
        );
    }

    Ok(())
}
