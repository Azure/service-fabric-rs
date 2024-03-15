pub struct IFabricPropertyManagementClient2Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2 }
impl Default for IFabricPropertyManagementClient2Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricPropertyManagementClient2Wrap {
    pub fn new() -> IFabricPropertyManagementClient2Wrap {
        IFabricPropertyManagementClient2Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2 > () , }
    }
    pub fn CreateName(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateName(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginCreateName(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn DeleteName(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteName(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteName(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn DeleteProperty(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteProperty(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }    pub fn EnumerateProperties (& self , name : & u16 , includeValues : windows :: Win32 :: Foundation :: BOOLEAN , previousResult : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndEnumerateProperties(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginEnumerateProperties(
                name,
                includeValues,
                previousResult,
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
    }    pub fn EnumerateSubNames (& self , name : & u16 , previousResult : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult , recursive : windows :: Win32 :: Foundation :: BOOLEAN , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndEnumerateSubNames(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginEnumerateSubNames(
                name,
                previousResult,
                recursive,
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
    }    pub fn GetProperty (& self , name : & u16 , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyValueResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetProperty(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginGetProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }    pub fn GetPropertyMetadata (& self , name : & u16 , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyMetadataResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPropertyMetadata(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginGetPropertyMetadata(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn NameExists(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<u8>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndNameExists(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginNameExists(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn PutCustomPropertyOperation(
        &self,
        name: &u16,
        propertyOperation : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutCustomPropertyOperation(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginPutCustomPropertyOperation(
                name,
                propertyOperation,
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
    pub fn PutPropertyDouble(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: f64,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyDouble(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginPutPropertyDouble(
                name,
                propertyName,
                data,
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
    pub fn PutPropertyGuid(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: &::windows_core::GUID,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyGuid(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginPutPropertyGuid(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn PutPropertyInt64(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: i64,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyInt64(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com
                .BeginPutPropertyInt64(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            crate::sync::FabricReceiver::new(rx2)
        } else {
            crate::sync::FabricReceiver::new(rx)
        }
    }
    pub fn PutPropertyWString(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyWString(ctx) };
            if tx.send(res).is_err() {
                debug_assert!(false, "Receiver is dropped.");
            }
        });
        let ctx = unsafe {
            self.com.BeginPutPropertyWString(
                name,
                propertyName,
                data,
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
