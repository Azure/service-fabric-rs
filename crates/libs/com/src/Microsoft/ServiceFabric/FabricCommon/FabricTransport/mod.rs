#[inline]
pub unsafe fn CreateFabricTransportClient<P0, P1, P2, P3>(
    interfaceid: *const ::windows_core::GUID,
    settings: *const FABRIC_TRANSPORT_SETTINGS,
    connectionaddress: P0,
    notificationhandler: P1,
    clienteventhandler: P2,
    messagedisposer: P3,
) -> ::windows_core::Result<IFabricTransportClient>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricTransportCallbackMessageHandler>,
    P2: ::windows_core::IntoParam<IFabricTransportClientEventHandler>,
    P3: ::windows_core::IntoParam<IFabricTransportMessageDisposer>,
{
    #[link(name = "fabrictransport")]
    extern "system" {
        pub fn CreateFabricTransportClient(
            interfaceid: *const ::windows_core::GUID,
            settings: *const FABRIC_TRANSPORT_SETTINGS,
            connectionaddress: ::windows_core::PCWSTR,
            notificationhandler: *mut ::core::ffi::c_void,
            clienteventhandler: *mut ::core::ffi::c_void,
            messagedisposer: *mut ::core::ffi::c_void,
            client: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    CreateFabricTransportClient(
        interfaceid,
        settings,
        connectionaddress.into_param().abi(),
        notificationhandler.into_param().abi(),
        clienteventhandler.into_param().abi(),
        messagedisposer.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateFabricTransportListener<P0, P1, P2>(
    interfaceid: *const ::windows_core::GUID,
    settings: *const FABRIC_TRANSPORT_SETTINGS,
    address: *const FABRIC_TRANSPORT_LISTEN_ADDRESS,
    requesthandler: P0,
    connectionhandler: P1,
    disposeprocessor: P2,
) -> ::windows_core::Result<IFabricTransportListener>
where
    P0: ::windows_core::IntoParam<IFabricTransportMessageHandler>,
    P1: ::windows_core::IntoParam<IFabricTransportConnectionHandler>,
    P2: ::windows_core::IntoParam<IFabricTransportMessageDisposer>,
{
    #[link(name = "fabrictransport")]
    extern "system" {
        pub fn CreateFabricTransportListener(
            interfaceid: *const ::windows_core::GUID,
            settings: *const FABRIC_TRANSPORT_SETTINGS,
            address: *const FABRIC_TRANSPORT_LISTEN_ADDRESS,
            requesthandler: *mut ::core::ffi::c_void,
            connectionhandler: *mut ::core::ffi::c_void,
            disposeprocessor: *mut ::core::ffi::c_void,
            listener: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    CreateFabricTransportListener(
        interfaceid,
        settings,
        address,
        requesthandler.into_param().abi(),
        connectionhandler.into_param().abi(),
        disposeprocessor.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
::windows_core::imp::com_interface!(
    IFabricTransportCallbackMessageHandler,
    IFabricTransportCallbackMessageHandler_Vtbl,
    0x9ba8ac7a_3464_4774_b9b9_1d7f0f1920ba
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransportCallbackMessageHandler,
    ::windows_core::IUnknown
);
impl IFabricTransportCallbackMessageHandler {
    pub unsafe fn HandleOneWay<P0>(&self, message: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
    {
        (::windows_core::Interface::vtable(self).HandleOneWay)(
            ::windows_core::Interface::as_raw(self),
            message.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricTransportCallbackMessageHandler {}
unsafe impl ::core::marker::Sync for IFabricTransportCallbackMessageHandler {}
#[repr(C)]
pub struct IFabricTransportCallbackMessageHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub HandleOneWay: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricTransportClient,
    IFabricTransportClient_Vtbl,
    0x5b0634fe_6a52_4bd9_8059_892c72c1d73a
);
::windows_core::imp::interface_hierarchy!(IFabricTransportClient, ::windows_core::IUnknown);
impl IFabricTransportClient {
    pub unsafe fn BeginRequest<P0, P1>(
        &self,
        message: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRequest)(
            ::windows_core::Interface::as_raw(self),
            message.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRequest<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<IFabricTransportMessage>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndRequest)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Send<P0>(&self, message: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
    {
        (::windows_core::Interface::vtable(self).Send)(
            ::windows_core::Interface::as_raw(self),
            message.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginOpen<P0>(
        &self,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginClose<P0>(
        &self,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginClose)(
            ::windows_core::Interface::as_raw(self),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndClose<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricTransportClient {}
unsafe impl ::core::marker::Sync for IFabricTransportClient {}
#[repr(C)]
pub struct IFabricTransportClient_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginRequest: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndRequest: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Send: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginClose: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndClose: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricTransportClientConnection,
    IFabricTransportClientConnection_Vtbl,
    0xa54c17f7_fe94_4838_b14d_e9b5c258e2d0
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransportClientConnection,
    ::windows_core::IUnknown
);
impl IFabricTransportClientConnection {
    pub unsafe fn Send<P0>(&self, message: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
    {
        (::windows_core::Interface::vtable(self).Send)(
            ::windows_core::Interface::as_raw(self),
            message.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn get_ClientId(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self).get_ClientId)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricTransportClientConnection {}
unsafe impl ::core::marker::Sync for IFabricTransportClientConnection {}
#[repr(C)]
pub struct IFabricTransportClientConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Send: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub get_ClientId: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> *mut u16,
}
::windows_core::imp::com_interface!(
    IFabricTransportClientEventHandler,
    IFabricTransportClientEventHandler_Vtbl,
    0x4935ab6f_a8bc_4b10_a69e_7a3ba3324892
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransportClientEventHandler,
    ::windows_core::IUnknown
);
impl IFabricTransportClientEventHandler {
    pub unsafe fn OnConnected<P0>(&self, connectionaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OnConnected)(
            ::windows_core::Interface::as_raw(self),
            connectionaddress.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnDisconnected<P0>(
        &self,
        connectionaddress: P0,
        error: ::windows_core::HRESULT,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OnDisconnected)(
            ::windows_core::Interface::as_raw(self),
            connectionaddress.into_param().abi(),
            error,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricTransportClientEventHandler {}
unsafe impl ::core::marker::Sync for IFabricTransportClientEventHandler {}
#[repr(C)]
pub struct IFabricTransportClientEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricTransportConnectionHandler,
    IFabricTransportConnectionHandler_Vtbl,
    0xb069692d_e8f0_4f25_a3b6_b2992598a64c
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransportConnectionHandler,
    ::windows_core::IUnknown
);
impl IFabricTransportConnectionHandler {
    pub unsafe fn BeginProcessConnect<P0, P1>(
        &self,
        clientconnection: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricTransportClientConnection>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginProcessConnect)(
            ::windows_core::Interface::as_raw(self),
            clientconnection.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndProcessConnect<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndProcessConnect)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginProcessDisconnect<P0>(
        &self,
        clientid: *const u16,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginProcessDisconnect)(
            ::windows_core::Interface::as_raw(self),
            clientid,
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndProcessDisconnect<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndProcessDisconnect)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricTransportConnectionHandler {}
unsafe impl ::core::marker::Sync for IFabricTransportConnectionHandler {}
#[repr(C)]
pub struct IFabricTransportConnectionHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginProcessConnect: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndProcessConnect: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginProcessDisconnect: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const u16,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndProcessDisconnect: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricTransportListener,
    IFabricTransportListener_Vtbl,
    0x1b63a266_1eeb_4f3e_8886_521458980d10
);
::windows_core::imp::interface_hierarchy!(IFabricTransportListener, ::windows_core::IUnknown);
impl IFabricTransportListener {
    pub unsafe fn BeginOpen<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginClose<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginClose)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndClose<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricTransportListener {}
unsafe impl ::core::marker::Sync for IFabricTransportListener {}
#[repr(C)]
pub struct IFabricTransportListener_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginClose: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndClose: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricTransportMessage,
    IFabricTransportMessage_Vtbl,
    0xb4357dab_ef06_465f_b453_938f3b0ad4b5
);
::windows_core::imp::interface_hierarchy!(IFabricTransportMessage, ::windows_core::IUnknown);
impl IFabricTransportMessage {
    pub unsafe fn GetHeaderAndBodyBuffer(
        &self,
        headerbuffer: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        msgbuffercount: *mut u32,
        msgbuffers: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
    ) {
        (::windows_core::Interface::vtable(self).GetHeaderAndBodyBuffer)(
            ::windows_core::Interface::as_raw(self),
            headerbuffer,
            msgbuffercount,
            msgbuffers,
        )
    }
    pub unsafe fn Dispose(&self) {
        (::windows_core::Interface::vtable(self).Dispose)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricTransportMessage {}
unsafe impl ::core::marker::Sync for IFabricTransportMessage {}
#[repr(C)]
pub struct IFabricTransportMessage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetHeaderAndBodyBuffer: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        *mut u32,
        *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
    ),
    pub Dispose: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricTransportMessageDisposer,
    IFabricTransportMessageDisposer_Vtbl,
    0x914097f3_a821_46ea_b3d9_feafe5f7c4a9
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransportMessageDisposer,
    ::windows_core::IUnknown
);
impl IFabricTransportMessageDisposer {
    pub unsafe fn Dispose(&self, messages: &[::core::option::Option<IFabricTransportMessage>]) {
        (::windows_core::Interface::vtable(self).Dispose)(
            ::windows_core::Interface::as_raw(self),
            messages.len().try_into().unwrap(),
            ::core::mem::transmute(messages.as_ptr()),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricTransportMessageDisposer {}
unsafe impl ::core::marker::Sync for IFabricTransportMessageDisposer {}
#[repr(C)]
pub struct IFabricTransportMessageDisposer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Dispose:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const *mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricTransportMessageHandler,
    IFabricTransportMessageHandler_Vtbl,
    0x6815bdb4_1479_4c44_8b9d_57d6d0cc9d64
);
::windows_core::imp::interface_hierarchy!(IFabricTransportMessageHandler, ::windows_core::IUnknown);
impl IFabricTransportMessageHandler {
    pub unsafe fn BeginProcessRequest<P0, P1>(
        &self,
        clientid: *const u16,
        message: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginProcessRequest)(
            ::windows_core::Interface::as_raw(self),
            clientid,
            message.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndProcessRequest<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<IFabricTransportMessage>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndProcessRequest)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HandleOneWay<P0>(
        &self,
        clientid: *const u16,
        message: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransportMessage>,
    {
        (::windows_core::Interface::vtable(self).HandleOneWay)(
            ::windows_core::Interface::as_raw(self),
            clientid,
            message.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricTransportMessageHandler {}
unsafe impl ::core::marker::Sync for IFabricTransportMessageHandler {}
#[repr(C)]
pub struct IFabricTransportMessageHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginProcessRequest: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const u16,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndProcessRequest: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HandleOneWay: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const u16,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct FABRIC_TRANSPORT_LISTEN_ADDRESS {
    pub IPAddressOrFQDN: ::windows_core::PCWSTR,
    pub Port: u32,
    pub Path: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FABRIC_TRANSPORT_LISTEN_ADDRESS {}
impl ::core::clone::Clone for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FABRIC_TRANSPORT_LISTEN_ADDRESS")
            .field("IPAddressOrFQDN", &self.IPAddressOrFQDN)
            .field("Port", &self.Port)
            .field("Path", &self.Path)
            .finish()
    }
}
impl ::windows_core::TypeKind for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.IPAddressOrFQDN == other.IPAddressOrFQDN
            && self.Port == other.Port
            && self.Path == other.Path
    }
}
impl ::core::cmp::Eq for FABRIC_TRANSPORT_LISTEN_ADDRESS {}
impl ::core::default::Default for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FABRIC_TRANSPORT_MESSAGE_BUFFER {
    pub BufferSize: u32,
    pub Buffer: *mut u8,
}
impl ::core::marker::Copy for FABRIC_TRANSPORT_MESSAGE_BUFFER {}
impl ::core::clone::Clone for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FABRIC_TRANSPORT_MESSAGE_BUFFER")
            .field("BufferSize", &self.BufferSize)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::windows_core::TypeKind for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for FABRIC_TRANSPORT_MESSAGE_BUFFER {}
impl ::core::default::Default for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FABRIC_TRANSPORT_SETTINGS {
    pub OperationTimeoutInSeconds: u32,
    pub KeepAliveTimeoutInSeconds: u32,
    pub MaxMessageSize: u32,
    pub MaxConcurrentCalls: u32,
    pub MaxQueueSize: u32,
    pub SecurityCredentials: *const super::super::FABRIC_SECURITY_CREDENTIALS,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for FABRIC_TRANSPORT_SETTINGS {}
impl ::core::clone::Clone for FABRIC_TRANSPORT_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FABRIC_TRANSPORT_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FABRIC_TRANSPORT_SETTINGS")
            .field("OperationTimeoutInSeconds", &self.OperationTimeoutInSeconds)
            .field("KeepAliveTimeoutInSeconds", &self.KeepAliveTimeoutInSeconds)
            .field("MaxMessageSize", &self.MaxMessageSize)
            .field("MaxConcurrentCalls", &self.MaxConcurrentCalls)
            .field("MaxQueueSize", &self.MaxQueueSize)
            .field("SecurityCredentials", &self.SecurityCredentials)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::windows_core::TypeKind for FABRIC_TRANSPORT_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.OperationTimeoutInSeconds == other.OperationTimeoutInSeconds
            && self.KeepAliveTimeoutInSeconds == other.KeepAliveTimeoutInSeconds
            && self.MaxMessageSize == other.MaxMessageSize
            && self.MaxConcurrentCalls == other.MaxConcurrentCalls
            && self.MaxQueueSize == other.MaxQueueSize
            && self.SecurityCredentials == other.SecurityCredentials
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FABRIC_TRANSPORT_SETTINGS {}
impl ::core::default::Default for FABRIC_TRANSPORT_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
