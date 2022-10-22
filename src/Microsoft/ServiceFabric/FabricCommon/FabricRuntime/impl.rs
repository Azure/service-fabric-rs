pub trait IFabricAtomicGroupStateProvider_Impl: Sized {
    fn BeginAtomicGroupCommit(
        &self,
        atomicgroupid: i64,
        commitsequencenumber: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndAtomicGroupCommit(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginAtomicGroupRollback(
        &self,
        atomicgroupid: i64,
        rollbackequencenumber: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndAtomicGroupRollback(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUndoProgress(
        &self,
        fromcommitsequencenumber: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUndoProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricAtomicGroupStateProvider {}
impl IFabricAtomicGroupStateProvider_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAtomicGroupStateProvider_Impl,
        const OFFSET: isize,
    >() -> IFabricAtomicGroupStateProvider_Vtbl {
        unsafe extern "system" fn BeginAtomicGroupCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: i64,
            commitsequencenumber: i64,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginAtomicGroupCommit(
                ::core::mem::transmute_copy(&atomicgroupid),
                ::core::mem::transmute_copy(&commitsequencenumber),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAtomicGroupCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAtomicGroupCommit(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginAtomicGroupRollback<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: i64,
            rollbackequencenumber: i64,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginAtomicGroupRollback(
                ::core::mem::transmute_copy(&atomicgroupid),
                ::core::mem::transmute_copy(&rollbackequencenumber),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAtomicGroupRollback<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAtomicGroupRollback(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUndoProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fromcommitsequencenumber: i64,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUndoProgress(
                ::core::mem::transmute_copy(&fromcommitsequencenumber),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUndoProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUndoProgress(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginAtomicGroupCommit: BeginAtomicGroupCommit::<Identity, Impl, OFFSET>,
            EndAtomicGroupCommit: EndAtomicGroupCommit::<Identity, Impl, OFFSET>,
            BeginAtomicGroupRollback: BeginAtomicGroupRollback::<Identity, Impl, OFFSET>,
            EndAtomicGroupRollback: EndAtomicGroupRollback::<Identity, Impl, OFFSET>,
            BeginUndoProgress: BeginUndoProgress::<Identity, Impl, OFFSET>,
            EndUndoProgress: EndUndoProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricAtomicGroupStateProvider as ::windows::core::Interface>::IID
    }
}
pub trait IFabricAtomicGroupStateReplicator_Impl: Sized {
    fn CreateAtomicGroup(&self) -> ::windows::core::Result<i64>;
    fn BeginReplicateAtomicGroupOperation(
        &self,
        atomicgroupid: i64,
        operationdata: &::core::option::Option<IFabricOperationData>,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
        operationsequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn EndReplicateAtomicGroupOperation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginReplicateAtomicGroupCommit(
        &self,
        atomicgroupid: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
        commitsequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn EndReplicateAtomicGroupCommit(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginReplicateAtomicGroupRollback(
        &self,
        atomicgroupid: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
        rollbacksequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn EndReplicateAtomicGroupRollback(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IFabricAtomicGroupStateReplicator {}
impl IFabricAtomicGroupStateReplicator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricAtomicGroupStateReplicator_Impl,
        const OFFSET: isize,
    >() -> IFabricAtomicGroupStateReplicator_Vtbl {
        unsafe extern "system" fn CreateAtomicGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAtomicGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(atomicgroupid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginReplicateAtomicGroupOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: i64,
            operationdata: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            operationsequencenumber: *mut i64,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginReplicateAtomicGroupOperation(
                ::core::mem::transmute_copy(&atomicgroupid),
                ::core::mem::transmute(&operationdata),
                ::core::mem::transmute(&callback),
                ::core::mem::transmute_copy(&operationsequencenumber),
                ::core::mem::transmute_copy(&context),
            )
            .into()
        }
        unsafe extern "system" fn EndReplicateAtomicGroupOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            operationsequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndReplicateAtomicGroupOperation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(operationsequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginReplicateAtomicGroupCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: i64,
            callback: *mut ::core::ffi::c_void,
            commitsequencenumber: *mut i64,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginReplicateAtomicGroupCommit(
                ::core::mem::transmute_copy(&atomicgroupid),
                ::core::mem::transmute(&callback),
                ::core::mem::transmute_copy(&commitsequencenumber),
                ::core::mem::transmute_copy(&context),
            )
            .into()
        }
        unsafe extern "system" fn EndReplicateAtomicGroupCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitsequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndReplicateAtomicGroupCommit(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitsequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginReplicateAtomicGroupRollback<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            atomicgroupid: i64,
            callback: *mut ::core::ffi::c_void,
            rollbacksequencenumber: *mut i64,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginReplicateAtomicGroupRollback(
                ::core::mem::transmute_copy(&atomicgroupid),
                ::core::mem::transmute(&callback),
                ::core::mem::transmute_copy(&rollbacksequencenumber),
                ::core::mem::transmute_copy(&context),
            )
            .into()
        }
        unsafe extern "system" fn EndReplicateAtomicGroupRollback<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricAtomicGroupStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            rollbacksequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndReplicateAtomicGroupRollback(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rollbacksequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAtomicGroup: CreateAtomicGroup::<Identity, Impl, OFFSET>,
            BeginReplicateAtomicGroupOperation: BeginReplicateAtomicGroupOperation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndReplicateAtomicGroupOperation: EndReplicateAtomicGroupOperation::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginReplicateAtomicGroupCommit: BeginReplicateAtomicGroupCommit::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndReplicateAtomicGroupCommit: EndReplicateAtomicGroupCommit::<Identity, Impl, OFFSET>,
            BeginReplicateAtomicGroupRollback: BeginReplicateAtomicGroupRollback::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndReplicateAtomicGroupRollback: EndReplicateAtomicGroupRollback::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricAtomicGroupStateReplicator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackage_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION;
    fn get_Path(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricCodePackage {}
impl IFabricCodePackage_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackage_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackage_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        unsafe extern "system" fn get_Path<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Path()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
            get_Path: get_Path::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackage2_Impl: Sized + IFabricCodePackage_Impl {
    fn get_SetupEntryPointRunAsPolicy(&self) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION;
    fn get_EntryPointRunAsPolicy(&self) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION;
}
impl ::windows::core::RuntimeName for IFabricCodePackage2 {}
impl IFabricCodePackage2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackage2_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackage2_Vtbl {
        unsafe extern "system" fn get_SetupEntryPointRunAsPolicy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackage2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_SetupEntryPointRunAsPolicy()
        }
        unsafe extern "system" fn get_EntryPointRunAsPolicy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackage2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_EntryPointRunAsPolicy()
        }
        Self {
            base__: IFabricCodePackage_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_SetupEntryPointRunAsPolicy: get_SetupEntryPointRunAsPolicy::<Identity, Impl, OFFSET>,
            get_EntryPointRunAsPolicy: get_EntryPointRunAsPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackage2 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext_Impl: Sized {
    fn get_ContextId(&self) -> ::windows::core::PWSTR;
    fn get_CodePackageName(&self) -> ::windows::core::PWSTR;
    fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR;
    fn get_WorkDirectory(&self) -> ::windows::core::PWSTR;
    fn get_LogDirectory(&self) -> ::windows::core::PWSTR;
    fn get_TempDirectory(&self) -> ::windows::core::PWSTR;
    fn get_ServiceTypes(&self) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST;
    fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST;
    fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION;
    fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST;
    fn GetServiceEndpointResource(
        &self,
        serviceendpointresourcename: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>;
    fn GetCodePackageNames(&self) -> ::windows::core::Result<super::IFabricStringListResult>;
    fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult>;
    fn GetDataPackageNames(&self) -> ::windows::core::Result<super::IFabricStringListResult>;
    fn GetCodePackage(
        &self,
        codepackagename: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricCodePackage>;
    fn GetConfigurationPackage(
        &self,
        configpackagename: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>;
    fn GetDataPackage(
        &self,
        datapackagename: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricDataPackage>;
    fn RegisterCodePackageChangeHandler(
        &self,
        callback: &::core::option::Option<IFabricCodePackageChangeHandler>,
    ) -> ::windows::core::Result<i64>;
    fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()>;
    fn RegisterConfigurationPackageChangeHandler(
        &self,
        callback: &::core::option::Option<IFabricConfigurationPackageChangeHandler>,
    ) -> ::windows::core::Result<i64>;
    fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()>;
    fn RegisterDataPackageChangeHandler(
        &self,
        callback: &::core::option::Option<IFabricDataPackageChangeHandler>,
    ) -> ::windows::core::Result<i64>;
    fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext {}
impl IFabricCodePackageActivationContext_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext_Vtbl {
        unsafe extern "system" fn get_ContextId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ContextId()
        }
        unsafe extern "system" fn get_CodePackageName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_CodePackageName()
        }
        unsafe extern "system" fn get_CodePackageVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_CodePackageVersion()
        }
        unsafe extern "system" fn get_WorkDirectory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_WorkDirectory()
        }
        unsafe extern "system" fn get_LogDirectory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_LogDirectory()
        }
        unsafe extern "system" fn get_TempDirectory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TempDirectory()
        }
        unsafe extern "system" fn get_ServiceTypes<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceTypes()
        }
        unsafe extern "system" fn get_ServiceGroupTypes<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceGroupTypes()
        }
        unsafe extern "system" fn get_ApplicationPrincipals<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationPrincipals()
        }
        unsafe extern "system" fn get_ServiceEndpointResources<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceEndpointResources()
        }
        unsafe extern "system" fn GetServiceEndpointResource<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            serviceendpointresourcename: ::windows::core::PCWSTR,
            bufferedvalue: *mut *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetServiceEndpointResource(::core::mem::transmute(&serviceendpointresourcename))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePackageNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            names: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodePackageNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurationPackageNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            names: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConfigurationPackageNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPackageNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            names: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataPackageNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codepackagename: ::windows::core::PCWSTR,
            codepackage: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodePackage(::core::mem::transmute(&codepackagename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(codepackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurationPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            configpackagename: ::windows::core::PCWSTR,
            configpackage: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConfigurationPackage(::core::mem::transmute(&configpackagename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(configpackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            datapackagename: ::windows::core::PCWSTR,
            datapackage: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataPackage(::core::mem::transmute(&datapackagename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datapackage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCodePackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterCodePackageChangeHandler(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCodePackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterCodePackageChangeHandler(::core::mem::transmute_copy(&callbackhandle))
                .into()
        }
        unsafe extern "system" fn RegisterConfigurationPackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterConfigurationPackageChangeHandler(::core::mem::transmute(&callback))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterConfigurationPackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterConfigurationPackageChangeHandler(::core::mem::transmute_copy(
                &callbackhandle,
            ))
            .into()
        }
        unsafe extern "system" fn RegisterDataPackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDataPackageChangeHandler(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDataPackageChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDataPackageChangeHandler(::core::mem::transmute_copy(&callbackhandle))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ContextId: get_ContextId::<Identity, Impl, OFFSET>,
            get_CodePackageName: get_CodePackageName::<Identity, Impl, OFFSET>,
            get_CodePackageVersion: get_CodePackageVersion::<Identity, Impl, OFFSET>,
            get_WorkDirectory: get_WorkDirectory::<Identity, Impl, OFFSET>,
            get_LogDirectory: get_LogDirectory::<Identity, Impl, OFFSET>,
            get_TempDirectory: get_TempDirectory::<Identity, Impl, OFFSET>,
            get_ServiceTypes: get_ServiceTypes::<Identity, Impl, OFFSET>,
            get_ServiceGroupTypes: get_ServiceGroupTypes::<Identity, Impl, OFFSET>,
            get_ApplicationPrincipals: get_ApplicationPrincipals::<Identity, Impl, OFFSET>,
            get_ServiceEndpointResources: get_ServiceEndpointResources::<Identity, Impl, OFFSET>,
            GetServiceEndpointResource: GetServiceEndpointResource::<Identity, Impl, OFFSET>,
            GetCodePackageNames: GetCodePackageNames::<Identity, Impl, OFFSET>,
            GetConfigurationPackageNames: GetConfigurationPackageNames::<Identity, Impl, OFFSET>,
            GetDataPackageNames: GetDataPackageNames::<Identity, Impl, OFFSET>,
            GetCodePackage: GetCodePackage::<Identity, Impl, OFFSET>,
            GetConfigurationPackage: GetConfigurationPackage::<Identity, Impl, OFFSET>,
            GetDataPackage: GetDataPackage::<Identity, Impl, OFFSET>,
            RegisterCodePackageChangeHandler: RegisterCodePackageChangeHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
            UnregisterCodePackageChangeHandler: UnregisterCodePackageChangeHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
            RegisterConfigurationPackageChangeHandler: RegisterConfigurationPackageChangeHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
            UnregisterConfigurationPackageChangeHandler:
                UnregisterConfigurationPackageChangeHandler::<Identity, Impl, OFFSET>,
            RegisterDataPackageChangeHandler: RegisterDataPackageChangeHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
            UnregisterDataPackageChangeHandler: UnregisterDataPackageChangeHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext2_Impl:
    Sized + IFabricCodePackageActivationContext_Impl
{
    fn get_ApplicationName(&self) -> *mut u16;
    fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR;
    fn GetServiceManifestName(&self) -> ::windows::core::Result<super::IFabricStringResult>;
    fn GetServiceManifestVersion(&self) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext2 {}
impl IFabricCodePackageActivationContext2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext2_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext2_Vtbl {
        unsafe extern "system" fn get_ApplicationName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut u16 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationName()
        }
        unsafe extern "system" fn get_ApplicationTypeName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypeName()
        }
        unsafe extern "system" fn GetServiceManifestName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicemanifestname: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceManifestName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicemanifestname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceManifestVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicemanifestversion: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceManifestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicemanifestversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricCodePackageActivationContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_ApplicationName: get_ApplicationName::<Identity, Impl, OFFSET>,
            get_ApplicationTypeName: get_ApplicationTypeName::<Identity, Impl, OFFSET>,
            GetServiceManifestName: GetServiceManifestName::<Identity, Impl, OFFSET>,
            GetServiceManifestVersion: GetServiceManifestVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext2 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext3_Impl:
    Sized + IFabricCodePackageActivationContext_Impl + IFabricCodePackageActivationContext2_Impl
{
    fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
    fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
    fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext3 {}
impl IFabricCodePackageActivationContext3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext3_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext3_Vtbl {
        unsafe extern "system" fn ReportApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportApplicationHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        unsafe extern "system" fn ReportDeployedApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDeployedApplicationHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        unsafe extern "system" fn ReportDeployedServicePackageHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDeployedServicePackageHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        Self {
            base__: IFabricCodePackageActivationContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportApplicationHealth: ReportApplicationHealth::<Identity, Impl, OFFSET>,
            ReportDeployedApplicationHealth: ReportDeployedApplicationHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            ReportDeployedServicePackageHealth: ReportDeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext3 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext4_Impl:
    Sized
    + IFabricCodePackageActivationContext_Impl
    + IFabricCodePackageActivationContext2_Impl
    + IFabricCodePackageActivationContext3_Impl
{
    fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
    fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
    fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext4 {}
impl IFabricCodePackageActivationContext4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext4_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext4_Vtbl {
        unsafe extern "system" fn ReportApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportApplicationHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        unsafe extern "system" fn ReportDeployedApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDeployedApplicationHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        unsafe extern "system" fn ReportDeployedServicePackageHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportDeployedServicePackageHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        Self {
            base__: IFabricCodePackageActivationContext3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportApplicationHealth2: ReportApplicationHealth2::<Identity, Impl, OFFSET>,
            ReportDeployedApplicationHealth2: ReportDeployedApplicationHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            ReportDeployedServicePackageHealth2: ReportDeployedServicePackageHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext4 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext2 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext5_Impl:
    Sized
    + IFabricCodePackageActivationContext_Impl
    + IFabricCodePackageActivationContext2_Impl
    + IFabricCodePackageActivationContext3_Impl
    + IFabricCodePackageActivationContext4_Impl
{
    fn get_ServiceListenAddress(&self) -> ::windows::core::PWSTR;
    fn get_ServicePublishAddress(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext5 {}
impl IFabricCodePackageActivationContext5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext5_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext5_Vtbl {
        unsafe extern "system" fn get_ServiceListenAddress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceListenAddress()
        }
        unsafe extern "system" fn get_ServicePublishAddress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServicePublishAddress()
        }
        Self {
            base__: IFabricCodePackageActivationContext4_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_ServiceListenAddress: get_ServiceListenAddress::<Identity, Impl, OFFSET>,
            get_ServicePublishAddress: get_ServicePublishAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext5 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext2 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext3 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivationContext6_Impl:
    Sized
    + IFabricCodePackageActivationContext_Impl
    + IFabricCodePackageActivationContext2_Impl
    + IFabricCodePackageActivationContext3_Impl
    + IFabricCodePackageActivationContext4_Impl
    + IFabricCodePackageActivationContext5_Impl
{
    fn GetDirectory(
        &self,
        logicaldirectoryname: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivationContext6 {}
impl IFabricCodePackageActivationContext6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivationContext6_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivationContext6_Vtbl {
        unsafe extern "system" fn GetDirectory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivationContext6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            logicaldirectoryname: ::windows::core::PCWSTR,
            directorypath: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectory(::core::mem::transmute(&logicaldirectoryname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(directorypath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricCodePackageActivationContext5_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDirectory: GetDirectory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivationContext6 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext2 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext3 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext4 as ::windows::core::Interface>::IID
            || iid == &<IFabricCodePackageActivationContext5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageActivator_Impl: Sized {
    fn BeginActivateCodePackage(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        environment: *const super::super::FABRIC_STRING_MAP,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndActivateCodePackage(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeactivateCodePackage(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeactivateCodePackage(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn AbortCodePackage(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
    ) -> ::windows::core::Result<()>;
    fn RegisterCodePackageEventHandler(
        &self,
        eventhandler: &::core::option::Option<IFabricCodePackageEventHandler>,
    ) -> ::windows::core::Result<u64>;
    fn UnregisterCodePackageEventHandler(&self, callbackhandle: u64)
        -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricCodePackageActivator {}
impl IFabricCodePackageActivator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageActivator_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageActivator_Vtbl {
        unsafe extern "system" fn BeginActivateCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codepackagenames: *const super::super::FABRIC_STRING_LIST,
            environment: *const super::super::FABRIC_STRING_MAP,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginActivateCodePackage(
                ::core::mem::transmute_copy(&codepackagenames),
                ::core::mem::transmute_copy(&environment),
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
        unsafe extern "system" fn EndActivateCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndActivateCodePackage(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeactivateCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codepackagenames: *const super::super::FABRIC_STRING_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeactivateCodePackage(
                ::core::mem::transmute_copy(&codepackagenames),
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
        unsafe extern "system" fn EndDeactivateCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeactivateCodePackage(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn AbortCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codepackagenames: *const super::super::FABRIC_STRING_LIST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortCodePackage(::core::mem::transmute_copy(&codepackagenames))
                .into()
        }
        unsafe extern "system" fn RegisterCodePackageEventHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            eventhandler: *mut ::core::ffi::c_void,
            callbackhandle: *mut u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterCodePackageEventHandler(::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCodePackageEventHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageActivator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterCodePackageEventHandler(::core::mem::transmute_copy(&callbackhandle))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginActivateCodePackage: BeginActivateCodePackage::<Identity, Impl, OFFSET>,
            EndActivateCodePackage: EndActivateCodePackage::<Identity, Impl, OFFSET>,
            BeginDeactivateCodePackage: BeginDeactivateCodePackage::<Identity, Impl, OFFSET>,
            EndDeactivateCodePackage: EndDeactivateCodePackage::<Identity, Impl, OFFSET>,
            AbortCodePackage: AbortCodePackage::<Identity, Impl, OFFSET>,
            RegisterCodePackageEventHandler: RegisterCodePackageEventHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
            UnregisterCodePackageEventHandler: UnregisterCodePackageEventHandler::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageActivator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageChangeHandler_Impl: Sized {
    fn OnPackageAdded(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        codepackage: &::core::option::Option<IFabricCodePackage>,
    );
    fn OnPackageRemoved(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        codepackage: &::core::option::Option<IFabricCodePackage>,
    );
    fn OnPackageModified(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        previouscodepackage: &::core::option::Option<IFabricCodePackage>,
        codepackage: &::core::option::Option<IFabricCodePackage>,
    );
}
impl ::windows::core::RuntimeName for IFabricCodePackageChangeHandler {}
impl IFabricCodePackageChangeHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageChangeHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageChangeHandler_Vtbl {
        unsafe extern "system" fn OnPackageAdded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            codepackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageAdded(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&codepackage),
            )
        }
        unsafe extern "system" fn OnPackageRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            codepackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageRemoved(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&codepackage),
            )
        }
        unsafe extern "system" fn OnPackageModified<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            previouscodepackage: *mut ::core::ffi::c_void,
            codepackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageModified(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&previouscodepackage),
                ::core::mem::transmute(&codepackage),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPackageAdded: OnPackageAdded::<Identity, Impl, OFFSET>,
            OnPackageRemoved: OnPackageRemoved::<Identity, Impl, OFFSET>,
            OnPackageModified: OnPackageModified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricCodePackageEventHandler_Impl: Sized {
    fn OnCodePackageEvent(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivator>,
        eventdesc: *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
    );
}
impl ::windows::core::RuntimeName for IFabricCodePackageEventHandler {}
impl IFabricCodePackageEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricCodePackageEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricCodePackageEventHandler_Vtbl {
        unsafe extern "system" fn OnCodePackageEvent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricCodePackageEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            eventdesc: *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCodePackageEvent(
                ::core::mem::transmute(&source),
                ::core::mem::transmute_copy(&eventdesc),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCodePackageEvent: OnCodePackageEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricCodePackageEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricConfigurationPackage_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION;
    fn get_Path(&self) -> ::windows::core::PWSTR;
    fn get_Settings(&self) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS;
    fn GetSection(
        &self,
        sectionname: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_CONFIGURATION_SECTION>;
    fn GetValue(
        &self,
        sectionname: &::windows::core::PCWSTR,
        parametername: &::windows::core::PCWSTR,
        isencrypted: *mut u8,
        bufferedvalue: *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()>;
    fn DecryptValue(
        &self,
        encryptedvalue: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricConfigurationPackage {}
impl IFabricConfigurationPackage_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricConfigurationPackage_Impl,
        const OFFSET: isize,
    >() -> IFabricConfigurationPackage_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        unsafe extern "system" fn get_Path<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Path()
        }
        unsafe extern "system" fn get_Settings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Settings()
        }
        unsafe extern "system" fn GetSection<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sectionname: ::windows::core::PCWSTR,
            bufferedvalue: *mut *mut super::super::FABRIC_CONFIGURATION_SECTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSection(::core::mem::transmute(&sectionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sectionname: ::windows::core::PCWSTR,
            parametername: ::windows::core::PCWSTR,
            isencrypted: *mut u8,
            bufferedvalue: *mut ::windows::core::PWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValue(
                ::core::mem::transmute(&sectionname),
                ::core::mem::transmute(&parametername),
                ::core::mem::transmute_copy(&isencrypted),
                ::core::mem::transmute_copy(&bufferedvalue),
            )
            .into()
        }
        unsafe extern "system" fn DecryptValue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            encryptedvalue: ::windows::core::PCWSTR,
            decryptedvalue: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DecryptValue(::core::mem::transmute(&encryptedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(decryptedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
            get_Path: get_Path::<Identity, Impl, OFFSET>,
            get_Settings: get_Settings::<Identity, Impl, OFFSET>,
            GetSection: GetSection::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DecryptValue: DecryptValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricConfigurationPackage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricConfigurationPackage2_Impl: Sized + IFabricConfigurationPackage_Impl {
    fn GetValues(
        &self,
        sectionname: &::windows::core::PCWSTR,
        parameterprefix: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST>;
}
impl ::windows::core::RuntimeName for IFabricConfigurationPackage2 {}
impl IFabricConfigurationPackage2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricConfigurationPackage2_Impl,
        const OFFSET: isize,
    >() -> IFabricConfigurationPackage2_Vtbl {
        unsafe extern "system" fn GetValues<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackage2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sectionname: ::windows::core::PCWSTR,
            parameterprefix: ::windows::core::PCWSTR,
            bufferedvalue: *mut *mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValues(
                ::core::mem::transmute(&sectionname),
                ::core::mem::transmute(&parameterprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricConfigurationPackage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetValues: GetValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricConfigurationPackage2 as ::windows::core::Interface>::IID
            || iid == &<IFabricConfigurationPackage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricConfigurationPackageChangeHandler_Impl: Sized {
    fn OnPackageAdded(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        configpackage: &::core::option::Option<IFabricConfigurationPackage>,
    );
    fn OnPackageRemoved(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        configpackage: &::core::option::Option<IFabricConfigurationPackage>,
    );
    fn OnPackageModified(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        previousconfigpackage: &::core::option::Option<IFabricConfigurationPackage>,
        configpackage: &::core::option::Option<IFabricConfigurationPackage>,
    );
}
impl ::windows::core::RuntimeName for IFabricConfigurationPackageChangeHandler {}
impl IFabricConfigurationPackageChangeHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricConfigurationPackageChangeHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricConfigurationPackageChangeHandler_Vtbl {
        unsafe extern "system" fn OnPackageAdded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            configpackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageAdded(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&configpackage),
            )
        }
        unsafe extern "system" fn OnPackageRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            configpackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageRemoved(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&configpackage),
            )
        }
        unsafe extern "system" fn OnPackageModified<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricConfigurationPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            previousconfigpackage: *mut ::core::ffi::c_void,
            configpackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageModified(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&previousconfigpackage),
                ::core::mem::transmute(&configpackage),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPackageAdded: OnPackageAdded::<Identity, Impl, OFFSET>,
            OnPackageRemoved: OnPackageRemoved::<Identity, Impl, OFFSET>,
            OnPackageModified: OnPackageModified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricConfigurationPackageChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricDataPackage_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION;
    fn get_Path(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricDataPackage {}
impl IFabricDataPackage_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDataPackage_Impl,
        const OFFSET: isize,
    >() -> IFabricDataPackage_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDataPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        unsafe extern "system" fn get_Path<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDataPackage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Path()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
            get_Path: get_Path::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricDataPackage as ::windows::core::Interface>::IID
    }
}
pub trait IFabricDataPackageChangeHandler_Impl: Sized {
    fn OnPackageAdded(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        datapackage: &::core::option::Option<IFabricDataPackage>,
    );
    fn OnPackageRemoved(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        datapackage: &::core::option::Option<IFabricDataPackage>,
    );
    fn OnPackageModified(
        &self,
        source: &::core::option::Option<IFabricCodePackageActivationContext>,
        previousdatapackage: &::core::option::Option<IFabricDataPackage>,
        datapackage: &::core::option::Option<IFabricDataPackage>,
    );
}
impl ::windows::core::RuntimeName for IFabricDataPackageChangeHandler {}
impl IFabricDataPackageChangeHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDataPackageChangeHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricDataPackageChangeHandler_Vtbl {
        unsafe extern "system" fn OnPackageAdded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDataPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            datapackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageAdded(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&datapackage),
            )
        }
        unsafe extern "system" fn OnPackageRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDataPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            datapackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageRemoved(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&datapackage),
            )
        }
        unsafe extern "system" fn OnPackageModified<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDataPackageChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            previousdatapackage: *mut ::core::ffi::c_void,
            datapackage: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPackageModified(
                ::core::mem::transmute(&source),
                ::core::mem::transmute(&previousdatapackage),
                ::core::mem::transmute(&datapackage),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPackageAdded: OnPackageAdded::<Identity, Impl, OFFSET>,
            OnPackageRemoved: OnPackageRemoved::<Identity, Impl, OFFSET>,
            OnPackageModified: OnPackageModified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricDataPackageChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricEseLocalStoreSettingsResult_Impl: Sized {
    fn get_Settings(&self) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS;
}
impl ::windows::core::RuntimeName for IFabricEseLocalStoreSettingsResult {}
impl IFabricEseLocalStoreSettingsResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricEseLocalStoreSettingsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricEseLocalStoreSettingsResult_Vtbl {
        unsafe extern "system" fn get_Settings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricEseLocalStoreSettingsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Settings()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Settings: get_Settings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricEseLocalStoreSettingsResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreEnumerator_Impl: Sized {
    fn EnumerateByKey(
        &self,
        keyprefix: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>;
    fn EnumerateMetadataByKey(
        &self,
        keyprefix: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreEnumerator {}
impl IFabricKeyValueStoreEnumerator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreEnumerator_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreEnumerator_Vtbl {
        unsafe extern "system" fn EnumerateByKey<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateByKey(::core::mem::transmute(&keyprefix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadataByKey<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateMetadataByKey(::core::mem::transmute(&keyprefix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumerateByKey: EnumerateByKey::<Identity, Impl, OFFSET>,
            EnumerateMetadataByKey: EnumerateMetadataByKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreEnumerator2_Impl:
    Sized + IFabricKeyValueStoreEnumerator_Impl
{
    fn EnumerateByKey2(
        &self,
        keyprefix: &::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>;
    fn EnumerateMetadataByKey2(
        &self,
        keyprefix: &::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreEnumerator2 {}
impl IFabricKeyValueStoreEnumerator2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreEnumerator2_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreEnumerator2_Vtbl {
        unsafe extern "system" fn EnumerateByKey2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreEnumerator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            strictprefix: ::windows::Win32::Foundation::BOOLEAN,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateByKey2(
                ::core::mem::transmute(&keyprefix),
                ::core::mem::transmute_copy(&strictprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadataByKey2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreEnumerator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            strictprefix: ::windows::Win32::Foundation::BOOLEAN,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateMetadataByKey2(
                ::core::mem::transmute(&keyprefix),
                ::core::mem::transmute_copy(&strictprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateByKey2: EnumerateByKey2::<Identity, Impl, OFFSET>,
            EnumerateMetadataByKey2: EnumerateMetadataByKey2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreEnumerator2 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<()>;
    fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemResult>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemEnumerator {}
impl IFabricKeyValueStoreItemEnumerator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemEnumerator_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext().into()
        }
        unsafe extern "system" fn get_Current<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::core::option::Option<IFabricKeyValueStoreItemResult> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Current()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            get_Current: get_Current::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemEnumerator2_Impl:
    Sized + IFabricKeyValueStoreItemEnumerator_Impl
{
    fn TryMoveNext(&self) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemEnumerator2 {}
impl IFabricKeyValueStoreItemEnumerator2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemEnumerator2_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemEnumerator2_Vtbl {
        unsafe extern "system" fn TryMoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemEnumerator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            success: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryMoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(success, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreItemEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(),
            TryMoveNext: TryMoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemEnumerator2 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreItemEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemMetadataEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<()>;
    fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemMetadataEnumerator {}
impl IFabricKeyValueStoreItemMetadataEnumerator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemMetadataEnumerator_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemMetadataEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemMetadataEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext().into()
        }
        unsafe extern "system" fn get_Current<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemMetadataEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Current()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            get_Current: get_Current::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemMetadataEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemMetadataEnumerator2_Impl:
    Sized + IFabricKeyValueStoreItemMetadataEnumerator_Impl
{
    fn TryMoveNext(&self) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemMetadataEnumerator2 {}
impl IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemMetadataEnumerator2_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl {
        unsafe extern "system" fn TryMoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemMetadataEnumerator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            success: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryMoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(success, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreItemMetadataEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(
            ),
            TryMoveNext: TryMoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemMetadataEnumerator2 as ::windows::core::Interface>::IID
            || iid
                == &<IFabricKeyValueStoreItemMetadataEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemMetadataResult_Impl: Sized {
    fn get_Metadata(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemMetadataResult {}
impl IFabricKeyValueStoreItemMetadataResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemMetadataResult_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemMetadataResult_Vtbl {
        unsafe extern "system" fn get_Metadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemMetadataResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Metadata()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Metadata: get_Metadata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemMetadataResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreItemResult_Impl: Sized {
    fn get_Item(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreItemResult {}
impl IFabricKeyValueStoreItemResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreItemResult_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreItemResult_Vtbl {
        unsafe extern "system" fn get_Item<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreItemResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Item()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreItemResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreNotification_Impl:
    Sized + IFabricKeyValueStoreItemResult_Impl
{
    fn IsDelete(&self) -> ::windows::Win32::Foundation::BOOLEAN;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreNotification {}
impl IFabricKeyValueStoreNotification_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreNotification_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreNotification_Vtbl {
        unsafe extern "system" fn IsDelete<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreNotification_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::Win32::Foundation::BOOLEAN {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDelete()
        }
        Self {
            base__: IFabricKeyValueStoreItemResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDelete: IsDelete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreNotification as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreItemResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreNotificationEnumerator_Impl: Sized {
    fn MoveNext(&self) -> ::windows::core::Result<()>;
    fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreNotification>;
    fn Reset(&self);
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreNotificationEnumerator {}
impl IFabricKeyValueStoreNotificationEnumerator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreNotificationEnumerator_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreNotificationEnumerator_Vtbl {
        unsafe extern "system" fn MoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreNotificationEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext().into()
        }
        unsafe extern "system" fn get_Current<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreNotificationEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::core::option::Option<IFabricKeyValueStoreNotification> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Current()
        }
        unsafe extern "system" fn Reset<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreNotificationEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            get_Current: get_Current::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreNotificationEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreNotificationEnumerator2_Impl:
    Sized + IFabricKeyValueStoreNotificationEnumerator_Impl
{
    fn TryMoveNext(&self) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreNotificationEnumerator2 {}
impl IFabricKeyValueStoreNotificationEnumerator2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreNotificationEnumerator2_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreNotificationEnumerator2_Vtbl {
        unsafe extern "system" fn TryMoveNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreNotificationEnumerator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            success: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryMoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(success, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreNotificationEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(
            ),
            TryMoveNext: TryMoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreNotificationEnumerator2 as ::windows::core::Interface>::IID
            || iid
                == &<IFabricKeyValueStoreNotificationEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica_Impl: Sized + IFabricStatefulServiceReplica_Impl {
    fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH>;
    fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()>;
    fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction>;
    fn Add(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
    ) -> ::windows::core::Result<()>;
    fn Remove(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>;
    fn Update(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>;
    fn Get(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>;
    fn GetMetadata(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>;
    fn Contains(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<u8>;
    fn Enumerate(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>;
    fn EnumerateByKey(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        keyprefix: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>;
    fn EnumerateMetadata(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>;
    fn EnumerateMetadataByKey(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        keyprefix: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica {}
impl IFabricKeyValueStoreReplica_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica_Vtbl {
        unsafe extern "system" fn GetCurrentEpoch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            currentepoch: *mut super::super::FABRIC_EPOCH,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentEpoch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentepoch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateReplicatorSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateReplicatorSettings(::core::mem::transmute_copy(&replicatorsettings))
                .into()
        }
        unsafe extern "system" fn CreateTransaction<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            valuesizeinbytes: i32,
            value: *const u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&valuesizeinbytes),
                ::core::mem::transmute_copy(&value),
            )
            .into()
        }
        unsafe extern "system" fn Remove<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            checksequencenumber: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&checksequencenumber),
            )
            .into()
        }
        unsafe extern "system" fn Update<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            valuesizeinbytes: i32,
            value: *const u8,
            checksequencenumber: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&valuesizeinbytes),
                ::core::mem::transmute_copy(&value),
                ::core::mem::transmute_copy(&checksequencenumber),
            )
            .into()
        }
        unsafe extern "system" fn Get<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadata(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contains<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            result: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Contains(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enumerate(::core::mem::transmute(&transaction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateByKey<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateByKey(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&keyprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateMetadata(::core::mem::transmute(&transaction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadataByKey<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateMetadataByKey(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&keyprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricStatefulServiceReplica_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCurrentEpoch: GetCurrentEpoch::<Identity, Impl, OFFSET>,
            UpdateReplicatorSettings: UpdateReplicatorSettings::<Identity, Impl, OFFSET>,
            CreateTransaction: CreateTransaction::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetMetadata: GetMetadata::<Identity, Impl, OFFSET>,
            Contains: Contains::<Identity, Impl, OFFSET>,
            Enumerate: Enumerate::<Identity, Impl, OFFSET>,
            EnumerateByKey: EnumerateByKey::<Identity, Impl, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, Impl, OFFSET>,
            EnumerateMetadataByKey: EnumerateMetadataByKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica2_Impl:
    Sized + IFabricStatefulServiceReplica_Impl + IFabricKeyValueStoreReplica_Impl
{
    fn Backup(&self, backupdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Restore(&self, backupdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica2 {}
impl IFabricKeyValueStoreReplica2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica2_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica2_Vtbl {
        unsafe extern "system" fn Backup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            backupdirectory: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Backup(::core::mem::transmute(&backupdirectory)).into()
        }
        unsafe extern "system" fn Restore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            backupdirectory: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore(::core::mem::transmute(&backupdirectory))
                .into()
        }
        unsafe extern "system" fn CreateTransaction2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
            transaction: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTransaction2(::core::mem::transmute_copy(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreReplica_Vtbl::new::<Identity, Impl, OFFSET>(),
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            CreateTransaction2: CreateTransaction2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica2 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica3_Impl:
    Sized
    + IFabricStatefulServiceReplica_Impl
    + IFabricKeyValueStoreReplica_Impl
    + IFabricKeyValueStoreReplica2_Impl
{
    fn BeginBackup(
        &self,
        backupdirectory: &::windows::core::PCWSTR,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: &::core::option::Option<IFabricStorePostBackupHandler>,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndBackup(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica3 {}
impl IFabricKeyValueStoreReplica3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica3_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica3_Vtbl {
        unsafe extern "system" fn BeginBackup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            backupdirectory: ::windows::core::PCWSTR,
            backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
            postbackuphandler: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginBackup(
                ::core::mem::transmute(&backupdirectory),
                ::core::mem::transmute_copy(&backupoption),
                ::core::mem::transmute(&postbackuphandler),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndBackup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndBackup(::core::mem::transmute(&context)).into()
        }
        Self {
            base__: IFabricKeyValueStoreReplica2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginBackup: BeginBackup::<Identity, Impl, OFFSET>,
            EndBackup: EndBackup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica3 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica4_Impl:
    Sized
    + IFabricStatefulServiceReplica_Impl
    + IFabricKeyValueStoreReplica_Impl
    + IFabricKeyValueStoreReplica2_Impl
    + IFabricKeyValueStoreReplica3_Impl
{
    fn BeginRestore(
        &self,
        backupdirectory: &::windows::core::PCWSTR,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestore(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica4 {}
impl IFabricKeyValueStoreReplica4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica4_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica4_Vtbl {
        unsafe extern "system" fn BeginRestore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            backupdirectory: ::windows::core::PCWSTR,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestore(
                ::core::mem::transmute(&backupdirectory),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRestore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestore(::core::mem::transmute(&context)).into()
        }
        Self {
            base__: IFabricKeyValueStoreReplica3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRestore: BeginRestore::<Identity, Impl, OFFSET>,
            EndRestore: EndRestore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica4 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica2 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica5_Impl:
    Sized
    + IFabricStatefulServiceReplica_Impl
    + IFabricKeyValueStoreReplica_Impl
    + IFabricKeyValueStoreReplica2_Impl
    + IFabricKeyValueStoreReplica3_Impl
    + IFabricKeyValueStoreReplica4_Impl
{
    fn TryAdd(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
    ) -> ::windows::core::Result<u8>;
    fn TryRemove(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>;
    fn TryUpdate(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>;
    fn TryGet(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>;
    fn TryGetMetadata(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        key: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>;
    fn EnumerateByKey2(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        keyprefix: &::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>;
    fn EnumerateMetadataByKey2(
        &self,
        transaction: &::core::option::Option<IFabricTransactionBase>,
        keyprefix: &::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica5 {}
impl IFabricKeyValueStoreReplica5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica5_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica5_Vtbl {
        unsafe extern "system" fn TryAdd<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            valuesizeinbytes: i32,
            value: *const u8,
            added: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryAdd(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&valuesizeinbytes),
                ::core::mem::transmute_copy(&value),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(added, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRemove<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            checksequencenumber: i64,
            exists: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryRemove(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&checksequencenumber),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUpdate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            valuesizeinbytes: i32,
            value: *const u8,
            checksequencenumber: i64,
            exists: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryUpdate(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
                ::core::mem::transmute_copy(&valuesizeinbytes),
                ::core::mem::transmute_copy(&value),
                ::core::mem::transmute_copy(&checksequencenumber),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGet<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryGet(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetMetadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            key: ::windows::core::PCWSTR,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryGetMetadata(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&key),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateByKey2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            strictprefix: ::windows::Win32::Foundation::BOOLEAN,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateByKey2(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&keyprefix),
                ::core::mem::transmute_copy(&strictprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadataByKey2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transaction: *mut ::core::ffi::c_void,
            keyprefix: ::windows::core::PCWSTR,
            strictprefix: ::windows::Win32::Foundation::BOOLEAN,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateMetadataByKey2(
                ::core::mem::transmute(&transaction),
                ::core::mem::transmute(&keyprefix),
                ::core::mem::transmute_copy(&strictprefix),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreReplica4_Vtbl::new::<Identity, Impl, OFFSET>(),
            TryAdd: TryAdd::<Identity, Impl, OFFSET>,
            TryRemove: TryRemove::<Identity, Impl, OFFSET>,
            TryUpdate: TryUpdate::<Identity, Impl, OFFSET>,
            TryGet: TryGet::<Identity, Impl, OFFSET>,
            TryGetMetadata: TryGetMetadata::<Identity, Impl, OFFSET>,
            EnumerateByKey2: EnumerateByKey2::<Identity, Impl, OFFSET>,
            EnumerateMetadataByKey2: EnumerateMetadataByKey2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica5 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica2 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica3 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricKeyValueStoreReplica6_Impl:
    Sized
    + IFabricStatefulServiceReplica_Impl
    + IFabricKeyValueStoreReplica_Impl
    + IFabricKeyValueStoreReplica2_Impl
    + IFabricKeyValueStoreReplica3_Impl
    + IFabricKeyValueStoreReplica4_Impl
    + IFabricKeyValueStoreReplica5_Impl
{
    fn BeginRestore2(
        &self,
        backupdirectory: &::windows::core::PCWSTR,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
}
impl ::windows::core::RuntimeName for IFabricKeyValueStoreReplica6 {}
impl IFabricKeyValueStoreReplica6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricKeyValueStoreReplica6_Impl,
        const OFFSET: isize,
    >() -> IFabricKeyValueStoreReplica6_Vtbl {
        unsafe extern "system" fn BeginRestore2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricKeyValueStoreReplica6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            backupdirectory: ::windows::core::PCWSTR,
            settings: *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestore2(
                ::core::mem::transmute(&backupdirectory),
                ::core::mem::transmute_copy(&settings),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricKeyValueStoreReplica5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRestore2: BeginRestore2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricKeyValueStoreReplica6 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica2 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica3 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica4 as ::windows::core::Interface>::IID
            || iid == &<IFabricKeyValueStoreReplica5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNodeContextResult_Impl: Sized {
    fn get_NodeContext(&self) -> *mut super::super::FABRIC_NODE_CONTEXT;
}
impl ::windows::core::RuntimeName for IFabricNodeContextResult {}
impl IFabricNodeContextResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeContextResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeContextResult_Vtbl {
        unsafe extern "system" fn get_NodeContext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeContextResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_CONTEXT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeContext()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeContext: get_NodeContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNodeContextResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNodeContextResult2_Impl: Sized + IFabricNodeContextResult_Impl {
    fn GetDirectory(
        &self,
        logicaldirectoryname: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricNodeContextResult2 {}
impl IFabricNodeContextResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeContextResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeContextResult2_Vtbl {
        unsafe extern "system" fn GetDirectory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeContextResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            logicaldirectoryname: ::windows::core::PCWSTR,
            directorypath: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectory(::core::mem::transmute(&logicaldirectoryname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(directorypath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricNodeContextResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDirectory: GetDirectory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNodeContextResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricNodeContextResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOperation_Impl: Sized {
    fn get_Metadata(&self) -> *mut super::super::FABRIC_OPERATION_METADATA;
    fn GetData(
        &self,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::Result<()>;
    fn Acknowledge(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricOperation {}
impl IFabricOperation_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOperation_Impl,
        const OFFSET: isize,
    >() -> IFabricOperation_Vtbl {
        unsafe extern "system" fn get_Metadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_OPERATION_METADATA {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Metadata()
        }
        unsafe extern "system" fn GetData<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: *mut u32,
            buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(
                ::core::mem::transmute_copy(&count),
                ::core::mem::transmute_copy(&buffers),
            )
            .into()
        }
        unsafe extern "system" fn Acknowledge<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acknowledge().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Metadata: get_Metadata::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            Acknowledge: Acknowledge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOperation as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOperationData_Impl: Sized {
    fn GetData(
        &self,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricOperationData {}
impl IFabricOperationData_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOperationData_Impl,
        const OFFSET: isize,
    >() -> IFabricOperationData_Vtbl {
        unsafe extern "system" fn GetData<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationData_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: *mut u32,
            buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(
                ::core::mem::transmute_copy(&count),
                ::core::mem::transmute_copy(&buffers),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOperationData as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOperationDataStream_Impl: Sized {
    fn BeginGetNext(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNext(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricOperationData>;
}
impl ::windows::core::RuntimeName for IFabricOperationDataStream {}
impl IFabricOperationDataStream_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOperationDataStream_Impl,
        const OFFSET: isize,
    >() -> IFabricOperationDataStream_Vtbl {
        unsafe extern "system" fn BeginGetNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationDataStream_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNext(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetNext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationDataStream_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            operationdata: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNext(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(operationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetNext: BeginGetNext::<Identity, Impl, OFFSET>,
            EndGetNext: EndGetNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOperationDataStream as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOperationStream_Impl: Sized {
    fn BeginGetOperation(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetOperation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricOperation>;
}
impl ::windows::core::RuntimeName for IFabricOperationStream {}
impl IFabricOperationStream_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOperationStream_Impl,
        const OFFSET: isize,
    >() -> IFabricOperationStream_Vtbl {
        unsafe extern "system" fn BeginGetOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationStream_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetOperation(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationStream_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            operation: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetOperation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(operation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetOperation: BeginGetOperation::<Identity, Impl, OFFSET>,
            EndGetOperation: EndGetOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOperationStream as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOperationStream2_Impl: Sized + IFabricOperationStream_Impl {
    fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricOperationStream2 {}
impl IFabricOperationStream2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOperationStream2_Impl,
        const OFFSET: isize,
    >() -> IFabricOperationStream2_Vtbl {
        unsafe extern "system" fn ReportFault<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOperationStream2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            faulttype: super::super::FABRIC_FAULT_TYPE,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportFault(::core::mem::transmute_copy(&faulttype))
                .into()
        }
        Self {
            base__: IFabricOperationStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportFault: ReportFault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOperationStream2 as ::windows::core::Interface>::IID
            || iid == &<IFabricOperationStream as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPrimaryReplicator_Impl: Sized + IFabricReplicator_Impl {
    fn BeginOnDataLoss(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOnDataLoss(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<u8>;
    fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows::core::Result<()>;
    fn BeginWaitForCatchUpQuorum(
        &self,
        catchupmode: super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndWaitForCatchUpQuorum(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows::core::Result<()>;
    fn BeginBuildReplica(
        &self,
        replica: *const super::super::FABRIC_REPLICA_INFORMATION,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndBuildReplica(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn RemoveReplica(&self, replicaid: i64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricPrimaryReplicator {}
impl IFabricPrimaryReplicator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPrimaryReplicator_Impl,
        const OFFSET: isize,
    >() -> IFabricPrimaryReplicator_Vtbl {
        unsafe extern "system" fn BeginOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOnDataLoss(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            isstatechanged: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOnDataLoss(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isstatechanged, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCatchUpReplicaSetConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
            previousconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCatchUpReplicaSetConfiguration(
                ::core::mem::transmute_copy(&currentconfiguration),
                ::core::mem::transmute_copy(&previousconfiguration),
            )
            .into()
        }
        unsafe extern "system" fn BeginWaitForCatchUpQuorum<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            catchupmode: super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginWaitForCatchUpQuorum(
                ::core::mem::transmute_copy(&catchupmode),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndWaitForCatchUpQuorum<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndWaitForCatchUpQuorum(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn UpdateCurrentReplicaSetConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCurrentReplicaSetConfiguration(::core::mem::transmute_copy(
                &currentconfiguration,
            ))
            .into()
        }
        unsafe extern "system" fn BeginBuildReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            replica: *const super::super::FABRIC_REPLICA_INFORMATION,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginBuildReplica(
                ::core::mem::transmute_copy(&replica),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndBuildReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndBuildReplica(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn RemoveReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPrimaryReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            replicaid: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveReplica(::core::mem::transmute_copy(&replicaid))
                .into()
        }
        Self {
            base__: IFabricReplicator_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginOnDataLoss: BeginOnDataLoss::<Identity, Impl, OFFSET>,
            EndOnDataLoss: EndOnDataLoss::<Identity, Impl, OFFSET>,
            UpdateCatchUpReplicaSetConfiguration: UpdateCatchUpReplicaSetConfiguration::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginWaitForCatchUpQuorum: BeginWaitForCatchUpQuorum::<Identity, Impl, OFFSET>,
            EndWaitForCatchUpQuorum: EndWaitForCatchUpQuorum::<Identity, Impl, OFFSET>,
            UpdateCurrentReplicaSetConfiguration: UpdateCurrentReplicaSetConfiguration::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginBuildReplica: BeginBuildReplica::<Identity, Impl, OFFSET>,
            EndBuildReplica: EndBuildReplica::<Identity, Impl, OFFSET>,
            RemoveReplica: RemoveReplica::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPrimaryReplicator as ::windows::core::Interface>::IID
            || iid == &<IFabricReplicator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricProcessExitHandler_Impl: Sized {
    fn FabricProcessExited(&self);
}
impl ::windows::core::RuntimeName for IFabricProcessExitHandler {}
impl IFabricProcessExitHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricProcessExitHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricProcessExitHandler_Vtbl {
        unsafe extern "system" fn FabricProcessExited<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricProcessExitHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FabricProcessExited()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FabricProcessExited: FabricProcessExited::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricProcessExitHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricReplicator_Impl: Sized {
    fn BeginOpen(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOpen(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginChangeRole(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndChangeRole(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUpdateEpoch(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateEpoch(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginClose(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndClose(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn Abort(&self);
    fn GetCurrentProgress(&self) -> ::windows::core::Result<i64>;
    fn GetCatchUpCapability(&self) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IFabricReplicator {}
impl IFabricReplicator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricReplicator_Impl,
        const OFFSET: isize,
    >() -> IFabricReplicator_Vtbl {
        unsafe extern "system" fn BeginOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
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
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            replicationaddress: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOpen(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(replicationaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginChangeRole<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            epoch: *const super::super::FABRIC_EPOCH,
            role: super::super::FABRIC_REPLICA_ROLE,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginChangeRole(
                ::core::mem::transmute_copy(&epoch),
                ::core::mem::transmute_copy(&role),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndChangeRole<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndChangeRole(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginUpdateEpoch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            epoch: *const super::super::FABRIC_EPOCH,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateEpoch(
                ::core::mem::transmute_copy(&epoch),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateEpoch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateEpoch(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
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
            Impl: IFabricReplicator_Impl,
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
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort()
        }
        unsafe extern "system" fn GetCurrentProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            lastsequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentProgress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastsequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCatchUpCapability<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fromsequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCatchUpCapability() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fromsequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginOpen: BeginOpen::<Identity, Impl, OFFSET>,
            EndOpen: EndOpen::<Identity, Impl, OFFSET>,
            BeginChangeRole: BeginChangeRole::<Identity, Impl, OFFSET>,
            EndChangeRole: EndChangeRole::<Identity, Impl, OFFSET>,
            BeginUpdateEpoch: BeginUpdateEpoch::<Identity, Impl, OFFSET>,
            EndUpdateEpoch: EndUpdateEpoch::<Identity, Impl, OFFSET>,
            BeginClose: BeginClose::<Identity, Impl, OFFSET>,
            EndClose: EndClose::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetCurrentProgress: GetCurrentProgress::<Identity, Impl, OFFSET>,
            GetCatchUpCapability: GetCatchUpCapability::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricReplicator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricReplicatorCatchupSpecificQuorum_Impl: Sized {}
impl ::windows::core::RuntimeName for IFabricReplicatorCatchupSpecificQuorum {}
impl IFabricReplicatorCatchupSpecificQuorum_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricReplicatorCatchupSpecificQuorum_Impl,
        const OFFSET: isize,
    >() -> IFabricReplicatorCatchupSpecificQuorum_Vtbl {
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricReplicatorCatchupSpecificQuorum as ::windows::core::Interface>::IID
    }
}
pub trait IFabricReplicatorSettingsResult_Impl: Sized {
    fn get_ReplicatorSettings(&self) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS;
}
impl ::windows::core::RuntimeName for IFabricReplicatorSettingsResult {}
impl IFabricReplicatorSettingsResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricReplicatorSettingsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricReplicatorSettingsResult_Vtbl {
        unsafe extern "system" fn get_ReplicatorSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicatorSettingsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicatorSettings()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicatorSettings: get_ReplicatorSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricReplicatorSettingsResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricRuntime_Impl: Sized {
    fn BeginRegisterStatelessServiceFactory(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatelessServiceFactory>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRegisterStatelessServiceFactory(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn RegisterStatelessServiceFactory(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatelessServiceFactory>,
    ) -> ::windows::core::Result<()>;
    fn BeginRegisterStatefulServiceFactory(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatefulServiceFactory>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRegisterStatefulServiceFactory(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn RegisterStatefulServiceFactory(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatefulServiceFactory>,
    ) -> ::windows::core::Result<()>;
    fn CreateServiceGroupFactoryBuilder(
        &self,
    ) -> ::windows::core::Result<IFabricServiceGroupFactoryBuilder>;
    fn BeginRegisterServiceGroupFactory(
        &self,
        groupservicetype: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricServiceGroupFactory>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRegisterServiceGroupFactory(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn RegisterServiceGroupFactory(
        &self,
        groupservicetype: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricServiceGroupFactory>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricRuntime {}
impl IFabricRuntime_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRuntime_Impl,
        const OFFSET: isize,
    >() -> IFabricRuntime_Vtbl {
        unsafe extern "system" fn BeginRegisterStatelessServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRegisterStatelessServiceFactory(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute(&factory),
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
        unsafe extern "system" fn EndRegisterStatelessServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRegisterStatelessServiceFactory(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn RegisterStatelessServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterStatelessServiceFactory(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute(&factory),
            )
            .into()
        }
        unsafe extern "system" fn BeginRegisterStatefulServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRegisterStatefulServiceFactory(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute(&factory),
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
        unsafe extern "system" fn EndRegisterStatefulServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRegisterStatefulServiceFactory(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn RegisterStatefulServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterStatefulServiceFactory(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute(&factory),
            )
            .into()
        }
        unsafe extern "system" fn CreateServiceGroupFactoryBuilder<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            builder: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateServiceGroupFactoryBuilder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(builder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRegisterServiceGroupFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            groupservicetype: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRegisterServiceGroupFactory(
                ::core::mem::transmute(&groupservicetype),
                ::core::mem::transmute(&factory),
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
        unsafe extern "system" fn EndRegisterServiceGroupFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRegisterServiceGroupFactory(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn RegisterServiceGroupFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRuntime_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            groupservicetype: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterServiceGroupFactory(
                ::core::mem::transmute(&groupservicetype),
                ::core::mem::transmute(&factory),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRegisterStatelessServiceFactory: BeginRegisterStatelessServiceFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRegisterStatelessServiceFactory: EndRegisterStatelessServiceFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            RegisterStatelessServiceFactory: RegisterStatelessServiceFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginRegisterStatefulServiceFactory: BeginRegisterStatefulServiceFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRegisterStatefulServiceFactory: EndRegisterStatefulServiceFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            RegisterStatefulServiceFactory: RegisterStatefulServiceFactory::<Identity, Impl, OFFSET>,
            CreateServiceGroupFactoryBuilder: CreateServiceGroupFactoryBuilder::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginRegisterServiceGroupFactory: BeginRegisterServiceGroupFactory::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRegisterServiceGroupFactory: EndRegisterServiceGroupFactory::<Identity, Impl, OFFSET>,
            RegisterServiceGroupFactory: RegisterServiceGroupFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricRuntime as ::windows::core::Interface>::IID
    }
}
pub trait IFabricSecondaryEventHandler_Impl: Sized {
    fn OnCopyComplete(
        &self,
        enumerator: &::core::option::Option<IFabricKeyValueStoreEnumerator>,
    ) -> ::windows::core::Result<()>;
    fn OnReplicationOperation(
        &self,
        enumerator: &::core::option::Option<IFabricKeyValueStoreNotificationEnumerator>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricSecondaryEventHandler {}
impl IFabricSecondaryEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecondaryEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricSecondaryEventHandler_Vtbl {
        unsafe extern "system" fn OnCopyComplete<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecondaryEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            enumerator: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCopyComplete(::core::mem::transmute(&enumerator))
                .into()
        }
        unsafe extern "system" fn OnReplicationOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecondaryEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            enumerator: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnReplicationOperation(::core::mem::transmute(&enumerator))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCopyComplete: OnCopyComplete::<Identity, Impl, OFFSET>,
            OnReplicationOperation: OnReplicationOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricSecondaryEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricSecurityCredentialsResult_Impl: Sized {
    fn get_SecurityCredentials(&self) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS;
}
impl ::windows::core::RuntimeName for IFabricSecurityCredentialsResult {}
impl IFabricSecurityCredentialsResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecurityCredentialsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricSecurityCredentialsResult_Vtbl {
        unsafe extern "system" fn get_SecurityCredentials<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecurityCredentialsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_SecurityCredentials()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_SecurityCredentials: get_SecurityCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricSecurityCredentialsResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupFactory_Impl: Sized {}
impl ::windows::core::RuntimeName for IFabricServiceGroupFactory {}
impl IFabricServiceGroupFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupFactory_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupFactory_Vtbl {
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupFactory as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupFactoryBuilder_Impl: Sized {
    fn AddStatelessServiceFactory(
        &self,
        memberservicetype: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatelessServiceFactory>,
    ) -> ::windows::core::Result<()>;
    fn AddStatefulServiceFactory(
        &self,
        memberservicetype: &::windows::core::PCWSTR,
        factory: &::core::option::Option<IFabricStatefulServiceFactory>,
    ) -> ::windows::core::Result<()>;
    fn RemoveServiceFactory(
        &self,
        memberservicetype: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
    fn ToServiceGroupFactory(&self) -> ::windows::core::Result<IFabricServiceGroupFactory>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupFactoryBuilder {}
impl IFabricServiceGroupFactoryBuilder_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupFactoryBuilder_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupFactoryBuilder_Vtbl {
        unsafe extern "system" fn AddStatelessServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupFactoryBuilder_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            memberservicetype: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStatelessServiceFactory(
                ::core::mem::transmute(&memberservicetype),
                ::core::mem::transmute(&factory),
            )
            .into()
        }
        unsafe extern "system" fn AddStatefulServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupFactoryBuilder_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            memberservicetype: ::windows::core::PCWSTR,
            factory: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStatefulServiceFactory(
                ::core::mem::transmute(&memberservicetype),
                ::core::mem::transmute(&factory),
            )
            .into()
        }
        unsafe extern "system" fn RemoveServiceFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupFactoryBuilder_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            memberservicetype: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveServiceFactory(::core::mem::transmute(&memberservicetype))
                .into()
        }
        unsafe extern "system" fn ToServiceGroupFactory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupFactoryBuilder_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            factory: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToServiceGroupFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddStatelessServiceFactory: AddStatelessServiceFactory::<Identity, Impl, OFFSET>,
            AddStatefulServiceFactory: AddStatefulServiceFactory::<Identity, Impl, OFFSET>,
            RemoveServiceFactory: RemoveServiceFactory::<Identity, Impl, OFFSET>,
            ToServiceGroupFactory: ToServiceGroupFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupFactoryBuilder as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupPartition_Impl: Sized {
    fn ResolveMember(
        &self,
        name: *const u16,
        riid: *const ::windows::core::GUID,
    ) -> ::windows::core::Result<*mut ::core::ffi::c_void>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupPartition {}
impl IFabricServiceGroupPartition_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupPartition_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupPartition_Vtbl {
        unsafe extern "system" fn ResolveMember<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupPartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            riid: *const ::windows::core::GUID,
            member: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResolveMember(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&riid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(member, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResolveMember: ResolveMember::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupPartition as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStateProvider_Impl: Sized {
    fn BeginUpdateEpoch(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateEpoch(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn GetLastCommittedSequenceNumber(&self) -> ::windows::core::Result<i64>;
    fn BeginOnDataLoss(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOnDataLoss(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<u8>;
    fn GetCopyContext(&self) -> ::windows::core::Result<IFabricOperationDataStream>;
    fn GetCopyState(
        &self,
        uptosequencenumber: i64,
        copycontextstream: &::core::option::Option<IFabricOperationDataStream>,
    ) -> ::windows::core::Result<IFabricOperationDataStream>;
}
impl ::windows::core::RuntimeName for IFabricStateProvider {}
impl IFabricStateProvider_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStateProvider_Impl,
        const OFFSET: isize,
    >() -> IFabricStateProvider_Vtbl {
        unsafe extern "system" fn BeginUpdateEpoch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            epoch: *const super::super::FABRIC_EPOCH,
            previousepochlastsequencenumber: i64,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateEpoch(
                ::core::mem::transmute_copy(&epoch),
                ::core::mem::transmute_copy(&previousepochlastsequencenumber),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdateEpoch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateEpoch(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn GetLastCommittedSequenceNumber<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastCommittedSequenceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOnDataLoss(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            isstatechanged: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOnDataLoss(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isstatechanged, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCopyContext<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            copycontextstream: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCopyContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copycontextstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCopyState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            uptosequencenumber: i64,
            copycontextstream: *mut ::core::ffi::c_void,
            copystatestream: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCopyState(
                ::core::mem::transmute_copy(&uptosequencenumber),
                ::core::mem::transmute(&copycontextstream),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copystatestream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUpdateEpoch: BeginUpdateEpoch::<Identity, Impl, OFFSET>,
            EndUpdateEpoch: EndUpdateEpoch::<Identity, Impl, OFFSET>,
            GetLastCommittedSequenceNumber: GetLastCommittedSequenceNumber::<Identity, Impl, OFFSET>,
            BeginOnDataLoss: BeginOnDataLoss::<Identity, Impl, OFFSET>,
            EndOnDataLoss: EndOnDataLoss::<Identity, Impl, OFFSET>,
            GetCopyContext: GetCopyContext::<Identity, Impl, OFFSET>,
            GetCopyState: GetCopyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStateProvider as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStateReplicator_Impl: Sized {
    fn BeginReplicate(
        &self,
        operationdata: &::core::option::Option<IFabricOperationData>,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
        sequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn EndReplicate(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn GetReplicationStream(&self) -> ::windows::core::Result<IFabricOperationStream>;
    fn GetCopyStream(&self) -> ::windows::core::Result<IFabricOperationStream>;
    fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStateReplicator {}
impl IFabricStateReplicator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStateReplicator_Impl,
        const OFFSET: isize,
    >() -> IFabricStateReplicator_Vtbl {
        unsafe extern "system" fn BeginReplicate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationdata: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            sequencenumber: *mut i64,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginReplicate(
                ::core::mem::transmute(&operationdata),
                ::core::mem::transmute(&callback),
                ::core::mem::transmute_copy(&sequencenumber),
                ::core::mem::transmute_copy(&context),
            )
            .into()
        }
        unsafe extern "system" fn EndReplicate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            sequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndReplicate(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReplicationStream<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            stream: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReplicationStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCopyStream<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            stream: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCopyStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateReplicatorSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateReplicatorSettings(::core::mem::transmute_copy(&replicatorsettings))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginReplicate: BeginReplicate::<Identity, Impl, OFFSET>,
            EndReplicate: EndReplicate::<Identity, Impl, OFFSET>,
            GetReplicationStream: GetReplicationStream::<Identity, Impl, OFFSET>,
            GetCopyStream: GetCopyStream::<Identity, Impl, OFFSET>,
            UpdateReplicatorSettings: UpdateReplicatorSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStateReplicator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStateReplicator2_Impl: Sized + IFabricStateReplicator_Impl {
    fn GetReplicatorSettings(&self) -> ::windows::core::Result<IFabricReplicatorSettingsResult>;
}
impl ::windows::core::RuntimeName for IFabricStateReplicator2 {}
impl IFabricStateReplicator2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStateReplicator2_Impl,
        const OFFSET: isize,
    >() -> IFabricStateReplicator2_Vtbl {
        unsafe extern "system" fn GetReplicatorSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStateReplicator2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            replicatorsettings: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReplicatorSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(replicatorsettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricStateReplicator_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetReplicatorSettings: GetReplicatorSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStateReplicator2 as ::windows::core::Interface>::IID
            || iid == &<IFabricStateReplicator as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServiceFactory_Impl: Sized {
    fn CreateReplica(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows::core::GUID,
        replicaid: i64,
    ) -> ::windows::core::Result<IFabricStatefulServiceReplica>;
}
impl ::windows::core::RuntimeName for IFabricStatefulServiceFactory {}
impl IFabricStatefulServiceFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServiceFactory_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServiceFactory_Vtbl {
        unsafe extern "system" fn CreateReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServiceFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            servicename: *const u16,
            initializationdatalength: u32,
            initializationdata: *const u8,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            servicereplica: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateReplica(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute_copy(&initializationdatalength),
                ::core::mem::transmute_copy(&initializationdata),
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&replicaid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicereplica, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateReplica: CreateReplica::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServiceFactory as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServicePartition_Impl: Sized {
    fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>;
    fn GetReadStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>;
    fn GetWriteStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>;
    fn CreateReplicator(
        &self,
        stateprovider: &::core::option::Option<IFabricStateProvider>,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
        statereplicator: *mut ::core::option::Option<IFabricStateReplicator>,
    ) -> ::windows::core::Result<()>;
    fn ReportLoad(
        &self,
        metriccount: u32,
        metrics: *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows::core::Result<()>;
    fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatefulServicePartition {}
impl IFabricStatefulServicePartition_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServicePartition_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServicePartition_Vtbl {
        unsafe extern "system" fn GetPartitionInfo<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bufferedvalue: *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            readstatus: *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            writestatus: *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriteStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(writestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReplicator<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            stateprovider: *mut ::core::ffi::c_void,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            replicator: *mut *mut ::core::ffi::c_void,
            statereplicator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateReplicator(
                ::core::mem::transmute(&stateprovider),
                ::core::mem::transmute_copy(&replicatorsettings),
                ::core::mem::transmute_copy(&replicator),
                ::core::mem::transmute_copy(&statereplicator),
            )
            .into()
        }
        unsafe extern "system" fn ReportLoad<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            metriccount: u32,
            metrics: *const super::super::FABRIC_LOAD_METRIC,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportLoad(
                ::core::mem::transmute_copy(&metriccount),
                ::core::mem::transmute_copy(&metrics),
            )
            .into()
        }
        unsafe extern "system" fn ReportFault<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            faulttype: super::super::FABRIC_FAULT_TYPE,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportFault(::core::mem::transmute_copy(&faulttype))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionInfo: GetPartitionInfo::<Identity, Impl, OFFSET>,
            GetReadStatus: GetReadStatus::<Identity, Impl, OFFSET>,
            GetWriteStatus: GetWriteStatus::<Identity, Impl, OFFSET>,
            CreateReplicator: CreateReplicator::<Identity, Impl, OFFSET>,
            ReportLoad: ReportLoad::<Identity, Impl, OFFSET>,
            ReportFault: ReportFault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServicePartition as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServicePartition1_Impl:
    Sized + IFabricStatefulServicePartition_Impl
{
    fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatefulServicePartition1 {}
impl IFabricStatefulServicePartition1_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServicePartition1_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServicePartition1_Vtbl {
        unsafe extern "system" fn ReportMoveCost<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            movecost: super::super::FABRIC_MOVE_COST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportMoveCost(::core::mem::transmute_copy(&movecost))
                .into()
        }
        Self {
            base__: IFabricStatefulServicePartition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportMoveCost: ReportMoveCost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServicePartition1 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServicePartition2_Impl:
    Sized + IFabricStatefulServicePartition_Impl + IFabricStatefulServicePartition1_Impl
{
    fn ReportReplicaHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
    fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatefulServicePartition2 {}
impl IFabricStatefulServicePartition2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServicePartition2_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServicePartition2_Vtbl {
        unsafe extern "system" fn ReportReplicaHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportReplicaHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        unsafe extern "system" fn ReportPartitionHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportPartitionHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        Self {
            base__: IFabricStatefulServicePartition1_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportReplicaHealth: ReportReplicaHealth::<Identity, Impl, OFFSET>,
            ReportPartitionHealth: ReportPartitionHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServicePartition2 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition1 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServicePartition3_Impl:
    Sized
    + IFabricStatefulServicePartition_Impl
    + IFabricStatefulServicePartition1_Impl
    + IFabricStatefulServicePartition2_Impl
{
    fn ReportReplicaHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
    fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatefulServicePartition3 {}
impl IFabricStatefulServicePartition3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServicePartition3_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServicePartition3_Vtbl {
        unsafe extern "system" fn ReportReplicaHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportReplicaHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        unsafe extern "system" fn ReportPartitionHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServicePartition3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportPartitionHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        Self {
            base__: IFabricStatefulServicePartition2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportReplicaHealth2: ReportReplicaHealth2::<Identity, Impl, OFFSET>,
            ReportPartitionHealth2: ReportPartitionHealth2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServicePartition3 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition1 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatefulServicePartition2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatefulServiceReplica_Impl: Sized {
    fn BeginOpen(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: &::core::option::Option<IFabricStatefulServicePartition>,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOpen(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricReplicator>;
    fn BeginChangeRole(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndChangeRole(
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
impl ::windows::core::RuntimeName for IFabricStatefulServiceReplica {}
impl IFabricStatefulServiceReplica_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatefulServiceReplica_Impl,
        const OFFSET: isize,
    >() -> IFabricStatefulServiceReplica_Vtbl {
        unsafe extern "system" fn BeginOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServiceReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
            partition: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOpen(
                ::core::mem::transmute_copy(&openmode),
                ::core::mem::transmute(&partition),
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
            Impl: IFabricStatefulServiceReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            replicator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOpen(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(replicator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginChangeRole<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServiceReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            newrole: super::super::FABRIC_REPLICA_ROLE,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginChangeRole(
                ::core::mem::transmute_copy(&newrole),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndChangeRole<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServiceReplica_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            serviceaddress: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndChangeRole(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serviceaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginClose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatefulServiceReplica_Impl,
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
            Impl: IFabricStatefulServiceReplica_Impl,
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
            Impl: IFabricStatefulServiceReplica_Impl,
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
            BeginChangeRole: BeginChangeRole::<Identity, Impl, OFFSET>,
            EndChangeRole: EndChangeRole::<Identity, Impl, OFFSET>,
            BeginClose: BeginClose::<Identity, Impl, OFFSET>,
            EndClose: EndClose::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatefulServiceReplica as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServiceFactory_Impl: Sized {
    fn CreateInstance(
        &self,
        servicetypename: &::windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: &::windows::core::GUID,
        instanceid: i64,
    ) -> ::windows::core::Result<IFabricStatelessServiceInstance>;
}
impl ::windows::core::RuntimeName for IFabricStatelessServiceFactory {}
impl IFabricStatelessServiceFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServiceFactory_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServiceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServiceFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicetypename: ::windows::core::PCWSTR,
            servicename: *const u16,
            initializationdatalength: u32,
            initializationdata: *const u8,
            partitionid: ::windows::core::GUID,
            instanceid: i64,
            serviceinstance: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute_copy(&initializationdatalength),
                ::core::mem::transmute_copy(&initializationdata),
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&instanceid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serviceinstance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatelessServiceFactory as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServiceInstance_Impl: Sized {
    fn BeginOpen(
        &self,
        partition: &::core::option::Option<IFabricStatelessServicePartition>,
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
impl ::windows::core::RuntimeName for IFabricStatelessServiceInstance {}
impl IFabricStatelessServiceInstance_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServiceInstance_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServiceInstance_Vtbl {
        unsafe extern "system" fn BeginOpen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServiceInstance_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partition: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOpen(
                ::core::mem::transmute(&partition),
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
            Impl: IFabricStatelessServiceInstance_Impl,
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
            Impl: IFabricStatelessServiceInstance_Impl,
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
            Impl: IFabricStatelessServiceInstance_Impl,
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
            Impl: IFabricStatelessServiceInstance_Impl,
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
        iid == &<IFabricStatelessServiceInstance as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServicePartition_Impl: Sized {
    fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>;
    fn ReportLoad(
        &self,
        metriccount: u32,
        metrics: *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows::core::Result<()>;
    fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatelessServicePartition {}
impl IFabricStatelessServicePartition_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServicePartition_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServicePartition_Vtbl {
        unsafe extern "system" fn GetPartitionInfo<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bufferedvalue: *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLoad<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            metriccount: u32,
            metrics: *const super::super::FABRIC_LOAD_METRIC,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportLoad(
                ::core::mem::transmute_copy(&metriccount),
                ::core::mem::transmute_copy(&metrics),
            )
            .into()
        }
        unsafe extern "system" fn ReportFault<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            faulttype: super::super::FABRIC_FAULT_TYPE,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportFault(::core::mem::transmute_copy(&faulttype))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionInfo: GetPartitionInfo::<Identity, Impl, OFFSET>,
            ReportLoad: ReportLoad::<Identity, Impl, OFFSET>,
            ReportFault: ReportFault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatelessServicePartition as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServicePartition1_Impl:
    Sized + IFabricStatelessServicePartition_Impl
{
    fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatelessServicePartition1 {}
impl IFabricStatelessServicePartition1_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServicePartition1_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServicePartition1_Vtbl {
        unsafe extern "system" fn ReportMoveCost<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            movecost: super::super::FABRIC_MOVE_COST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportMoveCost(::core::mem::transmute_copy(&movecost))
                .into()
        }
        Self {
            base__: IFabricStatelessServicePartition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportMoveCost: ReportMoveCost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatelessServicePartition1 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServicePartition2_Impl:
    Sized + IFabricStatelessServicePartition_Impl + IFabricStatelessServicePartition1_Impl
{
    fn ReportInstanceHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
    fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatelessServicePartition2 {}
impl IFabricStatelessServicePartition2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServicePartition2_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServicePartition2_Vtbl {
        unsafe extern "system" fn ReportInstanceHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportInstanceHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        unsafe extern "system" fn ReportPartitionHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportPartitionHealth(::core::mem::transmute_copy(&healthinfo))
                .into()
        }
        Self {
            base__: IFabricStatelessServicePartition1_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportInstanceHealth: ReportInstanceHealth::<Identity, Impl, OFFSET>,
            ReportPartitionHealth: ReportPartitionHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatelessServicePartition2 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition1 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStatelessServicePartition3_Impl:
    Sized
    + IFabricStatelessServicePartition_Impl
    + IFabricStatelessServicePartition1_Impl
    + IFabricStatelessServicePartition2_Impl
{
    fn ReportInstanceHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
    fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricStatelessServicePartition3 {}
impl IFabricStatelessServicePartition3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStatelessServicePartition3_Impl,
        const OFFSET: isize,
    >() -> IFabricStatelessServicePartition3_Vtbl {
        unsafe extern "system" fn ReportInstanceHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportInstanceHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        unsafe extern "system" fn ReportPartitionHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStatelessServicePartition3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportPartitionHealth2(
                ::core::mem::transmute_copy(&healthinfo),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        Self {
            base__: IFabricStatelessServicePartition2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportInstanceHealth2: ReportInstanceHealth2::<Identity, Impl, OFFSET>,
            ReportPartitionHealth2: ReportPartitionHealth2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStatelessServicePartition3 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition1 as ::windows::core::Interface>::IID
            || iid == &<IFabricStatelessServicePartition2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStoreEventHandler_Impl: Sized {
    fn OnDataLoss(&self);
}
impl ::windows::core::RuntimeName for IFabricStoreEventHandler {}
impl IFabricStoreEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStoreEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricStoreEventHandler_Vtbl {
        unsafe extern "system" fn OnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStoreEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataLoss()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataLoss: OnDataLoss::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStoreEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStoreEventHandler2_Impl: Sized + IFabricStoreEventHandler_Impl {
    fn BeginOnDataLoss(
        &self,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndOnDataLoss(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IFabricStoreEventHandler2 {}
impl IFabricStoreEventHandler2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStoreEventHandler2_Impl,
        const OFFSET: isize,
    >() -> IFabricStoreEventHandler2_Vtbl {
        unsafe extern "system" fn BeginOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStoreEventHandler2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginOnDataLoss(::core::mem::transmute(&callback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOnDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStoreEventHandler2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            isstatechanged: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOnDataLoss(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isstatechanged, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricStoreEventHandler_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginOnDataLoss: BeginOnDataLoss::<Identity, Impl, OFFSET>,
            EndOnDataLoss: EndOnDataLoss::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStoreEventHandler2 as ::windows::core::Interface>::IID
            || iid == &<IFabricStoreEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStorePostBackupHandler_Impl: Sized {
    fn BeginPostBackup(
        &self,
        info: *const super::super::FABRIC_STORE_BACKUP_INFO,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPostBackup(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IFabricStorePostBackupHandler {}
impl IFabricStorePostBackupHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStorePostBackupHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricStorePostBackupHandler_Vtbl {
        unsafe extern "system" fn BeginPostBackup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStorePostBackupHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            info: *const super::super::FABRIC_STORE_BACKUP_INFO,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPostBackup(
                ::core::mem::transmute_copy(&info),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPostBackup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStorePostBackupHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            status: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPostBackup(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginPostBackup: BeginPostBackup::<Identity, Impl, OFFSET>,
            EndPostBackup: EndPostBackup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStorePostBackupHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransaction_Impl: Sized + IFabricTransactionBase_Impl {
    fn BeginCommit(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCommit(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn Rollback(&self);
}
impl ::windows::core::RuntimeName for IFabricTransaction {}
impl IFabricTransaction_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransaction_Impl,
        const OFFSET: isize,
    >() -> IFabricTransaction_Vtbl {
        unsafe extern "system" fn BeginCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransaction_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCommit(
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
        unsafe extern "system" fn EndCommit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransaction_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitsequencenumber: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndCommit(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitsequencenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rollback<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransaction_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rollback()
        }
        Self {
            base__: IFabricTransactionBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCommit: BeginCommit::<Identity, Impl, OFFSET>,
            EndCommit: EndCommit::<Identity, Impl, OFFSET>,
            Rollback: Rollback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransaction as ::windows::core::Interface>::IID
            || iid == &<IFabricTransactionBase as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTransactionBase_Impl: Sized {
    fn get_Id(&self) -> *mut ::windows::core::GUID;
    fn get_IsolationLevel(&self) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL;
}
impl ::windows::core::RuntimeName for IFabricTransactionBase {}
impl IFabricTransactionBase_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTransactionBase_Impl,
        const OFFSET: isize,
    >() -> IFabricTransactionBase_Vtbl {
        unsafe extern "system" fn get_Id<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransactionBase_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut ::windows::core::GUID {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Id()
        }
        unsafe extern "system" fn get_IsolationLevel<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTransactionBase_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_IsolationLevel()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Id: get_Id::<Identity, Impl, OFFSET>,
            get_IsolationLevel: get_IsolationLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTransactionBase as ::windows::core::Interface>::IID
    }
}
