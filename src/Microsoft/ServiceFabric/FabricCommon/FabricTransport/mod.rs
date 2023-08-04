#[inline]
pub unsafe fn CreateFabricTransportClient<'a, P0, P1, P2, P3>(
    interfaceid: *const ::windows::core::GUID,
    settings: *const FABRIC_TRANSPORT_SETTINGS,
    connectionaddress: P0,
    notificationhandler: P1,
    clienteventhandler: P2,
    messagedisposer: P3,
) -> ::windows::core::Result<IFabricTransportClient>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportCallbackMessageHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportClientEventHandler>>,
    P3: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessageDisposer>>,
{
    #[link(name = "fabrictransport")]
    extern "system" {
        fn CreateFabricTransportClient(
            interfaceid: *const ::windows::core::GUID,
            settings: *const FABRIC_TRANSPORT_SETTINGS,
            connectionaddress: ::windows::core::PCWSTR,
            notificationhandler: *mut ::core::ffi::c_void,
            clienteventhandler: *mut ::core::ffi::c_void,
            messagedisposer: *mut ::core::ffi::c_void,
            client: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateFabricTransportClient(
        ::core::mem::transmute(interfaceid),
        ::core::mem::transmute(settings),
        connectionaddress.into(),
        notificationhandler.into().abi(),
        clienteventhandler.into().abi(),
        messagedisposer.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<IFabricTransportClient>(result__)
}
#[inline]
pub unsafe fn CreateFabricTransportListener<'a, P0, P1, P2>(
    interfaceid: *const ::windows::core::GUID,
    settings: *const FABRIC_TRANSPORT_SETTINGS,
    address: *const FABRIC_TRANSPORT_LISTEN_ADDRESS,
    requesthandler: P0,
    connectionhandler: P1,
    disposeprocessor: P2,
) -> ::windows::core::Result<IFabricTransportListener>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessageHandler>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportConnectionHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessageDisposer>>,
{
    #[link(name = "fabrictransport")]
    extern "system" {
        fn CreateFabricTransportListener(
            interfaceid: *const ::windows::core::GUID,
            settings: *const FABRIC_TRANSPORT_SETTINGS,
            address: *const FABRIC_TRANSPORT_LISTEN_ADDRESS,
            requesthandler: *mut ::core::ffi::c_void,
            connectionhandler: *mut ::core::ffi::c_void,
            disposeprocessor: *mut ::core::ffi::c_void,
            listener: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateFabricTransportListener(
        ::core::mem::transmute(interfaceid),
        ::core::mem::transmute(settings),
        ::core::mem::transmute(address),
        requesthandler.into().abi(),
        connectionhandler.into().abi(),
        disposeprocessor.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<IFabricTransportListener>(result__)
}
#[repr(transparent)]
pub struct IFabricTransportCallbackMessageHandler(::windows::core::IUnknown);
impl IFabricTransportCallbackMessageHandler {
    pub unsafe fn HandleOneWay<'a, P0>(&self, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
    {
        (::windows::core::Vtable::vtable(self).HandleOneWay)(
            ::windows::core::Vtable::as_raw(self),
            message.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricTransportCallbackMessageHandler,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricTransportCallbackMessageHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportCallbackMessageHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportCallbackMessageHandler {}
impl ::core::fmt::Debug for IFabricTransportCallbackMessageHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportCallbackMessageHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportCallbackMessageHandler {
    type Vtable = IFabricTransportCallbackMessageHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportCallbackMessageHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ba8ac7a_3464_4774_b9b9_1d7f0f1920ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportCallbackMessageHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleOneWay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricTransportClient(::windows::core::IUnknown);
impl IFabricTransportClient {
    pub unsafe fn BeginRequest<'a, P0, P1>(
        &self,
        message: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRequest)(
            ::windows::core::Vtable::as_raw(self),
            message.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRequest<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<IFabricTransportMessage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndRequest)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransportMessage>(result__)
    }
    pub unsafe fn Send<'a, P0>(&self, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
    {
        (::windows::core::Vtable::vtable(self).Send)(
            ::windows::core::Vtable::as_raw(self),
            message.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginOpen<'a, P0>(
        &self,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginClose<'a, P0>(
        &self,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginClose)(
            ::windows::core::Vtable::as_raw(self),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndClose<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndClose)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricTransportClient, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportClient {}
impl ::core::fmt::Debug for IFabricTransportClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportClient")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportClient {
    type Vtable = IFabricTransportClient_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportClient {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5b0634fe_6a52_4bd9_8059_892c72c1d73a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportClient_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        reply: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Send: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginClose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndClose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricTransportClientConnection(::windows::core::IUnknown);
impl IFabricTransportClientConnection {
    pub unsafe fn Send<'a, P0>(&self, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
    {
        (::windows::core::Vtable::vtable(self).Send)(
            ::windows::core::Vtable::as_raw(self),
            message.into().abi(),
        )
        .ok()
    }
    pub unsafe fn get_ClientId(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self).get_ClientId)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricTransportClientConnection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportClientConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportClientConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportClientConnection {}
impl ::core::fmt::Debug for IFabricTransportClientConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportClientConnection")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportClientConnection {
    type Vtable = IFabricTransportClientConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportClientConnection {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa54c17f7_fe94_4838_b14d_e9b5c258e2d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportClientConnection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Send: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub get_ClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut u16,
}
#[repr(transparent)]
pub struct IFabricTransportClientEventHandler(::windows::core::IUnknown);
impl IFabricTransportClientEventHandler {
    pub unsafe fn OnConnected<'a, P0>(&self, connectionaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).OnConnected)(
            ::windows::core::Vtable::as_raw(self),
            connectionaddress.into(),
        )
        .ok()
    }
    pub unsafe fn OnDisconnected<'a, P0>(
        &self,
        connectionaddress: P0,
        error: ::windows::core::HRESULT,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).OnDisconnected)(
            ::windows::core::Vtable::as_raw(self),
            connectionaddress.into(),
            error,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricTransportClientEventHandler,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricTransportClientEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportClientEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportClientEventHandler {}
impl ::core::fmt::Debug for IFabricTransportClientEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportClientEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportClientEventHandler {
    type Vtable = IFabricTransportClientEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportClientEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4935ab6f_a8bc_4b10_a69e_7a3ba3324892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportClientEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionaddress: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionaddress: ::windows::core::PCWSTR,
        error: ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricTransportConnectionHandler(::windows::core::IUnknown);
impl IFabricTransportConnectionHandler {
    pub unsafe fn BeginProcessConnect<'a, P0, P1>(
        &self,
        clientconnection: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportClientConnection>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginProcessConnect)(
            ::windows::core::Vtable::as_raw(self),
            clientconnection.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndProcessConnect<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndProcessConnect)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginProcessDisconnect<'a, P0>(
        &self,
        clientid: *const u16,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginProcessDisconnect)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(clientid),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndProcessDisconnect<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndProcessDisconnect)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricTransportConnectionHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportConnectionHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportConnectionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportConnectionHandler {}
impl ::core::fmt::Debug for IFabricTransportConnectionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportConnectionHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportConnectionHandler {
    type Vtable = IFabricTransportConnectionHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportConnectionHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb069692d_e8f0_4f25_a3b6_b2992598a64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportConnectionHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginProcessConnect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        clientconnection: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndProcessConnect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginProcessDisconnect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        clientid: *const u16,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndProcessDisconnect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricTransportListener(::windows::core::IUnknown);
impl IFabricTransportListener {
    pub unsafe fn BeginOpen<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn BeginClose<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginClose)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndClose<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndClose)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricTransportListener, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportListener {}
impl ::core::fmt::Debug for IFabricTransportListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportListener")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportListener {
    type Vtable = IFabricTransportListener_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportListener {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1b63a266_1eeb_4f3e_8886_521458980d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportListener_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        serviceaddress: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginClose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndClose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricTransportMessage(::windows::core::IUnknown);
impl IFabricTransportMessage {
    pub unsafe fn GetHeaderAndBodyBuffer(
        &self,
        headerbuffer: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        msgbuffercount: *mut u32,
        msgbuffers: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
    ) {
        (::windows::core::Vtable::vtable(self).GetHeaderAndBodyBuffer)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(headerbuffer),
            ::core::mem::transmute(msgbuffercount),
            ::core::mem::transmute(msgbuffers),
        )
    }
    pub unsafe fn Dispose(&self) {
        (::windows::core::Vtable::vtable(self).Dispose)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricTransportMessage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportMessage {}
impl ::core::fmt::Debug for IFabricTransportMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportMessage")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportMessage {
    type Vtable = IFabricTransportMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportMessage {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb4357dab_ef06_465f_b453_938f3b0ad4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportMessage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHeaderAndBodyBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        headerbuffer: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
        msgbuffercount: *mut u32,
        msgbuffers: *mut *mut FABRIC_TRANSPORT_MESSAGE_BUFFER,
    ),
    pub Dispose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricTransportMessageDisposer(::windows::core::IUnknown);
impl IFabricTransportMessageDisposer {
    pub unsafe fn Dispose(&self, messages: &[::core::option::Option<IFabricTransportMessage>]) {
        (::windows::core::Vtable::vtable(self).Dispose)(
            ::windows::core::Vtable::as_raw(self),
            messages.len() as _,
            ::core::mem::transmute(messages.as_ptr()),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricTransportMessageDisposer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportMessageDisposer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportMessageDisposer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportMessageDisposer {}
impl ::core::fmt::Debug for IFabricTransportMessageDisposer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportMessageDisposer")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportMessageDisposer {
    type Vtable = IFabricTransportMessageDisposer_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportMessageDisposer {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x914097f3_a821_46ea_b3d9_feafe5f7c4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportMessageDisposer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Dispose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
        messages: *const *mut ::core::ffi::c_void,
    ),
}
#[repr(transparent)]
pub struct IFabricTransportMessageHandler(::windows::core::IUnknown);
impl IFabricTransportMessageHandler {
    pub unsafe fn BeginProcessRequest<'a, P0, P1>(
        &self,
        clientid: *const u16,
        message: P0,
        timeoutmilliseconds: u32,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginProcessRequest)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(clientid),
            message.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndProcessRequest<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<IFabricTransportMessage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndProcessRequest)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransportMessage>(result__)
    }
    pub unsafe fn HandleOneWay<'a, P0>(
        &self,
        clientid: *const u16,
        message: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransportMessage>>,
    {
        (::windows::core::Vtable::vtable(self).HandleOneWay)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(clientid),
            message.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricTransportMessageHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransportMessageHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransportMessageHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransportMessageHandler {}
impl ::core::fmt::Debug for IFabricTransportMessageHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransportMessageHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransportMessageHandler {
    type Vtable = IFabricTransportMessageHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransportMessageHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6815bdb4_1479_4c44_8b9d_57d6d0cc9d64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransportMessageHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginProcessRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        clientid: *const u16,
        message: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndProcessRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        reply: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub HandleOneWay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        clientid: *const u16,
        message: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct FABRIC_TRANSPORT_LISTEN_ADDRESS {
    pub IPAddressOrFQDN: ::windows::core::PCWSTR,
    pub Port: u32,
    pub Path: ::windows::core::PCWSTR,
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
unsafe impl ::windows::core::Abi for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_LISTEN_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<FABRIC_TRANSPORT_LISTEN_ADDRESS>(),
            ) == 0
        }
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
unsafe impl ::windows::core::Abi for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_MESSAGE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<FABRIC_TRANSPORT_MESSAGE_BUFFER>(),
            ) == 0
        }
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
unsafe impl ::windows::core::Abi for FABRIC_TRANSPORT_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FABRIC_TRANSPORT_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<FABRIC_TRANSPORT_SETTINGS>(),
            ) == 0
        }
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
