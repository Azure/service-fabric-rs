pub trait IFabricTransportCallbackMessageHandler_Impl: Sized {
    fn HandleOneWay(
        &self,
        message: &::core::option::Option<IFabricTransportMessage>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricTransportCallbackMessageHandler {}
impl IFabricTransportCallbackMessageHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportCallbackMessageHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportCallbackMessageHandler_Vtbl {
        unsafe extern "system" fn HandleOneWay<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportCallbackMessageHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleOneWay(::core::mem::transmute(&message)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleOneWay: HandleOneWay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportCallbackMessageHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportClient_Impl: Sized {
    fn BeginRequest(
        &self,
        message: &::core::option::Option<IFabricTransportMessage>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRequest(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricTransportMessage>;
    fn Send(
        &self,
        message: &::core::option::Option<IFabricTransportMessage>,
    ) -> ::windows::core::Result<()>;
    fn BeginOpen(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOpen(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginClose(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndClose(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn Abort(&self);
}
impl ::windows::core::RuntimeName for IFabricTransportClient {}
impl IFabricTransportClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportClient_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportClient_Vtbl {
        unsafe extern "system" fn BeginRequest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRequest(
                ::core::mem::transmute(&message),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRequest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            reply: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRequest(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reply, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn BeginOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOpen(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndOpen(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginClose(
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndClose(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Abort<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRequest: BeginRequest::<Identity, Impl, OFFSET>,
            EndRequest: EndRequest::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            BeginOpen: BeginOpen::<Identity, Impl, OFFSET>,
            EndOpen: EndOpen::<Identity, Impl, OFFSET>,
            BeginClose: BeginClose::<Identity, Impl, OFFSET>,
            EndClose: EndClose::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportClientConnection_Impl: Sized {
    fn Send(
        &self,
        message: &::core::option::Option<IFabricTransportMessage>,
    ) -> ::windows::core::Result<()>;
    fn get_ClientId(&self) -> *mut u16;
}
impl ::windows::core::RuntimeName for IFabricTransportClientConnection {}
impl IFabricTransportClientConnection_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportClientConnection_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportClientConnection_Vtbl {
        unsafe extern "system" fn Send<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClientConnection_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn get_ClientId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClientConnection_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut u16 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClientId()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, Impl, OFFSET>,
            get_ClientId: get_ClientId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportClientConnection as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportClientEventHandler_Impl: Sized {
    fn OnConnected(
        &self,
        connectionaddress: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
    fn OnDisconnected(
        &self,
        connectionaddress: &::windows::core::PCWSTR,
        error: ::windows::core::HRESULT,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricTransportClientEventHandler {}
impl IFabricTransportClientEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportClientEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportClientEventHandler_Vtbl {
        unsafe extern "system" fn OnConnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClientEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionaddress: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected(::core::mem::transmute(&connectionaddress))
                .into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportClientEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionaddress: ::windows::core::PCWSTR,
            error: ::windows::core::HRESULT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected(
                ::core::mem::transmute(&connectionaddress),
                ::core::mem::transmute_copy(&error),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportClientEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportConnectionHandler_Impl: Sized {
    fn BeginProcessConnect(
        &self,
        clientconnection: &::core::option::Option<IFabricTransportClientConnection>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProcessConnect(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginProcessDisconnect(
        &self,
        clientid: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProcessDisconnect(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricTransportConnectionHandler {}
impl IFabricTransportConnectionHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportConnectionHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportConnectionHandler_Vtbl {
        unsafe extern "system" fn BeginProcessConnect<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportConnectionHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            clientconnection: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProcessConnect(
                ::core::mem::transmute(&clientconnection),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProcessConnect<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportConnectionHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProcessConnect(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginProcessDisconnect<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportConnectionHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            clientid: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProcessDisconnect(
                ::core::mem::transmute_copy(&clientid),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProcessDisconnect<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportConnectionHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProcessDisconnect(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginProcessConnect: BeginProcessConnect::<Identity, Impl, OFFSET>,
            EndProcessConnect: EndProcessConnect::<Identity, Impl, OFFSET>,
            BeginProcessDisconnect: BeginProcessDisconnect::<Identity, Impl, OFFSET>,
            EndProcessDisconnect: EndProcessDisconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportConnectionHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportListener_Impl: Sized {
    fn BeginOpen(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOpen(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginClose(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndClose(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn Abort(&self);
}
impl ::windows::core::RuntimeName for IFabricTransportListener {}
impl IFabricTransportListener_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportListener_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportListener_Vtbl {
        unsafe extern "system" fn BeginOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOpen(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            serviceaddress: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOpen(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serviceaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginClose(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndClose(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Abort<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginOpen: BeginOpen::<Identity, Impl, OFFSET>,
            EndOpen: EndOpen::<Identity, Impl, OFFSET>,
            BeginClose: BeginClose::<Identity, Impl, OFFSET>,
            EndClose: EndClose::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportListener as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportMessage_Impl: Sized {
    fn GetHeaderAndBodyBuffer(
        &self,
        headerbuffer: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        msgbuffercount: *mut u32,
        msgbuffers: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
    );
    fn Dispose(&self);
}
impl ::windows::core::RuntimeName for IFabricTransportMessage {}
impl IFabricTransportMessage_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportMessage_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportMessage_Vtbl {
        unsafe extern "system" fn GetHeaderAndBodyBuffer<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            headerbuffer: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
            msgbuffercount: *mut u32,
            msgbuffers: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHeaderAndBodyBuffer(
                ::core::mem::transmute_copy(&headerbuffer),
                ::core::mem::transmute_copy(&msgbuffercount),
                ::core::mem::transmute_copy(&msgbuffers),
            )
        }
        unsafe extern "system" fn Dispose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Dispose()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHeaderAndBodyBuffer: GetHeaderAndBodyBuffer::<Identity, Impl, OFFSET>,
            Dispose: Dispose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportMessage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportMessageDisposer_Impl: Sized {
    fn Dispose(&self, count: u32, messages: *const ::core::option::Option<IFabricTransportMessage>);
}
impl ::windows::core::RuntimeName for IFabricTransportMessageDisposer {}
impl IFabricTransportMessageDisposer_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportMessageDisposer_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportMessageDisposer_Vtbl {
        unsafe extern "system" fn Dispose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessageDisposer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: u32,
            messages: *const *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Dispose(
                ::core::mem::transmute_copy(&count),
                ::core::mem::transmute_copy(&messages),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Dispose: Dispose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportMessageDisposer as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransportMessageHandler_Impl: Sized {
    fn BeginProcessRequest(
        &self,
        clientid: *const u16,
        message: &::core::option::Option<IFabricTransportMessage>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProcessRequest(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricTransportMessage>;
    fn HandleOneWay(
        &self,
        clientid: *const u16,
        message: &::core::option::Option<IFabricTransportMessage>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricTransportMessageHandler {}
impl IFabricTransportMessageHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransportMessageHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricTransportMessageHandler_Vtbl {
        unsafe extern "system" fn BeginProcessRequest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessageHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            clientid: *const u16,
            message: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProcessRequest(
                ::core::mem::transmute_copy(&clientid),
                ::core::mem::transmute(&message),
                ::core::mem::transmute_copy(&timeoutmilliseconds),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndProcessRequest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessageHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            reply: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndProcessRequest(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reply, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleOneWay<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransportMessageHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            clientid: *const u16,
            message: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleOneWay(
                ::core::mem::transmute_copy(&clientid),
                ::core::mem::transmute(&message),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginProcessRequest: BeginProcessRequest::<Identity, Impl, OFFSET>,
            EndProcessRequest: EndProcessRequest::<Identity, Impl, OFFSET>,
            HandleOneWay: HandleOneWay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransportMessageHandler as ::windows::core::Interface>::IID
    }
}
