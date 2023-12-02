pub struct IFabricPropertyManagementClient2Wrap { c : :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2 }
impl IFabricPropertyManagementClient2Wrap {
    pub fn CreateName(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndCreateName(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe { self.c.BeginCreateName(name, timeoutMilliseconds, &callback) };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteName(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteName(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe { self.c.BeginDeleteName(name, timeoutMilliseconds, &callback) };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteProperty(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndDeleteProperty(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginDeleteProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn EnumerateProperties (& self , name : & u16 , includeValues : windows :: Win32 :: Foundation :: BOOLEAN , previousResult : & :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndEnumerateProperties(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginEnumerateProperties(
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
            rx2
        } else {
            rx
        }
    }    pub fn EnumerateSubNames (& self , name : & u16 , previousResult : & :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult , recursive : windows :: Win32 :: Foundation :: BOOLEAN , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndEnumerateSubNames(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginEnumerateSubNames(
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
            rx2
        } else {
            rx
        }
    }    pub fn GetProperty (& self , name : & u16 , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyValueResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetProperty(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }    pub fn GetPropertyMetadata (& self , name : & u16 , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> tokio :: sync :: oneshot :: Receiver < :: windows_core :: Result < :: fabric_base :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyMetadataResult >>{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndGetPropertyMetadata(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginGetPropertyMetadata(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn NameExists(
        &self,
        name: &u16,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<u8>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndNameExists(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe { self.c.BeginNameExists(name, timeoutMilliseconds, &callback) };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn PutCustomPropertyOperation(
        &self,
        name: &u16,
        propertyOperation : & :: fabric_base :: Microsoft :: ServiceFabric :: FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndPutCustomPropertyOperation(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c.BeginPutCustomPropertyOperation(
                name,
                propertyOperation,
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
    pub fn PutPropertyDouble(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: f64,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndPutPropertyDouble(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginPutPropertyDouble(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyGuid(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: &::windows_core::GUID,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndPutPropertyGuid(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginPutPropertyGuid(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyInt64(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: i64,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndPutPropertyInt64(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginPutPropertyInt64(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = tokio::sync::oneshot::channel();
            tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2");
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyWString(
        &self,
        name: &u16,
        propertyName: ::windows_core::PCWSTR,
        data: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> tokio::sync::oneshot::Receiver<::windows_core::Result<()>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.c.EndPutPropertyWString(ctx) };
            tx.send(res).expect("fail to send");
        });
        let ctx = unsafe {
            self.c
                .BeginPutPropertyWString(name, propertyName, data, timeoutMilliseconds, &callback)
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
