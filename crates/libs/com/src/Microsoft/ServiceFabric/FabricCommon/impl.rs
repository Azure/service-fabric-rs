#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`, `\"implement\"`*"]
pub trait IFabricAsyncOperationCallback_Impl: Sized {
    fn Invoke(&self, context: ::core::option::Option<&IFabricAsyncOperationContext>);
}
impl ::windows_core::RuntimeName for IFabricAsyncOperationCallback {}
impl IFabricAsyncOperationCallback_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAsyncOperationCallback_Impl,
        const OFFSET: isize,
    >() -> IFabricAsyncOperationCallback_Vtbl {
        unsafe extern "system" fn Invoke<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&context))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricAsyncOperationCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFabricAsyncOperationContext_Impl: Sized {
    fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN;
    fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN;
    fn Callback(&self) -> ::windows_core::Result<IFabricAsyncOperationCallback>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFabricAsyncOperationContext {}
#[cfg(feature = "Win32_Foundation")]
impl IFabricAsyncOperationContext_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAsyncOperationContext_Impl,
        const OFFSET: isize,
    >() -> IFabricAsyncOperationContext_Vtbl {
        unsafe extern "system" fn IsCompleted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::Win32::Foundation::BOOLEAN {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCompleted()
        }
        unsafe extern "system" fn CompletedSynchronously<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::Win32::Foundation::BOOLEAN {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompletedSynchronously()
        }
        unsafe extern "system" fn Callback<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Callback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            CompletedSynchronously: CompletedSynchronously::<Identity, Impl, OFFSET>,
            Callback: Callback::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricAsyncOperationContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`, `\"implement\"`*"]
pub trait IFabricGetReplicatorStatusResult_Impl: Sized {
    fn get_ReplicatorStatus(&self) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT;
}
impl ::windows_core::RuntimeName for IFabricGetReplicatorStatusResult {}
impl IFabricGetReplicatorStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicatorStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicatorStatusResult_Vtbl {
        unsafe extern "system" fn get_ReplicatorStatus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicatorStatusResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicatorStatus()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicatorStatus: get_ReplicatorStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricGetReplicatorStatusResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`, `\"implement\"`*"]
pub trait IFabricStringListResult_Impl: Sized {
    fn GetStrings(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows_core::PCWSTR,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFabricStringListResult {}
impl IFabricStringListResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStringListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStringListResult_Vtbl {
        unsafe extern "system" fn GetStrings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStringListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStrings(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStrings: GetStrings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricStringListResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ServiceFabric_FabricCommon\"`, `\"implement\"`*"]
pub trait IFabricStringResult_Impl: Sized {
    fn get_String(&self) -> ::windows_core::PCWSTR;
}
impl ::windows_core::RuntimeName for IFabricStringResult {}
impl IFabricStringResult_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStringResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStringResult_Vtbl {
        unsafe extern "system" fn get_String<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStringResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_String()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_String: get_String::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFabricStringResult as ::windows_core::ComInterface>::IID
    }
}
