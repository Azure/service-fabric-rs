pub struct IFabricApplicationManagementClient10Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationManagementClient10 }
impl Default for IFabricApplicationManagementClient10Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricApplicationManagementClient10Wrap {
    pub fn new() -> IFabricApplicationManagementClient10Wrap {
        IFabricApplicationManagementClient10Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationManagementClient10 > () , }
    }
    pub fn from_com(
        com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationManagementClient10,
    ) -> IFabricApplicationManagementClient10Wrap {
        IFabricApplicationManagementClient10Wrap { com }
    }
    pub fn CreateApplication(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_APPLICATION_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateApplication(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginCreateApplication(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteApplication(
        &self,
        applicationName: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteApplication(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteApplication(applicationName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteApplication2(
        &self,
        deleteDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_DELETE_APPLICATION_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteApplication2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteApplication2(deleteDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeployServicePackageToNode(
        &self,
        applicationTypeName: ::windows_core::PCWSTR,
        applicationTypeVersion: ::windows_core::PCWSTR,
        serviceManifestName: ::windows_core::PCWSTR,
        sharingPolicy: &::mssf_com::Microsoft::ServiceFabric::FABRIC_PACKAGE_SHARING_POLICY_LIST,
        nodeName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeployServicePackageToNode(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginDeployServicePackageToNode(
                applicationTypeName,
                applicationTypeVersion,
                serviceManifestName,
                sharingPolicy,
                nodeName,
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
    pub fn GetApplicationManifest(
        &self,
        applicationTypeName: ::windows_core::PCWSTR,
        applicationTypeVersion: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationManifest(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetApplicationManifest(
                applicationTypeName,
                applicationTypeVersion,
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
    }    pub fn GetApplicationUpgradeProgress (& self , applicationName : & u16 , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationUpgradeProgressResult2 >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetApplicationUpgradeProgress(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetApplicationUpgradeProgress(
                applicationName,
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
    pub fn MoveNextApplicationUpgradeDomain(
        &self,
        progress : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationUpgradeProgressResult2,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndMoveNextApplicationUpgradeDomain(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginMoveNextApplicationUpgradeDomain(progress, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn MoveNextApplicationUpgradeDomain2(
        &self,
        applicationName: &u16,
        nextUpgradeDomain: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndMoveNextApplicationUpgradeDomain2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginMoveNextApplicationUpgradeDomain2(
                applicationName,
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
    pub fn ProvisionApplicationType(
        &self,
        applicationBuildPath: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndProvisionApplicationType(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginProvisionApplicationType(
                applicationBuildPath,
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
    pub fn ProvisionApplicationType2(
        &self,
        description : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndProvisionApplicationType2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginProvisionApplicationType2(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn ProvisionApplicationType3(
        &self,
        description : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndProvisionApplicationType3(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginProvisionApplicationType3(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn RestartDeployedCodePackage(
        &self,
        restartCodePackageDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRestartDeployedCodePackage(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginRestartDeployedCodePackage(
                restartCodePackageDescription,
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
    pub fn RollbackApplicationUpgrade(
        &self,
        applicationName: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRollbackApplicationUpgrade(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginRollbackApplicationUpgrade(
                applicationName,
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
    pub fn UnprovisionApplicationType(
        &self,
        applicationTypeName: ::windows_core::PCWSTR,
        applicationTypeVersion: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUnprovisionApplicationType(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUnprovisionApplicationType(
                applicationTypeName,
                applicationTypeVersion,
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
    pub fn UnprovisionApplicationType2(
        &self,
        description : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUnprovisionApplicationType2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginUnprovisionApplicationType2(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UpdateApplication(
        &self,
        applicationUpdateDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpdateApplication(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUpdateApplication(
                applicationUpdateDescription,
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
    pub fn UpdateApplicationUpgrade(
        &self,
        description : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpdateApplicationUpgrade(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginUpdateApplicationUpgrade(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UpgradeApplication(
        &self,
        upgradeDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpgradeApplication(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginUpgradeApplication(upgradeDescription, timeoutMilliseconds, &callback)
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
