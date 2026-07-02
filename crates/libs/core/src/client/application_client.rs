// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::time::Duration;

use mssf_com::{
    FabricClient::{IFabricApplicationManagementClient, IFabricApplicationUpgradeProgressResult2},
    FabricTypes::FABRIC_URI,
};

use crate::{
    runtime::executor::BoxedCancelToken,
    sync::{FabricReceiver, fabric_begin_end_proxy},
    types::{ApplicationUpgradeProgress, Uri},
};

/// Provides functionality to perform application management operations, such as
/// querying application upgrade progress.
///
/// See C# API [here](https://learn.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.applicationmanagementclient?view=azure-dotnet).
///
/// We are only adding support for application upgrade progress for now - additional functionality can be added later.
#[derive(Debug, Clone)]
pub struct ApplicationManagementClient {
    com: IFabricApplicationManagementClient,
}

impl From<IFabricApplicationManagementClient> for ApplicationManagementClient {
    fn from(value: IFabricApplicationManagementClient) -> Self {
        Self { com: value }
    }
}

impl From<ApplicationManagementClient> for IFabricApplicationManagementClient {
    fn from(value: ApplicationManagementClient) -> Self {
        value.com
    }
}

// Internal implementation block - convert SF callbacks into async futures.
impl ApplicationManagementClient {
    fn get_application_upgrade_progress_internal(
        &self,
        application_name: FABRIC_URI,
        timeout_milliseconds: u32,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> FabricReceiver<crate::Result<IFabricApplicationUpgradeProgressResult2>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetApplicationUpgradeProgress(
                    application_name,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndGetApplicationUpgradeProgress(ctx) },
            cancellation_token,
        )
    }
}

// Public implementation block.
impl ApplicationManagementClient {
    /// Gets the upgrade progress for the specified application. 
    /// Equivalent of the `Get-ServiceFabricApplicationUpgrade` PowerShell cmdlet.
    ///
    /// The returned [`ApplicationUpgradeProgress`] includes the aggregate
    /// upgrade state as well as the per upgrade-domain status.
    ///
    /// Remarks: SF returns a valid result even for applications that are not
    /// currently upgrading; check [`ApplicationUpgradeProgress::is_active`] to
    /// determine whether an upgrade is in flight.
    pub async fn get_application_upgrade_progress(
        &self,
        application_name: &Uri,
        timeout: Duration,
        cancellation_token: Option<BoxedCancelToken>,
    ) -> crate::Result<ApplicationUpgradeProgress> {
        let com = self
            .get_application_upgrade_progress_internal(
                application_name.as_raw(),
                timeout.as_millis().try_into().unwrap(),
                cancellation_token,
            )
            .await??;
        Ok(ApplicationUpgradeProgress::from(&com))
    }
}
