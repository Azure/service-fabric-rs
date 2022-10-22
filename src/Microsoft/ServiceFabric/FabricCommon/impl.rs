pub trait IFabricAsyncOperationCallback_Impl: Sized {
    fn Invoke(&self, context: &::core::option::Option<IFabricAsyncOperationContext>);
}
impl ::windows::core::RuntimeName for IFabricAsyncOperationCallback {}
impl IFabricAsyncOperationCallback_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAsyncOperationCallback_Impl,
        const OFFSET: isize,
    >() -> IFabricAsyncOperationCallback_Vtbl {
        unsafe extern "system" fn Invoke<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::core::mem::transmute(&context))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricAsyncOperationCallback as ::windows::core::Interface>::IID
    }
}
pub trait IFabricAsyncOperationContext_Impl: Sized {
    fn IsCompleted(&self) -> ::windows::Win32::Foundation::BOOLEAN;
    fn CompletedSynchronously(&self) -> ::windows::Win32::Foundation::BOOLEAN;
    fn Callback(&self) -> ::windows::core::Result<IFabricAsyncOperationCallback>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricAsyncOperationContext {}
impl IFabricAsyncOperationContext_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAsyncOperationContext_Impl,
        const OFFSET: isize,
    >() -> IFabricAsyncOperationContext_Vtbl {
        unsafe extern "system" fn IsCompleted<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
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
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
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
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Callback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAsyncOperationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            CompletedSynchronously: CompletedSynchronously::<Identity, Impl, OFFSET>,
            Callback: Callback::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricAsyncOperationContext as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetReplicatorStatusResult_Impl: Sized {
    fn get_ReplicatorStatus(&self) -> *mut super::FABRIC_REPLICATOR_STATUS_QUERY_RESULT;
}
impl ::windows::core::RuntimeName for IFabricGetReplicatorStatusResult {}
impl IFabricGetReplicatorStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicatorStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicatorStatusResult_Vtbl {
        unsafe extern "system" fn get_ReplicatorStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
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
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicatorStatus: get_ReplicatorStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetReplicatorStatusResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStringListResult_Impl: Sized {
    fn GetStrings(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStringListResult {}
impl IFabricStringListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStringListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStringListResult_Vtbl {
        unsafe extern "system" fn GetStrings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStringListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut ::windows::core::PWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStrings(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStrings: GetStrings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStringListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStringResult_Impl: Sized {
    fn get_String(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricStringResult {}
impl IFabricStringResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStringResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStringResult_Vtbl {
        unsafe extern "system" fn get_String<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStringResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_String()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_String: get_String::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStringResult as ::windows::core::Interface>::IID
    }
}
