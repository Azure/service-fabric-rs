// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
use std::fs::{self};

use windows_bindgen::bindgen;

fn main() {
    let winmd = "./build/_deps/fabric_metadata-src/.windows/winmd/Microsoft.ServiceFabric.winmd";
    // create output dir if not exist
    fs::create_dir_all("crates/libs/com/src/Microsoft/ServiceFabric/").unwrap();

    {
        let out_file = "crates/libs/com/";

        let args = vec![
            "--in",
            winmd,
            "--in",
            "default",
            "--out",
            out_file,
            "--package",
            "--no-allow",
            "--reference",
            "windows,skip-root,Windows",
            "--filter",
        ];

        let filter_types = vec!["Microsoft.ServiceFabric.FabricTypes"];
        let filter_common = vec![
            "Microsoft.ServiceFabric.FabricCommon", // include fabric types
            // exclude functions
            "!Microsoft.ServiceFabric.FabricCommon.FabricDecryptText",
            "!Microsoft.ServiceFabric.FabricCommon.FabricDecryptValue",
            "!Microsoft.ServiceFabric.FabricCommon.FabricEncryptText",
            "!Microsoft.ServiceFabric.FabricCommon.FabricEncryptText2",
            "!Microsoft.ServiceFabric.FabricCommon.FabricEncryptValue",
            "!Microsoft.ServiceFabric.FabricCommon.FabricGetLastErrorMessage",
        ];

        let filter_runtime = vec![
            "Microsoft.ServiceFabric.FabricRuntime", // include fabric types
            // exclude functions
            "!Microsoft.ServiceFabric.FabricRuntime.FabricBeginCreateRuntime",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricBeginGetActivationContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricBeginGetCodePackageActivator",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricBeginGetNodeContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateKeyValueStoreReplica",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateKeyValueStoreReplica2",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateKeyValueStoreReplica3",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateKeyValueStoreReplica4",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateKeyValueStoreReplica5",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricCreateRuntime",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricEndCreateRuntime",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricEndGetActivationContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricEndGetCodePackageActivator",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricEndGetNodeContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricGetActivationContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricGetCodePackageActivator",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricGetNodeContext",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricLoadEseLocalStoreSettings",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricLoadReplicatorSettings",
            "!Microsoft.ServiceFabric.FabricRuntime.FabricLoadSecurityCredentials",
        ];

        let filter_client = vec![
            "Microsoft.ServiceFabric.FabricClient", // include fabric types
            // exclude functions
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateClient",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateClient2",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateClient3",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateLocalClient",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateLocalClient2",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateLocalClient3",
            "!Microsoft.ServiceFabric.FabricClient.FabricCreateLocalClient4",
            "!Microsoft.ServiceFabric.FabricClient.FabricGetDefaultRollingUpgradeMonitoringPolicy",
        ];

        bindgen(
            args.into_iter()
                .chain(filter_types.into_iter())
                .chain(filter_common.into_iter())
                .chain(filter_runtime.into_iter())
                .chain(filter_client.into_iter()),
        );
    }
}
