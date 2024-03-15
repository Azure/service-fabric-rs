#[cfg(feature = "ServiceFabric_FabricCommon_FabricClient")]
pub mod FabricClient;
#[cfg(feature = "ServiceFabric_FabricCommon_FabricRuntime")]
pub mod FabricRuntime;
#[cfg(feature = "ServiceFabric_FabricCommon_FabricTransport")]
pub mod FabricTransport;
#[inline]
pub unsafe fn FabricDecryptText<P0>(
    encryptedtext: P0,
    certstorelocation: super::FABRIC_X509_STORE_LOCATION,
) -> ::windows_core::Result<IFabricStringResult>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricDecryptText(
            encryptedtext: ::windows_core::PCWSTR,
            certstorelocation: super::FABRIC_X509_STORE_LOCATION,
            decryptedtext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricDecryptText(
        encryptedtext.into_param().abi(),
        certstorelocation,
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricDecryptValue<P0>(
    encryptedvalue: P0,
) -> ::windows_core::Result<IFabricStringResult>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricDecryptValue(
            encryptedvalue: ::windows_core::PCWSTR,
            decryptedvalue: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricDecryptValue(encryptedvalue.into_param().abi(), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricEncryptText<P0, P1, P2, P3>(
    text: P0,
    certthumbprint: P1,
    certstorename: P2,
    certstorelocation: super::FABRIC_X509_STORE_LOCATION,
    algorithmoid: P3,
) -> ::windows_core::Result<IFabricStringResult>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricEncryptText(
            text: ::windows_core::PCWSTR,
            certthumbprint: ::windows_core::PCWSTR,
            certstorename: ::windows_core::PCWSTR,
            certstorelocation: super::FABRIC_X509_STORE_LOCATION,
            algorithmoid: ::windows_core::PCSTR,
            encryptedvalue: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEncryptText(
        text.into_param().abi(),
        certthumbprint.into_param().abi(),
        certstorename.into_param().abi(),
        certstorelocation,
        algorithmoid.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricEncryptText2<P0, P1, P2>(
    text: P0,
    certfilepath: P1,
    algorithmoid: P2,
) -> ::windows_core::Result<IFabricStringResult>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricEncryptText2(
            text: ::windows_core::PCWSTR,
            certfilepath: ::windows_core::PCWSTR,
            algorithmoid: ::windows_core::PCSTR,
            encryptedvalue: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEncryptText2(
        text.into_param().abi(),
        certfilepath.into_param().abi(),
        algorithmoid.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricEncryptValue<P0, P1, P2>(
    certthumbprint: P0,
    certstorename: P1,
    text: P2,
) -> ::windows_core::Result<IFabricStringResult>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricEncryptValue(
            certthumbprint: ::windows_core::PCWSTR,
            certstorename: ::windows_core::PCWSTR,
            text: ::windows_core::PCWSTR,
            encryptedvalue: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEncryptValue(
        certthumbprint.into_param().abi(),
        certstorename.into_param().abi(),
        text.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricGetLastErrorMessage() -> ::windows_core::Result<IFabricStringResult> {
    #[link(name = "FabricCommon")]
    extern "system" {
        pub fn FabricGetLastErrorMessage(
            message: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricGetLastErrorMessage(&mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
::windows_core::imp::com_interface!(
    IFabricAsyncOperationCallback,
    IFabricAsyncOperationCallback_Vtbl,
    0x86f08d7e_14dd_4575_8489_b1d5d679029c
);
::windows_core::imp::interface_hierarchy!(IFabricAsyncOperationCallback, ::windows_core::IUnknown);
impl IFabricAsyncOperationCallback {
    pub unsafe fn Invoke<P0>(&self, context: P0)
    where
        P0: ::windows_core::IntoParam<IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).Invoke)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricAsyncOperationCallback {}
unsafe impl ::core::marker::Sync for IFabricAsyncOperationCallback {}
#[repr(C)]
pub struct IFabricAsyncOperationCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricAsyncOperationContext,
    IFabricAsyncOperationContext_Vtbl,
    0x841720bf_c9e8_4e6f_9c3f_6b7f4ac73bcd
);
::windows_core::imp::interface_hierarchy!(IFabricAsyncOperationContext, ::windows_core::IUnknown);
impl IFabricAsyncOperationContext {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows_core::Interface::vtable(self).IsCompleted)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows_core::Interface::vtable(self).CompletedSynchronously)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn Callback(&self) -> ::windows_core::Result<IFabricAsyncOperationCallback> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Callback)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self))
            .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricAsyncOperationContext {}
unsafe impl ::core::marker::Sync for IFabricAsyncOperationContext {}
#[repr(C)]
pub struct IFabricAsyncOperationContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCompleted: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::BOOLEAN,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCompleted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CompletedSynchronously: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    )
        -> ::windows::Win32::Foundation::BOOLEAN,
    #[cfg(not(feature = "Win32_Foundation"))]
    CompletedSynchronously: usize,
    pub Callback: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricGetReplicatorStatusResult,
    IFabricGetReplicatorStatusResult_Vtbl,
    0x30e10c61_a710_4f99_a623_bb1403265186
);
::windows_core::imp::interface_hierarchy!(
    IFabricGetReplicatorStatusResult,
    ::windows_core::IUnknown
);
impl IFabricGetReplicatorStatusResult {
    pub unsafe fn get_ReplicatorStatus(&self) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
        (::windows_core::Interface::vtable(self).get_ReplicatorStatus)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricGetReplicatorStatusResult {}
unsafe impl ::core::marker::Sync for IFabricGetReplicatorStatusResult {}
#[repr(C)]
pub struct IFabricGetReplicatorStatusResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_ReplicatorStatus:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT,
}
::windows_core::imp::com_interface!(
    IFabricStringListResult,
    IFabricStringListResult_Vtbl,
    0xafab1c53_757b_4b0e_8b7e_237aeee6bfe9
);
::windows_core::imp::interface_hierarchy!(IFabricStringListResult, ::windows_core::IUnknown);
impl IFabricStringListResult {
    pub unsafe fn GetStrings(
        &self,
        itemcount: *mut u32,
    ) -> ::windows_core::Result<*mut ::windows_core::PCWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrings)(
            ::windows_core::Interface::as_raw(self),
            itemcount,
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricStringListResult {}
unsafe impl ::core::marker::Sync for IFabricStringListResult {}
#[repr(C)]
pub struct IFabricStringListResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStrings: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut u32,
        *mut *mut ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStringResult,
    IFabricStringResult_Vtbl,
    0x4ae69614_7d0f_4cd4_b836_23017000d132
);
::windows_core::imp::interface_hierarchy!(IFabricStringResult, ::windows_core::IUnknown);
impl IFabricStringResult {
    pub unsafe fn get_String(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_String)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricStringResult {}
unsafe impl ::core::marker::Sync for IFabricStringResult {}
#[repr(C)]
pub struct IFabricStringResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_String: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
