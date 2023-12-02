pub struct IFabricServiceManagementClient6Wrap { c : :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceManagementClient6 }
impl IFabricServiceManagementClient6Wrap {
    pub fn CreateService(
        &self,
        description: &::fabric_base::Microsoft::ServiceFabric::FABRIC_SERVICE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateService(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginCreateService(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn CreateServiceFromTemplate2(
        &self,
        serviceFromTemplateDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateServiceFromTemplate2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginCreateServiceFromTemplate2(
                serviceFromTemplateDescription,
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
    pub fn DeleteService(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteService(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteService(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteService2(
        &self,
        deleteDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_DELETE_SERVICE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteService2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteService2(deleteDescription, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceDescription (& self , name : & u16 , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceDescriptionResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceDescription(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceDescription(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
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
    ) -> tokio::sync::oneshot::Receiver<
        ::windows_core::Result<
            ::fabric_base::Microsoft::ServiceFabric::FabricCommon::IFabricStringResult,
        >,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceManifest(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginGetServiceManifest(
                applicationTypeName,
                applicationTypeVersion,
                serviceManifestName,
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
    pub fn RegisterServiceNotificationFilter(
        &self,
        description : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<i64>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndRegisterServiceNotificationFilter(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginRegisterServiceNotificationFilter(
                description,
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
    pub fn RemoveReplica(
        &self,
        description: &::fabric_base::Microsoft::ServiceFabric::FABRIC_REMOVE_REPLICA_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndRemoveReplica(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginRemoveReplica(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn ResolveServicePartition (& self , name : & u16 , partitionKeyType : :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PARTITION_KEY_TYPE , partitionKey : & :: core :: ffi :: c_void , previousResult : & :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricResolvedServicePartitionResult , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricResolvedServicePartitionResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndResolveServicePartition(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginResolveServicePartition(
                name,
                partitionKeyType,
                partitionKey,
                previousResult,
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
    pub fn RestartReplica(
        &self,
        description: &::fabric_base::Microsoft::ServiceFabric::FABRIC_RESTART_REPLICA_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndRestartReplica(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginRestartReplica(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UnregisterServiceNotificationFilter(
        &self,
        filterId: i64,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUnregisterServiceNotificationFilter(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginUnregisterServiceNotificationFilter(
                filterId,
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
    pub fn UpdateService(
        &self,
        name: &u16,
        serviceUpdateDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUpdateService(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginUpdateService(
                name,
                serviceUpdateDescription,
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
}
