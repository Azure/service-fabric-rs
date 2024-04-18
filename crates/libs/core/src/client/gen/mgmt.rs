pub struct IFabricClusterManagementClient10Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterManagementClient10 }
impl Default for IFabricClusterManagementClient10Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricClusterManagementClient10Wrap {
    pub fn new() -> IFabricClusterManagementClient10Wrap {
        IFabricClusterManagementClient10Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterManagementClient10 > () , }
    }
    pub fn from_com(
        com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricClusterManagementClient10,
    ) -> IFabricClusterManagementClient10Wrap {
        IFabricClusterManagementClient10Wrap { com }
    }
    pub fn ActivateNode(
        &self,
        nodeName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndActivateNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginActivateNode(nodeName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeactivateNode(
        &self,
        nodeName: ::windows_core::PCWSTR,
        intent: ::mssf_com::Microsoft::ServiceFabric::FABRIC_NODE_DEACTIVATION_INTENT,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeactivateNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeactivateNode(nodeName, intent, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetClusterConfiguration(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterConfiguration(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterConfiguration(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetClusterConfiguration2(
        &self,
        apiVersion: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterConfiguration2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterConfiguration2(apiVersion, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetClusterConfigurationUpgradeStatus (& self , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricOrchestrationUpgradeStatusResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterConfigurationUpgradeStatus(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterConfigurationUpgradeStatus(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetClusterManifest(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterManifest(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterManifest(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetClusterManifest2(
        &self,
        queryDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetClusterManifest2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetClusterManifest2(queryDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetFabricUpgradeProgress (& self , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricUpgradeProgressResult2 >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetFabricUpgradeProgress(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetFabricUpgradeProgress(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetUpgradeOrchestrationServiceState(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetUpgradeOrchestrationServiceState(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetUpgradeOrchestrationServiceState(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetUpgradesPendingApproval(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetUpgradesPendingApproval(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetUpgradesPendingApproval(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn MoveNextFabricUpgradeDomain(
        &self,
        progress : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricUpgradeProgressResult2,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndMoveNextFabricUpgradeDomain(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginMoveNextFabricUpgradeDomain(progress, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn MoveNextFabricUpgradeDomain2(
        &self,
        nextUpgradeDomain: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndMoveNextFabricUpgradeDomain2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginMoveNextFabricUpgradeDomain2(
                nextUpgradeDomain,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn NodeStateRemoved(
        &self,
        nodeName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndNodeStateRemoved(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginNodeStateRemoved(nodeName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn ProvisionFabric(
        &self,
        codeFilepath: ::windows_core::PCWSTR,
        clusterManifestFilepath: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndProvisionFabric(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginProvisionFabric(
                codeFilepath,
                clusterManifestFilepath,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RecoverPartition(
        &self,
        partitionId: ::windows_core::GUID,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRecoverPartition(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRecoverPartition(partitionId, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RecoverPartitions(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRecoverPartitions(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRecoverPartitions(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RecoverServicePartitions(
        &self,
        serviceName: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRecoverServicePartitions(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRecoverServicePartitions(serviceName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RecoverSystemPartitions(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRecoverSystemPartitions(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRecoverSystemPartitions(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn ResetPartitionLoad(
        &self,
        partitionId: ::windows_core::GUID,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndResetPartitionLoad(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginResetPartitionLoad(partitionId, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RestartNode(
        &self,
        restartNodeDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_RESTART_NODE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRestartNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRestartNode(restartNodeDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RollbackFabricUpgrade(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRollbackFabricUpgrade(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRollbackFabricUpgrade(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn SetUpgradeOrchestrationServiceState (& self , state : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricUpgradeOrchestrationServiceStateResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndSetUpgradeOrchestrationServiceState(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginSetUpgradeOrchestrationServiceState(state, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn StartApprovedUpgrades(
        &self,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndStartApprovedUpgrades(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginStartApprovedUpgrades(timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn StartNode(
        &self,
        startNodeDescription: &::mssf_com::Microsoft::ServiceFabric::FABRIC_START_NODE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndStartNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginStartNode(startNodeDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn StopNode(
        &self,
        stopNodeDescription: &::mssf_com::Microsoft::ServiceFabric::FABRIC_STOP_NODE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndStopNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginStopNode(stopNodeDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn ToggleVerboseServicePlacementHealthReporting(
        &self,
        enabled: windows::Win32::Foundation::BOOLEAN,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe {
                self.com
                    .EndToggleVerboseServicePlacementHealthReporting(ctx)
            };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginToggleVerboseServicePlacementHealthReporting(
                enabled,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UnprovisionFabric(
        &self,
        codeVersion: ::windows_core::PCWSTR,
        configVersion: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUnprovisionFabric(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUnprovisionFabric(
                codeVersion,
                configVersion,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UpdateFabricUpgrade(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_UPGRADE_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpdateFabricUpgrade(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginUpdateFabricUpgrade(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UpgradeConfiguration(
        &self,
        startUpgradeDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_START_UPGRADE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpgradeConfiguration(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUpgradeConfiguration(
                startUpgradeDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UpgradeFabric(
        &self,
        upgradeDescription: &::mssf_com::Microsoft::ServiceFabric::FABRIC_UPGRADE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpgradeFabric(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginUpgradeFabric(upgradeDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
}
