pub struct IFabricServiceGroupManagementClient4Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceGroupManagementClient4 }
impl Default for IFabricServiceGroupManagementClient4Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricServiceGroupManagementClient4Wrap {
    pub fn new() -> IFabricServiceGroupManagementClient4Wrap {
        IFabricServiceGroupManagementClient4Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceGroupManagementClient4 > () , }
    }
    pub fn CreateServiceGroup(
        &self,
        description: &::mssf_com::Microsoft::ServiceFabric::FABRIC_SERVICE_GROUP_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateServiceGroup(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginCreateServiceGroup(description, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn CreateServiceGroupFromTemplate2(
        &self,
        serviceGroupFromTemplateDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateServiceGroupFromTemplate2(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginCreateServiceGroupFromTemplate2(
                serviceGroupFromTemplateDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn DeleteServiceGroup(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteServiceGroup(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteServiceGroup(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }    pub fn GetServiceGroupDescription (& self , name : & u16 , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricServiceGroupDescriptionResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetServiceGroupDescription(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginGetServiceGroupDescription(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn UpdateServiceGroup(
        &self,
        name: &u16,
        serviceGroupUpdateDescription : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndUpdateServiceGroup(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginUpdateServiceGroup(
                name,
                serviceGroupUpdateDescription,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
}
