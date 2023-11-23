#[cfg(feature = "ServiceFabric_FabricCommon_FabricClient")]
pub mod FabricClient;
#[cfg(feature = "ServiceFabric_FabricCommon_FabricRuntime")]
pub mod FabricRuntime;
#[cfg(feature = "ServiceFabric_FabricCommon_FabricTransport")]
pub mod FabricTransport;
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`*"]
#[repr(transparent)]
pub struct IFabricAsyncOperationCallback(::windows_core::IUnknown);
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
::windows_core::imp::interface_hierarchy!(IFabricAsyncOperationCallback, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IFabricAsyncOperationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricAsyncOperationCallback {}
impl ::core::fmt::Debug for IFabricAsyncOperationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricAsyncOperationCallback")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::core::marker::Send for IFabricAsyncOperationCallback {}
unsafe impl ::core::marker::Sync for IFabricAsyncOperationCallback {}
unsafe impl ::windows_core::Interface for IFabricAsyncOperationCallback {
    type Vtable = IFabricAsyncOperationCallback_Vtbl;
}
impl ::core::clone::Clone for IFabricAsyncOperationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFabricAsyncOperationCallback {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x86f08d7e_14dd_4575_8489_b1d5d679029c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAsyncOperationCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ),
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`*"]
#[repr(transparent)]
pub struct IFabricAsyncOperationContext(::windows_core::IUnknown);
impl IFabricAsyncOperationContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows_core::Interface::vtable(self).IsCompleted)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
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
        .from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self))
            .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFabricAsyncOperationContext, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IFabricAsyncOperationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricAsyncOperationContext {}
impl ::core::fmt::Debug for IFabricAsyncOperationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricAsyncOperationContext")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::core::marker::Send for IFabricAsyncOperationContext {}
unsafe impl ::core::marker::Sync for IFabricAsyncOperationContext {}
unsafe impl ::windows_core::Interface for IFabricAsyncOperationContext {
    type Vtable = IFabricAsyncOperationContext_Vtbl;
}
impl ::core::clone::Clone for IFabricAsyncOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFabricAsyncOperationContext {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x841720bf_c9e8_4e6f_9c3f_6b7f4ac73bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAsyncOperationContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::BOOLEAN,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCompleted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CompletedSynchronously: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    )
        -> ::windows::Win32::Foundation::BOOLEAN,
    #[cfg(not(feature = "Win32_Foundation"))]
    CompletedSynchronously: usize,
    pub Callback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`*"]
#[repr(transparent)]
pub struct IFabricGetReplicatorStatusResult(::windows_core::IUnknown);
impl IFabricGetReplicatorStatusResult {
    pub unsafe fn get_ReplicatorStatus(&self) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
        (::windows_core::Interface::vtable(self).get_ReplicatorStatus)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
::windows_core::imp::interface_hierarchy!(
    IFabricGetReplicatorStatusResult,
    ::windows_core::IUnknown
);
impl ::core::cmp::PartialEq for IFabricGetReplicatorStatusResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricGetReplicatorStatusResult {}
impl ::core::fmt::Debug for IFabricGetReplicatorStatusResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricGetReplicatorStatusResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::core::marker::Send for IFabricGetReplicatorStatusResult {}
unsafe impl ::core::marker::Sync for IFabricGetReplicatorStatusResult {}
unsafe impl ::windows_core::Interface for IFabricGetReplicatorStatusResult {
    type Vtable = IFabricGetReplicatorStatusResult_Vtbl;
}
impl ::core::clone::Clone for IFabricGetReplicatorStatusResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFabricGetReplicatorStatusResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x30e10c61_a710_4f99_a623_bb1403265186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricGetReplicatorStatusResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_ReplicatorStatus:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT,
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`*"]
#[repr(transparent)]
pub struct IFabricStringListResult(::windows_core::IUnknown);
impl IFabricStringListResult {
    pub unsafe fn GetStrings(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStrings)(
            ::windows_core::Interface::as_raw(self),
            itemcount,
            buffereditems,
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFabricStringListResult, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IFabricStringListResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStringListResult {}
impl ::core::fmt::Debug for IFabricStringListResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStringListResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::core::marker::Send for IFabricStringListResult {}
unsafe impl ::core::marker::Sync for IFabricStringListResult {}
unsafe impl ::windows_core::Interface for IFabricStringListResult {
    type Vtable = IFabricStringListResult_Vtbl;
}
impl ::core::clone::Clone for IFabricStringListResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFabricStringListResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xafab1c53_757b_4b0e_8b7e_237aeee6bfe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStringListResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStrings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`*"]
#[repr(transparent)]
pub struct IFabricStringResult(::windows_core::IUnknown);
impl IFabricStringResult {
    pub unsafe fn get_String(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_String)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
::windows_core::imp::interface_hierarchy!(IFabricStringResult, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IFabricStringResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStringResult {}
impl ::core::fmt::Debug for IFabricStringResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStringResult").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for IFabricStringResult {}
unsafe impl ::core::marker::Sync for IFabricStringResult {}
unsafe impl ::windows_core::Interface for IFabricStringResult {
    type Vtable = IFabricStringResult_Vtbl;
}
impl ::core::clone::Clone for IFabricStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFabricStringResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4ae69614_7d0f_4cd4_b836_23017000d132);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStringResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_String:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
