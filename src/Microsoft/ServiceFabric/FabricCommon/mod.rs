pub mod FabricClient;
pub mod FabricRuntime;
pub mod FabricTransport;
#[repr(transparent)]
pub struct IFabricAsyncOperationCallback(::windows::core::IUnknown);
impl IFabricAsyncOperationCallback {
    pub unsafe fn Invoke<'a, P0>(&self, context: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).Invoke)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricAsyncOperationCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricAsyncOperationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFabricAsyncOperationCallback {
    type Vtable = IFabricAsyncOperationCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricAsyncOperationCallback {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x86f08d7e_14dd_4575_8489_b1d5d679029c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAsyncOperationCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ),
}
#[repr(transparent)]
pub struct IFabricAsyncOperationContext(::windows::core::IUnknown);
impl IFabricAsyncOperationContext {
    pub unsafe fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows::core::Vtable::vtable(self).IsCompleted)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows::core::Vtable::vtable(self).CompletedSynchronously)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn Callback(&self) -> ::windows::core::Result<IFabricAsyncOperationCallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Callback)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricAsyncOperationCallback>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IFabricAsyncOperationContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricAsyncOperationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFabricAsyncOperationContext {
    type Vtable = IFabricAsyncOperationContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricAsyncOperationContext {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x841720bf_c9e8_4e6f_9c3f_6b7f4ac73bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAsyncOperationContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::BOOLEAN,
    pub CompletedSynchronously: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    )
        -> ::windows::Win32::Foundation::BOOLEAN,
    pub Callback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Cancel:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricGetReplicatorStatusResult(::windows::core::IUnknown);
impl IFabricGetReplicatorStatusResult {
    pub unsafe fn get_ReplicatorStatus(&self) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
        (::windows::core::Vtable::vtable(self).get_ReplicatorStatus)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricGetReplicatorStatusResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricGetReplicatorStatusResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFabricGetReplicatorStatusResult {
    type Vtable = IFabricGetReplicatorStatusResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricGetReplicatorStatusResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x30e10c61_a710_4f99_a623_bb1403265186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricGetReplicatorStatusResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_ReplicatorStatus:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT,
}
#[repr(transparent)]
pub struct IFabricStringListResult(::windows::core::IUnknown);
impl IFabricStringListResult {
    pub unsafe fn GetStrings(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStrings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(itemcount),
            ::core::mem::transmute(buffereditems),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricStringListResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStringListResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFabricStringListResult {
    type Vtable = IFabricStringListResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStringListResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafab1c53_757b_4b0e_8b7e_237aeee6bfe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStringListResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStrings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows::core::PWSTR,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStringResult(::windows::core::IUnknown);
impl IFabricStringResult {
    pub unsafe fn get_String(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_String)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricStringResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Vtable for IFabricStringResult {
    type Vtable = IFabricStringResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStringResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4ae69614_7d0f_4cd4_b836_23017000d132);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStringResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_String:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
