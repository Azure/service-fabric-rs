pub struct IFabricApplicationManagementClient10Wrap { c : :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationManagementClient10 }
impl IFabricApplicationManagementClient10Wrap {
    pub fn CreateApplication(
        &self,
        description: &::fabric_base::Microsoft::ServiceFabric::FABRIC_APPLICATION_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateApplication(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginCreateApplication(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteApplication(
        &self,
        applicationName: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteApplication(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteApplication(applicationName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteApplication2(
        &self,
        deleteDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DELETE_APPLICATION_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteApplication2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteApplication2(deleteDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
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
        sharingPolicy: &::fabric_base::Microsoft::ServiceFabric::FABRIC_PACKAGE_SHARING_POLICY_LIST,
        nodeName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeployServicePackageToNode(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginDeployServicePackageToNode(
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
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
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
    ) -> tokio::sync::oneshot::Receiver<
        ::windows_core::Result<
            ::fabric_base::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationManifest(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetApplicationManifest(
                applicationTypeName,
                applicationTypeVersion,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetApplicationUpgradeProgress (& self , applicationName : & u16 , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationUpgradeProgressResult2 >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetApplicationUpgradeProgress(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetApplicationUpgradeProgress(
                applicationName,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn MoveNextApplicationUpgradeDomain(
        &self,
        progress : & :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricApplicationUpgradeProgressResult2,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndMoveNextApplicationUpgradeDomain(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginMoveNextApplicationUpgradeDomain(progress, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
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
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndMoveNextApplicationUpgradeDomain2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginMoveNextApplicationUpgradeDomain2(
                applicationName,
                nextUpgradeDomain,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn ProvisionApplicationType(
        &self,
        applicationBuildPath: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndProvisionApplicationType(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginProvisionApplicationType(
                applicationBuildPath,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn ProvisionApplicationType2(
        &self,
        description : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndProvisionApplicationType2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginProvisionApplicationType2(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn ProvisionApplicationType3(
        &self,
        description : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndProvisionApplicationType3(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginProvisionApplicationType3(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn RestartDeployedCodePackage(
        &self,
        restartCodePackageDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndRestartDeployedCodePackage(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginRestartDeployedCodePackage(
                restartCodePackageDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn RollbackApplicationUpgrade(
        &self,
        applicationName: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndRollbackApplicationUpgrade(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginRollbackApplicationUpgrade(applicationName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
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
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUnprovisionApplicationType(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginUnprovisionApplicationType(
                applicationTypeName,
                applicationTypeVersion,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UnprovisionApplicationType2(
        &self,
        description : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUnprovisionApplicationType2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginUnprovisionApplicationType2(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UpdateApplication(
        &self,
        applicationUpdateDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUpdateApplication(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginUpdateApplication(
                applicationUpdateDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UpdateApplicationUpgrade(
        &self,
        description : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUpdateApplicationUpgrade(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginUpdateApplicationUpgrade(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UpgradeApplication(
        &self,
        upgradeDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUpgradeApplication(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginUpgradeApplication(upgradeDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
}
