// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::FabricClient::{FabricCreateLocalClient, IFabricQueryClient};
use mssf_com::FabricCommon::IFabricAsyncOperationCallback;
use mssf_com::FabricTypes::{FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_RESULT_ITEM};
use mssf_core::sync::wait::WaitableCallback;
use windows_core::Interface;

fn main() -> windows::core::Result<()> {
    println!("GetNodeCli");

    let rawclient = unsafe {
        FabricCreateLocalClient(&IFabricQueryClient::IID).expect("cannot get localclient")
    };
    // todo: figure out owner ship
    let c: IFabricQueryClient = unsafe { IFabricQueryClient::from_raw(rawclient) };

    let (token, callback) = WaitableCallback::channel();

    let callback_arg: IFabricAsyncOperationCallback = callback.cast().expect("castfailed");

    let querydescription = FABRIC_NODE_QUERY_DESCRIPTION::default();

    let ctx = unsafe {
        c.BeginGetNodeList(&querydescription, 1000, &callback_arg)
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

    println!("node_count {}", node_count);

    if !node_list.is_null() {
        let node: FABRIC_NODE_QUERY_RESULT_ITEM = unsafe { *node_list };
        // this is ugly
        // println!("node info: name: {:#?}", node);
        println!("node info: name: {}", unsafe { node.NodeName.display() });
    }

    Ok(())
}
