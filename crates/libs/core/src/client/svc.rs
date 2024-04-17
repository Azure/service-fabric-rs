pub struct IFabricServiceManagementClient6Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceManagementClient6 }
impl Default for IFabricServiceManagementClient6Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricServiceManagementClient6Wrap {
    pub fn new() -> IFabricServiceManagementClient6Wrap {
        IFabricServiceManagementClient6Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceManagementClient6 > () , }
    }
    pub fn CreateService(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_SERVICE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateService(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginCreateService(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn CreateServiceFromTemplate2(
        &self,
        serviceFromTemplateDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateServiceFromTemplate2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginCreateServiceFromTemplate2(
                serviceFromTemplateDescription,
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
    pub fn DeleteService(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteService(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteService(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteService2(
        &self,
        deleteDescription: &::mssf_com::Microsoft::ServiceFabric::FABRIC_DELETE_SERVICE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteService2(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteService2(deleteDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceDescription (& self , name : & u16 , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceDescriptionResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceDescription(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceDescription(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn GetServiceManifest(
        &self,
        applicationTypeName: ::windows_core::PCWSTR,
        applicationTypeVersion: ::windows_core::PCWSTR,
        serviceManifestName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<
        ::windows_core::Result<
            ::mssf_com::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceManifest(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginGetServiceManifest(
                applicationTypeName,
                applicationTypeVersion,
                serviceManifestName,
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
    pub fn RegisterServiceNotificationFilter(
        &self,
        description : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<i64>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRegisterServiceNotificationFilter(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginRegisterServiceNotificationFilter(
                description,
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
    pub fn RemoveReplica(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_REMOVE_REPLICA_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRemoveReplica(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRemoveReplica(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn ResolveServicePartition (& self , name : & u16 , partitionKeyType : :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_KEY_TYPE , partitionKey : & :: core :: ffi :: c_void , previousResult : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricResolvedServicePartitionResult , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricResolvedServicePartitionResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndResolveServicePartition(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginResolveServicePartition(
                name,
                partitionKeyType,
                partitionKey,
                previousResult,
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
    pub fn RestartReplica(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_RESTART_REPLICA_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndRestartReplica(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginRestartReplica(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn UnregisterServiceNotificationFilter(
        &self,
        filterId: i64,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUnregisterServiceNotificationFilter(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUnregisterServiceNotificationFilter(
                filterId,
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
    pub fn UpdateService(
        &self,
        name: &u16,
        serviceUpdateDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpdateService(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginUpdateService(
                name,
                serviceUpdateDescription,
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
}
