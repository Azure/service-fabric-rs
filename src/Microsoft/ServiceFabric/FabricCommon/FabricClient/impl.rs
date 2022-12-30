pub trait IFabricApplicationHealthResult_Impl: Sized {
    fn get_ApplicationHealth(&self) -> *mut super::super::FABRIC_APPLICATION_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricApplicationHealthResult {}
impl IFabricApplicationHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationHealthResult_Vtbl {
        unsafe extern "system" fn get_ApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationHealth: get_ApplicationHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient_Impl: Sized {
    fn BeginProvisionApplicationType(
        &self,
        applicationbuildpath: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginCreateApplication(
        &self,
        description: *const super::super::FABRIC_APPLICATION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateApplication(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUpgradeApplication(
        &self,
        upgradedescription: *const super::super::FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeApplication(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetApplicationUpgradeProgress(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationUpgradeProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricApplicationUpgradeProgressResult2>;
    fn BeginMoveNextApplicationUpgradeDomain(
        &self,
        progress: &::core::option::Option<IFabricApplicationUpgradeProgressResult2>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextApplicationUpgradeDomain(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteApplication(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteApplication(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUnprovisionApplicationType(
        &self,
        applicationtypename: &::windows::core::PCWSTR,
        applicationtypeversion: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionApplicationType(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient {}
impl IFabricApplicationManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationbuildpath: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType(
                ::core::mem::transmute(&applicationbuildpath),
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
        unsafe extern "system" fn EndProvisionApplicationType<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginCreateApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_APPLICATION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateApplication(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndCreateApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateApplication(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpgradeApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            upgradedescription: *const super::super::FABRIC_APPLICATION_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeApplication(
                ::core::mem::transmute_copy(&upgradedescription),
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
        unsafe extern "system" fn EndUpgradeApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeApplication(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetApplicationUpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationUpgradeProgress(
                ::core::mem::transmute_copy(&applicationname),
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
        unsafe extern "system" fn EndGetApplicationUpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationUpgradeProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextApplicationUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            progress: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextApplicationUpgradeDomain(
                ::core::mem::transmute(&progress),
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
        unsafe extern "system" fn EndMoveNextApplicationUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextApplicationUpgradeDomain(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteApplication(
                ::core::mem::transmute_copy(&applicationname),
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
        unsafe extern "system" fn EndDeleteApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteApplication(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUnprovisionApplicationType<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows::core::PCWSTR,
            applicationtypeversion: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionApplicationType(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
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
        unsafe extern "system" fn EndUnprovisionApplicationType<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionApplicationType(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginProvisionApplicationType: BeginProvisionApplicationType::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType: EndProvisionApplicationType::<Identity, Impl, OFFSET>,
            BeginCreateApplication: BeginCreateApplication::<Identity, Impl, OFFSET>,
            EndCreateApplication: EndCreateApplication::<Identity, Impl, OFFSET>,
            BeginUpgradeApplication: BeginUpgradeApplication::<Identity, Impl, OFFSET>,
            EndUpgradeApplication: EndUpgradeApplication::<Identity, Impl, OFFSET>,
            BeginGetApplicationUpgradeProgress: BeginGetApplicationUpgradeProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationUpgradeProgress: EndGetApplicationUpgradeProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginMoveNextApplicationUpgradeDomain: BeginMoveNextApplicationUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextApplicationUpgradeDomain: EndMoveNextApplicationUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginDeleteApplication: BeginDeleteApplication::<Identity, Impl, OFFSET>,
            EndDeleteApplication: EndDeleteApplication::<Identity, Impl, OFFSET>,
            BeginUnprovisionApplicationType: BeginUnprovisionApplicationType::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnprovisionApplicationType: EndUnprovisionApplicationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient10_Impl:
    Sized + IFabricApplicationManagementClient9_Impl
{
    fn BeginProvisionApplicationType3(
        &self,
        description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType3(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient10 {}
impl IFabricApplicationManagementClient10_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient10_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType3<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION_BASE,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType3(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndProvisionApplicationType3<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType3(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginProvisionApplicationType3: BeginProvisionApplicationType3::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType3: EndProvisionApplicationType3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient10 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient9 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient2_Impl:
    Sized + IFabricApplicationManagementClient_Impl
{
    fn BeginGetApplicationManifest(
        &self,
        applicationtypename: &::windows::core::PCWSTR,
        applicationtypeversion: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationManifest(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginMoveNextApplicationUpgradeDomain2(
        &self,
        applicationname: *const u16,
        nextupgradedomain: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextApplicationUpgradeDomain2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient2 {}
impl IFabricApplicationManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient2_Vtbl {
        unsafe extern "system" fn BeginGetApplicationManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows::core::PCWSTR,
            applicationtypeversion: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationManifest(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
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
        unsafe extern "system" fn EndGetApplicationManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationManifest(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextApplicationUpgradeDomain2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            nextupgradedomain: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextApplicationUpgradeDomain2(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&nextupgradedomain),
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
        unsafe extern "system" fn EndMoveNextApplicationUpgradeDomain2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextApplicationUpgradeDomain2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationManifest: BeginGetApplicationManifest::<Identity, Impl, OFFSET>,
            EndGetApplicationManifest: EndGetApplicationManifest::<Identity, Impl, OFFSET>,
            BeginMoveNextApplicationUpgradeDomain2: BeginMoveNextApplicationUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextApplicationUpgradeDomain2: EndMoveNextApplicationUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient3_Impl:
    Sized + IFabricApplicationManagementClient2_Impl
{
    fn BeginUpdateApplicationUpgrade(
        &self,
        description: *const super::super::FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateApplicationUpgrade(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRestartDeployedCodePackage(
        &self,
        restartcodepackagedescription : *const super::super:: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartDeployedCodePackage(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn CopyApplicationPackage(
        &self,
        imagestoreconnectionstring: &::windows::core::PCWSTR,
        applicationpackagepath: &::windows::core::PCWSTR,
        applicationpackagepathinimagestore: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
    fn RemoveApplicationPackage(
        &self,
        imagestoreconnectionstring: &::windows::core::PCWSTR,
        applicationpackagepathinimagestore: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient3 {}
impl IFabricApplicationManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient3_Vtbl {
        unsafe extern "system" fn BeginUpdateApplicationUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_APPLICATION_UPGRADE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateApplicationUpgrade(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndUpdateApplicationUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateApplicationUpgrade(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginRestartDeployedCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartcodepackagedescription : *const super::super:: FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartDeployedCodePackage(
                ::core::mem::transmute_copy(&restartcodepackagedescription),
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
        unsafe extern "system" fn EndRestartDeployedCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartDeployedCodePackage(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn CopyApplicationPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows::core::PCWSTR,
            applicationpackagepath: ::windows::core::PCWSTR,
            applicationpackagepathinimagestore: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyApplicationPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&applicationpackagepath),
                ::core::mem::transmute(&applicationpackagepathinimagestore),
            )
            .into()
        }
        unsafe extern "system" fn RemoveApplicationPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows::core::PCWSTR,
            applicationpackagepathinimagestore: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplicationPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&applicationpackagepathinimagestore),
            )
            .into()
        }
        Self {
            base__: IFabricApplicationManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateApplicationUpgrade: BeginUpdateApplicationUpgrade::<Identity, Impl, OFFSET>,
            EndUpdateApplicationUpgrade: EndUpdateApplicationUpgrade::<Identity, Impl, OFFSET>,
            BeginRestartDeployedCodePackage: BeginRestartDeployedCodePackage::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRestartDeployedCodePackage: EndRestartDeployedCodePackage::<Identity, Impl, OFFSET>,
            CopyApplicationPackage: CopyApplicationPackage::<Identity, Impl, OFFSET>,
            RemoveApplicationPackage: RemoveApplicationPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient4_Impl:
    Sized + IFabricApplicationManagementClient3_Impl
{
    fn BeginDeployServicePackageToNode(
        &self,
        applicationtypename: &::windows::core::PCWSTR,
        applicationtypeversion: &::windows::core::PCWSTR,
        servicemanifestname: &::windows::core::PCWSTR,
        sharingpolicy: *const super::super::FABRIC_PACKAGE_SHARING_POLICY_LIST,
        nodename: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeployServicePackageToNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient4 {}
impl IFabricApplicationManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient4_Vtbl {
        unsafe extern "system" fn BeginDeployServicePackageToNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows::core::PCWSTR,
            applicationtypeversion: ::windows::core::PCWSTR,
            servicemanifestname: ::windows::core::PCWSTR,
            sharingpolicy: *const super::super::FABRIC_PACKAGE_SHARING_POLICY_LIST,
            nodename: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeployServicePackageToNode(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute(&servicemanifestname),
                ::core::mem::transmute_copy(&sharingpolicy),
                ::core::mem::transmute(&nodename),
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
        unsafe extern "system" fn EndDeployServicePackageToNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeployServicePackageToNode(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeployServicePackageToNode: BeginDeployServicePackageToNode::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndDeployServicePackageToNode: EndDeployServicePackageToNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient5_Impl:
    Sized + IFabricApplicationManagementClient4_Impl
{
    fn BeginRollbackApplicationUpgrade(
        &self,
        applicationname: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRollbackApplicationUpgrade(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient5 {}
impl IFabricApplicationManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient5_Vtbl {
        unsafe extern "system" fn BeginRollbackApplicationUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRollbackApplicationUpgrade(
                ::core::mem::transmute_copy(&applicationname),
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
        unsafe extern "system" fn EndRollbackApplicationUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRollbackApplicationUpgrade(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRollbackApplicationUpgrade: BeginRollbackApplicationUpgrade::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRollbackApplicationUpgrade: EndRollbackApplicationUpgrade::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient6_Impl:
    Sized + IFabricApplicationManagementClient5_Impl
{
    fn BeginUpdateApplication(
        &self,
        applicationupdatedescription: *const super::super::FABRIC_APPLICATION_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateApplication(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient6 {}
impl IFabricApplicationManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient6_Vtbl {
        unsafe extern "system" fn BeginUpdateApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationupdatedescription : *const super::super:: FABRIC_APPLICATION_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateApplication(
                ::core::mem::transmute_copy(&applicationupdatedescription),
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
        unsafe extern "system" fn EndUpdateApplication<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateApplication(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateApplication: BeginUpdateApplication::<Identity, Impl, OFFSET>,
            EndUpdateApplication: EndUpdateApplication::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient7_Impl:
    Sized + IFabricApplicationManagementClient6_Impl
{
    fn BeginDeleteApplication2(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_APPLICATION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteApplication2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient7 {}
impl IFabricApplicationManagementClient7_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient7_Vtbl {
        unsafe extern "system" fn BeginDeleteApplication2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_APPLICATION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteApplication2(
                ::core::mem::transmute_copy(&deletedescription),
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
        unsafe extern "system" fn EndDeleteApplication2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteApplication2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeleteApplication2: BeginDeleteApplication2::<Identity, Impl, OFFSET>,
            EndDeleteApplication2: EndDeleteApplication2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient8_Impl:
    Sized + IFabricApplicationManagementClient7_Impl
{
    fn BeginProvisionApplicationType2(
        &self,
        description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionApplicationType2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient8 {}
impl IFabricApplicationManagementClient8_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient8_Vtbl {
        unsafe extern "system" fn BeginProvisionApplicationType2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_PROVISION_APPLICATION_TYPE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionApplicationType2(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndProvisionApplicationType2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionApplicationType2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginProvisionApplicationType2: BeginProvisionApplicationType2::<Identity, Impl, OFFSET>,
            EndProvisionApplicationType2: EndProvisionApplicationType2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationManagementClient9_Impl:
    Sized + IFabricApplicationManagementClient8_Impl
{
    fn BeginUnprovisionApplicationType2(
        &self,
        description: *const super::super::FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionApplicationType2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationManagementClient9 {}
impl IFabricApplicationManagementClient9_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationManagementClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationManagementClient9_Vtbl {
        unsafe extern "system" fn BeginUnprovisionApplicationType2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_UNPROVISION_APPLICATION_TYPE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionApplicationType2(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndUnprovisionApplicationType2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionApplicationType2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricApplicationManagementClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUnprovisionApplicationType2: BeginUnprovisionApplicationType2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnprovisionApplicationType2: EndUnprovisionApplicationType2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationManagementClient9 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationManagementClient8 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationUpgradeProgressResult_Impl: Sized {
    fn get_ApplicationName(&self) -> *mut u16;
    fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR;
    fn get_TargetApplicationTypeVersion(&self) -> ::windows::core::PWSTR;
    fn get_UpgradeState(&self) -> super::super::FABRIC_APPLICATION_UPGRADE_STATE;
    fn GetUpgradeDomains(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows::core::Result<()>;
    fn GetChangedUpgradeDomains(
        &self,
        previousprogress: &::core::option::Option<IFabricApplicationUpgradeProgressResult>,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricApplicationUpgradeProgressResult {}
impl IFabricApplicationUpgradeProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult_Vtbl {
        unsafe extern "system" fn get_ApplicationName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
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
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypeName()
        }
        unsafe extern "system" fn get_TargetApplicationTypeVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetApplicationTypeVersion()
        }
        unsafe extern "system" fn get_UpgradeState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_APPLICATION_UPGRADE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeState()
        }
        unsafe extern "system" fn GetUpgradeDomains<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpgradeDomains(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        unsafe extern "system" fn GetChangedUpgradeDomains<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            previousprogress: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChangedUpgradeDomains(
                ::core::mem::transmute(&previousprogress),
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationName: get_ApplicationName::<Identity, Impl, OFFSET>,
            get_ApplicationTypeName: get_ApplicationTypeName::<Identity, Impl, OFFSET>,
            get_TargetApplicationTypeVersion: get_TargetApplicationTypeVersion::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_UpgradeState: get_UpgradeState::<Identity, Impl, OFFSET>,
            GetUpgradeDomains: GetUpgradeDomains::<Identity, Impl, OFFSET>,
            GetChangedUpgradeDomains: GetChangedUpgradeDomains::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationUpgradeProgressResult2_Impl:
    Sized + IFabricApplicationUpgradeProgressResult_Impl
{
    fn get_RollingUpgradeMode(&self) -> super::super::FABRIC_ROLLING_UPGRADE_MODE;
    fn get_NextUpgradeDomain(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricApplicationUpgradeProgressResult2 {}
impl IFabricApplicationUpgradeProgressResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult2_Vtbl {
        unsafe extern "system" fn get_RollingUpgradeMode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ROLLING_UPGRADE_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_RollingUpgradeMode()
        }
        unsafe extern "system" fn get_NextUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NextUpgradeDomain()
        }
        Self {
            base__: IFabricApplicationUpgradeProgressResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_RollingUpgradeMode: get_RollingUpgradeMode::<Identity, Impl, OFFSET>,
            get_NextUpgradeDomain: get_NextUpgradeDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationUpgradeProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricApplicationUpgradeProgressResult3_Impl:
    Sized + IFabricApplicationUpgradeProgressResult2_Impl
{
    fn get_UpgradeProgress(&self) -> *mut super::super::FABRIC_APPLICATION_UPGRADE_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricApplicationUpgradeProgressResult3 {}
impl IFabricApplicationUpgradeProgressResult3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricApplicationUpgradeProgressResult3_Impl,
        const OFFSET: isize,
    >() -> IFabricApplicationUpgradeProgressResult3_Vtbl {
        unsafe extern "system" fn get_UpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricApplicationUpgradeProgressResult3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeProgress()
        }
        Self {
            base__: IFabricApplicationUpgradeProgressResult2_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_UpgradeProgress: get_UpgradeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricApplicationUpgradeProgressResult3 as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationUpgradeProgressResult as ::windows::core::Interface>::IID
            || iid == &<IFabricApplicationUpgradeProgressResult2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricChaosDescriptionResult_Impl: Sized {
    fn get_ChaosDescriptionResult(&self) -> *mut super::super::FABRIC_CHAOS_DESCRIPTION;
}
impl ::windows::core::RuntimeName for IFabricChaosDescriptionResult {}
impl IFabricChaosDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosDescriptionResult_Vtbl {
        unsafe extern "system" fn get_ChaosDescriptionResult<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosDescriptionResult()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosDescriptionResult: get_ChaosDescriptionResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricChaosDescriptionResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricChaosEventsSegmentResult_Impl: Sized {
    fn get_ChaosEventsSegmentResult(&self) -> *mut super::super::FABRIC_CHAOS_EVENTS_SEGMENT;
}
impl ::windows::core::RuntimeName for IFabricChaosEventsSegmentResult {}
impl IFabricChaosEventsSegmentResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosEventsSegmentResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosEventsSegmentResult_Vtbl {
        unsafe extern "system" fn get_ChaosEventsSegmentResult<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosEventsSegmentResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_EVENTS_SEGMENT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosEventsSegmentResult()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosEventsSegmentResult: get_ChaosEventsSegmentResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricChaosEventsSegmentResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricChaosReportResult_Impl: Sized {
    fn get_ChaosReportResult(&self) -> *mut super::super::FABRIC_CHAOS_REPORT;
}
impl ::windows::core::RuntimeName for IFabricChaosReportResult {}
impl IFabricChaosReportResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosReportResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosReportResult_Vtbl {
        unsafe extern "system" fn get_ChaosReportResult<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosReportResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_REPORT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosReportResult()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosReportResult: get_ChaosReportResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricChaosReportResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricChaosScheduleDescriptionResult_Impl: Sized {
    fn get_ChaosScheduleDescriptionResult(
        &self,
    ) -> *mut super::super::FABRIC_CHAOS_SCHEDULE_DESCRIPTION;
}
impl ::windows::core::RuntimeName for IFabricChaosScheduleDescriptionResult {}
impl IFabricChaosScheduleDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricChaosScheduleDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricChaosScheduleDescriptionResult_Vtbl {
        unsafe extern "system" fn get_ChaosScheduleDescriptionResult<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricChaosScheduleDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CHAOS_SCHEDULE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ChaosScheduleDescriptionResult()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ChaosScheduleDescriptionResult: get_ChaosScheduleDescriptionResult::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricChaosScheduleDescriptionResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClientConnectionEventHandler_Impl: Sized {
    fn OnConnected(
        &self,
        __midl__ifabricclientconnectioneventhandler0000: &::core::option::Option<
            IFabricGatewayInformationResult,
        >,
    ) -> ::windows::core::Result<()>;
    fn OnDisconnected(
        &self,
        __midl__ifabricclientconnectioneventhandler0001: &::core::option::Option<
            IFabricGatewayInformationResult,
        >,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClientConnectionEventHandler {}
impl IFabricClientConnectionEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientConnectionEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricClientConnectionEventHandler_Vtbl {
        unsafe extern "system" fn OnConnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricclientconnectioneventhandler0000: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected(::core::mem::transmute(
                &__midl__ifabricclientconnectioneventhandler0000,
            ))
            .into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricclientconnectioneventhandler0001: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected(::core::mem::transmute(
                &__midl__ifabricclientconnectioneventhandler0001,
            ))
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClientConnectionEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClientConnectionEventHandler2_Impl:
    Sized + IFabricClientConnectionEventHandler_Impl
{
    fn OnClaimsRetrieval(
        &self,
        metadata: *const super::super::FABRIC_CLAIMS_RETRIEVAL_METADATA,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricClientConnectionEventHandler2 {}
impl IFabricClientConnectionEventHandler2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientConnectionEventHandler2_Impl,
        const OFFSET: isize,
    >() -> IFabricClientConnectionEventHandler2_Vtbl {
        unsafe extern "system" fn OnClaimsRetrieval<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientConnectionEventHandler2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            metadata: *const super::super::FABRIC_CLAIMS_RETRIEVAL_METADATA,
            token: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnClaimsRetrieval(::core::mem::transmute_copy(&metadata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClientConnectionEventHandler_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClaimsRetrieval: OnClaimsRetrieval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClientConnectionEventHandler2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClientConnectionEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClientSettings_Impl: Sized {
    fn SetSecurityCredentials(
        &self,
        securitycredentials: *const super::super::FABRIC_SECURITY_CREDENTIALS,
    ) -> ::windows::core::Result<()>;
    fn SetKeepAlive(&self, keepaliveintervalinseconds: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClientSettings {}
impl IFabricClientSettings_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettings_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettings_Vtbl {
        unsafe extern "system" fn SetSecurityCredentials<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            securitycredentials: *const super::super::FABRIC_SECURITY_CREDENTIALS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityCredentials(::core::mem::transmute_copy(&securitycredentials))
                .into()
        }
        unsafe extern "system" fn SetKeepAlive<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            keepaliveintervalinseconds: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepAlive(::core::mem::transmute_copy(&keepaliveintervalinseconds))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSecurityCredentials: SetSecurityCredentials::<Identity, Impl, OFFSET>,
            SetKeepAlive: SetKeepAlive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClientSettings as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClientSettings2_Impl: Sized + IFabricClientSettings_Impl {
    fn GetSettings(&self) -> ::windows::core::Result<IFabricClientSettingsResult>;
    fn SetSettings(
        &self,
        fabricclientsettings: *const super::super::FABRIC_CLIENT_SETTINGS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClientSettings2 {}
impl IFabricClientSettings2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettings2_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettings2_Vtbl {
        unsafe extern "system" fn GetSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettings2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fabricclientsettings: *const super::super::FABRIC_CLIENT_SETTINGS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSettings(::core::mem::transmute_copy(&fabricclientsettings))
                .into()
        }
        Self {
            base__: IFabricClientSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            SetSettings: SetSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClientSettings2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClientSettings as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClientSettingsResult_Impl: Sized {
    fn get_Settings(&self) -> *mut super::super::FABRIC_CLIENT_SETTINGS;
}
impl ::windows::core::RuntimeName for IFabricClientSettingsResult {}
impl IFabricClientSettingsResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClientSettingsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricClientSettingsResult_Vtbl {
        unsafe extern "system" fn get_Settings<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClientSettingsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLIENT_SETTINGS {
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
        iid == &<IFabricClientSettingsResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterHealthResult_Impl: Sized {
    fn get_ClusterHealth(&self) -> *mut super::super::FABRIC_CLUSTER_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricClusterHealthResult {}
impl IFabricClusterHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterHealthResult_Vtbl {
        unsafe extern "system" fn get_ClusterHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterHealth: get_ClusterHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient_Impl: Sized {
    fn BeginNodeStateRemoved(
        &self,
        nodename: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndNodeStateRemoved(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRecoverPartitions(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverPartitions(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient {}
impl IFabricClusterManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient_Vtbl {
        unsafe extern "system" fn BeginNodeStateRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginNodeStateRemoved(
                ::core::mem::transmute(&nodename),
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
        unsafe extern "system" fn EndNodeStateRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndNodeStateRemoved(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverPartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverPartitions(
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
        unsafe extern "system" fn EndRecoverPartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverPartitions(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginNodeStateRemoved: BeginNodeStateRemoved::<Identity, Impl, OFFSET>,
            EndNodeStateRemoved: EndNodeStateRemoved::<Identity, Impl, OFFSET>,
            BeginRecoverPartitions: BeginRecoverPartitions::<Identity, Impl, OFFSET>,
            EndRecoverPartitions: EndRecoverPartitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient10_Impl:
    Sized + IFabricClusterManagementClient9_Impl
{
    fn BeginGetClusterConfiguration2(
        &self,
        apiversion: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfiguration2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient10 {}
impl IFabricClusterManagementClient10_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient10_Vtbl {
        unsafe extern "system" fn BeginGetClusterConfiguration2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            apiversion: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfiguration2(
                ::core::mem::transmute(&apiversion),
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
        unsafe extern "system" fn EndGetClusterConfiguration2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfiguration2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterConfiguration2: BeginGetClusterConfiguration2::<Identity, Impl, OFFSET>,
            EndGetClusterConfiguration2: EndGetClusterConfiguration2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient10 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient9 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient2_Impl:
    Sized + IFabricClusterManagementClient_Impl
{
    fn BeginDeactivateNode(
        &self,
        nodename: &::windows::core::PCWSTR,
        intent: super::super::FABRIC_NODE_DEACTIVATION_INTENT,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeactivateNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginActivateNode(
        &self,
        nodename: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndActivateNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginProvisionFabric(
        &self,
        codefilepath: &::windows::core::PCWSTR,
        clustermanifestfilepath: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndProvisionFabric(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUpgradeFabric(
        &self,
        upgradedescription: *const super::super::FABRIC_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeFabric(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetFabricUpgradeProgress(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetFabricUpgradeProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricUpgradeProgressResult2>;
    fn BeginMoveNextFabricUpgradeDomain(
        &self,
        progress: &::core::option::Option<IFabricUpgradeProgressResult2>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextFabricUpgradeDomain(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginMoveNextFabricUpgradeDomain2(
        &self,
        nextupgradedomain: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveNextFabricUpgradeDomain2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUnprovisionFabric(
        &self,
        codeversion: &::windows::core::PCWSTR,
        configversion: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnprovisionFabric(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetClusterManifest(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterManifest(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginRecoverPartition(
        &self,
        partitionid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverPartition(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRecoverServicePartitions(
        &self,
        servicename: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverServicePartitions(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRecoverSystemPartitions(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRecoverSystemPartitions(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient2 {}
impl IFabricClusterManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient2_Vtbl {
        unsafe extern "system" fn BeginDeactivateNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows::core::PCWSTR,
            intent: super::super::FABRIC_NODE_DEACTIVATION_INTENT,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeactivateNode(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&intent),
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
        unsafe extern "system" fn EndDeactivateNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeactivateNode(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginActivateNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginActivateNode(
                ::core::mem::transmute(&nodename),
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
        unsafe extern "system" fn EndActivateNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndActivateNode(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginProvisionFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codefilepath: ::windows::core::PCWSTR,
            clustermanifestfilepath: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginProvisionFabric(
                ::core::mem::transmute(&codefilepath),
                ::core::mem::transmute(&clustermanifestfilepath),
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
        unsafe extern "system" fn EndProvisionFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndProvisionFabric(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpgradeFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            upgradedescription: *const super::super::FABRIC_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeFabric(
                ::core::mem::transmute_copy(&upgradedescription),
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
        unsafe extern "system" fn EndUpgradeFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeFabric(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetFabricUpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetFabricUpgradeProgress(
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
        unsafe extern "system" fn EndGetFabricUpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetFabricUpgradeProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveNextFabricUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            progress: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextFabricUpgradeDomain(
                ::core::mem::transmute(&progress),
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
        unsafe extern "system" fn EndMoveNextFabricUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextFabricUpgradeDomain(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginMoveNextFabricUpgradeDomain2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nextupgradedomain: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveNextFabricUpgradeDomain2(
                ::core::mem::transmute(&nextupgradedomain),
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
        unsafe extern "system" fn EndMoveNextFabricUpgradeDomain2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMoveNextFabricUpgradeDomain2(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUnprovisionFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            codeversion: ::windows::core::PCWSTR,
            configversion: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnprovisionFabric(
                ::core::mem::transmute(&codeversion),
                ::core::mem::transmute(&configversion),
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
        unsafe extern "system" fn EndUnprovisionFabric<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnprovisionFabric(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterManifest(
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
        unsafe extern "system" fn EndGetClusterManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterManifest(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRecoverPartition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverPartition(
                ::core::mem::transmute(&partitionid),
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
        unsafe extern "system" fn EndRecoverPartition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverPartition(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverServicePartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverServicePartitions(
                ::core::mem::transmute_copy(&servicename),
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
        unsafe extern "system" fn EndRecoverServicePartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverServicePartitions(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginRecoverSystemPartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRecoverSystemPartitions(
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
        unsafe extern "system" fn EndRecoverSystemPartitions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRecoverSystemPartitions(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeactivateNode: BeginDeactivateNode::<Identity, Impl, OFFSET>,
            EndDeactivateNode: EndDeactivateNode::<Identity, Impl, OFFSET>,
            BeginActivateNode: BeginActivateNode::<Identity, Impl, OFFSET>,
            EndActivateNode: EndActivateNode::<Identity, Impl, OFFSET>,
            BeginProvisionFabric: BeginProvisionFabric::<Identity, Impl, OFFSET>,
            EndProvisionFabric: EndProvisionFabric::<Identity, Impl, OFFSET>,
            BeginUpgradeFabric: BeginUpgradeFabric::<Identity, Impl, OFFSET>,
            EndUpgradeFabric: EndUpgradeFabric::<Identity, Impl, OFFSET>,
            BeginGetFabricUpgradeProgress: BeginGetFabricUpgradeProgress::<Identity, Impl, OFFSET>,
            EndGetFabricUpgradeProgress: EndGetFabricUpgradeProgress::<Identity, Impl, OFFSET>,
            BeginMoveNextFabricUpgradeDomain: BeginMoveNextFabricUpgradeDomain::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextFabricUpgradeDomain: EndMoveNextFabricUpgradeDomain::<Identity, Impl, OFFSET>,
            BeginMoveNextFabricUpgradeDomain2: BeginMoveNextFabricUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndMoveNextFabricUpgradeDomain2: EndMoveNextFabricUpgradeDomain2::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginUnprovisionFabric: BeginUnprovisionFabric::<Identity, Impl, OFFSET>,
            EndUnprovisionFabric: EndUnprovisionFabric::<Identity, Impl, OFFSET>,
            BeginGetClusterManifest: BeginGetClusterManifest::<Identity, Impl, OFFSET>,
            EndGetClusterManifest: EndGetClusterManifest::<Identity, Impl, OFFSET>,
            BeginRecoverPartition: BeginRecoverPartition::<Identity, Impl, OFFSET>,
            EndRecoverPartition: EndRecoverPartition::<Identity, Impl, OFFSET>,
            BeginRecoverServicePartitions: BeginRecoverServicePartitions::<Identity, Impl, OFFSET>,
            EndRecoverServicePartitions: EndRecoverServicePartitions::<Identity, Impl, OFFSET>,
            BeginRecoverSystemPartitions: BeginRecoverSystemPartitions::<Identity, Impl, OFFSET>,
            EndRecoverSystemPartitions: EndRecoverSystemPartitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient3_Impl:
    Sized + IFabricClusterManagementClient2_Impl
{
    fn BeginUpdateFabricUpgrade(
        &self,
        description: *const super::super::FABRIC_UPGRADE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateFabricUpgrade(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginStopNode(
        &self,
        stopnodedescription: *const super::super::FABRIC_STOP_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRestartNode(
        &self,
        restartnodedescription: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginStartNode(
        &self,
        startnodedescription: *const super::super::FABRIC_START_NODE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn CopyClusterPackage(
        &self,
        imagestoreconnectionstring: &::windows::core::PCWSTR,
        clustermanifestpath: &::windows::core::PCWSTR,
        clustermanifestpathinimagestore: &::windows::core::PCWSTR,
        codepackagepath: &::windows::core::PCWSTR,
        codepackagepathinimagestore: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
    fn RemoveClusterPackage(
        &self,
        imagestoreconnectionstring: &::windows::core::PCWSTR,
        clustermanifestpathinimagestore: &::windows::core::PCWSTR,
        codepackagepathinimagestore: &::windows::core::PCWSTR,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient3 {}
impl IFabricClusterManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient3_Vtbl {
        unsafe extern "system" fn BeginUpdateFabricUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_UPGRADE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateFabricUpgrade(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndUpdateFabricUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateFabricUpgrade(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginStopNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            stopnodedescription: *const super::super::FABRIC_STOP_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopNode(
                ::core::mem::transmute_copy(&stopnodedescription),
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
        unsafe extern "system" fn EndStopNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStopNode(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginRestartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartnodedescription: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartNode(
                ::core::mem::transmute_copy(&restartnodedescription),
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
        unsafe extern "system" fn EndRestartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartNode(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginStartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            startnodedescription: *const super::super::FABRIC_START_NODE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNode(
                ::core::mem::transmute_copy(&startnodedescription),
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
        unsafe extern "system" fn EndStartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartNode(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn CopyClusterPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows::core::PCWSTR,
            clustermanifestpath: ::windows::core::PCWSTR,
            clustermanifestpathinimagestore: ::windows::core::PCWSTR,
            codepackagepath: ::windows::core::PCWSTR,
            codepackagepathinimagestore: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyClusterPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&clustermanifestpath),
                ::core::mem::transmute(&clustermanifestpathinimagestore),
                ::core::mem::transmute(&codepackagepath),
                ::core::mem::transmute(&codepackagepathinimagestore),
            )
            .into()
        }
        unsafe extern "system" fn RemoveClusterPackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            imagestoreconnectionstring: ::windows::core::PCWSTR,
            clustermanifestpathinimagestore: ::windows::core::PCWSTR,
            codepackagepathinimagestore: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveClusterPackage(
                ::core::mem::transmute(&imagestoreconnectionstring),
                ::core::mem::transmute(&clustermanifestpathinimagestore),
                ::core::mem::transmute(&codepackagepathinimagestore),
            )
            .into()
        }
        Self {
            base__: IFabricClusterManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateFabricUpgrade: BeginUpdateFabricUpgrade::<Identity, Impl, OFFSET>,
            EndUpdateFabricUpgrade: EndUpdateFabricUpgrade::<Identity, Impl, OFFSET>,
            BeginStopNode: BeginStopNode::<Identity, Impl, OFFSET>,
            EndStopNode: EndStopNode::<Identity, Impl, OFFSET>,
            BeginRestartNode: BeginRestartNode::<Identity, Impl, OFFSET>,
            EndRestartNode: EndRestartNode::<Identity, Impl, OFFSET>,
            BeginStartNode: BeginStartNode::<Identity, Impl, OFFSET>,
            EndStartNode: EndStartNode::<Identity, Impl, OFFSET>,
            CopyClusterPackage: CopyClusterPackage::<Identity, Impl, OFFSET>,
            RemoveClusterPackage: RemoveClusterPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient4_Impl:
    Sized + IFabricClusterManagementClient3_Impl
{
    fn BeginRollbackFabricUpgrade(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRollbackFabricUpgrade(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient4 {}
impl IFabricClusterManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient4_Vtbl {
        unsafe extern "system" fn BeginRollbackFabricUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRollbackFabricUpgrade(
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
        unsafe extern "system" fn EndRollbackFabricUpgrade<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRollbackFabricUpgrade(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRollbackFabricUpgrade: BeginRollbackFabricUpgrade::<Identity, Impl, OFFSET>,
            EndRollbackFabricUpgrade: EndRollbackFabricUpgrade::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient5_Impl:
    Sized + IFabricClusterManagementClient4_Impl
{
    fn BeginResetPartitionLoad(
        &self,
        partitionid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndResetPartitionLoad(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient5 {}
impl IFabricClusterManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient5_Vtbl {
        unsafe extern "system" fn BeginResetPartitionLoad<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginResetPartitionLoad(
                ::core::mem::transmute(&partitionid),
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
        unsafe extern "system" fn EndResetPartitionLoad<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndResetPartitionLoad(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginResetPartitionLoad: BeginResetPartitionLoad::<Identity, Impl, OFFSET>,
            EndResetPartitionLoad: EndResetPartitionLoad::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient6_Impl:
    Sized + IFabricClusterManagementClient5_Impl
{
    fn BeginToggleVerboseServicePlacementHealthReporting(
        &self,
        enabled: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndToggleVerboseServicePlacementHealthReporting(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient6 {}
impl IFabricClusterManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient6_Vtbl {
        unsafe extern "system" fn BeginToggleVerboseServicePlacementHealthReporting<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            enabled: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginToggleVerboseServicePlacementHealthReporting(
                ::core::mem::transmute_copy(&enabled),
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
        unsafe extern "system" fn EndToggleVerboseServicePlacementHealthReporting<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndToggleVerboseServicePlacementHealthReporting(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginToggleVerboseServicePlacementHealthReporting:
                BeginToggleVerboseServicePlacementHealthReporting::<Identity, Impl, OFFSET>,
            EndToggleVerboseServicePlacementHealthReporting:
                EndToggleVerboseServicePlacementHealthReporting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient7_Impl:
    Sized + IFabricClusterManagementClient6_Impl
{
    fn BeginUpgradeConfiguration(
        &self,
        startupgradedescription: *const super::super::FABRIC_START_UPGRADE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpgradeConfiguration(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetClusterConfigurationUpgradeStatus(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfigurationUpgradeStatus(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricOrchestrationUpgradeStatusResult>;
    fn BeginGetClusterConfiguration(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterConfiguration(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginGetUpgradesPendingApproval(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUpgradesPendingApproval(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginStartApprovedUpgrades(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartApprovedUpgrades(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient7 {}
impl IFabricClusterManagementClient7_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient7_Vtbl {
        unsafe extern "system" fn BeginUpgradeConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            startupgradedescription: *const super::super::FABRIC_START_UPGRADE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpgradeConfiguration(
                ::core::mem::transmute_copy(&startupgradedescription),
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
        unsafe extern "system" fn EndUpgradeConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpgradeConfiguration(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterConfigurationUpgradeStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfigurationUpgradeStatus(
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
        unsafe extern "system" fn EndGetClusterConfigurationUpgradeStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfigurationUpgradeStatus(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetClusterConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterConfiguration(
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
        unsafe extern "system" fn EndGetClusterConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterConfiguration(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetUpgradesPendingApproval<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUpgradesPendingApproval(
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
        unsafe extern "system" fn EndGetUpgradesPendingApproval<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndGetUpgradesPendingApproval(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginStartApprovedUpgrades<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartApprovedUpgrades(
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
        unsafe extern "system" fn EndStartApprovedUpgrades<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartApprovedUpgrades(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricClusterManagementClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpgradeConfiguration: BeginUpgradeConfiguration::<Identity, Impl, OFFSET>,
            EndUpgradeConfiguration: EndUpgradeConfiguration::<Identity, Impl, OFFSET>,
            BeginGetClusterConfigurationUpgradeStatus: BeginGetClusterConfigurationUpgradeStatus::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetClusterConfigurationUpgradeStatus: EndGetClusterConfigurationUpgradeStatus::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetClusterConfiguration: BeginGetClusterConfiguration::<Identity, Impl, OFFSET>,
            EndGetClusterConfiguration: EndGetClusterConfiguration::<Identity, Impl, OFFSET>,
            BeginGetUpgradesPendingApproval: BeginGetUpgradesPendingApproval::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUpgradesPendingApproval: EndGetUpgradesPendingApproval::<Identity, Impl, OFFSET>,
            BeginStartApprovedUpgrades: BeginStartApprovedUpgrades::<Identity, Impl, OFFSET>,
            EndStartApprovedUpgrades: EndStartApprovedUpgrades::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient8_Impl:
    Sized + IFabricClusterManagementClient7_Impl
{
    fn BeginGetClusterManifest2(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterManifest2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient8 {}
impl IFabricClusterManagementClient8_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient8_Vtbl {
        unsafe extern "system" fn BeginGetClusterManifest2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_MANIFEST_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterManifest2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetClusterManifest2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterManifest2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterManifest2: BeginGetClusterManifest2::<Identity, Impl, OFFSET>,
            EndGetClusterManifest2: EndGetClusterManifest2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricClusterManagementClient9_Impl:
    Sized + IFabricClusterManagementClient8_Impl
{
    fn BeginGetUpgradeOrchestrationServiceState(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUpgradeOrchestrationServiceState(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginSetUpgradeOrchestrationServiceState(
        &self,
        state: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetUpgradeOrchestrationServiceState(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricUpgradeOrchestrationServiceStateResult>;
}
impl ::windows::core::RuntimeName for IFabricClusterManagementClient9 {}
impl IFabricClusterManagementClient9_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricClusterManagementClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricClusterManagementClient9_Vtbl {
        unsafe extern "system" fn BeginGetUpgradeOrchestrationServiceState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUpgradeOrchestrationServiceState(
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
        unsafe extern "system" fn EndGetUpgradeOrchestrationServiceState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetUpgradeOrchestrationServiceState(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetUpgradeOrchestrationServiceState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            state: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetUpgradeOrchestrationServiceState(
                ::core::mem::transmute(&state),
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
        unsafe extern "system" fn EndSetUpgradeOrchestrationServiceState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricClusterManagementClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSetUpgradeOrchestrationServiceState(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricClusterManagementClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetUpgradeOrchestrationServiceState: BeginGetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUpgradeOrchestrationServiceState: EndGetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginSetUpgradeOrchestrationServiceState: BeginSetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndSetUpgradeOrchestrationServiceState: EndSetUpgradeOrchestrationServiceState::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricClusterManagementClient9 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricClusterManagementClient8 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricDeployedApplicationHealthResult_Impl: Sized {
    fn get_DeployedApplicationHealth(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricDeployedApplicationHealthResult {}
impl IFabricDeployedApplicationHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDeployedApplicationHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricDeployedApplicationHealthResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDeployedApplicationHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationHealth: get_DeployedApplicationHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricDeployedApplicationHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricDeployedServicePackageHealthResult_Impl: Sized {
    fn get_DeployedServicePackageHealth(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricDeployedServicePackageHealthResult {}
impl IFabricDeployedServicePackageHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricDeployedServicePackageHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricDeployedServicePackageHealthResult_Vtbl {
        unsafe extern "system" fn get_DeployedServicePackageHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricDeployedServicePackageHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServicePackageHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServicePackageHealth: get_DeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricDeployedServicePackageHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricFaultManagementClient_Impl: Sized {
    fn BeginRestartNode(
        &self,
        description: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricRestartNodeResult>;
    fn BeginStartNode(
        &self,
        description: *const super::super::FABRIC_START_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricStartNodeResult>;
    fn BeginStopNode(
        &self,
        description: *const super::super::FABRIC_STOP_NODE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopNode(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricStopNodeResult>;
    fn BeginRestartDeployedCodePackage(
        &self,
        description: *const super::super::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartDeployedCodePackage(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricRestartDeployedCodePackageResult>;
    fn BeginMovePrimary(
        &self,
        description: *const super::super::FABRIC_MOVE_PRIMARY_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMovePrimary(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricMovePrimaryResult>;
    fn BeginMoveSecondary(
        &self,
        description: *const super::super::FABRIC_MOVE_SECONDARY_DESCRIPTION2,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndMoveSecondary(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricMoveSecondaryResult>;
}
impl ::windows::core::RuntimeName for IFabricFaultManagementClient {}
impl IFabricFaultManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricFaultManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricFaultManagementClient_Vtbl {
        unsafe extern "system" fn BeginRestartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartNode(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndRestartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRestartNode(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_START_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNode(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndStartNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndStartNode(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStopNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_STOP_NODE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopNode(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndStopNode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndStopNode(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRestartDeployedCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_DEPLOYED_CODE_PACKAGE_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartDeployedCodePackage(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndRestartDeployedCodePackage<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRestartDeployedCodePackage(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMovePrimary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_MOVE_PRIMARY_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMovePrimary(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndMovePrimary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndMovePrimary(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMoveSecondary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_MOVE_SECONDARY_DESCRIPTION2,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginMoveSecondary(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndMoveSecondary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricFaultManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndMoveSecondary(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRestartNode: BeginRestartNode::<Identity, Impl, OFFSET>,
            EndRestartNode: EndRestartNode::<Identity, Impl, OFFSET>,
            BeginStartNode: BeginStartNode::<Identity, Impl, OFFSET>,
            EndStartNode: EndStartNode::<Identity, Impl, OFFSET>,
            BeginStopNode: BeginStopNode::<Identity, Impl, OFFSET>,
            EndStopNode: EndStopNode::<Identity, Impl, OFFSET>,
            BeginRestartDeployedCodePackage: BeginRestartDeployedCodePackage::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRestartDeployedCodePackage: EndRestartDeployedCodePackage::<Identity, Impl, OFFSET>,
            BeginMovePrimary: BeginMovePrimary::<Identity, Impl, OFFSET>,
            EndMovePrimary: EndMovePrimary::<Identity, Impl, OFFSET>,
            BeginMoveSecondary: BeginMoveSecondary::<Identity, Impl, OFFSET>,
            EndMoveSecondary: EndMoveSecondary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricFaultManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGatewayInformationResult_Impl: Sized {
    fn get_GatewayInformation(&self) -> *mut super::super::FABRIC_GATEWAY_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGatewayInformationResult {}
impl IFabricGatewayInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGatewayInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGatewayInformationResult_Vtbl {
        unsafe extern "system" fn get_GatewayInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGatewayInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_GATEWAY_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_GatewayInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_GatewayInformation: get_GatewayInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGatewayInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationListResult_Impl: Sized {
    fn get_ApplicationList(&self) -> *mut super::super::FABRIC_APPLICATION_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationListResult {}
impl IFabricGetApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationList: get_ApplicationList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationListResult2_Impl:
    Sized + IFabricGetApplicationListResult_Impl
{
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationListResult2 {}
impl IFabricGetApplicationListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetApplicationListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationListResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricGetApplicationListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationLoadInformationResult_Impl: Sized {
    fn get_ApplicationLoadInformation(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_LOAD_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationLoadInformationResult {}
impl IFabricGetApplicationLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ApplicationLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationLoadInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationLoadInformation: get_ApplicationLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationLoadInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationNameResult_Impl: Sized {
    fn get_ApplicationName(&self) -> *mut super::super::FABRIC_APPLICATION_NAME_QUERY_RESULT;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationNameResult {}
impl IFabricGetApplicationNameResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationNameResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationNameResult_Vtbl {
        unsafe extern "system" fn get_ApplicationName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNameResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_NAME_QUERY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationName()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationName: get_ApplicationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationNameResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationNetworkListResult_Impl: Sized {
    fn get_ApplicationNetworkList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationNetworkListResult {}
impl IFabricGetApplicationNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationNetworkListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationNetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationNetworkList: get_ApplicationNetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationNetworkListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationTypeListResult_Impl: Sized {
    fn get_ApplicationTypeList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationTypeListResult {}
impl IFabricGetApplicationTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationTypeListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypeList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationTypeList: get_ApplicationTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationTypeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetApplicationTypePagedListResult_Impl: Sized {
    fn get_ApplicationTypePagedList(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetApplicationTypePagedListResult {}
impl IFabricGetApplicationTypePagedListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetApplicationTypePagedListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetApplicationTypePagedListResult_Vtbl {
        unsafe extern "system" fn get_ApplicationTypePagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypePagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_APPLICATION_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ApplicationTypePagedList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetApplicationTypePagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ApplicationTypePagedList: get_ApplicationTypePagedList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetApplicationTypePagedListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetClusterHealthChunkResult_Impl: Sized {
    fn get_ClusterHealthChunk(&self) -> *mut super::super::FABRIC_CLUSTER_HEALTH_CHUNK;
}
impl ::windows::core::RuntimeName for IFabricGetClusterHealthChunkResult {}
impl IFabricGetClusterHealthChunkResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetClusterHealthChunkResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetClusterHealthChunkResult_Vtbl {
        unsafe extern "system" fn get_ClusterHealthChunk<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetClusterHealthChunkResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_HEALTH_CHUNK {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterHealthChunk()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterHealthChunk: get_ClusterHealthChunk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetClusterHealthChunkResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetClusterLoadInformationResult_Impl: Sized {
    fn get_ClusterLoadInformation(&self) -> *mut super::super::FABRIC_CLUSTER_LOAD_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetClusterLoadInformationResult {}
impl IFabricGetClusterLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetClusterLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetClusterLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ClusterLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetClusterLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CLUSTER_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ClusterLoadInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ClusterLoadInformation: get_ClusterLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetClusterLoadInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedApplicationListResult_Impl: Sized {
    fn get_DeployedApplicationList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedApplicationListResult {}
impl IFabricGetDeployedApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedApplicationListResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationList: get_DeployedApplicationList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedApplicationListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedApplicationPagedListResult_Impl: Sized {
    fn get_DeployedApplicationPagedList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedApplicationPagedListResult {}
impl IFabricGetDeployedApplicationPagedListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedApplicationPagedListResult_Vtbl {
        unsafe extern "system" fn get_DeployedApplicationPagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedApplicationPagedList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedApplicationPagedListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedApplicationPagedList: get_DeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedApplicationPagedListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedCodePackageListResult_Impl: Sized {
    fn get_DeployedCodePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedCodePackageListResult {}
impl IFabricGetDeployedCodePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedCodePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedCodePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedCodePackageList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedCodePackageList: get_DeployedCodePackageList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedCodePackageListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedNetworkCodePackageListResult_Impl: Sized {
    fn get_DeployedNetworkCodePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedNetworkCodePackageListResult {}
impl IFabricGetDeployedNetworkCodePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedNetworkCodePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedNetworkCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedNetworkCodePackageList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkCodePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedNetworkCodePackageList: get_DeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedNetworkCodePackageListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedNetworkListResult_Impl: Sized {
    fn get_DeployedNetworkList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedNetworkListResult {}
impl IFabricGetDeployedNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedNetworkListResult_Vtbl {
        unsafe extern "system" fn get_DeployedNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedNetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedNetworkList: get_DeployedNetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedNetworkListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedReplicaListResult_Impl: Sized {
    fn get_DeployedReplicaList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedReplicaListResult {}
impl IFabricGetDeployedReplicaListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedReplicaListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedReplicaListResult_Vtbl {
        unsafe extern "system" fn get_DeployedReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedReplicaListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedReplicaList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedReplicaList: get_DeployedReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedReplicaListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedServicePackageListResult_Impl: Sized {
    fn get_DeployedServicePackageList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedServicePackageListResult {}
impl IFabricGetDeployedServicePackageListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServicePackageListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServicePackageListResult_Vtbl {
        unsafe extern "system" fn get_DeployedServicePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServicePackageListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServicePackageList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServicePackageList: get_DeployedServicePackageList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedServicePackageListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedServiceReplicaDetailResult_Impl: Sized {
    fn get_ReplicaDetail(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedServiceReplicaDetailResult {}
impl IFabricGetDeployedServiceReplicaDetailResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServiceReplicaDetailResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServiceReplicaDetailResult_Vtbl {
        unsafe extern "system" fn get_ReplicaDetail<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServiceReplicaDetailResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_RESULT_ITEM {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaDetail()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaDetail: get_ReplicaDetail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedServiceReplicaDetailResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetDeployedServiceTypeListResult_Impl: Sized {
    fn get_DeployedServiceTypeList(
        &self,
    ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetDeployedServiceTypeListResult {}
impl IFabricGetDeployedServiceTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetDeployedServiceTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetDeployedServiceTypeListResult_Vtbl {
        unsafe extern "system" fn get_DeployedServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetDeployedServiceTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_DeployedServiceTypeList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_DeployedServiceTypeList: get_DeployedServiceTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetDeployedServiceTypeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNetworkApplicationListResult_Impl: Sized {
    fn get_NetworkApplicationList(
        &self,
    ) -> *mut super::super::FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetNetworkApplicationListResult {}
impl IFabricGetNetworkApplicationListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkApplicationListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkApplicationListResult_Vtbl {
        unsafe extern "system" fn get_NetworkApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_APPLICATION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkApplicationList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkApplicationListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkApplicationList: get_NetworkApplicationList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNetworkApplicationListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNetworkListResult_Impl: Sized {
    fn get_NetworkList(&self) -> *mut super::super::FABRIC_NETWORK_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetNetworkListResult {}
impl IFabricGetNetworkListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkListResult_Vtbl {
        unsafe extern "system" fn get_NetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkList: get_NetworkList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNetworkListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNetworkNodeListResult_Impl: Sized {
    fn get_NetworkNodeList(&self) -> *mut super::super::FABRIC_NETWORK_NODE_QUERY_RESULT_LIST;
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetNetworkNodeListResult {}
impl IFabricGetNetworkNodeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNetworkNodeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNetworkNodeListResult_Vtbl {
        unsafe extern "system" fn get_NetworkNodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NETWORK_NODE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NetworkNodeList()
        }
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNetworkNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NetworkNodeList: get_NetworkNodeList::<Identity, Impl, OFFSET>,
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNetworkNodeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNodeListResult_Impl: Sized {
    fn get_NodeList(&self) -> *mut super::super::FABRIC_NODE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetNodeListResult {}
impl IFabricGetNodeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeListResult_Vtbl {
        unsafe extern "system" fn get_NodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeList: get_NodeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNodeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNodeListResult2_Impl: Sized + IFabricGetNodeListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetNodeListResult2 {}
impl IFabricGetNodeListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetNodeListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNodeListResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricGetNodeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetNodeLoadInformationResult_Impl: Sized {
    fn get_NodeLoadInformation(&self) -> *mut super::super::FABRIC_NODE_LOAD_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetNodeLoadInformationResult {}
impl IFabricGetNodeLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetNodeLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetNodeLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_NodeLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetNodeLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeLoadInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeLoadInformation: get_NodeLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetNodeLoadInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetPartitionListResult_Impl: Sized {
    fn get_PartitionList(&self) -> *mut super::super::FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetPartitionListResult {}
impl IFabricGetPartitionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionListResult_Vtbl {
        unsafe extern "system" fn get_PartitionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_PARTITION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionList: get_PartitionList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetPartitionListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetPartitionListResult2_Impl: Sized + IFabricGetPartitionListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetPartitionListResult2 {}
impl IFabricGetPartitionListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetPartitionListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetPartitionListResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricGetPartitionListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetPartitionLoadInformationResult_Impl: Sized {
    fn get_PartitionLoadInformation(&self) -> *mut super::super::FABRIC_PARTITION_LOAD_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetPartitionLoadInformationResult {}
impl IFabricGetPartitionLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetPartitionLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetPartitionLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_PartitionLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetPartitionLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionLoadInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionLoadInformation: get_PartitionLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetPartitionLoadInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetProvisionedCodeVersionListResult_Impl: Sized {
    fn get_ProvisionedCodeVersionList(
        &self,
    ) -> *mut super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetProvisionedCodeVersionListResult {}
impl IFabricGetProvisionedCodeVersionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetProvisionedCodeVersionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetProvisionedCodeVersionListResult_Vtbl {
        unsafe extern "system" fn get_ProvisionedCodeVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetProvisionedCodeVersionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ProvisionedCodeVersionList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ProvisionedCodeVersionList: get_ProvisionedCodeVersionList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetProvisionedCodeVersionListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetProvisionedConfigVersionListResult_Impl: Sized {
    fn get_ProvisionedConfigVersionList(
        &self,
    ) -> *mut super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetProvisionedConfigVersionListResult {}
impl IFabricGetProvisionedConfigVersionListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetProvisionedConfigVersionListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetProvisionedConfigVersionListResult_Vtbl {
        unsafe extern "system" fn get_ProvisionedConfigVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetProvisionedConfigVersionListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ProvisionedConfigVersionList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ProvisionedConfigVersionList: get_ProvisionedConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetProvisionedConfigVersionListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetRepairTaskListResult_Impl: Sized {
    fn get_Tasks(&self) -> *mut super::super::FABRIC_REPAIR_TASK_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetRepairTaskListResult {}
impl IFabricGetRepairTaskListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetRepairTaskListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetRepairTaskListResult_Vtbl {
        unsafe extern "system" fn get_Tasks<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetRepairTaskListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPAIR_TASK_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Tasks()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Tasks: get_Tasks::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetRepairTaskListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetReplicaListResult_Impl: Sized {
    fn get_ReplicaList(&self) -> *mut super::super::FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetReplicaListResult {}
impl IFabricGetReplicaListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaListResult_Vtbl {
        unsafe extern "system" fn get_ReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_REPLICA_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaList: get_ReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetReplicaListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetReplicaListResult2_Impl: Sized + IFabricGetReplicaListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetReplicaListResult2 {}
impl IFabricGetReplicaListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetReplicaListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetReplicaListResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricGetReplicaListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetReplicaLoadInformationResult_Impl: Sized {
    fn get_ReplicaLoadInformation(&self) -> *mut super::super::FABRIC_REPLICA_LOAD_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetReplicaLoadInformationResult {}
impl IFabricGetReplicaLoadInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetReplicaLoadInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetReplicaLoadInformationResult_Vtbl {
        unsafe extern "system" fn get_ReplicaLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetReplicaLoadInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICA_LOAD_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaLoadInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaLoadInformation: get_ReplicaLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetReplicaLoadInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetRollingUpgradeMonitoringPolicyResult_Impl: Sized {
    fn get_Policy(&self) -> *mut super::super::FABRIC_ROLLING_UPGRADE_MONITORING_POLICY;
}
impl ::windows::core::RuntimeName for IFabricGetRollingUpgradeMonitoringPolicyResult {}
impl IFabricGetRollingUpgradeMonitoringPolicyResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetRollingUpgradeMonitoringPolicyResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetRollingUpgradeMonitoringPolicyResult_Vtbl {
        unsafe extern "system" fn get_Policy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetRollingUpgradeMonitoringPolicyResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ROLLING_UPGRADE_MONITORING_POLICY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Policy()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Policy: get_Policy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetRollingUpgradeMonitoringPolicyResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceGroupMemberListResult_Impl: Sized {
    fn get_ServiceGroupMemberList(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetServiceGroupMemberListResult {}
impl IFabricGetServiceGroupMemberListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceGroupMemberListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceGroupMemberListResult_Vtbl {
        unsafe extern "system" fn get_ServiceGroupMemberList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceGroupMemberListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceGroupMemberList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceGroupMemberList: get_ServiceGroupMemberList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceGroupMemberListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceGroupMemberTypeListResult_Impl: Sized {
    fn get_ServiceGroupMemberTypeList(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetServiceGroupMemberTypeListResult {}
impl IFabricGetServiceGroupMemberTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceGroupMemberTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceGroupMemberTypeListResult_Vtbl {
        unsafe extern "system" fn get_ServiceGroupMemberTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceGroupMemberTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceGroupMemberTypeList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceGroupMemberTypeList: get_ServiceGroupMemberTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceGroupMemberTypeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceListResult_Impl: Sized {
    fn get_ServiceList(&self) -> *mut super::super::FABRIC_SERVICE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetServiceListResult {}
impl IFabricGetServiceListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceListResult_Vtbl {
        unsafe extern "system" fn get_ServiceList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceList: get_ServiceList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceListResult2_Impl: Sized + IFabricGetServiceListResult_Impl {
    fn get_PagingStatus(&self) -> *mut super::super::FABRIC_PAGING_STATUS;
}
impl ::windows::core::RuntimeName for IFabricGetServiceListResult2 {}
impl IFabricGetServiceListResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceListResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceListResult2_Vtbl {
        unsafe extern "system" fn get_PagingStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceListResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PAGING_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PagingStatus()
        }
        Self {
            base__: IFabricGetServiceListResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_PagingStatus: get_PagingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceListResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricGetServiceListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceNameResult_Impl: Sized {
    fn get_ServiceName(&self) -> *mut super::super::FABRIC_SERVICE_NAME_QUERY_RESULT;
}
impl ::windows::core::RuntimeName for IFabricGetServiceNameResult {}
impl IFabricGetServiceNameResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceNameResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceNameResult_Vtbl {
        unsafe extern "system" fn get_ServiceName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceNameResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_NAME_QUERY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceName()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceName: get_ServiceName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceNameResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetServiceTypeListResult_Impl: Sized {
    fn get_ServiceTypeList(&self) -> *mut super::super::FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricGetServiceTypeListResult {}
impl IFabricGetServiceTypeListResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetServiceTypeListResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetServiceTypeListResult_Vtbl {
        unsafe extern "system" fn get_ServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetServiceTypeListResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_TYPE_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceTypeList()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceTypeList: get_ServiceTypeList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetServiceTypeListResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricGetUnplacedReplicaInformationResult_Impl: Sized {
    fn get_UnplacedReplicaInformation(
        &self,
    ) -> *mut super::super::FABRIC_UNPLACED_REPLICA_INFORMATION;
}
impl ::windows::core::RuntimeName for IFabricGetUnplacedReplicaInformationResult {}
impl IFabricGetUnplacedReplicaInformationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricGetUnplacedReplicaInformationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricGetUnplacedReplicaInformationResult_Vtbl {
        unsafe extern "system" fn get_UnplacedReplicaInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricGetUnplacedReplicaInformationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UNPLACED_REPLICA_INFORMATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UnplacedReplicaInformation()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_UnplacedReplicaInformation: get_UnplacedReplicaInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricGetUnplacedReplicaInformationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricHealthClient_Impl: Sized {
    fn ReportHealth(
        &self,
        healthreport: *const super::super::FABRIC_HEALTH_REPORT,
    ) -> ::windows::core::Result<()>;
    fn BeginGetClusterHealth(
        &self,
        healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricClusterHealthResult>;
    fn BeginGetNodeHealth(
        &self,
        nodename: &::windows::core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricNodeHealthResult>;
    fn BeginGetApplicationHealth(
        &self,
        applicationname: *const u16,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricApplicationHealthResult>;
    fn BeginGetServiceHealth(
        &self,
        servicename: *const u16,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricServiceHealthResult>;
    fn BeginGetPartitionHealth(
        &self,
        partitionid: &::windows::core::GUID,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPartitionHealthResult>;
    fn BeginGetReplicaHealth(
        &self,
        partitionid: &::windows::core::GUID,
        replicaid: i64,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricReplicaHealthResult>;
    fn BeginGetDeployedApplicationHealth(
        &self,
        applicationname: *const u16,
        nodename: &::windows::core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricDeployedApplicationHealthResult>;
    fn BeginGetDeployedServicePackageHealth(
        &self,
        applicationname: *const u16,
        servicemanifestname: &::windows::core::PCWSTR,
        nodename: &::windows::core::PCWSTR,
        healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageHealth(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricDeployedServicePackageHealthResult>;
}
impl ::windows::core::RuntimeName for IFabricHealthClient {}
impl IFabricHealthClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient_Vtbl {
        unsafe extern "system" fn ReportHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthreport: *const super::super::FABRIC_HEALTH_REPORT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportHealth(::core::mem::transmute_copy(&healthreport))
                .into()
        }
        unsafe extern "system" fn BeginGetClusterHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealth(
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetClusterHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNodeHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            nodename: ::windows::core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_CLUSTER_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeHealth(
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetNodeHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceHealth(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetServiceHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows::core::GUID,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionHealth(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetPartitionHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaHealth(
                ::core::mem::transmute(&partitionid),
                ::core::mem::transmute_copy(&replicaid),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetReplicaHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            nodename: ::windows::core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetDeployedApplicationHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicemanifestname: ::windows::core::PCWSTR,
            nodename: ::windows::core::PCWSTR,
            healthpolicy: *const super::super::FABRIC_APPLICATION_HEALTH_POLICY,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageHealth(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute(&servicemanifestname),
                ::core::mem::transmute(&nodename),
                ::core::mem::transmute_copy(&healthpolicy),
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
        unsafe extern "system" fn EndGetDeployedServicePackageHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServicePackageHealth(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportHealth: ReportHealth::<Identity, Impl, OFFSET>,
            BeginGetClusterHealth: BeginGetClusterHealth::<Identity, Impl, OFFSET>,
            EndGetClusterHealth: EndGetClusterHealth::<Identity, Impl, OFFSET>,
            BeginGetNodeHealth: BeginGetNodeHealth::<Identity, Impl, OFFSET>,
            EndGetNodeHealth: EndGetNodeHealth::<Identity, Impl, OFFSET>,
            BeginGetApplicationHealth: BeginGetApplicationHealth::<Identity, Impl, OFFSET>,
            EndGetApplicationHealth: EndGetApplicationHealth::<Identity, Impl, OFFSET>,
            BeginGetServiceHealth: BeginGetServiceHealth::<Identity, Impl, OFFSET>,
            EndGetServiceHealth: EndGetServiceHealth::<Identity, Impl, OFFSET>,
            BeginGetPartitionHealth: BeginGetPartitionHealth::<Identity, Impl, OFFSET>,
            EndGetPartitionHealth: EndGetPartitionHealth::<Identity, Impl, OFFSET>,
            BeginGetReplicaHealth: BeginGetReplicaHealth::<Identity, Impl, OFFSET>,
            EndGetReplicaHealth: EndGetReplicaHealth::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationHealth: BeginGetDeployedApplicationHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationHealth: EndGetDeployedApplicationHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServicePackageHealth: BeginGetDeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageHealth: EndGetDeployedServicePackageHealth::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricHealthClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricHealthClient2_Impl: Sized + IFabricHealthClient_Impl {
    fn BeginGetClusterHealth2(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricClusterHealthResult>;
    fn BeginGetNodeHealth2(
        &self,
        querydescription: *const super::super::FABRIC_NODE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricNodeHealthResult>;
    fn BeginGetApplicationHealth2(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricApplicationHealthResult>;
    fn BeginGetServiceHealth2(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricServiceHealthResult>;
    fn BeginGetPartitionHealth2(
        &self,
        querydescription: *const super::super::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPartitionHealthResult>;
    fn BeginGetReplicaHealth2(
        &self,
        querydescription: *const super::super::FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricReplicaHealthResult>;
    fn BeginGetDeployedApplicationHealth2(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricDeployedApplicationHealthResult>;
    fn BeginGetDeployedServicePackageHealth2(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageHealth2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricDeployedServicePackageHealthResult>;
}
impl ::windows::core::RuntimeName for IFabricHealthClient2 {}
impl IFabricHealthClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient2_Vtbl {
        unsafe extern "system" fn BeginGetClusterHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetClusterHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNodeHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNodeHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_PARTITION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetPartitionHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_REPLICA_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetReplicaHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_APPLICATION_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedApplicationHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_HEALTH_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageHealth2(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedServicePackageHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServicePackageHealth2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricHealthClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterHealth2: BeginGetClusterHealth2::<Identity, Impl, OFFSET>,
            EndGetClusterHealth2: EndGetClusterHealth2::<Identity, Impl, OFFSET>,
            BeginGetNodeHealth2: BeginGetNodeHealth2::<Identity, Impl, OFFSET>,
            EndGetNodeHealth2: EndGetNodeHealth2::<Identity, Impl, OFFSET>,
            BeginGetApplicationHealth2: BeginGetApplicationHealth2::<Identity, Impl, OFFSET>,
            EndGetApplicationHealth2: EndGetApplicationHealth2::<Identity, Impl, OFFSET>,
            BeginGetServiceHealth2: BeginGetServiceHealth2::<Identity, Impl, OFFSET>,
            EndGetServiceHealth2: EndGetServiceHealth2::<Identity, Impl, OFFSET>,
            BeginGetPartitionHealth2: BeginGetPartitionHealth2::<Identity, Impl, OFFSET>,
            EndGetPartitionHealth2: EndGetPartitionHealth2::<Identity, Impl, OFFSET>,
            BeginGetReplicaHealth2: BeginGetReplicaHealth2::<Identity, Impl, OFFSET>,
            EndGetReplicaHealth2: EndGetReplicaHealth2::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationHealth2: BeginGetDeployedApplicationHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationHealth2: EndGetDeployedApplicationHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServicePackageHealth2: BeginGetDeployedServicePackageHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageHealth2: EndGetDeployedServicePackageHealth2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricHealthClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricHealthClient3_Impl: Sized + IFabricHealthClient2_Impl {
    fn BeginGetClusterHealthChunk(
        &self,
        querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterHealthChunk(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetClusterHealthChunkResult>;
}
impl ::windows::core::RuntimeName for IFabricHealthClient3 {}
impl IFabricHealthClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient3_Vtbl {
        unsafe extern "system" fn BeginGetClusterHealthChunk<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_CLUSTER_HEALTH_CHUNK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterHealthChunk(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetClusterHealthChunk<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterHealthChunk(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricHealthClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetClusterHealthChunk: BeginGetClusterHealthChunk::<Identity, Impl, OFFSET>,
            EndGetClusterHealthChunk: EndGetClusterHealthChunk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricHealthClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricHealthClient4_Impl: Sized + IFabricHealthClient3_Impl {
    fn ReportHealth2(
        &self,
        healthreport: *const super::super::FABRIC_HEALTH_REPORT,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricHealthClient4 {}
impl IFabricHealthClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricHealthClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricHealthClient4_Vtbl {
        unsafe extern "system" fn ReportHealth2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricHealthClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            healthreport: *const super::super::FABRIC_HEALTH_REPORT,
            sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportHealth2(
                ::core::mem::transmute_copy(&healthreport),
                ::core::mem::transmute_copy(&sendoptions),
            )
            .into()
        }
        Self {
            base__: IFabricHealthClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportHealth2: ReportHealth2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricHealthClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricHealthClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricInfrastructureServiceClient_Impl: Sized {
    fn BeginInvokeInfrastructureCommand(
        &self,
        servicename: *const u16,
        command: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndInvokeInfrastructureCommand(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginInvokeInfrastructureQuery(
        &self,
        servicename: *const u16,
        command: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndInvokeInfrastructureQuery(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
}
impl ::windows::core::RuntimeName for IFabricInfrastructureServiceClient {}
impl IFabricInfrastructureServiceClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricInfrastructureServiceClient_Impl,
        const OFFSET: isize,
    >() -> IFabricInfrastructureServiceClient_Vtbl {
        unsafe extern "system" fn BeginInvokeInfrastructureCommand<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            command: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeInfrastructureCommand(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&command),
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
        unsafe extern "system" fn EndInvokeInfrastructureCommand<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndInvokeInfrastructureCommand(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginInvokeInfrastructureQuery<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicename: *const u16,
            command: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInvokeInfrastructureQuery(
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&command),
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
        unsafe extern "system" fn EndInvokeInfrastructureQuery<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricInfrastructureServiceClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndInvokeInfrastructureQuery(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginInvokeInfrastructureCommand: BeginInvokeInfrastructureCommand::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndInvokeInfrastructureCommand: EndInvokeInfrastructureCommand::<Identity, Impl, OFFSET>,
            BeginInvokeInfrastructureQuery: BeginInvokeInfrastructureQuery::<Identity, Impl, OFFSET>,
            EndInvokeInfrastructureQuery: EndInvokeInfrastructureQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricInfrastructureServiceClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricMovePrimaryResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_MOVE_PRIMARY_RESULT;
}
impl ::windows::core::RuntimeName for IFabricMovePrimaryResult {}
impl IFabricMovePrimaryResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricMovePrimaryResult_Impl,
        const OFFSET: isize,
    >() -> IFabricMovePrimaryResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricMovePrimaryResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_MOVE_PRIMARY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricMovePrimaryResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricMoveSecondaryResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_MOVE_SECONDARY_RESULT;
}
impl ::windows::core::RuntimeName for IFabricMoveSecondaryResult {}
impl IFabricMoveSecondaryResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricMoveSecondaryResult_Impl,
        const OFFSET: isize,
    >() -> IFabricMoveSecondaryResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricMoveSecondaryResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_MOVE_SECONDARY_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricMoveSecondaryResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNameEnumerationResult_Impl: Sized {
    fn get_EnumerationStatus(&self) -> super::super::FABRIC_ENUMERATION_STATUS;
    fn GetNames(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut *mut u16,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricNameEnumerationResult {}
impl IFabricNameEnumerationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNameEnumerationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNameEnumerationResult_Vtbl {
        unsafe extern "system" fn get_EnumerationStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNameEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ENUMERATION_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_EnumerationStatus()
        }
        unsafe extern "system" fn GetNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNameEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut *mut u16,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_EnumerationStatus: get_EnumerationStatus::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNameEnumerationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNetworkManagementClient_Impl: Sized {
    fn BeginCreateNetwork(
        &self,
        networkname: &::windows::core::PCWSTR,
        description: *const super::super::FABRIC_NETWORK_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateNetwork(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteNetwork(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_NETWORK_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteNetwork(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNetworkListResult>;
    fn BeginGetNetworkApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkApplicationList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNetworkApplicationListResult>;
    fn BeginGetNetworkNodeList(
        &self,
        querydescription: *const super::super::FABRIC_NETWORK_NODE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNetworkNodeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNetworkNodeListResult>;
    fn BeginGetApplicationNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationNetworkList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationNetworkListResult>;
    fn BeginGetDeployedNetworkList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedNetworkList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedNetworkListResult>;
    fn BeginGetDeployedNetworkCodePackageList(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedNetworkCodePackageList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedNetworkCodePackageListResult>;
}
impl ::windows::core::RuntimeName for IFabricNetworkManagementClient {}
impl IFabricNetworkManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNetworkManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricNetworkManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateNetwork<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            networkname: ::windows::core::PCWSTR,
            description: *const super::super::FABRIC_NETWORK_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateNetwork(
                ::core::mem::transmute(&networkname),
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndCreateNetwork<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateNetwork(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteNetwork<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_NETWORK_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteNetwork(
                ::core::mem::transmute_copy(&deletedescription),
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
        unsafe extern "system" fn EndDeleteNetwork<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteNetwork(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNetworkApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkApplicationList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNetworkApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkApplicationList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetNetworkNodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NETWORK_NODE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNetworkNodeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNetworkNodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNetworkNodeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationNetworkList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationNetworkList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_NETWORK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedNetworkList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedNetworkList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedNetworkList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedNetworkCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_NETWORK_CODE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedNetworkCodePackageList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedNetworkCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNetworkManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedNetworkCodePackageList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateNetwork: BeginCreateNetwork::<Identity, Impl, OFFSET>,
            EndCreateNetwork: EndCreateNetwork::<Identity, Impl, OFFSET>,
            BeginDeleteNetwork: BeginDeleteNetwork::<Identity, Impl, OFFSET>,
            EndDeleteNetwork: EndDeleteNetwork::<Identity, Impl, OFFSET>,
            BeginGetNetworkList: BeginGetNetworkList::<Identity, Impl, OFFSET>,
            EndGetNetworkList: EndGetNetworkList::<Identity, Impl, OFFSET>,
            BeginGetNetworkApplicationList: BeginGetNetworkApplicationList::<Identity, Impl, OFFSET>,
            EndGetNetworkApplicationList: EndGetNetworkApplicationList::<Identity, Impl, OFFSET>,
            BeginGetNetworkNodeList: BeginGetNetworkNodeList::<Identity, Impl, OFFSET>,
            EndGetNetworkNodeList: EndGetNetworkNodeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationNetworkList: BeginGetApplicationNetworkList::<Identity, Impl, OFFSET>,
            EndGetApplicationNetworkList: EndGetApplicationNetworkList::<Identity, Impl, OFFSET>,
            BeginGetDeployedNetworkList: BeginGetDeployedNetworkList::<Identity, Impl, OFFSET>,
            EndGetDeployedNetworkList: EndGetDeployedNetworkList::<Identity, Impl, OFFSET>,
            BeginGetDeployedNetworkCodePackageList: BeginGetDeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedNetworkCodePackageList: EndGetDeployedNetworkCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNetworkManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNodeHealthResult_Impl: Sized {
    fn get_NodeHealth(&self) -> *mut super::super::FABRIC_NODE_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricNodeHealthResult {}
impl IFabricNodeHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeHealthResult_Vtbl {
        unsafe extern "system" fn get_NodeHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NodeHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_NodeHealth: get_NodeHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNodeHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricNodeTransitionProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_NODE_TRANSITION_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricNodeTransitionProgressResult {}
impl IFabricNodeTransitionProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricNodeTransitionProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricNodeTransitionProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricNodeTransitionProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_TRANSITION_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricNodeTransitionProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricOrchestrationUpgradeStatusResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_ORCHESTRATION_UPGRADE_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricOrchestrationUpgradeStatusResult {}
impl IFabricOrchestrationUpgradeStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricOrchestrationUpgradeStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricOrchestrationUpgradeStatusResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricOrchestrationUpgradeStatusResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ORCHESTRATION_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricOrchestrationUpgradeStatusResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPartitionDataLossProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_DATA_LOSS_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricPartitionDataLossProgressResult {}
impl IFabricPartitionDataLossProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionDataLossProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionDataLossProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionDataLossProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_DATA_LOSS_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPartitionDataLossProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPartitionHealthResult_Impl: Sized {
    fn get_PartitionHealth(&self) -> *mut super::super::FABRIC_PARTITION_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricPartitionHealthResult {}
impl IFabricPartitionHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionHealthResult_Vtbl {
        unsafe extern "system" fn get_PartitionHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PartitionHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_PartitionHealth: get_PartitionHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPartitionHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPartitionQuorumLossProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_QUORUM_LOSS_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricPartitionQuorumLossProgressResult {}
impl IFabricPartitionQuorumLossProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionQuorumLossProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionQuorumLossProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionQuorumLossProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_QUORUM_LOSS_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPartitionQuorumLossProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPartitionRestartProgressResult_Impl: Sized {
    fn get_Progress(&self) -> *mut super::super::FABRIC_PARTITION_RESTART_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricPartitionRestartProgressResult {}
impl IFabricPartitionRestartProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPartitionRestartProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPartitionRestartProgressResult_Vtbl {
        unsafe extern "system" fn get_Progress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPartitionRestartProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_PARTITION_RESTART_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Progress()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Progress: get_Progress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPartitionRestartProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyBatchResult_Impl: Sized {
    fn GetProperty(
        &self,
        operationindexinrequest: u32,
    ) -> ::windows::core::Result<IFabricPropertyValueResult>;
}
impl ::windows::core::RuntimeName for IFabricPropertyBatchResult {}
impl IFabricPropertyBatchResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyBatchResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyBatchResult_Vtbl {
        unsafe extern "system" fn GetProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyBatchResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationindexinrequest: u32,
            property: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&operationindexinrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPropertyBatchResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyEnumerationResult_Impl: Sized {
    fn get_EnumerationStatus(&self) -> super::super::FABRIC_ENUMERATION_STATUS;
    fn get_PropertyCount(&self) -> u32;
    fn GetProperty(&self, index: u32) -> ::windows::core::Result<IFabricPropertyValueResult>;
}
impl ::windows::core::RuntimeName for IFabricPropertyEnumerationResult {}
impl IFabricPropertyEnumerationResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyEnumerationResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyEnumerationResult_Vtbl {
        unsafe extern "system" fn get_EnumerationStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ENUMERATION_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_EnumerationStatus()
        }
        unsafe extern "system" fn get_PropertyCount<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_PropertyCount()
        }
        unsafe extern "system" fn GetProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyEnumerationResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            property: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_EnumerationStatus: get_EnumerationStatus::<Identity, Impl, OFFSET>,
            get_PropertyCount: get_PropertyCount::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPropertyEnumerationResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyManagementClient_Impl: Sized {
    fn BeginCreateName(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateName(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteName(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteName(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginNameExists(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndNameExists(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<u8>;
    fn BeginEnumerateSubNames(
        &self,
        name: *const u16,
        previousresult: &::core::option::Option<IFabricNameEnumerationResult>,
        recursive: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndEnumerateSubNames(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricNameEnumerationResult>;
    fn BeginPutPropertyBinary(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        datalength: u32,
        data: *const u8,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyBinary(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginPutPropertyInt64(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        data: i64,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyInt64(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginPutPropertyDouble(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        data: f64,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyDouble(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginPutPropertyWString(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        data: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyWString(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginPutPropertyGuid(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        data: *const ::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutPropertyGuid(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteProperty(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteProperty(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetPropertyMetadata(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPropertyMetadata(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPropertyMetadataResult>;
    fn BeginGetProperty(
        &self,
        name: *const u16,
        propertyname: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProperty(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPropertyValueResult>;
    fn BeginSubmitPropertyBatch(
        &self,
        name: *const u16,
        operationcount: u32,
        operations: *const super::super::FABRIC_PROPERTY_BATCH_OPERATION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndSubmitPropertyBatch(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
        failedoperationindexinrequest: *mut u32,
        result: *mut ::core::option::Option<IFabricPropertyBatchResult>,
    ) -> ::windows::core::Result<()>;
    fn BeginEnumerateProperties(
        &self,
        name: *const u16,
        includevalues: ::windows::Win32::Foundation::BOOLEAN,
        previousresult: &::core::option::Option<IFabricPropertyEnumerationResult>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndEnumerateProperties(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPropertyEnumerationResult>;
}
impl ::windows::core::RuntimeName for IFabricPropertyManagementClient {}
impl IFabricPropertyManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateName(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndCreateName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateName(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginDeleteName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteName(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndDeleteName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteName(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginNameExists<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginNameExists(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndNameExists<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            value: *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndNameExists(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumerateSubNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            previousresult: *mut ::core::ffi::c_void,
            recursive: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginEnumerateSubNames(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&previousresult),
                ::core::mem::transmute_copy(&recursive),
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
        unsafe extern "system" fn EndEnumerateSubNames<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndEnumerateSubNames(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPutPropertyBinary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            datalength: u32,
            data: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyBinary(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&datalength),
                ::core::mem::transmute_copy(&data),
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
        unsafe extern "system" fn EndPutPropertyBinary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyBinary(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyInt64<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            data: i64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyInt64(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
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
        unsafe extern "system" fn EndPutPropertyInt64<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyInt64(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyDouble<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            data: f64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyDouble(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
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
        unsafe extern "system" fn EndPutPropertyDouble<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyDouble(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyWString<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            data: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyWString(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute(&data),
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
        unsafe extern "system" fn EndPutPropertyWString<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyWString(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginPutPropertyGuid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            data: *const ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutPropertyGuid(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute_copy(&data),
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
        unsafe extern "system" fn EndPutPropertyGuid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutPropertyGuid(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteProperty(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
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
        unsafe extern "system" fn EndDeleteProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteProperty(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPropertyMetadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPropertyMetadata(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
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
        unsafe extern "system" fn EndGetPropertyMetadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPropertyMetadata(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyname: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProperty(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute(&propertyname),
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
        unsafe extern "system" fn EndGetProperty<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetProperty(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSubmitPropertyBatch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            operationcount: u32,
            operations: *const super::super::FABRIC_PROPERTY_BATCH_OPERATION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSubmitPropertyBatch(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&operationcount),
                ::core::mem::transmute_copy(&operations),
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
        unsafe extern "system" fn EndSubmitPropertyBatch<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            failedoperationindexinrequest: *mut u32,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSubmitPropertyBatch(
                ::core::mem::transmute(&context),
                ::core::mem::transmute_copy(&failedoperationindexinrequest),
                ::core::mem::transmute_copy(&result),
            )
            .into()
        }
        unsafe extern "system" fn BeginEnumerateProperties<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            includevalues: ::windows::Win32::Foundation::BOOLEAN,
            previousresult: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginEnumerateProperties(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&includevalues),
                ::core::mem::transmute(&previousresult),
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
        unsafe extern "system" fn EndEnumerateProperties<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndEnumerateProperties(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateName: BeginCreateName::<Identity, Impl, OFFSET>,
            EndCreateName: EndCreateName::<Identity, Impl, OFFSET>,
            BeginDeleteName: BeginDeleteName::<Identity, Impl, OFFSET>,
            EndDeleteName: EndDeleteName::<Identity, Impl, OFFSET>,
            BeginNameExists: BeginNameExists::<Identity, Impl, OFFSET>,
            EndNameExists: EndNameExists::<Identity, Impl, OFFSET>,
            BeginEnumerateSubNames: BeginEnumerateSubNames::<Identity, Impl, OFFSET>,
            EndEnumerateSubNames: EndEnumerateSubNames::<Identity, Impl, OFFSET>,
            BeginPutPropertyBinary: BeginPutPropertyBinary::<Identity, Impl, OFFSET>,
            EndPutPropertyBinary: EndPutPropertyBinary::<Identity, Impl, OFFSET>,
            BeginPutPropertyInt64: BeginPutPropertyInt64::<Identity, Impl, OFFSET>,
            EndPutPropertyInt64: EndPutPropertyInt64::<Identity, Impl, OFFSET>,
            BeginPutPropertyDouble: BeginPutPropertyDouble::<Identity, Impl, OFFSET>,
            EndPutPropertyDouble: EndPutPropertyDouble::<Identity, Impl, OFFSET>,
            BeginPutPropertyWString: BeginPutPropertyWString::<Identity, Impl, OFFSET>,
            EndPutPropertyWString: EndPutPropertyWString::<Identity, Impl, OFFSET>,
            BeginPutPropertyGuid: BeginPutPropertyGuid::<Identity, Impl, OFFSET>,
            EndPutPropertyGuid: EndPutPropertyGuid::<Identity, Impl, OFFSET>,
            BeginDeleteProperty: BeginDeleteProperty::<Identity, Impl, OFFSET>,
            EndDeleteProperty: EndDeleteProperty::<Identity, Impl, OFFSET>,
            BeginGetPropertyMetadata: BeginGetPropertyMetadata::<Identity, Impl, OFFSET>,
            EndGetPropertyMetadata: EndGetPropertyMetadata::<Identity, Impl, OFFSET>,
            BeginGetProperty: BeginGetProperty::<Identity, Impl, OFFSET>,
            EndGetProperty: EndGetProperty::<Identity, Impl, OFFSET>,
            BeginSubmitPropertyBatch: BeginSubmitPropertyBatch::<Identity, Impl, OFFSET>,
            EndSubmitPropertyBatch: EndSubmitPropertyBatch::<Identity, Impl, OFFSET>,
            BeginEnumerateProperties: BeginEnumerateProperties::<Identity, Impl, OFFSET>,
            EndEnumerateProperties: EndEnumerateProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPropertyManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyManagementClient2_Impl:
    Sized + IFabricPropertyManagementClient_Impl
{
    fn BeginPutCustomPropertyOperation(
        &self,
        name: *const u16,
        propertyoperation: *const super::super::FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndPutCustomPropertyOperation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricPropertyManagementClient2 {}
impl IFabricPropertyManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyManagementClient2_Vtbl {
        unsafe extern "system" fn BeginPutCustomPropertyOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            propertyoperation: *const super::super::FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginPutCustomPropertyOperation(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&propertyoperation),
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
        unsafe extern "system" fn EndPutCustomPropertyOperation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPutCustomPropertyOperation(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricPropertyManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginPutCustomPropertyOperation: BeginPutCustomPropertyOperation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndPutCustomPropertyOperation: EndPutCustomPropertyOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPropertyManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricPropertyManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyMetadataResult_Impl: Sized {
    fn get_Metadata(&self) -> *mut super::super::FABRIC_NAMED_PROPERTY_METADATA;
}
impl ::windows::core::RuntimeName for IFabricPropertyMetadataResult {}
impl IFabricPropertyMetadataResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyMetadataResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyMetadataResult_Vtbl {
        unsafe extern "system" fn get_Metadata<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyMetadataResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NAMED_PROPERTY_METADATA {
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
        iid == &<IFabricPropertyMetadataResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricPropertyValueResult_Impl: Sized {
    fn get_Property(&self) -> *mut super::super::FABRIC_NAMED_PROPERTY;
    fn GetValueAsBinary(
        &self,
        bytecount: *mut u32,
        bufferedvalue: *mut *mut u8,
    ) -> ::windows::core::Result<()>;
    fn GetValueAsInt64(&self) -> ::windows::core::Result<i64>;
    fn GetValueAsDouble(&self) -> ::windows::core::Result<f64>;
    fn GetValueAsWString(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetValueAsGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IFabricPropertyValueResult {}
impl IFabricPropertyValueResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricPropertyValueResult_Impl,
        const OFFSET: isize,
    >() -> IFabricPropertyValueResult_Vtbl {
        unsafe extern "system" fn get_Property<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NAMED_PROPERTY {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Property()
        }
        unsafe extern "system" fn GetValueAsBinary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytecount: *mut u32,
            bufferedvalue: *mut *mut u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValueAsBinary(
                ::core::mem::transmute_copy(&bytecount),
                ::core::mem::transmute_copy(&bufferedvalue),
            )
            .into()
        }
        unsafe extern "system" fn GetValueAsInt64<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsInt64() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsDouble<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut f64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsDouble() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsWString<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bufferedvalue: *mut ::windows::core::PWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsWString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsGuid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricPropertyValueResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::windows::core::GUID,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValueAsGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Property: get_Property::<Identity, Impl, OFFSET>,
            GetValueAsBinary: GetValueAsBinary::<Identity, Impl, OFFSET>,
            GetValueAsInt64: GetValueAsInt64::<Identity, Impl, OFFSET>,
            GetValueAsDouble: GetValueAsDouble::<Identity, Impl, OFFSET>,
            GetValueAsWString: GetValueAsWString::<Identity, Impl, OFFSET>,
            GetValueAsGuid: GetValueAsGuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricPropertyValueResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient_Impl: Sized {
    fn BeginGetNodeList(
        &self,
        querydescription: *const super::super::FABRIC_NODE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNodeListResult>;
    fn BeginGetApplicationTypeList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationTypeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationTypeListResult>;
    fn BeginGetServiceTypeList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceTypeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceTypeListResult>;
    fn BeginGetApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationListResult>;
    fn BeginGetServiceList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceListResult>;
    fn BeginGetPartitionList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetPartitionListResult>;
    fn BeginGetReplicaList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetReplicaListResult>;
    fn BeginGetDeployedApplicationList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedApplicationListResult>;
    fn BeginGetDeployedServicePackageList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServicePackageList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedServicePackageListResult>;
    fn BeginGetDeployedServiceTypeList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedServiceTypeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedServiceTypeListResult>;
    fn BeginGetDeployedCodePackageList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedCodePackageList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedCodePackageListResult>;
    fn BeginGetDeployedReplicaList(
        &self,
        querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedReplicaList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedReplicaListResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient {}
impl IFabricQueryClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient_Vtbl {
        unsafe extern "system" fn BeginGetNodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNodeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationTypeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationTypeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceTypeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceTypeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetPartitionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_REPLICA_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedApplicationList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServicePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServicePackageList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedServicePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServicePackageList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_SERVICE_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedServiceTypeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedServiceTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedServiceTypeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_DEPLOYED_CODE_PACKAGE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedCodePackageList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedCodePackageList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedCodePackageList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetDeployedReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedReplicaList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedReplicaList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedReplicaList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetNodeList: BeginGetNodeList::<Identity, Impl, OFFSET>,
            EndGetNodeList: EndGetNodeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationTypeList: BeginGetApplicationTypeList::<Identity, Impl, OFFSET>,
            EndGetApplicationTypeList: EndGetApplicationTypeList::<Identity, Impl, OFFSET>,
            BeginGetServiceTypeList: BeginGetServiceTypeList::<Identity, Impl, OFFSET>,
            EndGetServiceTypeList: EndGetServiceTypeList::<Identity, Impl, OFFSET>,
            BeginGetApplicationList: BeginGetApplicationList::<Identity, Impl, OFFSET>,
            EndGetApplicationList: EndGetApplicationList::<Identity, Impl, OFFSET>,
            BeginGetServiceList: BeginGetServiceList::<Identity, Impl, OFFSET>,
            EndGetServiceList: EndGetServiceList::<Identity, Impl, OFFSET>,
            BeginGetPartitionList: BeginGetPartitionList::<Identity, Impl, OFFSET>,
            EndGetPartitionList: EndGetPartitionList::<Identity, Impl, OFFSET>,
            BeginGetReplicaList: BeginGetReplicaList::<Identity, Impl, OFFSET>,
            EndGetReplicaList: EndGetReplicaList::<Identity, Impl, OFFSET>,
            BeginGetDeployedApplicationList: BeginGetDeployedApplicationList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationList: EndGetDeployedApplicationList::<Identity, Impl, OFFSET>,
            BeginGetDeployedServicePackageList: BeginGetDeployedServicePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServicePackageList: EndGetDeployedServicePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetDeployedServiceTypeList: BeginGetDeployedServiceTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedServiceTypeList: EndGetDeployedServiceTypeList::<Identity, Impl, OFFSET>,
            BeginGetDeployedCodePackageList: BeginGetDeployedCodePackageList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedCodePackageList: EndGetDeployedCodePackageList::<Identity, Impl, OFFSET>,
            BeginGetDeployedReplicaList: BeginGetDeployedReplicaList::<Identity, Impl, OFFSET>,
            EndGetDeployedReplicaList: EndGetDeployedReplicaList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient10_Impl: Sized + IFabricQueryClient9_Impl {
    fn BeginGetDeployedApplicationPagedList(
        &self,
        querydescription: *const super::super::FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedApplicationPagedList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedApplicationPagedListResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient10 {}
impl IFabricQueryClient10_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient10_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient10_Vtbl {
        unsafe extern "system" fn BeginGetDeployedApplicationPagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PAGED_DEPLOYED_APPLICATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedApplicationPagedList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedApplicationPagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient10_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedApplicationPagedList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient9_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetDeployedApplicationPagedList: BeginGetDeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetDeployedApplicationPagedList: EndGetDeployedApplicationPagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient10 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient9 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient2_Impl: Sized + IFabricQueryClient_Impl {
    fn BeginGetDeployedReplicaDetail(
        &self,
        querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetDeployedReplicaDetail(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetDeployedServiceReplicaDetailResult>;
    fn BeginGetClusterLoadInformation(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetClusterLoadInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetClusterLoadInformationResult>;
    fn BeginGetPartitionLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionLoadInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetPartitionLoadInformationResult>;
    fn BeginGetProvisionedFabricCodeVersionList(
        &self,
        querydescription: *const super::super::FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProvisionedFabricCodeVersionList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetProvisionedCodeVersionListResult>;
    fn BeginGetProvisionedFabricConfigVersionList(
        &self,
        querydescription: *const super::super::FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetProvisionedFabricConfigVersionList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetProvisionedConfigVersionListResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient2 {}
impl IFabricQueryClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient2_Vtbl {
        unsafe extern "system" fn BeginGetDeployedReplicaDetail<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_DEPLOYED_SERVICE_REPLICA_DETAIL_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetDeployedReplicaDetail(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetDeployedReplicaDetail<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetDeployedReplicaDetail(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetClusterLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetClusterLoadInformation(
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
        unsafe extern "system" fn EndGetClusterLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetClusterLoadInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetPartitionLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PARTITION_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetPartitionLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionLoadInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProvisionedFabricCodeVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PROVISIONED_CODE_VERSION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProvisionedFabricCodeVersionList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetProvisionedFabricCodeVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetProvisionedFabricCodeVersionList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetProvisionedFabricConfigVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_PROVISIONED_CONFIG_VERSION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetProvisionedFabricConfigVersionList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetProvisionedFabricConfigVersionList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetProvisionedFabricConfigVersionList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetDeployedReplicaDetail: BeginGetDeployedReplicaDetail::<Identity, Impl, OFFSET>,
            EndGetDeployedReplicaDetail: EndGetDeployedReplicaDetail::<Identity, Impl, OFFSET>,
            BeginGetClusterLoadInformation: BeginGetClusterLoadInformation::<Identity, Impl, OFFSET>,
            EndGetClusterLoadInformation: EndGetClusterLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetPartitionLoadInformation: BeginGetPartitionLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionLoadInformation: EndGetPartitionLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetProvisionedFabricCodeVersionList: BeginGetProvisionedFabricCodeVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetProvisionedFabricCodeVersionList: EndGetProvisionedFabricCodeVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginGetProvisionedFabricConfigVersionList: BeginGetProvisionedFabricConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetProvisionedFabricConfigVersionList: EndGetProvisionedFabricConfigVersionList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient3_Impl: Sized + IFabricQueryClient2_Impl {
    fn BeginGetNodeLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeLoadInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNodeLoadInformationResult>;
    fn BeginGetReplicaLoadInformation(
        &self,
        querydescription: *const super::super::FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetReplicaLoadInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetReplicaLoadInformationResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient3 {}
impl IFabricQueryClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient3_Vtbl {
        unsafe extern "system" fn BeginGetNodeLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_NODE_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetNodeLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeLoadInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetReplicaLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_REPLICA_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetReplicaLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetReplicaLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaLoadInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetNodeLoadInformation: BeginGetNodeLoadInformation::<Identity, Impl, OFFSET>,
            EndGetNodeLoadInformation: EndGetNodeLoadInformation::<Identity, Impl, OFFSET>,
            BeginGetReplicaLoadInformation: BeginGetReplicaLoadInformation::<Identity, Impl, OFFSET>,
            EndGetReplicaLoadInformation: EndGetReplicaLoadInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient4_Impl: Sized + IFabricQueryClient3_Impl {
    fn BeginGetServiceGroupMemberList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupMemberList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceGroupMemberListResult>;
    fn BeginGetServiceGroupMemberTypeList(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupMemberTypeList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceGroupMemberTypeListResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient4 {}
impl IFabricQueryClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient4_Vtbl {
        unsafe extern "system" fn BeginGetServiceGroupMemberList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_GROUP_MEMBER_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupMemberList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceGroupMemberList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupMemberList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetServiceGroupMemberTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_SERVICE_GROUP_MEMBER_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupMemberTypeList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceGroupMemberTypeList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupMemberTypeList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceGroupMemberList: BeginGetServiceGroupMemberList::<Identity, Impl, OFFSET>,
            EndGetServiceGroupMemberList: EndGetServiceGroupMemberList::<Identity, Impl, OFFSET>,
            BeginGetServiceGroupMemberTypeList: BeginGetServiceGroupMemberTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetServiceGroupMemberTypeList: EndGetServiceGroupMemberTypeList::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient5_Impl: Sized + IFabricQueryClient4_Impl {
    fn BeginGetUnplacedReplicaInformation(
        &self,
        querydescription : *const super::super:: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetUnplacedReplicaInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetUnplacedReplicaInformationResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient5 {}
impl IFabricQueryClient5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient5_Vtbl {
        unsafe extern "system" fn BeginGetUnplacedReplicaInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_UNPLACED_REPLICA_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetUnplacedReplicaInformation(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetUnplacedReplicaInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetUnplacedReplicaInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetUnplacedReplicaInformation: BeginGetUnplacedReplicaInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetUnplacedReplicaInformation: EndGetUnplacedReplicaInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient6_Impl: Sized + IFabricQueryClient5_Impl {
    fn EndGetNodeList2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetNodeListResult2>;
    fn EndGetApplicationList2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationListResult2>;
    fn EndGetServiceList2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceListResult2>;
    fn EndGetPartitionList2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetPartitionListResult2>;
    fn EndGetReplicaList2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetReplicaListResult2>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient6 {}
impl IFabricQueryClient6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient6_Vtbl {
        unsafe extern "system" fn EndGetNodeList2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeList2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetApplicationList2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationList2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetServiceList2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceList2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetPartitionList2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionList2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetReplicaList2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetReplicaList2(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            EndGetNodeList2: EndGetNodeList2::<Identity, Impl, OFFSET>,
            EndGetApplicationList2: EndGetApplicationList2::<Identity, Impl, OFFSET>,
            EndGetServiceList2: EndGetServiceList2::<Identity, Impl, OFFSET>,
            EndGetPartitionList2: EndGetPartitionList2::<Identity, Impl, OFFSET>,
            EndGetReplicaList2: EndGetReplicaList2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient7_Impl: Sized + IFabricQueryClient6_Impl {
    fn BeginGetApplicationLoadInformation(
        &self,
        querydescription : *const super::super:: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationLoadInformation(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationLoadInformationResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient7 {}
impl IFabricQueryClient7_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient7_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient7_Vtbl {
        unsafe extern "system" fn BeginGetApplicationLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription : *const super::super:: FABRIC_APPLICATION_LOAD_INFORMATION_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationLoadInformation(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationLoadInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient7_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationLoadInformation(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient6_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationLoadInformation: BeginGetApplicationLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationLoadInformation: EndGetApplicationLoadInformation::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient6 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient8_Impl: Sized + IFabricQueryClient7_Impl {
    fn BeginGetServiceName(
        &self,
        querydescription: *const super::super::FABRIC_SERVICE_NAME_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceName(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetServiceNameResult>;
    fn BeginGetApplicationName(
        &self,
        querydescription: *const super::super::FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationName(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationNameResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient8 {}
impl IFabricQueryClient8_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient8_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient8_Vtbl {
        unsafe extern "system" fn BeginGetServiceName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_SERVICE_NAME_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceName(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetServiceName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceName(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetApplicationName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_APPLICATION_NAME_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationName(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient8_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationName(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient7_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceName: BeginGetServiceName::<Identity, Impl, OFFSET>,
            EndGetServiceName: EndGetServiceName::<Identity, Impl, OFFSET>,
            BeginGetApplicationName: BeginGetApplicationName::<Identity, Impl, OFFSET>,
            EndGetApplicationName: EndGetApplicationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient8 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient7 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricQueryClient9_Impl: Sized + IFabricQueryClient8_Impl {
    fn BeginGetApplicationTypePagedList(
        &self,
        querydescription: *const super::super::PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetApplicationTypePagedList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetApplicationTypePagedListResult>;
}
impl ::windows::core::RuntimeName for IFabricQueryClient9 {}
impl IFabricQueryClient9_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricQueryClient9_Impl,
        const OFFSET: isize,
    >() -> IFabricQueryClient9_Vtbl {
        unsafe extern "system" fn BeginGetApplicationTypePagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::PAGED_FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetApplicationTypePagedList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetApplicationTypePagedList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricQueryClient9_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetApplicationTypePagedList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricQueryClient8_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetApplicationTypePagedList: BeginGetApplicationTypePagedList::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetApplicationTypePagedList: EndGetApplicationTypePagedList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricQueryClient9 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient7 as ::windows::core::Interface>::IID
            || iid == &<IFabricQueryClient8 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricRepairManagementClient_Impl: Sized {
    fn BeginCreateRepairTask(
        &self,
        repairtask: *const super::super::FABRIC_REPAIR_TASK,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateRepairTask(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginCancelRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_CANCEL_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCancelRepairTask(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginForceApproveRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_APPROVE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndForceApproveRepairTask(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginDeleteRepairTask(
        &self,
        requestdescription: *const super::super::FABRIC_REPAIR_DELETE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteRepairTask(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginUpdateRepairExecutionState(
        &self,
        repairtask: *const super::super::FABRIC_REPAIR_TASK,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateRepairExecutionState(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginGetRepairTaskList(
        &self,
        querydescription: *const super::super::FABRIC_REPAIR_TASK_QUERY_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetRepairTaskList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricGetRepairTaskListResult>;
}
impl ::windows::core::RuntimeName for IFabricRepairManagementClient {}
impl IFabricRepairManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRepairManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricRepairManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            repairtask: *const super::super::FABRIC_REPAIR_TASK,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateRepairTask(
                ::core::mem::transmute_copy(&repairtask),
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
        unsafe extern "system" fn EndCreateRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndCreateRepairTask(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCancelRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_CANCEL_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCancelRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
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
        unsafe extern "system" fn EndCancelRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndCancelRepairTask(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginForceApproveRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_APPROVE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginForceApproveRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
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
        unsafe extern "system" fn EndForceApproveRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndForceApproveRepairTask(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDeleteRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            requestdescription: *const super::super::FABRIC_REPAIR_DELETE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteRepairTask(
                ::core::mem::transmute_copy(&requestdescription),
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
        unsafe extern "system" fn EndDeleteRepairTask<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteRepairTask(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginUpdateRepairExecutionState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            repairtask: *const super::super::FABRIC_REPAIR_TASK,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateRepairExecutionState(
                ::core::mem::transmute_copy(&repairtask),
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
        unsafe extern "system" fn EndUpdateRepairExecutionState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndUpdateRepairExecutionState(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetRepairTaskList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            querydescription: *const super::super::FABRIC_REPAIR_TASK_QUERY_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetRepairTaskList(
                ::core::mem::transmute_copy(&querydescription),
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
        unsafe extern "system" fn EndGetRepairTaskList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetRepairTaskList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateRepairTask: BeginCreateRepairTask::<Identity, Impl, OFFSET>,
            EndCreateRepairTask: EndCreateRepairTask::<Identity, Impl, OFFSET>,
            BeginCancelRepairTask: BeginCancelRepairTask::<Identity, Impl, OFFSET>,
            EndCancelRepairTask: EndCancelRepairTask::<Identity, Impl, OFFSET>,
            BeginForceApproveRepairTask: BeginForceApproveRepairTask::<Identity, Impl, OFFSET>,
            EndForceApproveRepairTask: EndForceApproveRepairTask::<Identity, Impl, OFFSET>,
            BeginDeleteRepairTask: BeginDeleteRepairTask::<Identity, Impl, OFFSET>,
            EndDeleteRepairTask: EndDeleteRepairTask::<Identity, Impl, OFFSET>,
            BeginUpdateRepairExecutionState: BeginUpdateRepairExecutionState::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUpdateRepairExecutionState: EndUpdateRepairExecutionState::<Identity, Impl, OFFSET>,
            BeginGetRepairTaskList: BeginGetRepairTaskList::<Identity, Impl, OFFSET>,
            EndGetRepairTaskList: EndGetRepairTaskList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricRepairManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricRepairManagementClient2_Impl: Sized + IFabricRepairManagementClient_Impl {
    fn BeginUpdateRepairTaskHealthPolicy(
        &self,
        updatedescription: *const super::super::FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateRepairTaskHealthPolicy(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IFabricRepairManagementClient2 {}
impl IFabricRepairManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRepairManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricRepairManagementClient2_Vtbl {
        unsafe extern "system" fn BeginUpdateRepairTaskHealthPolicy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updatedescription : *const super::super:: FABRIC_REPAIR_TASK_HEALTH_POLICY_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateRepairTaskHealthPolicy(
                ::core::mem::transmute_copy(&updatedescription),
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
        unsafe extern "system" fn EndUpdateRepairTaskHealthPolicy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRepairManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            commitversion: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndUpdateRepairTaskHealthPolicy(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commitversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricRepairManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateRepairTaskHealthPolicy: BeginUpdateRepairTaskHealthPolicy::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUpdateRepairTaskHealthPolicy: EndUpdateRepairTaskHealthPolicy::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricRepairManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricRepairManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricReplicaHealthResult_Impl: Sized {
    fn get_ReplicaHealth(&self) -> *mut super::super::FABRIC_REPLICA_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricReplicaHealthResult {}
impl IFabricReplicaHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricReplicaHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricReplicaHealthResult_Vtbl {
        unsafe extern "system" fn get_ReplicaHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricReplicaHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICA_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ReplicaHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ReplicaHealth: get_ReplicaHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricReplicaHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricResolvedServicePartitionResult_Impl: Sized {
    fn get_Partition(&self) -> *mut super::super::FABRIC_RESOLVED_SERVICE_PARTITION;
    fn GetEndpoint(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_RESOLVED_SERVICE_ENDPOINT>;
    fn CompareVersion(
        &self,
        other: &::core::option::Option<IFabricResolvedServicePartitionResult>,
    ) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IFabricResolvedServicePartitionResult {}
impl IFabricResolvedServicePartitionResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricResolvedServicePartitionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricResolvedServicePartitionResult_Vtbl {
        unsafe extern "system" fn get_Partition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RESOLVED_SERVICE_PARTITION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Partition()
        }
        unsafe extern "system" fn GetEndpoint<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            endpoint: *mut *mut super::super::FABRIC_RESOLVED_SERVICE_ENDPOINT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(endpoint, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricResolvedServicePartitionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            other: *mut ::core::ffi::c_void,
            compareresult: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareVersion(::core::mem::transmute(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compareresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Partition: get_Partition::<Identity, Impl, OFFSET>,
            GetEndpoint: GetEndpoint::<Identity, Impl, OFFSET>,
            CompareVersion: CompareVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricResolvedServicePartitionResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricRestartDeployedCodePackageResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_RESULT;
}
impl ::windows::core::RuntimeName for IFabricRestartDeployedCodePackageResult {}
impl IFabricRestartDeployedCodePackageResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRestartDeployedCodePackageResult_Impl,
        const OFFSET: isize,
    >() -> IFabricRestartDeployedCodePackageResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRestartDeployedCodePackageResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DEPLOYED_CODE_PACKAGE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricRestartDeployedCodePackageResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricRestartNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows::core::RuntimeName for IFabricRestartNodeResult {}
impl IFabricRestartNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricRestartNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricRestartNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricRestartNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricRestartNodeResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricSecretReferencesResult_Impl: Sized {
    fn get_SecretReferences(&self) -> *mut super::super::FABRIC_SECRET_REFERENCE_LIST;
}
impl ::windows::core::RuntimeName for IFabricSecretReferencesResult {}
impl IFabricSecretReferencesResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretReferencesResult_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretReferencesResult_Vtbl {
        unsafe extern "system" fn get_SecretReferences<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretReferencesResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECRET_REFERENCE_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_SecretReferences()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_SecretReferences: get_SecretReferences::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricSecretReferencesResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricSecretStoreClient_Impl: Sized {
    fn BeginGetSecrets(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        includevalue: ::windows::Win32::Foundation::BOOLEAN,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetSecrets(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricSecretsResult>;
    fn BeginSetSecrets(
        &self,
        secrets: *const super::super::FABRIC_SECRET_LIST,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetSecrets(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricSecretsResult>;
    fn BeginRemoveSecrets(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRemoveSecrets(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricSecretReferencesResult>;
    fn BeginGetSecretVersions(
        &self,
        secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetSecretVersions(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricSecretReferencesResult>;
}
impl ::windows::core::RuntimeName for IFabricSecretStoreClient {}
impl IFabricSecretStoreClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretStoreClient_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretStoreClient_Vtbl {
        unsafe extern "system" fn BeginGetSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            includevalue: ::windows::Win32::Foundation::BOOLEAN,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetSecrets(
                ::core::mem::transmute_copy(&secretreferences),
                ::core::mem::transmute_copy(&includevalue),
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
        unsafe extern "system" fn EndGetSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetSecrets(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secrets: *const super::super::FABRIC_SECRET_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetSecrets(
                ::core::mem::transmute_copy(&secrets),
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
        unsafe extern "system" fn EndSetSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSetSecrets(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRemoveSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRemoveSecrets(
                ::core::mem::transmute_copy(&secretreferences),
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
        unsafe extern "system" fn EndRemoveSecrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRemoveSecrets(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetSecretVersions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            secretreferences: *const super::super::FABRIC_SECRET_REFERENCE_LIST,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetSecretVersions(
                ::core::mem::transmute_copy(&secretreferences),
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
        unsafe extern "system" fn EndGetSecretVersions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretStoreClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetSecretVersions(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetSecrets: BeginGetSecrets::<Identity, Impl, OFFSET>,
            EndGetSecrets: EndGetSecrets::<Identity, Impl, OFFSET>,
            BeginSetSecrets: BeginSetSecrets::<Identity, Impl, OFFSET>,
            EndSetSecrets: EndSetSecrets::<Identity, Impl, OFFSET>,
            BeginRemoveSecrets: BeginRemoveSecrets::<Identity, Impl, OFFSET>,
            EndRemoveSecrets: EndRemoveSecrets::<Identity, Impl, OFFSET>,
            BeginGetSecretVersions: BeginGetSecretVersions::<Identity, Impl, OFFSET>,
            EndGetSecretVersions: EndGetSecretVersions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricSecretStoreClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricSecretsResult_Impl: Sized {
    fn get_Secrets(&self) -> *mut super::super::FABRIC_SECRET_LIST;
}
impl ::windows::core::RuntimeName for IFabricSecretsResult {}
impl IFabricSecretsResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricSecretsResult_Impl,
        const OFFSET: isize,
    >() -> IFabricSecretsResult_Vtbl {
        unsafe extern "system" fn get_Secrets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricSecretsResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECRET_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Secrets()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Secrets: get_Secrets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricSecretsResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceDescriptionResult_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_SERVICE_DESCRIPTION;
}
impl ::windows::core::RuntimeName for IFabricServiceDescriptionResult {}
impl IFabricServiceDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceDescriptionResult_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceDescriptionResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceEndpointsVersion_Impl: Sized {
    fn Compare(
        &self,
        other: &::core::option::Option<IFabricServiceEndpointsVersion>,
    ) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IFabricServiceEndpointsVersion {}
impl IFabricServiceEndpointsVersion_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceEndpointsVersion_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceEndpointsVersion_Vtbl {
        unsafe extern "system" fn Compare<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceEndpointsVersion_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            other: *mut ::core::ffi::c_void,
            compareresult: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::core::mem::transmute(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compareresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compare: Compare::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceEndpointsVersion as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupDescriptionResult_Impl: Sized {
    fn get_Description(&self) -> *mut super::super::FABRIC_SERVICE_GROUP_DESCRIPTION;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupDescriptionResult {}
impl IFabricServiceGroupDescriptionResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupDescriptionResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupDescriptionResult_Vtbl {
        unsafe extern "system" fn get_Description<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupDescriptionResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_GROUP_DESCRIPTION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Description()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupDescriptionResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupManagementClient_Impl: Sized {
    fn BeginCreateServiceGroup(
        &self,
        description: *const super::super::FABRIC_SERVICE_GROUP_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroup(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteServiceGroup(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteServiceGroup(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetServiceGroupDescription(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceGroupDescription(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricServiceGroupDescriptionResult>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupManagementClient {}
impl IFabricServiceGroupManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_GROUP_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroup(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndCreateServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroup(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteServiceGroup(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndDeleteServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteServiceGroup(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetServiceGroupDescription<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceGroupDescription(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndGetServiceGroupDescription<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceGroupDescription(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateServiceGroup: BeginCreateServiceGroup::<Identity, Impl, OFFSET>,
            EndCreateServiceGroup: EndCreateServiceGroup::<Identity, Impl, OFFSET>,
            BeginDeleteServiceGroup: BeginDeleteServiceGroup::<Identity, Impl, OFFSET>,
            EndDeleteServiceGroup: EndDeleteServiceGroup::<Identity, Impl, OFFSET>,
            BeginGetServiceGroupDescription: BeginGetServiceGroupDescription::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetServiceGroupDescription: EndGetServiceGroupDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupManagementClient2_Impl:
    Sized + IFabricServiceGroupManagementClient_Impl
{
    fn BeginUpdateServiceGroup(
        &self,
        name: *const u16,
        servicegroupupdatedescription: *const super::super::FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateServiceGroup(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupManagementClient2 {}
impl IFabricServiceGroupManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient2_Vtbl {
        unsafe extern "system" fn BeginUpdateServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            servicegroupupdatedescription : *const super::super:: FABRIC_SERVICE_GROUP_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateServiceGroup(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&servicegroupupdatedescription),
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
        unsafe extern "system" fn EndUpdateServiceGroup<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateServiceGroup(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUpdateServiceGroup: BeginUpdateServiceGroup::<Identity, Impl, OFFSET>,
            EndUpdateServiceGroup: EndUpdateServiceGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupManagementClient3_Impl:
    Sized + IFabricServiceGroupManagementClient2_Impl
{
    fn BeginCreateServiceGroupFromTemplate(
        &self,
        applicationname: *const u16,
        servicename: *const u16,
        servicetypename: &::windows::core::PCWSTR,
        initializationdatasize: u32,
        initializationdata: *const u8,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroupFromTemplate(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupManagementClient3 {}
impl IFabricServiceGroupManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient3_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroupFromTemplate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicename: *const u16,
            servicetypename: ::windows::core::PCWSTR,
            initializationdatasize: u32,
            initializationdata: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroupFromTemplate(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&initializationdatasize),
                ::core::mem::transmute_copy(&initializationdata),
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
        unsafe extern "system" fn EndCreateServiceGroupFromTemplate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroupFromTemplate(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceGroupFromTemplate: BeginCreateServiceGroupFromTemplate::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceGroupFromTemplate: EndCreateServiceGroupFromTemplate::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceGroupManagementClient4_Impl:
    Sized + IFabricServiceGroupManagementClient3_Impl
{
    fn BeginCreateServiceGroupFromTemplate2(
        &self,
        servicegroupfromtemplatedescription : *const super::super:: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceGroupFromTemplate2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceGroupManagementClient4 {}
impl IFabricServiceGroupManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceGroupManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceGroupManagementClient4_Vtbl {
        unsafe extern "system" fn BeginCreateServiceGroupFromTemplate2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicegroupfromtemplatedescription : *const super::super:: FABRIC_SERVICE_GROUP_FROM_TEMPLATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceGroupFromTemplate2(
                ::core::mem::transmute_copy(&servicegroupfromtemplatedescription),
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
        unsafe extern "system" fn EndCreateServiceGroupFromTemplate2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceGroupManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceGroupFromTemplate2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceGroupManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceGroupFromTemplate2: BeginCreateServiceGroupFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceGroupFromTemplate2: EndCreateServiceGroupFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceGroupManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceGroupManagementClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceHealthResult_Impl: Sized {
    fn get_ServiceHealth(&self) -> *mut super::super::FABRIC_SERVICE_HEALTH;
}
impl ::windows::core::RuntimeName for IFabricServiceHealthResult {}
impl IFabricServiceHealthResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceHealthResult_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceHealthResult_Vtbl {
        unsafe extern "system" fn get_ServiceHealth<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceHealthResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_HEALTH {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ServiceHealth()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ServiceHealth: get_ServiceHealth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceHealthResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient_Impl: Sized {
    fn BeginCreateService(
        &self,
        description: *const super::super::FABRIC_SERVICE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateService(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginCreateServiceFromTemplate(
        &self,
        applicationname: *const u16,
        servicename: *const u16,
        servicetypename: &::windows::core::PCWSTR,
        initializationdatasize: u32,
        initializationdata: *const u8,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceFromTemplate(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginDeleteService(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteService(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetServiceDescription(
        &self,
        name: *const u16,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceDescription(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricServiceDescriptionResult>;
    fn RegisterServicePartitionResolutionChangeHandler(
        &self,
        name: *const u16,
        keytype: super::super::FABRIC_PARTITION_KEY_TYPE,
        partitionkey: *const ::core::ffi::c_void,
        callback: &::core::option::Option<IFabricServicePartitionResolutionChangeHandler>,
    ) -> ::windows::core::Result<i64>;
    fn UnregisterServicePartitionResolutionChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()>;
    fn BeginResolveServicePartition(
        &self,
        name: *const u16,
        partitionkeytype: super::super::FABRIC_PARTITION_KEY_TYPE,
        partitionkey: *const ::core::ffi::c_void,
        previousresult: &::core::option::Option<IFabricResolvedServicePartitionResult>,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndResolveServicePartition(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricResolvedServicePartitionResult>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient {}
impl IFabricServiceManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient_Vtbl {
        unsafe extern "system" fn BeginCreateService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateService(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndCreateService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateService(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginCreateServiceFromTemplate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationname: *const u16,
            servicename: *const u16,
            servicetypename: ::windows::core::PCWSTR,
            initializationdatasize: u32,
            initializationdata: *const u8,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceFromTemplate(
                ::core::mem::transmute_copy(&applicationname),
                ::core::mem::transmute_copy(&servicename),
                ::core::mem::transmute(&servicetypename),
                ::core::mem::transmute_copy(&initializationdatasize),
                ::core::mem::transmute_copy(&initializationdata),
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
        unsafe extern "system" fn EndCreateServiceFromTemplate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceFromTemplate(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginDeleteService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteService(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndDeleteService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteService(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetServiceDescription<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceDescription(
                ::core::mem::transmute_copy(&name),
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
        unsafe extern "system" fn EndGetServiceDescription<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceDescription(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServicePartitionResolutionChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            keytype: super::super::FABRIC_PARTITION_KEY_TYPE,
            partitionkey: *const ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterServicePartitionResolutionChangeHandler(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&keytype),
                ::core::mem::transmute_copy(&partitionkey),
                ::core::mem::transmute(&callback),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(callbackhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterServicePartitionResolutionChangeHandler<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterServicePartitionResolutionChangeHandler(::core::mem::transmute_copy(
                &callbackhandle,
            ))
            .into()
        }
        unsafe extern "system" fn BeginResolveServicePartition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            partitionkeytype: super::super::FABRIC_PARTITION_KEY_TYPE,
            partitionkey: *const ::core::ffi::c_void,
            previousresult: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginResolveServicePartition(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&partitionkeytype),
                ::core::mem::transmute_copy(&partitionkey),
                ::core::mem::transmute(&previousresult),
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
        unsafe extern "system" fn EndResolveServicePartition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndResolveServicePartition(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateService: BeginCreateService::<Identity, Impl, OFFSET>,
            EndCreateService: EndCreateService::<Identity, Impl, OFFSET>,
            BeginCreateServiceFromTemplate: BeginCreateServiceFromTemplate::<Identity, Impl, OFFSET>,
            EndCreateServiceFromTemplate: EndCreateServiceFromTemplate::<Identity, Impl, OFFSET>,
            BeginDeleteService: BeginDeleteService::<Identity, Impl, OFFSET>,
            EndDeleteService: EndDeleteService::<Identity, Impl, OFFSET>,
            BeginGetServiceDescription: BeginGetServiceDescription::<Identity, Impl, OFFSET>,
            EndGetServiceDescription: EndGetServiceDescription::<Identity, Impl, OFFSET>,
            RegisterServicePartitionResolutionChangeHandler:
                RegisterServicePartitionResolutionChangeHandler::<Identity, Impl, OFFSET>,
            UnregisterServicePartitionResolutionChangeHandler:
                UnregisterServicePartitionResolutionChangeHandler::<Identity, Impl, OFFSET>,
            BeginResolveServicePartition: BeginResolveServicePartition::<Identity, Impl, OFFSET>,
            EndResolveServicePartition: EndResolveServicePartition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient2_Impl:
    Sized + IFabricServiceManagementClient_Impl
{
    fn BeginGetServiceManifest(
        &self,
        applicationtypename: &::windows::core::PCWSTR,
        applicationtypeversion: &::windows::core::PCWSTR,
        servicemanifestname: &::windows::core::PCWSTR,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetServiceManifest(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<super::IFabricStringResult>;
    fn BeginUpdateService(
        &self,
        name: *const u16,
        serviceupdatedescription: *const super::super::FABRIC_SERVICE_UPDATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUpdateService(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient2 {}
impl IFabricServiceManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient2_Vtbl {
        unsafe extern "system" fn BeginGetServiceManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            applicationtypename: ::windows::core::PCWSTR,
            applicationtypeversion: ::windows::core::PCWSTR,
            servicemanifestname: ::windows::core::PCWSTR,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetServiceManifest(
                ::core::mem::transmute(&applicationtypename),
                ::core::mem::transmute(&applicationtypeversion),
                ::core::mem::transmute(&servicemanifestname),
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
        unsafe extern "system" fn EndGetServiceManifest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetServiceManifest(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: *const u16,
            serviceupdatedescription: *const super::super::FABRIC_SERVICE_UPDATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUpdateService(
                ::core::mem::transmute_copy(&name),
                ::core::mem::transmute_copy(&serviceupdatedescription),
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
        unsafe extern "system" fn EndUpdateService<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdateService(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetServiceManifest: BeginGetServiceManifest::<Identity, Impl, OFFSET>,
            EndGetServiceManifest: EndGetServiceManifest::<Identity, Impl, OFFSET>,
            BeginUpdateService: BeginUpdateService::<Identity, Impl, OFFSET>,
            EndUpdateService: EndUpdateService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient3_Impl:
    Sized + IFabricServiceManagementClient2_Impl
{
    fn BeginRemoveReplica(
        &self,
        description: *const super::super::FABRIC_REMOVE_REPLICA_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRemoveReplica(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginRestartReplica(
        &self,
        description: *const super::super::FABRIC_RESTART_REPLICA_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRestartReplica(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient3 {}
impl IFabricServiceManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient3_Vtbl {
        unsafe extern "system" fn BeginRemoveReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_REMOVE_REPLICA_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRemoveReplica(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndRemoveReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRemoveReplica(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginRestartReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_RESTART_REPLICA_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRestartReplica(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndRestartReplica<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRestartReplica(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRemoveReplica: BeginRemoveReplica::<Identity, Impl, OFFSET>,
            EndRemoveReplica: EndRemoveReplica::<Identity, Impl, OFFSET>,
            BeginRestartReplica: BeginRestartReplica::<Identity, Impl, OFFSET>,
            EndRestartReplica: EndRestartReplica::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient4_Impl:
    Sized + IFabricServiceManagementClient3_Impl
{
    fn BeginRegisterServiceNotificationFilter(
        &self,
        description: *const super::super::FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndRegisterServiceNotificationFilter(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<i64>;
    fn BeginUnregisterServiceNotificationFilter(
        &self,
        filterid: i64,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndUnregisterServiceNotificationFilter(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient4 {}
impl IFabricServiceManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient4_Vtbl {
        unsafe extern "system" fn BeginRegisterServiceNotificationFilter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_SERVICE_NOTIFICATION_FILTER_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRegisterServiceNotificationFilter(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndRegisterServiceNotificationFilter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            filterid: *mut i64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndRegisterServiceNotificationFilter(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filterid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUnregisterServiceNotificationFilter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            filterid: i64,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnregisterServiceNotificationFilter(
                ::core::mem::transmute_copy(&filterid),
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
        unsafe extern "system" fn EndUnregisterServiceNotificationFilter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnregisterServiceNotificationFilter(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRegisterServiceNotificationFilter: BeginRegisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndRegisterServiceNotificationFilter: EndRegisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginUnregisterServiceNotificationFilter: BeginUnregisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndUnregisterServiceNotificationFilter: EndUnregisterServiceNotificationFilter::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient5_Impl:
    Sized + IFabricServiceManagementClient4_Impl
{
    fn BeginDeleteService2(
        &self,
        deletedescription: *const super::super::FABRIC_DELETE_SERVICE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndDeleteService2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient5 {}
impl IFabricServiceManagementClient5_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient5_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient5_Vtbl {
        unsafe extern "system" fn BeginDeleteService2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            deletedescription: *const super::super::FABRIC_DELETE_SERVICE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDeleteService2(
                ::core::mem::transmute_copy(&deletedescription),
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
        unsafe extern "system" fn EndDeleteService2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient5_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDeleteService2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient4_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginDeleteService2: BeginDeleteService2::<Identity, Impl, OFFSET>,
            EndDeleteService2: EndDeleteService2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient5 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient4 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceManagementClient6_Impl:
    Sized + IFabricServiceManagementClient5_Impl
{
    fn BeginCreateServiceFromTemplate2(
        &self,
        servicefromtemplatedescription : *const super::super:: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCreateServiceFromTemplate2(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceManagementClient6 {}
impl IFabricServiceManagementClient6_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceManagementClient6_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceManagementClient6_Vtbl {
        unsafe extern "system" fn BeginCreateServiceFromTemplate2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            servicefromtemplatedescription : *const super::super:: FABRIC_SERVICE_FROM_TEMPLATE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCreateServiceFromTemplate2(
                ::core::mem::transmute_copy(&servicefromtemplatedescription),
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
        unsafe extern "system" fn EndCreateServiceFromTemplate2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceManagementClient6_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCreateServiceFromTemplate2(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: IFabricServiceManagementClient5_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginCreateServiceFromTemplate2: BeginCreateServiceFromTemplate2::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndCreateServiceFromTemplate2: EndCreateServiceFromTemplate2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceManagementClient6 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricServiceManagementClient5 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceNotification_Impl: Sized {
    fn get_Notification(&self) -> *mut super::super::FABRIC_SERVICE_NOTIFICATION;
    fn GetVersion(&self) -> ::windows::core::Result<IFabricServiceEndpointsVersion>;
}
impl ::windows::core::RuntimeName for IFabricServiceNotification {}
impl IFabricServiceNotification_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceNotification_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceNotification_Vtbl {
        unsafe extern "system" fn get_Notification<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotification_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SERVICE_NOTIFICATION {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Notification()
        }
        unsafe extern "system" fn GetVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotification_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Notification: get_Notification::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceNotification as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServiceNotificationEventHandler_Impl: Sized {
    fn OnNotification(
        &self,
        __midl__ifabricservicenotificationeventhandler0000: &::core::option::Option<
            IFabricServiceNotification,
        >,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricServiceNotificationEventHandler {}
impl IFabricServiceNotificationEventHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServiceNotificationEventHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricServiceNotificationEventHandler_Vtbl {
        unsafe extern "system" fn OnNotification<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServiceNotificationEventHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            __midl__ifabricservicenotificationeventhandler0000: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNotification(::core::mem::transmute(
                &__midl__ifabricservicenotificationeventhandler0000,
            ))
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNotification: OnNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServiceNotificationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricServicePartitionResolutionChangeHandler_Impl: Sized {
    fn OnChange(
        &self,
        source: &::core::option::Option<IFabricServiceManagementClient>,
        handlerid: i64,
        partition: &::core::option::Option<IFabricResolvedServicePartitionResult>,
        error: ::windows::core::HRESULT,
    );
}
impl ::windows::core::RuntimeName for IFabricServicePartitionResolutionChangeHandler {}
impl IFabricServicePartitionResolutionChangeHandler_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricServicePartitionResolutionChangeHandler_Impl,
        const OFFSET: isize,
    >() -> IFabricServicePartitionResolutionChangeHandler_Vtbl {
        unsafe extern "system" fn OnChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricServicePartitionResolutionChangeHandler_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            handlerid: i64,
            partition: *mut ::core::ffi::c_void,
            error: ::windows::core::HRESULT,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChange(
                ::core::mem::transmute(&source),
                ::core::mem::transmute_copy(&handlerid),
                ::core::mem::transmute(&partition),
                ::core::mem::transmute_copy(&error),
            )
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnChange: OnChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricServicePartitionResolutionChangeHandler as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStartNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows::core::RuntimeName for IFabricStartNodeResult {}
impl IFabricStartNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStartNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStartNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStartNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStartNodeResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricStopNodeResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::FABRIC_NODE_RESULT;
}
impl ::windows::core::RuntimeName for IFabricStopNodeResult {}
impl IFabricStopNodeResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricStopNodeResult_Impl,
        const OFFSET: isize,
    >() -> IFabricStopNodeResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricStopNodeResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_NODE_RESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricStopNodeResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTestCommandStatusResult_Impl: Sized {
    fn get_Result(&self) -> *mut super::super::TEST_COMMAND_QUERY_RESULT_LIST;
}
impl ::windows::core::RuntimeName for IFabricTestCommandStatusResult {}
impl IFabricTestCommandStatusResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestCommandStatusResult_Impl,
        const OFFSET: isize,
    >() -> IFabricTestCommandStatusResult_Vtbl {
        unsafe extern "system" fn get_Result<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestCommandStatusResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::TEST_COMMAND_QUERY_RESULT_LIST {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_Result()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Result: get_Result::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTestCommandStatusResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTestManagementClient_Impl: Sized {
    fn BeginStartPartitionDataLoss(
        &self,
        invokedatalossdescription : *const super::super:: FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionDataLoss(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetPartitionDataLossProgress(
        &self,
        operationid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionDataLossProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPartitionDataLossProgressResult>;
    fn BeginStartPartitionQuorumLoss(
        &self,
        invokequorumlossdescription : *const super::super:: FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionQuorumLoss(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetPartitionQuorumLossProgress(
        &self,
        operationid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionQuorumLossProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPartitionQuorumLossProgressResult>;
    fn BeginStartPartitionRestart(
        &self,
        restartpartitiondescription : *const super::super:: FABRIC_START_PARTITION_RESTART_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartPartitionRestart(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetPartitionRestartProgress(
        &self,
        operationid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetPartitionRestartProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricPartitionRestartProgressResult>;
    fn BeginGetTestCommandStatusList(
        &self,
        operationid: *const super::super::FABRIC_TEST_COMMAND_LIST_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetTestCommandStatusList(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricTestCommandStatusResult>;
    fn BeginCancelTestCommand(
        &self,
        invokedatalossdescription: *const super::super::FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndCancelTestCommand(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricTestManagementClient {}
impl IFabricTestManagementClient_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient_Vtbl {
        unsafe extern "system" fn BeginStartPartitionDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokedatalossdescription : *const super::super:: FABRIC_START_PARTITION_DATA_LOSS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionDataLoss(
                ::core::mem::transmute_copy(&invokedatalossdescription),
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
        unsafe extern "system" fn EndStartPartitionDataLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionDataLoss(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionDataLossProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionDataLossProgress(
                ::core::mem::transmute(&operationid),
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
        unsafe extern "system" fn EndGetPartitionDataLossProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionDataLossProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartPartitionQuorumLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokequorumlossdescription : *const super::super:: FABRIC_START_PARTITION_QUORUM_LOSS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionQuorumLoss(
                ::core::mem::transmute_copy(&invokequorumlossdescription),
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
        unsafe extern "system" fn EndStartPartitionQuorumLoss<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionQuorumLoss(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionQuorumLossProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionQuorumLossProgress(
                ::core::mem::transmute(&operationid),
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
        unsafe extern "system" fn EndGetPartitionQuorumLossProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionQuorumLossProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginStartPartitionRestart<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartpartitiondescription : *const super::super:: FABRIC_START_PARTITION_RESTART_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartPartitionRestart(
                ::core::mem::transmute_copy(&restartpartitiondescription),
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
        unsafe extern "system" fn EndStartPartitionRestart<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartPartitionRestart(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetPartitionRestartProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetPartitionRestartProgress(
                ::core::mem::transmute(&operationid),
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
        unsafe extern "system" fn EndGetPartitionRestartProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetPartitionRestartProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetTestCommandStatusList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: *const super::super::FABRIC_TEST_COMMAND_LIST_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetTestCommandStatusList(
                ::core::mem::transmute_copy(&operationid),
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
        unsafe extern "system" fn EndGetTestCommandStatusList<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetTestCommandStatusList(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCancelTestCommand<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            invokedatalossdescription: *const super::super::FABRIC_CANCEL_TEST_COMMAND_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginCancelTestCommand(
                ::core::mem::transmute_copy(&invokedatalossdescription),
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
        unsafe extern "system" fn EndCancelTestCommand<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCancelTestCommand(::core::mem::transmute(&context))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginStartPartitionDataLoss: BeginStartPartitionDataLoss::<Identity, Impl, OFFSET>,
            EndStartPartitionDataLoss: EndStartPartitionDataLoss::<Identity, Impl, OFFSET>,
            BeginGetPartitionDataLossProgress: BeginGetPartitionDataLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionDataLossProgress: EndGetPartitionDataLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginStartPartitionQuorumLoss: BeginStartPartitionQuorumLoss::<Identity, Impl, OFFSET>,
            EndStartPartitionQuorumLoss: EndStartPartitionQuorumLoss::<Identity, Impl, OFFSET>,
            BeginGetPartitionQuorumLossProgress: BeginGetPartitionQuorumLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionQuorumLossProgress: EndGetPartitionQuorumLossProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            BeginStartPartitionRestart: BeginStartPartitionRestart::<Identity, Impl, OFFSET>,
            EndStartPartitionRestart: EndStartPartitionRestart::<Identity, Impl, OFFSET>,
            BeginGetPartitionRestartProgress: BeginGetPartitionRestartProgress::<
                Identity,
                Impl,
                OFFSET,
            >,
            EndGetPartitionRestartProgress: EndGetPartitionRestartProgress::<Identity, Impl, OFFSET>,
            BeginGetTestCommandStatusList: BeginGetTestCommandStatusList::<Identity, Impl, OFFSET>,
            EndGetTestCommandStatusList: EndGetTestCommandStatusList::<Identity, Impl, OFFSET>,
            BeginCancelTestCommand: BeginCancelTestCommand::<Identity, Impl, OFFSET>,
            EndCancelTestCommand: EndCancelTestCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTestManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTestManagementClient2_Impl: Sized + IFabricTestManagementClient_Impl {
    fn BeginStartChaos(
        &self,
        restartpartitiondescription: *const super::super::FABRIC_START_CHAOS_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartChaos(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginStopChaos(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStopChaos(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetChaosReport(
        &self,
        getchaosreportdescription: *const super::super::FABRIC_GET_CHAOS_REPORT_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosReport(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricChaosReportResult>;
}
impl ::windows::core::RuntimeName for IFabricTestManagementClient2 {}
impl IFabricTestManagementClient2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient2_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient2_Vtbl {
        unsafe extern "system" fn BeginStartChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            restartpartitiondescription: *const super::super::FABRIC_START_CHAOS_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartChaos(
                ::core::mem::transmute_copy(&restartpartitiondescription),
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
        unsafe extern "system" fn EndStartChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartChaos(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginStopChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStopChaos(
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
        unsafe extern "system" fn EndStopChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStopChaos(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn BeginGetChaosReport<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            getchaosreportdescription: *const super::super::FABRIC_GET_CHAOS_REPORT_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosReport(
                ::core::mem::transmute_copy(&getchaosreportdescription),
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
        unsafe extern "system" fn EndGetChaosReport<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosReport(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginStartChaos: BeginStartChaos::<Identity, Impl, OFFSET>,
            EndStartChaos: EndStartChaos::<Identity, Impl, OFFSET>,
            BeginStopChaos: BeginStopChaos::<Identity, Impl, OFFSET>,
            EndStopChaos: EndStopChaos::<Identity, Impl, OFFSET>,
            BeginGetChaosReport: BeginGetChaosReport::<Identity, Impl, OFFSET>,
            EndGetChaosReport: EndGetChaosReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTestManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTestManagementClient3_Impl: Sized + IFabricTestManagementClient2_Impl {
    fn BeginStartNodeTransition(
        &self,
        description: *const super::super::FABRIC_NODE_TRANSITION_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndStartNodeTransition(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetNodeTransitionProgress(
        &self,
        operationid: &::windows::core::GUID,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetNodeTransitionProgress(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricNodeTransitionProgressResult>;
}
impl ::windows::core::RuntimeName for IFabricTestManagementClient3 {}
impl IFabricTestManagementClient3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient3_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient3_Vtbl {
        unsafe extern "system" fn BeginStartNodeTransition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            description: *const super::super::FABRIC_NODE_TRANSITION_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginStartNodeTransition(
                ::core::mem::transmute_copy(&description),
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
        unsafe extern "system" fn EndStartNodeTransition<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndStartNodeTransition(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetNodeTransitionProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            operationid: ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetNodeTransitionProgress(
                ::core::mem::transmute(&operationid),
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
        unsafe extern "system" fn EndGetNodeTransitionProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetNodeTransitionProgress(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginStartNodeTransition: BeginStartNodeTransition::<Identity, Impl, OFFSET>,
            EndStartNodeTransition: EndStartNodeTransition::<Identity, Impl, OFFSET>,
            BeginGetNodeTransitionProgress: BeginGetNodeTransitionProgress::<Identity, Impl, OFFSET>,
            EndGetNodeTransitionProgress: EndGetNodeTransitionProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTestManagementClient3 as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricTestManagementClient4_Impl: Sized + IFabricTestManagementClient3_Impl {
    fn BeginGetChaos(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaos(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricChaosDescriptionResult>;
    fn BeginGetChaosSchedule(
        &self,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosSchedule(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricChaosScheduleDescriptionResult>;
    fn BeginSetChaosSchedule(
        &self,
        setchaosscheduledescription: *const super::super::FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndSetChaosSchedule(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>;
    fn BeginGetChaosEvents(
        &self,
        chaoseventsdescription: *const super::super::FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION,
        timeoutmilliseconds: u32,
        callback: &::core::option::Option<super::IFabricAsyncOperationCallback>,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>;
    fn EndGetChaosEvents(
        &self,
        context: &::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<IFabricChaosEventsSegmentResult>;
}
impl ::windows::core::RuntimeName for IFabricTestManagementClient4 {}
impl IFabricTestManagementClient4_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricTestManagementClient4_Impl,
        const OFFSET: isize,
    >() -> IFabricTestManagementClient4_Vtbl {
        unsafe extern "system" fn BeginGetChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaos(
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
        unsafe extern "system" fn EndGetChaos<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaos(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetChaosSchedule<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosSchedule(
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
        unsafe extern "system" fn EndGetChaosSchedule<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosSchedule(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSetChaosSchedule<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            setchaosscheduledescription : *const super::super:: FABRIC_CHAOS_SERVICE_SCHEDULE_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSetChaosSchedule(
                ::core::mem::transmute_copy(&setchaosscheduledescription),
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
        unsafe extern "system" fn EndSetChaosSchedule<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSetChaosSchedule(::core::mem::transmute(&context))
                .into()
        }
        unsafe extern "system" fn BeginGetChaosEvents<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            chaoseventsdescription: *const super::super::FABRIC_CHAOS_EVENTS_SEGMENT_DESCRIPTION,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetChaosEvents(
                ::core::mem::transmute_copy(&chaoseventsdescription),
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
        unsafe extern "system" fn EndGetChaosEvents<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricTestManagementClient4_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            result: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetChaosEvents(::core::mem::transmute(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFabricTestManagementClient3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetChaos: BeginGetChaos::<Identity, Impl, OFFSET>,
            EndGetChaos: EndGetChaos::<Identity, Impl, OFFSET>,
            BeginGetChaosSchedule: BeginGetChaosSchedule::<Identity, Impl, OFFSET>,
            EndGetChaosSchedule: EndGetChaosSchedule::<Identity, Impl, OFFSET>,
            BeginSetChaosSchedule: BeginSetChaosSchedule::<Identity, Impl, OFFSET>,
            EndSetChaosSchedule: EndSetChaosSchedule::<Identity, Impl, OFFSET>,
            BeginGetChaosEvents: BeginGetChaosEvents::<Identity, Impl, OFFSET>,
            EndGetChaosEvents: EndGetChaosEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricTestManagementClient4 as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient2 as ::windows::core::Interface>::IID
            || iid == &<IFabricTestManagementClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IFabricUpgradeOrchestrationServiceStateResult_Impl: Sized {
    fn get_State(&self) -> *mut super::super::FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE;
}
impl ::windows::core::RuntimeName for IFabricUpgradeOrchestrationServiceStateResult {}
impl IFabricUpgradeOrchestrationServiceStateResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeOrchestrationServiceStateResult_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeOrchestrationServiceStateResult_Vtbl {
        unsafe extern "system" fn get_State<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeOrchestrationServiceStateResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UPGRADE_ORCHESTRATION_SERVICE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_State()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_State: get_State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricUpgradeOrchestrationServiceStateResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricUpgradeProgressResult_Impl: Sized {
    fn get_TargetCodeVersion(&self) -> ::windows::core::PWSTR;
    fn get_TargetConfigVersion(&self) -> ::windows::core::PWSTR;
    fn get_UpgradeState(&self) -> super::super::FABRIC_UPGRADE_STATE;
    fn GetUpgradeDomains(
        &self,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows::core::Result<()>;
    fn GetChangedUpgradeDomains(
        &self,
        previousprogress: &::core::option::Option<IFabricUpgradeProgressResult>,
        itemcount: *mut u32,
        buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFabricUpgradeProgressResult {}
impl IFabricUpgradeProgressResult_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult_Vtbl {
        unsafe extern "system" fn get_TargetCodeVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetCodeVersion()
        }
        unsafe extern "system" fn get_TargetConfigVersion<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_TargetConfigVersion()
        }
        unsafe extern "system" fn get_UpgradeState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_UPGRADE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeState()
        }
        unsafe extern "system" fn GetUpgradeDomains<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpgradeDomains(
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        unsafe extern "system" fn GetChangedUpgradeDomains<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            previousprogress: *mut ::core::ffi::c_void,
            itemcount: *mut u32,
            buffereditems: *mut *mut super::super::FABRIC_UPGRADE_DOMAIN_STATUS_DESCRIPTION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChangedUpgradeDomains(
                ::core::mem::transmute(&previousprogress),
                ::core::mem::transmute_copy(&itemcount),
                ::core::mem::transmute_copy(&buffereditems),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_TargetCodeVersion: get_TargetCodeVersion::<Identity, Impl, OFFSET>,
            get_TargetConfigVersion: get_TargetConfigVersion::<Identity, Impl, OFFSET>,
            get_UpgradeState: get_UpgradeState::<Identity, Impl, OFFSET>,
            GetUpgradeDomains: GetUpgradeDomains::<Identity, Impl, OFFSET>,
            GetChangedUpgradeDomains: GetChangedUpgradeDomains::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricUpgradeProgressResult2_Impl: Sized + IFabricUpgradeProgressResult_Impl {
    fn get_RollingUpgradeMode(&self) -> super::super::FABRIC_ROLLING_UPGRADE_MODE;
    fn get_NextUpgradeDomain(&self) -> ::windows::core::PWSTR;
}
impl ::windows::core::RuntimeName for IFabricUpgradeProgressResult2 {}
impl IFabricUpgradeProgressResult2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult2_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult2_Vtbl {
        unsafe extern "system" fn get_RollingUpgradeMode<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_ROLLING_UPGRADE_MODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_RollingUpgradeMode()
        }
        unsafe extern "system" fn get_NextUpgradeDomain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_NextUpgradeDomain()
        }
        Self {
            base__: IFabricUpgradeProgressResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_RollingUpgradeMode: get_RollingUpgradeMode::<Identity, Impl, OFFSET>,
            get_NextUpgradeDomain: get_NextUpgradeDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult2 as ::windows::core::Interface>::IID
            || iid == &<IFabricUpgradeProgressResult as ::windows::core::Interface>::IID
    }
}
pub trait IFabricUpgradeProgressResult3_Impl: Sized + IFabricUpgradeProgressResult2_Impl {
    fn get_UpgradeProgress(&self) -> *mut super::super::FABRIC_UPGRADE_PROGRESS;
}
impl ::windows::core::RuntimeName for IFabricUpgradeProgressResult3 {}
impl IFabricUpgradeProgressResult3_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFabricUpgradeProgressResult3_Impl,
        const OFFSET: isize,
    >() -> IFabricUpgradeProgressResult3_Vtbl {
        unsafe extern "system" fn get_UpgradeProgress<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFabricUpgradeProgressResult3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_UPGRADE_PROGRESS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_UpgradeProgress()
        }
        Self {
            base__: IFabricUpgradeProgressResult2_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_UpgradeProgress: get_UpgradeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFabricUpgradeProgressResult3 as ::windows::core::Interface>::IID
            || iid == &<IFabricUpgradeProgressResult as ::windows::core::Interface>::IID
            || iid == &<IFabricUpgradeProgressResult2 as ::windows::core::Interface>::IID
    }
}
