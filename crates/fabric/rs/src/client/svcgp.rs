pub struct IFabricServiceGroupManagementClient4Wrap { c : :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceGroupManagementClient4 }
impl IFabricServiceGroupManagementClient4Wrap {
    pub fn CreateServiceGroup(
        &self,
        description: &::fabric_base::Microsoft::ServiceFabric::FABRIC_SERVICE_GROUP_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateServiceGroup(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginCreateServiceGroup(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn CreateServiceGroupFromTemplate2(
        &self,
        serviceGroupFromTemplateDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateServiceGroupFromTemplate2(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginCreateServiceGroupFromTemplate2(
                serviceGroupFromTemplateDescription,
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
    pub fn DeleteServiceGroup(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteServiceGroup(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteServiceGroup(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetServiceGroupDescription (& self , name : & u16 , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceGroupDescriptionResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetServiceGroupDescription(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetServiceGroupDescription(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn UpdateServiceGroup(
        &self,
        name: &u16,
        serviceGroupUpdateDescription : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndUpdateServiceGroup(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginUpdateServiceGroup(
                name,
                serviceGroupUpdateDescription,
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
