#[inline]
pub unsafe fn FabricBeginCreateRuntime<P0, P1>(
    riid: *const ::windows_core::GUID,
    exithandler: P0,
    timeoutmilliseconds: u32,
    callback: P1,
) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::windows_core::IntoParam<IFabricProcessExitHandler>,
    P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricBeginCreateRuntime(
            riid: *const ::windows_core::GUID,
            exithandler: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricBeginCreateRuntime(
        riid,
        exithandler.into_param().abi(),
        timeoutmilliseconds,
        callback.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricBeginGetActivationContext<P0>(
    riid: *const ::windows_core::GUID,
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricBeginGetActivationContext(
            riid: *const ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricBeginGetActivationContext(
        riid,
        timeoutmilliseconds,
        callback.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricBeginGetCodePackageActivator<P0>(
    riid: *const ::windows_core::GUID,
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricBeginGetCodePackageActivator(
            riid: *const ::windows_core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricBeginGetCodePackageActivator(
        riid,
        timeoutmilliseconds,
        callback.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricBeginGetNodeContext<P0>(
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricBeginGetNodeContext(
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricBeginGetNodeContext(
        timeoutmilliseconds,
        callback.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica<P0, P1>(
    riid: *const ::windows_core::GUID,
    storename: P0,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricStoreEventHandler>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateKeyValueStoreReplica(
            riid: *const ::windows_core::GUID,
            storename: ::windows_core::PCWSTR,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateKeyValueStoreReplica(
        riid,
        storename.into_param().abi(),
        ::core::mem::transmute(partitionid),
        replicaid,
        replicatorsettings,
        localstorekind,
        localstoresettings,
        storeeventhandler.into_param().abi(),
        &mut result__,
    )
    .map(|| result__)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica2<P0, P1, P2>(
    riid: *const ::windows_core::GUID,
    storename: P0,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricStoreEventHandler>,
    P2: ::windows_core::IntoParam<IFabricSecondaryEventHandler>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateKeyValueStoreReplica2(
            riid: *const ::windows_core::GUID,
            storename: ::windows_core::PCWSTR,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateKeyValueStoreReplica2(
        riid,
        storename.into_param().abi(),
        ::core::mem::transmute(partitionid),
        replicaid,
        replicatorsettings,
        localstorekind,
        localstoresettings,
        storeeventhandler.into_param().abi(),
        secondaryeventhandler.into_param().abi(),
        notificationmode,
        &mut result__,
    )
    .map(|| result__)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica3<P0, P1, P2>(
    riid: *const ::windows_core::GUID,
    storename: P0,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricStoreEventHandler>,
    P2: ::windows_core::IntoParam<IFabricSecondaryEventHandler>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateKeyValueStoreReplica3(
            riid: *const ::windows_core::GUID,
            storename: ::windows_core::PCWSTR,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateKeyValueStoreReplica3(
        riid,
        storename.into_param().abi(),
        ::core::mem::transmute(partitionid),
        replicaid,
        replicatorsettings,
        localstorekind,
        localstoresettings,
        storeeventhandler.into_param().abi(),
        secondaryeventhandler.into_param().abi(),
        notificationmode,
        &mut result__,
    )
    .map(|| result__)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica4<P0, P1, P2>(
    riid: *const ::windows_core::GUID,
    storename: P0,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    servicename: *const u16,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricStoreEventHandler>,
    P2: ::windows_core::IntoParam<IFabricSecondaryEventHandler>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateKeyValueStoreReplica4(
            riid: *const ::windows_core::GUID,
            storename: ::windows_core::PCWSTR,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            servicename: *const u16,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateKeyValueStoreReplica4(
        riid,
        storename.into_param().abi(),
        ::core::mem::transmute(partitionid),
        replicaid,
        servicename,
        replicatorsettings,
        localstorekind,
        localstoresettings,
        storeeventhandler.into_param().abi(),
        secondaryeventhandler.into_param().abi(),
        notificationmode,
        &mut result__,
    )
    .map(|| result__)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica5<P0, P1, P2>(
    riid: *const ::windows_core::GUID,
    storename: P0,
    partitionid: ::windows_core::GUID,
    replicaid: i64,
    servicename: *const u16,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    kvssettings: *const super::super::FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IFabricStoreEventHandler>,
    P2: ::windows_core::IntoParam<IFabricSecondaryEventHandler>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateKeyValueStoreReplica5(
            riid: *const ::windows_core::GUID,
            storename: ::windows_core::PCWSTR,
            partitionid: ::windows_core::GUID,
            replicaid: i64,
            servicename: *const u16,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            kvssettings: *const super::super::FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateKeyValueStoreReplica5(
        riid,
        storename.into_param().abi(),
        ::core::mem::transmute(partitionid),
        replicaid,
        servicename,
        replicatorsettings,
        kvssettings,
        localstorekind,
        localstoresettings,
        storeeventhandler.into_param().abi(),
        secondaryeventhandler.into_param().abi(),
        &mut result__,
    )
    .map(|| result__)
}
#[inline]
pub unsafe fn FabricCreateRuntime(
    riid: *const ::windows_core::GUID,
) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricCreateRuntime(
            riid: *const ::windows_core::GUID,
            fabricruntime: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricCreateRuntime(riid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricEndCreateRuntime<P0>(
    context: P0,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricEndCreateRuntime(
            context: *mut ::core::ffi::c_void,
            fabricruntime: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEndCreateRuntime(context.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricEndGetActivationContext<P0>(
    context: P0,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricEndGetActivationContext(
            context: *mut ::core::ffi::c_void,
            activationcontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEndGetActivationContext(context.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricEndGetCodePackageActivator<P0>(
    context: P0,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricEndGetCodePackageActivator(
            context: *mut ::core::ffi::c_void,
            activator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEndGetCodePackageActivator(context.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricEndGetNodeContext<P0>(
    context: P0,
) -> ::windows_core::Result<*mut ::core::ffi::c_void>
where
    P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricEndGetNodeContext(
            context: *mut ::core::ffi::c_void,
            nodecontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricEndGetNodeContext(context.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricGetActivationContext(
    riid: *const ::windows_core::GUID,
) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricGetActivationContext(
            riid: *const ::windows_core::GUID,
            activationcontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricGetActivationContext(riid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricGetCodePackageActivator(
    riid: *const ::windows_core::GUID,
) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricGetCodePackageActivator(
            riid: *const ::windows_core::GUID,
            activator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricGetCodePackageActivator(riid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricGetNodeContext() -> ::windows_core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricGetNodeContext(
            nodecontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricGetNodeContext(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FabricLoadEseLocalStoreSettings<P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows_core::Result<IFabricEseLocalStoreSettingsResult>
where
    P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricLoadEseLocalStoreSettings(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows_core::PCWSTR,
            sectionname: ::windows_core::PCWSTR,
            settings: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricLoadEseLocalStoreSettings(
        codepackageactivationcontext.into_param().abi(),
        configurationpackagename.into_param().abi(),
        sectionname.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricLoadReplicatorSettings<P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows_core::Result<IFabricReplicatorSettingsResult>
where
    P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricLoadReplicatorSettings(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows_core::PCWSTR,
            sectionname: ::windows_core::PCWSTR,
            replicatorsettings: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricLoadReplicatorSettings(
        codepackageactivationcontext.into_param().abi(),
        configurationpackagename.into_param().abi(),
        sectionname.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FabricLoadSecurityCredentials<P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows_core::Result<IFabricSecurityCredentialsResult>
where
    P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "FabricRuntime")]
    extern "system" {
        pub fn FabricLoadSecurityCredentials(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows_core::PCWSTR,
            sectionname: ::windows_core::PCWSTR,
            securitycredentials: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    FabricLoadSecurityCredentials(
        codepackageactivationcontext.into_param().abi(),
        configurationpackagename.into_param().abi(),
        sectionname.into_param().abi(),
        &mut result__,
    )
    .and_then(|| ::windows_core::Type::from_abi(result__))
}
::windows_core::imp::com_interface!(
    IFabricAtomicGroupStateProvider,
    IFabricAtomicGroupStateProvider_Vtbl,
    0x2b670953_6148_4f7d_a920_b390de43d913
);
::windows_core::imp::interface_hierarchy!(
    IFabricAtomicGroupStateProvider,
    ::windows_core::IUnknown
);
impl IFabricAtomicGroupStateProvider {
    pub unsafe fn BeginAtomicGroupCommit<P0>(
        &self,
        atomicgroupid: i64,
        commitsequencenumber: i64,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginAtomicGroupCommit)(
            ::windows_core::Interface::as_raw(self),
            atomicgroupid,
            commitsequencenumber,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndAtomicGroupCommit<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndAtomicGroupCommit)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginAtomicGroupRollback<P0>(
        &self,
        atomicgroupid: i64,
        rollbackequencenumber: i64,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginAtomicGroupRollback)(
            ::windows_core::Interface::as_raw(self),
            atomicgroupid,
            rollbackequencenumber,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndAtomicGroupRollback<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndAtomicGroupRollback)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUndoProgress<P0>(
        &self,
        fromcommitsequencenumber: i64,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginUndoProgress)(
            ::windows_core::Interface::as_raw(self),
            fromcommitsequencenumber,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndUndoProgress<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndUndoProgress)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricAtomicGroupStateProvider {}
unsafe impl ::core::marker::Sync for IFabricAtomicGroupStateProvider {}
#[repr(C)]
pub struct IFabricAtomicGroupStateProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginAtomicGroupCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        i64,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndAtomicGroupCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginAtomicGroupRollback: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        i64,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndAtomicGroupRollback: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginUndoProgress: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndUndoProgress: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricAtomicGroupStateReplicator,
    IFabricAtomicGroupStateReplicator_Vtbl,
    0x80d2155c_4fc2_4fde_9696_c2f39b471c3d
);
::windows_core::imp::interface_hierarchy!(
    IFabricAtomicGroupStateReplicator,
    ::windows_core::IUnknown
);
impl IFabricAtomicGroupStateReplicator {
    pub unsafe fn CreateAtomicGroup(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateAtomicGroup)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupOperation<P0, P1>(
        &self,
        atomicgroupid: i64,
        operationdata: P0,
        callback: P1,
        operationsequencenumber: *mut i64,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricOperationData>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginReplicateAtomicGroupOperation)(
            ::windows_core::Interface::as_raw(self),
            atomicgroupid,
            operationdata.into_param().abi(),
            callback.into_param().abi(),
            operationsequencenumber,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndReplicateAtomicGroupOperation<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndReplicateAtomicGroupOperation)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupCommit<P0>(
        &self,
        atomicgroupid: i64,
        callback: P0,
        commitsequencenumber: *mut i64,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginReplicateAtomicGroupCommit)(
            ::windows_core::Interface::as_raw(self),
            atomicgroupid,
            callback.into_param().abi(),
            commitsequencenumber,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndReplicateAtomicGroupCommit<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndReplicateAtomicGroupCommit)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupRollback<P0>(
        &self,
        atomicgroupid: i64,
        callback: P0,
        rollbacksequencenumber: *mut i64,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginReplicateAtomicGroupRollback)(
            ::windows_core::Interface::as_raw(self),
            atomicgroupid,
            callback.into_param().abi(),
            rollbacksequencenumber,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndReplicateAtomicGroupRollback<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndReplicateAtomicGroupRollback)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricAtomicGroupStateReplicator {}
unsafe impl ::core::marker::Sync for IFabricAtomicGroupStateReplicator {}
#[repr(C)]
pub struct IFabricAtomicGroupStateReplicator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateAtomicGroup:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i64) -> ::windows_core::HRESULT,
    pub BeginReplicateAtomicGroupOperation: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
        *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub EndReplicateAtomicGroupOperation: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub BeginReplicateAtomicGroupCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        *mut ::core::ffi::c_void,
        *mut i64,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndReplicateAtomicGroupCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub BeginReplicateAtomicGroupRollback: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        *mut ::core::ffi::c_void,
        *mut i64,
        *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub EndReplicateAtomicGroupRollback: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricCodePackage,
    IFabricCodePackage_Vtbl,
    0x20792b45_4d13_41a4_af13_346e529f00c5
);
::windows_core::imp::interface_hierarchy!(IFabricCodePackage, ::windows_core::IUnknown);
impl IFabricCodePackage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_Description)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_Path(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_Path)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackage {}
unsafe impl ::core::marker::Sync for IFabricCodePackage {}
#[repr(C)]
pub struct IFabricCodePackage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Description:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Description: usize,
    pub get_Path: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
}
::windows_core::imp::com_interface!(
    IFabricCodePackage2,
    IFabricCodePackage2_Vtbl,
    0xcdf0a4e6_ad80_4cd6_b67e_e4c002428600
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackage2,
    ::windows_core::IUnknown,
    IFabricCodePackage
);
impl IFabricCodePackage2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_Description)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_Path(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).base__.get_Path)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_SetupEntryPointRunAsPolicy(
        &self,
    ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_SetupEntryPointRunAsPolicy)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_EntryPointRunAsPolicy(
        &self,
    ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_EntryPointRunAsPolicy)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackage2 {}
unsafe impl ::core::marker::Sync for IFabricCodePackage2 {}
#[repr(C)]
pub struct IFabricCodePackage2_Vtbl {
    pub base__: IFabricCodePackage_Vtbl,
    pub get_SetupEntryPointRunAsPolicy:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION,
    pub get_EntryPointRunAsPolicy:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext_Vtbl,
    0x68a971e2_f15f_4d95_a79c_8a257909659e
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext,
    ::windows_core::IUnknown
);
impl IFabricCodePackageActivationContext {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_ContextId)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_CodePackageName)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_CodePackageVersion)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_WorkDirectory)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_LogDirectory)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_TempDirectory)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self).get_ServiceTypes)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self).get_ServiceGroupTypes)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_ApplicationPrincipals)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self).get_ServiceEndpointResources)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePackageNames)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDataPackageNames)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_ContextId:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_CodePackageName:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_CodePackageVersion:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_WorkDirectory:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_LogDirectory:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_TempDirectory:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_ServiceTypes:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ServiceGroupTypes:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ServiceGroupTypes: usize,
    pub get_ApplicationPrincipals:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION,
    pub get_ServiceEndpointResources:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST,
    pub GetServiceEndpointResource: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
    ) -> ::windows_core::HRESULT,
    pub GetCodePackageNames: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetConfigurationPackageNames: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetDataPackageNames: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetConfigurationPackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetDataPackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RegisterCodePackageChangeHandler: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub UnregisterCodePackageChangeHandler:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, i64) -> ::windows_core::HRESULT,
    pub RegisterConfigurationPackageChangeHandler:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut i64,
        ) -> ::windows_core::HRESULT,
    pub UnregisterConfigurationPackageChangeHandler:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, i64) -> ::windows_core::HRESULT,
    pub RegisterDataPackageChangeHandler: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub UnregisterDataPackageChangeHandler:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, i64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext2_Vtbl,
    0x6c83d5c1_1954_4b80_9175_0d0e7c8715c9
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext2,
    ::windows_core::IUnknown,
    IFabricCodePackageActivationContext
);
impl IFabricCodePackageActivationContext2 {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).base__.get_ContextId)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_CodePackageName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_CodePackageVersion)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_WorkDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_LogDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_TempDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ServiceTypes)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ServiceGroupTypes)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ApplicationPrincipals)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ServiceEndpointResources)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetCodePackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetDataPackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self).get_ApplicationName)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_ApplicationTypeName)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceManifestName)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceManifestVersion)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext2 {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext2 {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext2_Vtbl {
    pub base__: IFabricCodePackageActivationContext_Vtbl,
    pub get_ApplicationName: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> *mut u16,
    pub get_ApplicationTypeName:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub GetServiceManifestName: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetServiceManifestVersion: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext3,
    IFabricCodePackageActivationContext3_Vtbl,
    0x6efee900_f491_4b03_bc5b_3a70de103593
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext3,
    ::windows_core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2
);
impl IFabricCodePackageActivationContext3 {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ContextId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_CodePackageName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_CodePackageVersion)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_WorkDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_LogDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_TempDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ServiceTypes)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetCodePackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetDataPackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ApplicationName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ApplicationTypeName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetServiceManifestName)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetServiceManifestVersion)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportApplicationHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDeployedApplicationHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDeployedServicePackageHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext3 {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext3 {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext3_Vtbl {
    pub base__: IFabricCodePackageActivationContext2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportApplicationHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportApplicationHealth: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportDeployedApplicationHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportDeployedApplicationHealth: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportDeployedServicePackageHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportDeployedServicePackageHealth: usize,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext4,
    IFabricCodePackageActivationContext4_Vtbl,
    0x99efebb6_a7b4_4d45_b45e_f191a66eef03
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext4,
    ::windows_core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3
);
impl IFabricCodePackageActivationContext4 {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ContextId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetCodePackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetDataPackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ApplicationName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .get_ApplicationTypeName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetServiceManifestName)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportApplicationHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows_core::Interface::as_raw(self), healthinfo
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDeployedApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDeployedServicePackageHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext4 {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext4 {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext4_Vtbl {
    pub base__: IFabricCodePackageActivationContext3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportApplicationHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportApplicationHealth2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportDeployedApplicationHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportDeployedApplicationHealth2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportDeployedServicePackageHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportDeployedServicePackageHealth2: usize,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext5,
    IFabricCodePackageActivationContext5_Vtbl,
    0xfe45387e_8711_4949_ac36_31dc95035513
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext5,
    ::windows_core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3,
    IFabricCodePackageActivationContext4
);
impl IFabricCodePackageActivationContext5 {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ContextId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCodePackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetDataPackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationTypeName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceManifestName)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportApplicationHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows_core::Interface::as_raw(self), healthinfo
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportDeployedApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportDeployedServicePackageHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    pub unsafe fn get_ServiceListenAddress(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_ServiceListenAddress)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_ServicePublishAddress(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_ServicePublishAddress)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext5 {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext5 {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext5_Vtbl {
    pub base__: IFabricCodePackageActivationContext4_Vtbl,
    pub get_ServiceListenAddress:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    pub get_ServicePublishAddress:
        unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivationContext6,
    IFabricCodePackageActivationContext6_Vtbl,
    0xfa5fda9b_472c_45a0_9b60_a374691227a4
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageActivationContext6,
    ::windows_core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3,
    IFabricCodePackageActivationContext4,
    IFabricCodePackageActivationContext5
);
impl IFabricCodePackageActivationContext6 {
    pub unsafe fn get_ContextId(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ContextId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows_core::Interface::as_raw(self),
            serviceendpointresourcename.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCodePackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringListResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetDataPackageNames)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePackage<P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows_core::Result<IFabricCodePackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigurationPackage<P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows_core::Result<IFabricConfigurationPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows_core::Interface::as_raw(self),
            configpackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDataPackage<P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows_core::Result<IFabricDataPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows_core::Interface::as_raw(self),
            datapackagename.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterCodePackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricConfigurationPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<IFabricDataPackageChangeHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationTypeName)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceManifestName)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows_core::Result<super::IFabricStringResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows_core::Interface::as_raw(self), &mut result__
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportApplicationHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows_core::Interface::as_raw(self), healthinfo
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportDeployedApplicationHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportDeployedServicePackageHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    pub unsafe fn get_ServiceListenAddress(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ServiceListenAddress)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_ServicePublishAddress(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_ServicePublishAddress)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDirectory<P0>(
        &self,
        logicaldirectoryname: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDirectory)(
            ::windows_core::Interface::as_raw(self),
            logicaldirectoryname.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivationContext6 {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivationContext6 {}
#[repr(C)]
pub struct IFabricCodePackageActivationContext6_Vtbl {
    pub base__: IFabricCodePackageActivationContext5_Vtbl,
    pub GetDirectory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageActivator,
    IFabricCodePackageActivator_Vtbl,
    0x70be1b10_b259_46fc_b813_0b75720e7183
);
::windows_core::imp::interface_hierarchy!(IFabricCodePackageActivator, ::windows_core::IUnknown);
impl IFabricCodePackageActivator {
    pub unsafe fn BeginActivateCodePackage<P0>(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        environment: *const super::super::FABRIC_STRING_MAP,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginActivateCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagenames,
            environment,
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndActivateCodePackage<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndActivateCodePackage)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginDeactivateCodePackage<P0>(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginDeactivateCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagenames,
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndDeactivateCodePackage<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndDeactivateCodePackage)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AbortCodePackage(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortCodePackage)(
            ::windows_core::Interface::as_raw(self),
            codepackagenames,
        )
        .ok()
    }
    pub unsafe fn RegisterCodePackageEventHandler<P0>(
        &self,
        eventhandler: P0,
    ) -> ::windows_core::Result<u64>
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageEventHandler>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterCodePackageEventHandler)(
            ::windows_core::Interface::as_raw(self),
            eventhandler.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UnregisterCodePackageEventHandler(
        &self,
        callbackhandle: u64,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterCodePackageEventHandler)(
            ::windows_core::Interface::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageActivator {}
unsafe impl ::core::marker::Sync for IFabricCodePackageActivator {}
#[repr(C)]
pub struct IFabricCodePackageActivator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginActivateCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_STRING_LIST,
        *const super::super::FABRIC_STRING_MAP,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndActivateCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginDeactivateCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_STRING_LIST,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndDeactivateCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AbortCodePackage: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_STRING_LIST,
    ) -> ::windows_core::HRESULT,
    pub RegisterCodePackageEventHandler: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut u64,
    ) -> ::windows_core::HRESULT,
    pub UnregisterCodePackageEventHandler:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, u64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricCodePackageChangeHandler,
    IFabricCodePackageChangeHandler_Vtbl,
    0xb90d36cd_acb5_427a_b318_3b045981d0cc
);
::windows_core::imp::interface_hierarchy!(
    IFabricCodePackageChangeHandler,
    ::windows_core::IUnknown
);
impl IFabricCodePackageChangeHandler {
    pub unsafe fn OnPackageAdded<P0, P1>(&self, source: P0, codepackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricCodePackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageAdded)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            codepackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<P0, P1>(&self, source: P0, codepackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricCodePackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageRemoved)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            codepackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageModified<P0, P1, P2>(
        &self,
        source: P0,
        previouscodepackage: P1,
        codepackage: P2,
    ) where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricCodePackage>,
        P2: ::windows_core::IntoParam<IFabricCodePackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageModified)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            previouscodepackage.into_param().abi(),
            codepackage.into_param().abi(),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageChangeHandler {}
unsafe impl ::core::marker::Sync for IFabricCodePackageChangeHandler {}
#[repr(C)]
pub struct IFabricCodePackageChangeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
}
::windows_core::imp::com_interface!(
    IFabricCodePackageEventHandler,
    IFabricCodePackageEventHandler_Vtbl,
    0x899e0ca8_16df_458e_8915_d0307b4ab101
);
::windows_core::imp::interface_hierarchy!(IFabricCodePackageEventHandler, ::windows_core::IUnknown);
impl IFabricCodePackageEventHandler {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCodePackageEvent<P0>(
        &self,
        source: P0,
        eventdesc: *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
    ) where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivator>,
    {
        (::windows_core::Interface::vtable(self).OnCodePackageEvent)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            eventdesc,
        )
    }
}
unsafe impl ::core::marker::Send for IFabricCodePackageEventHandler {}
unsafe impl ::core::marker::Sync for IFabricCodePackageEventHandler {}
#[repr(C)]
pub struct IFabricCodePackageEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCodePackageEvent: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
    ),
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCodePackageEvent: usize,
}
::windows_core::imp::com_interface!(
    IFabricConfigurationPackage,
    IFabricConfigurationPackage_Vtbl,
    0xac4c3bfa_2563_46b7_a71d_2dca7b0a8f4d
);
::windows_core::imp::interface_hierarchy!(IFabricConfigurationPackage, ::windows_core::IUnknown);
impl IFabricConfigurationPackage {
    pub unsafe fn get_Description(
        &self,
    ) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_Description)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_Path(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_Path)(::windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS {
        (::windows_core::Interface::vtable(self).get_Settings)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSection<P0>(
        &self,
        sectionname: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_CONFIGURATION_SECTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSection)(
            ::windows_core::Interface::as_raw(self),
            sectionname.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetValue<P0, P1>(
        &self,
        sectionname: P0,
        parametername: P1,
        isencrypted: *mut u8,
    ) -> ::windows_core::Result<::windows_core::PCWSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(
            ::windows_core::Interface::as_raw(self),
            sectionname.into_param().abi(),
            parametername.into_param().abi(),
            isencrypted,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn DecryptValue<P0>(
        &self,
        encryptedvalue: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DecryptValue)(
            ::windows_core::Interface::as_raw(self),
            encryptedvalue.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricConfigurationPackage {}
unsafe impl ::core::marker::Sync for IFabricConfigurationPackage {}
#[repr(C)]
pub struct IFabricConfigurationPackage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_Description:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION,
    pub get_Path: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Settings: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    )
        -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Settings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSection: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut super::super::FABRIC_CONFIGURATION_SECTION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSection: usize,
    pub GetValue: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows_core::PCWSTR,
        *mut u8,
        *mut ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
    pub DecryptValue: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricConfigurationPackage2,
    IFabricConfigurationPackage2_Vtbl,
    0xd3161f31_708a_4f83_91ff_f2af15f74a2f
);
::windows_core::imp::interface_hierarchy!(
    IFabricConfigurationPackage2,
    ::windows_core::IUnknown,
    IFabricConfigurationPackage
);
impl IFabricConfigurationPackage2 {
    pub unsafe fn get_Description(
        &self,
    ) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_Description)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_Path(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).base__.get_Path)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS {
        (::windows_core::Interface::vtable(self).base__.get_Settings)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSection<P0>(
        &self,
        sectionname: P0,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_CONFIGURATION_SECTION>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSection)(
            ::windows_core::Interface::as_raw(self),
            sectionname.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetValue<P0, P1>(
        &self,
        sectionname: P0,
        parametername: P1,
        isencrypted: *mut u8,
    ) -> ::windows_core::Result<::windows_core::PCWSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(
            ::windows_core::Interface::as_raw(self),
            sectionname.into_param().abi(),
            parametername.into_param().abi(),
            isencrypted,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn DecryptValue<P0>(
        &self,
        encryptedvalue: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DecryptValue)(
            ::windows_core::Interface::as_raw(self),
            encryptedvalue.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValues<P0, P1>(
        &self,
        sectionname: P0,
        parameterprefix: P1,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValues)(
            ::windows_core::Interface::as_raw(self),
            sectionname.into_param().abi(),
            parameterprefix.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricConfigurationPackage2 {}
unsafe impl ::core::marker::Sync for IFabricConfigurationPackage2 {}
#[repr(C)]
pub struct IFabricConfigurationPackage2_Vtbl {
    pub base__: IFabricConfigurationPackage_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValues: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows_core::PCWSTR,
        *mut *mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValues: usize,
}
::windows_core::imp::com_interface!(
    IFabricConfigurationPackageChangeHandler,
    IFabricConfigurationPackageChangeHandler_Vtbl,
    0xc3954d48_b5ee_4ff4_9bc0_c30f6d0d3a85
);
::windows_core::imp::interface_hierarchy!(
    IFabricConfigurationPackageChangeHandler,
    ::windows_core::IUnknown
);
impl IFabricConfigurationPackageChangeHandler {
    pub unsafe fn OnPackageAdded<P0, P1>(&self, source: P0, configpackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricConfigurationPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageAdded)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            configpackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<P0, P1>(&self, source: P0, configpackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricConfigurationPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageRemoved)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            configpackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageModified<P0, P1, P2>(
        &self,
        source: P0,
        previousconfigpackage: P1,
        configpackage: P2,
    ) where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricConfigurationPackage>,
        P2: ::windows_core::IntoParam<IFabricConfigurationPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageModified)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            previousconfigpackage.into_param().abi(),
            configpackage.into_param().abi(),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricConfigurationPackageChangeHandler {}
unsafe impl ::core::marker::Sync for IFabricConfigurationPackageChangeHandler {}
#[repr(C)]
pub struct IFabricConfigurationPackageChangeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
}
::windows_core::imp::com_interface!(
    IFabricDataPackage,
    IFabricDataPackage_Vtbl,
    0xaa67de09_3657_435f_a2f6_b3a17a0a4371
);
::windows_core::imp::interface_hierarchy!(IFabricDataPackage, ::windows_core::IUnknown);
impl IFabricDataPackage {
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION {
        (::windows_core::Interface::vtable(self).get_Description)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn get_Path(&self) -> ::windows_core::PCWSTR {
        (::windows_core::Interface::vtable(self).get_Path)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricDataPackage {}
unsafe impl ::core::marker::Sync for IFabricDataPackage {}
#[repr(C)]
pub struct IFabricDataPackage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_Description:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION,
    pub get_Path: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::PCWSTR,
}
::windows_core::imp::com_interface!(
    IFabricDataPackageChangeHandler,
    IFabricDataPackageChangeHandler_Vtbl,
    0x8d0a726f_bd17_4b32_807b_be2a8024b2e0
);
::windows_core::imp::interface_hierarchy!(
    IFabricDataPackageChangeHandler,
    ::windows_core::IUnknown
);
impl IFabricDataPackageChangeHandler {
    pub unsafe fn OnPackageAdded<P0, P1>(&self, source: P0, datapackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricDataPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageAdded)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            datapackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<P0, P1>(&self, source: P0, datapackage: P1)
    where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricDataPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageRemoved)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            datapackage.into_param().abi(),
        )
    }
    pub unsafe fn OnPackageModified<P0, P1, P2>(
        &self,
        source: P0,
        previousdatapackage: P1,
        datapackage: P2,
    ) where
        P0: ::windows_core::IntoParam<IFabricCodePackageActivationContext>,
        P1: ::windows_core::IntoParam<IFabricDataPackage>,
        P2: ::windows_core::IntoParam<IFabricDataPackage>,
    {
        (::windows_core::Interface::vtable(self).OnPackageModified)(
            ::windows_core::Interface::as_raw(self),
            source.into_param().abi(),
            previousdatapackage.into_param().abi(),
            datapackage.into_param().abi(),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricDataPackageChangeHandler {}
unsafe impl ::core::marker::Sync for IFabricDataPackageChangeHandler {}
#[repr(C)]
pub struct IFabricDataPackageChangeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ),
}
::windows_core::imp::com_interface!(
    IFabricEseLocalStoreSettingsResult,
    IFabricEseLocalStoreSettingsResult_Vtbl,
    0xaace77ae_d8e1_4144_b1ee_5ac74fd54f65
);
::windows_core::imp::interface_hierarchy!(
    IFabricEseLocalStoreSettingsResult,
    ::windows_core::IUnknown
);
impl IFabricEseLocalStoreSettingsResult {
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS {
        (::windows_core::Interface::vtable(self).get_Settings)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricEseLocalStoreSettingsResult {}
unsafe impl ::core::marker::Sync for IFabricEseLocalStoreSettingsResult {}
#[repr(C)]
pub struct IFabricEseLocalStoreSettingsResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_Settings:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreEnumerator,
    IFabricKeyValueStoreEnumerator_Vtbl,
    0x6722b848_15bb_4528_bf54_c7bbe27b6f9a
);
::windows_core::imp::interface_hierarchy!(IFabricKeyValueStoreEnumerator, ::windows_core::IUnknown);
impl IFabricKeyValueStoreEnumerator {
    pub unsafe fn EnumerateByKey<P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreEnumerator {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreEnumerator {}
#[repr(C)]
pub struct IFabricKeyValueStoreEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub EnumerateByKey: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EnumerateMetadataByKey: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreEnumerator2,
    IFabricKeyValueStoreEnumerator2_Vtbl,
    0x63dfd264_4f2b_4be6_8234_1fa200165fe9
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreEnumerator2,
    ::windows_core::IUnknown,
    IFabricKeyValueStoreEnumerator
);
impl IFabricKeyValueStoreEnumerator2 {
    pub unsafe fn EnumerateByKey<P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateByKey2<P0, P1>(
        &self,
        keyprefix: P0,
        strictprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateByKey2)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateMetadataByKey2<P0, P1>(
        &self,
        keyprefix: P0,
        strictprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateMetadataByKey2)(
            ::windows_core::Interface::as_raw(self),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreEnumerator2 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreEnumerator2 {}
#[repr(C)]
pub struct IFabricKeyValueStoreEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreEnumerator_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateByKey2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows::Win32::Foundation::BOOLEAN,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateByKey2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateMetadataByKey2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows::Win32::Foundation::BOOLEAN,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateMetadataByKey2: usize,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemEnumerator,
    IFabricKeyValueStoreItemEnumerator_Vtbl,
    0xc202788f_54d3_44a6_8f3c_b4bbfcdb95d2
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreItemEnumerator,
    ::windows_core::IUnknown
);
impl IFabricKeyValueStoreItemEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemResult> {
        (::windows_core::Interface::vtable(self).get_Current)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemEnumerator {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemEnumerator {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreItemResult,
    >,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemEnumerator2,
    IFabricKeyValueStoreItemEnumerator2_Vtbl,
    0xda143bbc_81e1_48cd_afd7_b642bc5b9bfd
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreItemEnumerator2,
    ::windows_core::IUnknown,
    IFabricKeyValueStoreItemEnumerator
);
impl IFabricKeyValueStoreItemEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MoveNext)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemResult> {
        (::windows_core::Interface::vtable(self).base__.get_Current)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryMoveNext)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemEnumerator2 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemEnumerator2 {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreItemEnumerator_Vtbl,
    pub TryMoveNext:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemMetadataEnumerator,
    IFabricKeyValueStoreItemMetadataEnumerator_Vtbl,
    0x0bc06aee_fffa_4450_9099_116a5f0e0b53
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataEnumerator,
    ::windows_core::IUnknown
);
impl IFabricKeyValueStoreItemMetadataEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn get_Current(
        &self,
    ) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult> {
        (::windows_core::Interface::vtable(self).get_Current)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemMetadataEnumerator {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemMetadataEnumerator {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreItemMetadataResult,
    >,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemMetadataEnumerator2,
    IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl,
    0x8803d53e_dd73_40fc_a662_1bfe999419ea
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataEnumerator2,
    ::windows_core::IUnknown,
    IFabricKeyValueStoreItemMetadataEnumerator
);
impl IFabricKeyValueStoreItemMetadataEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MoveNext)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn get_Current(
        &self,
    ) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult> {
        (::windows_core::Interface::vtable(self).base__.get_Current)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryMoveNext)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemMetadataEnumerator2 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemMetadataEnumerator2 {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreItemMetadataEnumerator_Vtbl,
    pub TryMoveNext:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemMetadataResult,
    IFabricKeyValueStoreItemMetadataResult_Vtbl,
    0x17c483a1_69e6_4bdc_a058_54fd4a1839fd
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataResult,
    ::windows_core::IUnknown
);
impl IFabricKeyValueStoreItemMetadataResult {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Metadata(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
        (::windows_core::Interface::vtable(self).get_Metadata)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemMetadataResult {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemMetadataResult {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemMetadataResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Metadata:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Metadata: usize,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreItemResult,
    IFabricKeyValueStoreItemResult_Vtbl,
    0xc1f1c89d_b0b8_44dc_bc97_6c074c1a805e
);
::windows_core::imp::interface_hierarchy!(IFabricKeyValueStoreItemResult, ::windows_core::IUnknown);
impl IFabricKeyValueStoreItemResult {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Item(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM {
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreItemResult {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreItemResult {}
#[repr(C)]
pub struct IFabricKeyValueStoreItemResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreNotification,
    IFabricKeyValueStoreNotification_Vtbl,
    0xcb660aa6_c51e_4f05_9526_93982b550e8f
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreNotification,
    ::windows_core::IUnknown,
    IFabricKeyValueStoreItemResult
);
impl IFabricKeyValueStoreNotification {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Item(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM {
        (::windows_core::Interface::vtable(self).base__.get_Item)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDelete(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows_core::Interface::vtable(self).IsDelete)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreNotification {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreNotification {}
#[repr(C)]
pub struct IFabricKeyValueStoreNotification_Vtbl {
    pub base__: IFabricKeyValueStoreItemResult_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDelete: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::BOOLEAN,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDelete: usize,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreNotificationEnumerator,
    IFabricKeyValueStoreNotificationEnumerator_Vtbl,
    0xef25bc08_be76_43c7_adad_20f01fba3399
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreNotificationEnumerator,
    ::windows_core::IUnknown
);
impl IFabricKeyValueStoreNotificationEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreNotification> {
        (::windows_core::Interface::vtable(self).get_Current)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn Reset(&self) {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreNotificationEnumerator {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreNotificationEnumerator {}
#[repr(C)]
pub struct IFabricKeyValueStoreNotificationEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreNotification,
    >,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreNotificationEnumerator2,
    IFabricKeyValueStoreNotificationEnumerator2_Vtbl,
    0x55eec7c6_ae81_407a_b84c_22771d314ac7
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreNotificationEnumerator2,
    ::windows_core::IUnknown,
    IFabricKeyValueStoreNotificationEnumerator
);
impl IFabricKeyValueStoreNotificationEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MoveNext)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreNotification> {
        (::windows_core::Interface::vtable(self).base__.get_Current)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn Reset(&self) {
        (::windows_core::Interface::vtable(self).base__.Reset)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryMoveNext)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreNotificationEnumerator2 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreNotificationEnumerator2 {}
#[repr(C)]
pub struct IFabricKeyValueStoreNotificationEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreNotificationEnumerator_Vtbl,
    pub TryMoveNext:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica_Vtbl,
    0x97da35c4_38ed_4a2a_8f37_fbeb56382235
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica
);
impl IFabricKeyValueStoreReplica {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndChangeRole)(
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
        (::windows_core::Interface::vtable(self).base__.BeginClose)(
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
        (::windows_core::Interface::vtable(self).base__.EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self).base__.Abort)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCurrentEpoch)(
            ::windows_core::Interface::as_raw(self),
            currentepoch,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self),
            replicatorsettings,
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransaction)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica_Vtbl {
    pub base__: IFabricStatefulServiceReplica_Vtbl,
    pub GetCurrentEpoch: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateReplicatorSettings: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateReplicatorSettings: usize,
    pub CreateTransaction: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i32,
        *const u8,
    ) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i64,
    ) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i32,
        *const u8,
        i64,
    ) -> ::windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetMetadata: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub Enumerate: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EnumerateByKey: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EnumerateMetadata: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EnumerateMetadataByKey: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica2_Vtbl,
    0xfef805b2_5aca_4caa_9c51_fb3bd577a792
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica2,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica
);
impl IFabricKeyValueStoreReplica2 {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .BeginClose)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self).base__.base__.Abort)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .GetCurrentEpoch)(::windows_core::Interface::as_raw(self), currentepoch)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .CreateTransaction)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Backup<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Backup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Restore<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Restore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTransaction2)(
            ::windows_core::Interface::as_raw(self),
            settings,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica2 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica2 {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica2_Vtbl {
    pub base__: IFabricKeyValueStoreReplica_Vtbl,
    pub Backup: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
    pub CreateTransaction2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica3,
    IFabricKeyValueStoreReplica3_Vtbl,
    0xc1297172_a8aa_4096_bdcc_1ece0c5d8c8f
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica3,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2
);
impl IFabricKeyValueStoreReplica3 {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Abort)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetCurrentEpoch)(::windows_core::Interface::as_raw(self), currentepoch)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .CreateTransaction)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Backup<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Backup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Restore<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Restore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .CreateTransaction2)(
            ::windows_core::Interface::as_raw(self),
            settings,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginBackup<P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStorePostBackupHandler>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginBackup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            backupoption,
            postbackuphandler.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndBackup<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndBackup)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica3 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica3 {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica3_Vtbl {
    pub base__: IFabricKeyValueStoreReplica2_Vtbl,
    pub BeginBackup: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        super::super::FABRIC_STORE_BACKUP_OPTION,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndBackup: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica4,
    IFabricKeyValueStoreReplica4_Vtbl,
    0xff16d2f1_41a9_4c64_804a_a20bf28c04f3
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica4,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3
);
impl IFabricKeyValueStoreReplica4 {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(::windows_core::Interface::as_raw(self), currentepoch)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .CreateTransaction)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Backup<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Backup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Restore<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .Restore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .CreateTransaction2)(
            ::windows_core::Interface::as_raw(self),
            settings,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginBackup<P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStorePostBackupHandler>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginBackup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            backupoption,
            postbackuphandler.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndBackup<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).base__.EndBackup)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRestore<P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRestore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRestore<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndRestore)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica4 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica4 {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica4_Vtbl {
    pub base__: IFabricKeyValueStoreReplica3_Vtbl,
    pub BeginRestore: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndRestore: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica5,
    IFabricKeyValueStoreReplica5_Vtbl,
    0x34f2da40_6227_448a_be72_c517b0d69432
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica5,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3,
    IFabricKeyValueStoreReplica4
);
impl IFabricKeyValueStoreReplica5 {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(::windows_core::Interface::as_raw(self), currentepoch)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Backup<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Backup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Restore<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .Restore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .CreateTransaction2)(
            ::windows_core::Interface::as_raw(self),
            settings,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginBackup<P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStorePostBackupHandler>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .BeginBackup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            backupoption,
            postbackuphandler.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndBackup<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EndBackup)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRestore<P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginRestore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRestore<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).base__.EndRestore)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TryAdd<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryAdd)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryRemove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryRemove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryUpdate<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryUpdate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryGet<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryGet)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TryGetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryGetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateByKey2<P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateByKey2)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateMetadataByKey2<P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateMetadataByKey2)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica5 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica5 {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica5_Vtbl {
    pub base__: IFabricKeyValueStoreReplica4_Vtbl,
    pub TryAdd: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i32,
        *const u8,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub TryRemove: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i64,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub TryUpdate: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        i32,
        *const u8,
        i64,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub TryGet: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryGetMetadata: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateByKey2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows::Win32::Foundation::BOOLEAN,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateByKey2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateMetadataByKey2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        ::windows::Win32::Foundation::BOOLEAN,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateMetadataByKey2: usize,
}
::windows_core::imp::com_interface!(
    IFabricKeyValueStoreReplica6,
    IFabricKeyValueStoreReplica6_Vtbl,
    0x56e77be1_e81f_4e42_8522_162c2d608184
);
::windows_core::imp::interface_hierarchy!(
    IFabricKeyValueStoreReplica6,
    ::windows_core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3,
    IFabricKeyValueStoreReplica4,
    IFabricKeyValueStoreReplica5
);
impl IFabricKeyValueStoreReplica6 {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(
        &self,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(::windows_core::Interface::as_raw(self), currentepoch)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Add)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Remove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Update)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Get)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Contains<P0, P1>(&self, transaction: P0, key: P1) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Contains)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Enumerate<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadata<P0>(
        &self,
        transaction: P0,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateMetadataByKey<P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Backup<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Backup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Restore<P0>(&self, backupdirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Restore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows_core::Result<IFabricTransaction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction2)(
            ::windows_core::Interface::as_raw(self),
            settings,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginBackup<P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStorePostBackupHandler>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .BeginBackup)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            backupoption,
            postbackuphandler.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndBackup<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .EndBackup)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRestore<P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .BeginRestore)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRestore<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .EndRestore)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TryAdd<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TryAdd)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryRemove<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TryRemove)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            checksequencenumber,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryUpdate<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TryUpdate)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            value.len().try_into().unwrap(),
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn TryGet<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TryGet)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TryGetMetadata<P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .TryGetMetadata)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            key.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateByKey2<P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateByKey2)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateMetadataByKey2<P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows_core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::windows_core::IntoParam<IFabricTransactionBase>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EnumerateMetadataByKey2)(
            ::windows_core::Interface::as_raw(self),
            transaction.into_param().abi(),
            keyprefix.into_param().abi(),
            strictprefix.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginRestore2<P0, P1>(
        &self,
        backupdirectory: P0,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRestore2)(
            ::windows_core::Interface::as_raw(self),
            backupdirectory.into_param().abi(),
            settings,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricKeyValueStoreReplica6 {}
unsafe impl ::core::marker::Sync for IFabricKeyValueStoreReplica6 {}
#[repr(C)]
pub struct IFabricKeyValueStoreReplica6_Vtbl {
    pub base__: IFabricKeyValueStoreReplica5_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginRestore2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginRestore2: usize,
}
::windows_core::imp::com_interface!(
    IFabricNodeContextResult,
    IFabricNodeContextResult_Vtbl,
    0x0952f885_6f5a_4ed3_abe4_90c403d1e3ce
);
::windows_core::imp::interface_hierarchy!(IFabricNodeContextResult, ::windows_core::IUnknown);
impl IFabricNodeContextResult {
    pub unsafe fn get_NodeContext(&self) -> *mut super::super::FABRIC_NODE_CONTEXT {
        (::windows_core::Interface::vtable(self).get_NodeContext)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricNodeContextResult {}
unsafe impl ::core::marker::Sync for IFabricNodeContextResult {}
#[repr(C)]
pub struct IFabricNodeContextResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_NodeContext: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    ) -> *mut super::super::FABRIC_NODE_CONTEXT,
}
::windows_core::imp::com_interface!(
    IFabricNodeContextResult2,
    IFabricNodeContextResult2_Vtbl,
    0x472bf2e1_d617_4b5c_a91d_fabed9ff3550
);
::windows_core::imp::interface_hierarchy!(
    IFabricNodeContextResult2,
    ::windows_core::IUnknown,
    IFabricNodeContextResult
);
impl IFabricNodeContextResult2 {
    pub unsafe fn get_NodeContext(&self) -> *mut super::super::FABRIC_NODE_CONTEXT {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_NodeContext)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDirectory<P0>(
        &self,
        logicaldirectoryname: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDirectory)(
            ::windows_core::Interface::as_raw(self),
            logicaldirectoryname.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricNodeContextResult2 {}
unsafe impl ::core::marker::Sync for IFabricNodeContextResult2 {}
#[repr(C)]
pub struct IFabricNodeContextResult2_Vtbl {
    pub base__: IFabricNodeContextResult_Vtbl,
    pub GetDirectory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricOperation,
    IFabricOperation_Vtbl,
    0xf4ad6bfa_e23c_4a48_9617_c099cd59a23a
);
::windows_core::imp::interface_hierarchy!(IFabricOperation, ::windows_core::IUnknown);
impl IFabricOperation {
    pub unsafe fn get_Metadata(&self) -> *mut super::super::FABRIC_OPERATION_METADATA {
        (::windows_core::Interface::vtable(self).get_Metadata)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn GetData(
        &self,
        count: *mut u32,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_OPERATION_DATA_BUFFER> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetData)(
            ::windows_core::Interface::as_raw(self),
            count,
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Acknowledge(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acknowledge)(::windows_core::Interface::as_raw(
            self,
        ))
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricOperation {}
unsafe impl ::core::marker::Sync for IFabricOperation {}
#[repr(C)]
pub struct IFabricOperation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_Metadata: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
    )
        -> *mut super::super::FABRIC_OPERATION_METADATA,
    pub GetData: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut u32,
        *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows_core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricOperationData,
    IFabricOperationData_Vtbl,
    0xbab8ad87_37b7_482a_985d_baf38a785dcd
);
::windows_core::imp::interface_hierarchy!(IFabricOperationData, ::windows_core::IUnknown);
impl IFabricOperationData {
    pub unsafe fn GetData(
        &self,
        count: *mut u32,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_OPERATION_DATA_BUFFER> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetData)(
            ::windows_core::Interface::as_raw(self),
            count,
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricOperationData {}
unsafe impl ::core::marker::Sync for IFabricOperationData {}
#[repr(C)]
pub struct IFabricOperationData_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut u32,
        *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricOperationDataStream,
    IFabricOperationDataStream_Vtbl,
    0xc4e9084c_be92_49c9_8c18_d44d088c2e32
);
::windows_core::imp::interface_hierarchy!(IFabricOperationDataStream, ::windows_core::IUnknown);
impl IFabricOperationDataStream {
    pub unsafe fn BeginGetNext<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetNext)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndGetNext<P0>(&self, context: P0) -> ::windows_core::Result<IFabricOperationData>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndGetNext)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricOperationDataStream {}
unsafe impl ::core::marker::Sync for IFabricOperationDataStream {}
#[repr(C)]
pub struct IFabricOperationDataStream_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginGetNext: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndGetNext: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricOperationStream,
    IFabricOperationStream_Vtbl,
    0xa98fb97a_d6b0_408a_a878_a9edb09c2587
);
::windows_core::imp::interface_hierarchy!(IFabricOperationStream, ::windows_core::IUnknown);
impl IFabricOperationStream {
    pub unsafe fn BeginGetOperation<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetOperation)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndGetOperation<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<IFabricOperation>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndGetOperation)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricOperationStream {}
unsafe impl ::core::marker::Sync for IFabricOperationStream {}
#[repr(C)]
pub struct IFabricOperationStream_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginGetOperation: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndGetOperation: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricOperationStream2,
    IFabricOperationStream2_Vtbl,
    0x0930199b_590a_4065_bec9_5f93b6aae086
);
::windows_core::imp::interface_hierarchy!(
    IFabricOperationStream2,
    ::windows_core::IUnknown,
    IFabricOperationStream
);
impl IFabricOperationStream2 {
    pub unsafe fn BeginGetOperation<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .BeginGetOperation)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndGetOperation<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<IFabricOperation>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .EndGetOperation)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportFault)(
            ::windows_core::Interface::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricOperationStream2 {}
unsafe impl ::core::marker::Sync for IFabricOperationStream2 {}
#[repr(C)]
pub struct IFabricOperationStream2_Vtbl {
    pub base__: IFabricOperationStream_Vtbl,
    pub ReportFault: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricPrimaryReplicator,
    IFabricPrimaryReplicator_Vtbl,
    0x564e50dd_c3a4_4600_a60e_6658874307ae
);
::windows_core::imp::interface_hierarchy!(
    IFabricPrimaryReplicator,
    ::windows_core::IUnknown,
    IFabricReplicator
);
impl IFabricPrimaryReplicator {
    pub unsafe fn BeginOpen<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginOpen)(
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
        (::windows_core::Interface::vtable(self).base__.EndOpen)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            epoch,
            role,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).base__.EndChangeRole)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUpdateEpoch<P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .BeginUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            epoch,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndUpdateEpoch<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self)
            .base__
            .EndUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginClose<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginClose)(
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
        (::windows_core::Interface::vtable(self).base__.EndClose)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows_core::Interface::vtable(self).base__.Abort)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetCurrentProgress)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetCatchUpCapability)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn BeginOnDataLoss<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOnDataLoss<P0>(&self, context: P0) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateCatchUpReplicaSetConfiguration)(
            ::windows_core::Interface::as_raw(self),
            currentconfiguration,
            previousconfiguration,
        )
        .ok()
    }
    pub unsafe fn BeginWaitForCatchUpQuorum<P0>(
        &self,
        catchupmode: super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginWaitForCatchUpQuorum)(
            ::windows_core::Interface::as_raw(self),
            catchupmode,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndWaitForCatchUpQuorum<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndWaitForCatchUpQuorum)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateCurrentReplicaSetConfiguration)(
            ::windows_core::Interface::as_raw(self),
            currentconfiguration,
        )
        .ok()
    }
    pub unsafe fn BeginBuildReplica<P0>(
        &self,
        replica: *const super::super::FABRIC_REPLICA_INFORMATION,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginBuildReplica)(
            ::windows_core::Interface::as_raw(self),
            replica,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndBuildReplica<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndBuildReplica)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveReplica(&self, replicaid: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveReplica)(
            ::windows_core::Interface::as_raw(self),
            replicaid,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricPrimaryReplicator {}
unsafe impl ::core::marker::Sync for IFabricPrimaryReplicator {}
#[repr(C)]
pub struct IFabricPrimaryReplicator_Vtbl {
    pub base__: IFabricReplicator_Vtbl,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub UpdateCatchUpReplicaSetConfiguration: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    )
        -> ::windows_core::HRESULT,
    pub BeginWaitForCatchUpQuorum: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndWaitForCatchUpQuorum: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UpdateCurrentReplicaSetConfiguration: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    )
        -> ::windows_core::HRESULT,
    pub BeginBuildReplica: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICA_INFORMATION,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndBuildReplica: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveReplica:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, i64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricProcessExitHandler,
    IFabricProcessExitHandler_Vtbl,
    0xc58d50a2_01f0_4267_bbe7_223b565c1346
);
::windows_core::imp::interface_hierarchy!(IFabricProcessExitHandler, ::windows_core::IUnknown);
impl IFabricProcessExitHandler {
    pub unsafe fn FabricProcessExited(&self) {
        (::windows_core::Interface::vtable(self).FabricProcessExited)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricProcessExitHandler {}
unsafe impl ::core::marker::Sync for IFabricProcessExitHandler {}
#[repr(C)]
pub struct IFabricProcessExitHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FabricProcessExited: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricReplicator,
    IFabricReplicator_Vtbl,
    0x067f144a_e5be_4f5e_a181_8b5593e20242
);
::windows_core::imp::interface_hierarchy!(IFabricReplicator, ::windows_core::IUnknown);
impl IFabricReplicator {
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
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            epoch,
            role,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndChangeRole)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUpdateEpoch<P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            epoch,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndUpdateEpoch<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
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
    pub unsafe fn GetCurrentProgress(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrentProgress)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCatchUpCapability(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCatchUpCapability)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricReplicator {}
unsafe impl ::core::marker::Sync for IFabricReplicator {}
#[repr(C)]
pub struct IFabricReplicator_Vtbl {
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
    pub BeginChangeRole: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_EPOCH,
        super::super::FABRIC_REPLICA_ROLE,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndChangeRole: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginUpdateEpoch: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_EPOCH,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndUpdateEpoch: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
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
    pub GetCurrentProgress:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i64) -> ::windows_core::HRESULT,
    pub GetCatchUpCapability:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricReplicatorCatchupSpecificQuorum,
    IFabricReplicatorCatchupSpecificQuorum_Vtbl,
    0xaa3116fe_277d_482d_bd16_5366fa405757
);
::windows_core::imp::interface_hierarchy!(
    IFabricReplicatorCatchupSpecificQuorum,
    ::windows_core::IUnknown
);
impl IFabricReplicatorCatchupSpecificQuorum {}
unsafe impl ::core::marker::Send for IFabricReplicatorCatchupSpecificQuorum {}
unsafe impl ::core::marker::Sync for IFabricReplicatorCatchupSpecificQuorum {}
#[repr(C)]
pub struct IFabricReplicatorCatchupSpecificQuorum_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
::windows_core::imp::com_interface!(
    IFabricReplicatorSettingsResult,
    IFabricReplicatorSettingsResult_Vtbl,
    0x718954f3_dc1e_4060_9806_0cbf36f71051
);
::windows_core::imp::interface_hierarchy!(
    IFabricReplicatorSettingsResult,
    ::windows_core::IUnknown
);
impl IFabricReplicatorSettingsResult {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ReplicatorSettings(&self) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS {
        (::windows_core::Interface::vtable(self).get_ReplicatorSettings)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricReplicatorSettingsResult {}
unsafe impl ::core::marker::Sync for IFabricReplicatorSettingsResult {}
#[repr(C)]
pub struct IFabricReplicatorSettingsResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ReplicatorSettings:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ReplicatorSettings: usize,
}
::windows_core::imp::com_interface!(
    IFabricRuntime,
    IFabricRuntime_Vtbl,
    0xcc53af8e_74cd_11df_ac3e_0024811e3892
);
::windows_core::imp::interface_hierarchy!(IFabricRuntime, ::windows_core::IUnknown);
impl IFabricRuntime {
    pub unsafe fn BeginRegisterStatelessServiceFactory<P0, P1, P2>(
        &self,
        servicetypename: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatelessServiceFactory>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRegisterStatelessServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            factory.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRegisterStatelessServiceFactory<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndRegisterStatelessServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterStatelessServiceFactory<P0, P1>(
        &self,
        servicetypename: P0,
        factory: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatelessServiceFactory>,
    {
        (::windows_core::Interface::vtable(self).RegisterStatelessServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            factory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRegisterStatefulServiceFactory<P0, P1, P2>(
        &self,
        servicetypename: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatefulServiceFactory>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRegisterStatefulServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            factory.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRegisterStatefulServiceFactory<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndRegisterStatefulServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterStatefulServiceFactory<P0, P1>(
        &self,
        servicetypename: P0,
        factory: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatefulServiceFactory>,
    {
        (::windows_core::Interface::vtable(self).RegisterStatefulServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            factory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateServiceGroupFactoryBuilder(
        &self,
    ) -> ::windows_core::Result<IFabricServiceGroupFactoryBuilder> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateServiceGroupFactoryBuilder)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BeginRegisterServiceGroupFactory<P0, P1, P2>(
        &self,
        groupservicetype: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricServiceGroupFactory>,
        P2: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginRegisterServiceGroupFactory)(
            ::windows_core::Interface::as_raw(self),
            groupservicetype.into_param().abi(),
            factory.into_param().abi(),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndRegisterServiceGroupFactory<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndRegisterServiceGroupFactory)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterServiceGroupFactory<P0, P1>(
        &self,
        groupservicetype: P0,
        factory: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricServiceGroupFactory>,
    {
        (::windows_core::Interface::vtable(self).RegisterServiceGroupFactory)(
            ::windows_core::Interface::as_raw(self),
            groupservicetype.into_param().abi(),
            factory.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricRuntime {}
unsafe impl ::core::marker::Sync for IFabricRuntime {}
#[repr(C)]
pub struct IFabricRuntime_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginRegisterStatelessServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub EndRegisterStatelessServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub RegisterStatelessServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginRegisterStatefulServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub EndRegisterStatefulServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub RegisterStatefulServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateServiceGroupFactoryBuilder: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginRegisterServiceGroupFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndRegisterServiceGroupFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RegisterServiceGroupFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricSecondaryEventHandler,
    IFabricSecondaryEventHandler_Vtbl,
    0x7d124a7d_258e_49f2_a9b0_e800406103fb
);
::windows_core::imp::interface_hierarchy!(IFabricSecondaryEventHandler, ::windows_core::IUnknown);
impl IFabricSecondaryEventHandler {
    pub unsafe fn OnCopyComplete<P0>(&self, enumerator: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricKeyValueStoreEnumerator>,
    {
        (::windows_core::Interface::vtable(self).OnCopyComplete)(
            ::windows_core::Interface::as_raw(self),
            enumerator.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnReplicationOperation<P0>(&self, enumerator: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFabricKeyValueStoreNotificationEnumerator>,
    {
        (::windows_core::Interface::vtable(self).OnReplicationOperation)(
            ::windows_core::Interface::as_raw(self),
            enumerator.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricSecondaryEventHandler {}
unsafe impl ::core::marker::Sync for IFabricSecondaryEventHandler {}
#[repr(C)]
pub struct IFabricSecondaryEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnCopyComplete: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnReplicationOperation: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricSecurityCredentialsResult,
    IFabricSecurityCredentialsResult_Vtbl,
    0x049a111d_6a30_48e9_8f69_470760d3efb9
);
::windows_core::imp::interface_hierarchy!(
    IFabricSecurityCredentialsResult,
    ::windows_core::IUnknown
);
impl IFabricSecurityCredentialsResult {
    pub unsafe fn get_SecurityCredentials(&self) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS {
        (::windows_core::Interface::vtable(self).get_SecurityCredentials)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricSecurityCredentialsResult {}
unsafe impl ::core::marker::Sync for IFabricSecurityCredentialsResult {}
#[repr(C)]
pub struct IFabricSecurityCredentialsResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_SecurityCredentials:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS,
}
::windows_core::imp::com_interface!(
    IFabricServiceGroupFactory,
    IFabricServiceGroupFactory_Vtbl,
    0x3860d61d_1e51_4a65_b109_d93c11311657
);
::windows_core::imp::interface_hierarchy!(IFabricServiceGroupFactory, ::windows_core::IUnknown);
impl IFabricServiceGroupFactory {}
unsafe impl ::core::marker::Send for IFabricServiceGroupFactory {}
unsafe impl ::core::marker::Sync for IFabricServiceGroupFactory {}
#[repr(C)]
pub struct IFabricServiceGroupFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
::windows_core::imp::com_interface!(
    IFabricServiceGroupFactoryBuilder,
    IFabricServiceGroupFactoryBuilder_Vtbl,
    0xa9fe8b06_19b1_49e6_8911_41d9d9219e1c
);
::windows_core::imp::interface_hierarchy!(
    IFabricServiceGroupFactoryBuilder,
    ::windows_core::IUnknown
);
impl IFabricServiceGroupFactoryBuilder {
    pub unsafe fn AddStatelessServiceFactory<P0, P1>(
        &self,
        memberservicetype: P0,
        factory: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatelessServiceFactory>,
    {
        (::windows_core::Interface::vtable(self).AddStatelessServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            memberservicetype.into_param().abi(),
            factory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AddStatefulServiceFactory<P0, P1>(
        &self,
        memberservicetype: P0,
        factory: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IFabricStatefulServiceFactory>,
    {
        (::windows_core::Interface::vtable(self).AddStatefulServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            memberservicetype.into_param().abi(),
            factory.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveServiceFactory<P0>(
        &self,
        memberservicetype: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveServiceFactory)(
            ::windows_core::Interface::as_raw(self),
            memberservicetype.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ToServiceGroupFactory(
        &self,
    ) -> ::windows_core::Result<IFabricServiceGroupFactory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ToServiceGroupFactory)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricServiceGroupFactoryBuilder {}
unsafe impl ::core::marker::Sync for IFabricServiceGroupFactoryBuilder {}
#[repr(C)]
pub struct IFabricServiceGroupFactoryBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddStatelessServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddStatefulServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveServiceFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
    ) -> ::windows_core::HRESULT,
    pub ToServiceGroupFactory: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricServiceGroupPartition,
    IFabricServiceGroupPartition_Vtbl,
    0x2b24299a_7489_467f_8e7f_4507bff73b86
);
::windows_core::imp::interface_hierarchy!(IFabricServiceGroupPartition, ::windows_core::IUnknown);
impl IFabricServiceGroupPartition {
    pub unsafe fn ResolveMember(
        &self,
        name: *const u16,
        riid: *const ::windows_core::GUID,
    ) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResolveMember)(
            ::windows_core::Interface::as_raw(self),
            name,
            riid,
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricServiceGroupPartition {}
unsafe impl ::core::marker::Sync for IFabricServiceGroupPartition {}
#[repr(C)]
pub struct IFabricServiceGroupPartition_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ResolveMember: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const u16,
        *const ::windows_core::GUID,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStateProvider,
    IFabricStateProvider_Vtbl,
    0x3ebfec79_bd27_43f3_8be8_da38ee723951
);
::windows_core::imp::interface_hierarchy!(IFabricStateProvider, ::windows_core::IUnknown);
impl IFabricStateProvider {
    pub unsafe fn BeginUpdateEpoch<P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            epoch,
            previousepochlastsequencenumber,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndUpdateEpoch<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        (::windows_core::Interface::vtable(self).EndUpdateEpoch)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetLastCommittedSequenceNumber(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastCommittedSequenceNumber)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn BeginOnDataLoss<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOnDataLoss<P0>(&self, context: P0) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetCopyContext(&self) -> ::windows_core::Result<IFabricOperationDataStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCopyContext)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCopyState<P0>(
        &self,
        uptosequencenumber: i64,
        copycontextstream: P0,
    ) -> ::windows_core::Result<IFabricOperationDataStream>
    where
        P0: ::windows_core::IntoParam<IFabricOperationDataStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCopyState)(
            ::windows_core::Interface::as_raw(self),
            uptosequencenumber,
            copycontextstream.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricStateProvider {}
unsafe impl ::core::marker::Sync for IFabricStateProvider {}
#[repr(C)]
pub struct IFabricStateProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginUpdateEpoch: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_EPOCH,
        i64,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndUpdateEpoch: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLastCommittedSequenceNumber:
        unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i64) -> ::windows_core::HRESULT,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut u8,
    ) -> ::windows_core::HRESULT,
    pub GetCopyContext: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCopyState: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        i64,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStateReplicator,
    IFabricStateReplicator_Vtbl,
    0x89e9a978_c771_44f2_92e8_3bf271cabe9c
);
::windows_core::imp::interface_hierarchy!(IFabricStateReplicator, ::windows_core::IUnknown);
impl IFabricStateReplicator {
    pub unsafe fn BeginReplicate<P0, P1>(
        &self,
        operationdata: P0,
        callback: P1,
        sequencenumber: *mut i64,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricOperationData>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginReplicate)(
            ::windows_core::Interface::as_raw(self),
            operationdata.into_param().abi(),
            callback.into_param().abi(),
            sequencenumber,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndReplicate<P0>(&self, context: P0) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndReplicate)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetReplicationStream(&self) -> ::windows_core::Result<IFabricOperationStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReplicationStream)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCopyStream(&self) -> ::windows_core::Result<IFabricOperationStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCopyStream)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self),
            replicatorsettings,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStateReplicator {}
unsafe impl ::core::marker::Sync for IFabricStateReplicator {}
#[repr(C)]
pub struct IFabricStateReplicator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginReplicate: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndReplicate: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub GetReplicationStream: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCopyStream: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateReplicatorSettings: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateReplicatorSettings: usize,
}
::windows_core::imp::com_interface!(
    IFabricStateReplicator2,
    IFabricStateReplicator2_Vtbl,
    0x4a28d542_658f_46f9_9bf4_79b7cae25c5d
);
::windows_core::imp::interface_hierarchy!(
    IFabricStateReplicator2,
    ::windows_core::IUnknown,
    IFabricStateReplicator
);
impl IFabricStateReplicator2 {
    pub unsafe fn BeginReplicate<P0, P1>(
        &self,
        operationdata: P0,
        callback: P1,
        sequencenumber: *mut i64,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricOperationData>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .BeginReplicate)(
            ::windows_core::Interface::as_raw(self),
            operationdata.into_param().abi(),
            callback.into_param().abi(),
            sequencenumber,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndReplicate<P0>(&self, context: P0) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndReplicate)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetReplicationStream(&self) -> ::windows_core::Result<IFabricOperationStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetReplicationStream)(::windows_core::Interface::as_raw(self), &mut result__)
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCopyStream(&self) -> ::windows_core::Result<IFabricOperationStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCopyStream)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .UpdateReplicatorSettings)(
            ::windows_core::Interface::as_raw(self), replicatorsettings
        )
        .ok()
    }
    pub unsafe fn GetReplicatorSettings(
        &self,
    ) -> ::windows_core::Result<IFabricReplicatorSettingsResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReplicatorSettings)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricStateReplicator2 {}
unsafe impl ::core::marker::Sync for IFabricStateReplicator2 {}
#[repr(C)]
pub struct IFabricStateReplicator2_Vtbl {
    pub base__: IFabricStateReplicator_Vtbl,
    pub GetReplicatorSettings: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServiceFactory,
    IFabricStatefulServiceFactory_Vtbl,
    0x77ff0c6b_6780_48ec_b4b0_61989327b0f2
);
::windows_core::imp::interface_hierarchy!(IFabricStatefulServiceFactory, ::windows_core::IUnknown);
impl IFabricStatefulServiceFactory {
    pub unsafe fn CreateReplica<P0>(
        &self,
        servicetypename: P0,
        servicename: *const u16,
        initializationdata: &[u8],
        partitionid: ::windows_core::GUID,
        replicaid: i64,
    ) -> ::windows_core::Result<IFabricStatefulServiceReplica>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateReplica)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            servicename,
            initializationdata.len().try_into().unwrap(),
            ::core::mem::transmute(initializationdata.as_ptr()),
            ::core::mem::transmute(partitionid),
            replicaid,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricStatefulServiceFactory {}
unsafe impl ::core::marker::Sync for IFabricStatefulServiceFactory {}
#[repr(C)]
pub struct IFabricStatefulServiceFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateReplica: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *const u16,
        u32,
        *const u8,
        ::windows_core::GUID,
        i64,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServicePartition,
    IFabricStatefulServicePartition_Vtbl,
    0x5beccc37_8655_4f20_bd43_f50691d7cd16
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatefulServicePartition,
    ::windows_core::IUnknown
);
impl IFabricStatefulServicePartition {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartitionInfo)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReadStatus)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWriteStatus)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateReplicator<P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
    ) -> ::windows_core::Result<IFabricStateReplicator>
    where
        P0: ::windows_core::IntoParam<IFabricStateProvider>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateReplicator)(
            ::windows_core::Interface::as_raw(self),
            stateprovider.into_param().abi(),
            replicatorsettings,
            ::core::mem::transmute(replicator),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportFault)(
            ::windows_core::Interface::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatefulServicePartition {}
unsafe impl ::core::marker::Sync for IFabricStatefulServicePartition {}
#[repr(C)]
pub struct IFabricStatefulServicePartition_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPartitionInfo: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
    ) -> ::windows_core::HRESULT,
    pub GetReadStatus: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    ) -> ::windows_core::HRESULT,
    pub GetWriteStatus: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateReplicator: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_REPLICATOR_SETTINGS,
        *mut *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateReplicator: usize,
    pub ReportLoad: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        u32,
        *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows_core::HRESULT,
    pub ReportFault: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServicePartition1,
    IFabricStatefulServicePartition1_Vtbl,
    0xc9c66f2f_9dff_4c87_bbe4_a08b4c4074cf
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatefulServicePartition1,
    ::windows_core::IUnknown,
    IFabricStatefulServicePartition
);
impl IFabricStatefulServicePartition1 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReadStatus)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetWriteStatus)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateReplicator<P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
    ) -> ::windows_core::Result<IFabricStateReplicator>
    where
        P0: ::windows_core::IntoParam<IFabricStateProvider>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .CreateReplicator)(
            ::windows_core::Interface::as_raw(self),
            stateprovider.into_param().abi(),
            replicatorsettings,
            ::core::mem::transmute(replicator),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReportFault)(
            ::windows_core::Interface::as_raw(self),
            faulttype,
        )
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportMoveCost)(
            ::windows_core::Interface::as_raw(self),
            movecost,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatefulServicePartition1 {}
unsafe impl ::core::marker::Sync for IFabricStatefulServicePartition1 {}
#[repr(C)]
pub struct IFabricStatefulServicePartition1_Vtbl {
    pub base__: IFabricStatefulServicePartition_Vtbl,
    pub ReportMoveCost: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServicePartition2,
    IFabricStatefulServicePartition2_Vtbl,
    0xdf27b476_fa25_459f_a7d3_87d3eec9c73c
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatefulServicePartition2,
    ::windows_core::IUnknown,
    IFabricStatefulServicePartition,
    IFabricStatefulServicePartition1
);
impl IFabricStatefulServicePartition2 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetReadStatus)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetWriteStatus)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateReplicator<P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
    ) -> ::windows_core::Result<IFabricStateReplicator>
    where
        P0: ::windows_core::IntoParam<IFabricStateProvider>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .CreateReplicator)(
            ::windows_core::Interface::as_raw(self),
            stateprovider.into_param().abi(),
            replicatorsettings,
            ::core::mem::transmute(replicator),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportFault)(::windows_core::Interface::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportMoveCost)(::windows_core::Interface::as_raw(self), movecost)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportReplicaHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportReplicaHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportPartitionHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatefulServicePartition2 {}
unsafe impl ::core::marker::Sync for IFabricStatefulServicePartition2 {}
#[repr(C)]
pub struct IFabricStatefulServicePartition2_Vtbl {
    pub base__: IFabricStatefulServicePartition1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportReplicaHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportReplicaHealth: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportPartitionHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportPartitionHealth: usize,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServicePartition3,
    IFabricStatefulServicePartition3_Vtbl,
    0x51f1269d_b061_4c1c_96cf_6508cece813b
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatefulServicePartition3,
    ::windows_core::IUnknown,
    IFabricStatefulServicePartition,
    IFabricStatefulServicePartition1,
    IFabricStatefulServicePartition2
);
impl IFabricStatefulServicePartition3 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetReadStatus)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows_core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetWriteStatus)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateReplicator<P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
    ) -> ::windows_core::Result<IFabricStateReplicator>
    where
        P0: ::windows_core::IntoParam<IFabricStateProvider>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .CreateReplicator)(
            ::windows_core::Interface::as_raw(self),
            stateprovider.into_param().abi(),
            replicatorsettings,
            ::core::mem::transmute(replicator),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportFault)(::windows_core::Interface::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportMoveCost)(::windows_core::Interface::as_raw(self), movecost)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportReplicaHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportReplicaHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportPartitionHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportReplicaHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportReplicaHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportPartitionHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatefulServicePartition3 {}
unsafe impl ::core::marker::Sync for IFabricStatefulServicePartition3 {}
#[repr(C)]
pub struct IFabricStatefulServicePartition3_Vtbl {
    pub base__: IFabricStatefulServicePartition2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportReplicaHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportReplicaHealth2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportPartitionHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportPartitionHealth2: usize,
}
::windows_core::imp::com_interface!(
    IFabricStatefulServiceReplica,
    IFabricStatefulServiceReplica_Vtbl,
    0x8ae3be0e_505d_4dc1_ad8f_0cb0f9576b8a
);
::windows_core::imp::interface_hierarchy!(IFabricStatefulServiceReplica, ::windows_core::IUnknown);
impl IFabricStatefulServiceReplica {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatefulServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            openmode,
            partition.into_param().abi(),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOpen<P0>(&self, context: P0) -> ::windows_core::Result<IFabricReplicator>
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
    pub unsafe fn BeginChangeRole<P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginChangeRole)(
            ::windows_core::Interface::as_raw(self),
            newrole,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndChangeRole<P0>(
        &self,
        context: P0,
    ) -> ::windows_core::Result<super::IFabricStringResult>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndChangeRole)(
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
unsafe impl ::core::marker::Send for IFabricStatefulServiceReplica {}
unsafe impl ::core::marker::Sync for IFabricStatefulServiceReplica {}
#[repr(C)]
pub struct IFabricStatefulServiceReplica_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_REPLICA_OPEN_MODE,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BeginChangeRole: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_REPLICA_ROLE,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndChangeRole: unsafe extern "system" fn(
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
    IFabricStatelessServiceFactory,
    IFabricStatelessServiceFactory_Vtbl,
    0xcc53af8f_74cd_11df_ac3e_0024811e3892
);
::windows_core::imp::interface_hierarchy!(IFabricStatelessServiceFactory, ::windows_core::IUnknown);
impl IFabricStatelessServiceFactory {
    pub unsafe fn CreateInstance<P0>(
        &self,
        servicetypename: P0,
        servicename: *const u16,
        initializationdata: &[u8],
        partitionid: ::windows_core::GUID,
        instanceid: i64,
    ) -> ::windows_core::Result<IFabricStatelessServiceInstance>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstance)(
            ::windows_core::Interface::as_raw(self),
            servicetypename.into_param().abi(),
            servicename,
            initializationdata.len().try_into().unwrap(),
            ::core::mem::transmute(initializationdata.as_ptr()),
            ::core::mem::transmute(partitionid),
            instanceid,
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
unsafe impl ::core::marker::Send for IFabricStatelessServiceFactory {}
unsafe impl ::core::marker::Sync for IFabricStatelessServiceFactory {}
#[repr(C)]
pub struct IFabricStatelessServiceFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        ::windows_core::PCWSTR,
        *const u16,
        u32,
        *const u8,
        ::windows_core::GUID,
        i64,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatelessServiceInstance,
    IFabricStatelessServiceInstance_Vtbl,
    0xcc53af90_74cd_11df_ac3e_0024811e3892
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatelessServiceInstance,
    ::windows_core::IUnknown
);
impl IFabricStatelessServiceInstance {
    pub unsafe fn BeginOpen<P0, P1>(
        &self,
        partition: P0,
        callback: P1,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<IFabricStatelessServicePartition>,
        P1: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOpen)(
            ::windows_core::Interface::as_raw(self),
            partition.into_param().abi(),
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
unsafe impl ::core::marker::Send for IFabricStatelessServiceInstance {}
unsafe impl ::core::marker::Sync for IFabricStatelessServiceInstance {}
#[repr(C)]
pub struct IFabricStatelessServiceInstance_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
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
    IFabricStatelessServicePartition,
    IFabricStatelessServicePartition_Vtbl,
    0xcc53af91_74cd_11df_ac3e_0024811e3892
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatelessServicePartition,
    ::windows_core::IUnknown
);
impl IFabricStatelessServicePartition {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartitionInfo)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportFault)(
            ::windows_core::Interface::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatelessServicePartition {}
unsafe impl ::core::marker::Sync for IFabricStatelessServicePartition {}
#[repr(C)]
pub struct IFabricStatelessServicePartition_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPartitionInfo: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
    ) -> ::windows_core::HRESULT,
    pub ReportLoad: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        u32,
        *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows_core::HRESULT,
    pub ReportFault: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatelessServicePartition1,
    IFabricStatelessServicePartition1_Vtbl,
    0xbf6bb505_7bd0_4371_b6c0_cba319a5e50b
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatelessServicePartition1,
    ::windows_core::IUnknown,
    IFabricStatelessServicePartition
);
impl IFabricStatelessServicePartition1 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReportFault)(
            ::windows_core::Interface::as_raw(self),
            faulttype,
        )
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportMoveCost)(
            ::windows_core::Interface::as_raw(self),
            movecost,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatelessServicePartition1 {}
unsafe impl ::core::marker::Sync for IFabricStatelessServicePartition1 {}
#[repr(C)]
pub struct IFabricStatelessServicePartition1_Vtbl {
    pub base__: IFabricStatelessServicePartition_Vtbl,
    pub ReportMoveCost: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStatelessServicePartition2,
    IFabricStatelessServicePartition2_Vtbl,
    0x9ff35b6c_9d97_4312_93ad_7f34cbdb4ca4
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatelessServicePartition2,
    ::windows_core::IUnknown,
    IFabricStatelessServicePartition,
    IFabricStatelessServicePartition1
);
impl IFabricStatelessServicePartition2 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportFault)(::windows_core::Interface::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportMoveCost)(::windows_core::Interface::as_raw(self), movecost)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportInstanceHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportInstanceHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportPartitionHealth)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatelessServicePartition2 {}
unsafe impl ::core::marker::Sync for IFabricStatelessServicePartition2 {}
#[repr(C)]
pub struct IFabricStatelessServicePartition2_Vtbl {
    pub base__: IFabricStatelessServicePartition1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportInstanceHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportInstanceHealth: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportPartitionHealth: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportPartitionHealth: usize,
}
::windows_core::imp::com_interface!(
    IFabricStatelessServicePartition3,
    IFabricStatelessServicePartition3_Vtbl,
    0xf2fa2000_70a7_4ed5_9d3e_0b7deca2433f
);
::windows_core::imp::interface_hierarchy!(
    IFabricStatelessServicePartition3,
    ::windows_core::IUnknown,
    IFabricStatelessServicePartition,
    IFabricStatelessServicePartition1,
    IFabricStatelessServicePartition2
);
impl IFabricStatelessServicePartition3 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows_core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .GetPartitionInfo)(::windows_core::Interface::as_raw(self), &mut result__)
        .map(|| result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportLoad)(
            ::windows_core::Interface::as_raw(self),
            metrics.len().try_into().unwrap(),
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .base__
            .ReportFault)(::windows_core::Interface::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .base__
            .ReportMoveCost)(::windows_core::Interface::as_raw(self), movecost)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportInstanceHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportInstanceHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self)
            .base__
            .ReportPartitionHealth)(::windows_core::Interface::as_raw(self), healthinfo)
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportInstanceHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportInstanceHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportPartitionHealth2)(
            ::windows_core::Interface::as_raw(self),
            healthinfo,
            sendoptions,
        )
        .ok()
    }
}
unsafe impl ::core::marker::Send for IFabricStatelessServicePartition3 {}
unsafe impl ::core::marker::Sync for IFabricStatelessServicePartition3 {}
#[repr(C)]
pub struct IFabricStatelessServicePartition3_Vtbl {
    pub base__: IFabricStatelessServicePartition2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportInstanceHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportInstanceHealth2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportPartitionHealth2: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_HEALTH_INFORMATION,
        *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportPartitionHealth2: usize,
}
::windows_core::imp::com_interface!(
    IFabricStoreEventHandler,
    IFabricStoreEventHandler_Vtbl,
    0x220e6da4_985b_4dee_8fe9_77521b838795
);
::windows_core::imp::interface_hierarchy!(IFabricStoreEventHandler, ::windows_core::IUnknown);
impl IFabricStoreEventHandler {
    pub unsafe fn OnDataLoss(&self) {
        (::windows_core::Interface::vtable(self).OnDataLoss)(::windows_core::Interface::as_raw(
            self,
        ))
    }
}
unsafe impl ::core::marker::Send for IFabricStoreEventHandler {}
unsafe impl ::core::marker::Sync for IFabricStoreEventHandler {}
#[repr(C)]
pub struct IFabricStoreEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnDataLoss: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricStoreEventHandler2,
    IFabricStoreEventHandler2_Vtbl,
    0xcce4523f_614b_4d6a_98a3_1e197c0213ea
);
::windows_core::imp::interface_hierarchy!(
    IFabricStoreEventHandler2,
    ::windows_core::IUnknown,
    IFabricStoreEventHandler
);
impl IFabricStoreEventHandler2 {
    pub unsafe fn OnDataLoss(&self) {
        (::windows_core::Interface::vtable(self).base__.OnDataLoss)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn BeginOnDataLoss<P0>(
        &self,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndOnDataLoss<P0>(&self, context: P0) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndOnDataLoss)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricStoreEventHandler2 {}
unsafe impl ::core::marker::Sync for IFabricStoreEventHandler2 {}
#[repr(C)]
pub struct IFabricStoreEventHandler2_Vtbl {
    pub base__: IFabricStoreEventHandler_Vtbl,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut u8,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricStorePostBackupHandler,
    IFabricStorePostBackupHandler_Vtbl,
    0x2af2e8a6_41df_4e32_9d2a_d73a711e652a
);
::windows_core::imp::interface_hierarchy!(IFabricStorePostBackupHandler, ::windows_core::IUnknown);
impl IFabricStorePostBackupHandler {
    pub unsafe fn BeginPostBackup<P0>(
        &self,
        info: *const super::super::FABRIC_STORE_BACKUP_INFO,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginPostBackup)(
            ::windows_core::Interface::as_raw(self),
            info,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndPostBackup<P0>(&self, context: P0) -> ::windows_core::Result<u8>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndPostBackup)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
}
unsafe impl ::core::marker::Send for IFabricStorePostBackupHandler {}
unsafe impl ::core::marker::Sync for IFabricStorePostBackupHandler {}
#[repr(C)]
pub struct IFabricStorePostBackupHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginPostBackup: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *const super::super::FABRIC_STORE_BACKUP_INFO,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndPostBackup: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut u8,
    ) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(
    IFabricTransaction,
    IFabricTransaction_Vtbl,
    0x19ee48b4_6d4d_470b_ac1e_2d3996a173c8
);
::windows_core::imp::interface_hierarchy!(
    IFabricTransaction,
    ::windows_core::IUnknown,
    IFabricTransactionBase
);
impl IFabricTransaction {
    pub unsafe fn get_Id(&self) -> *mut ::windows_core::GUID {
        (::windows_core::Interface::vtable(self).base__.get_Id)(::windows_core::Interface::as_raw(
            self,
        ))
    }
    pub unsafe fn get_IsolationLevel(&self) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL {
        (::windows_core::Interface::vtable(self)
            .base__
            .get_IsolationLevel)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn BeginCommit<P0>(
        &self,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows_core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BeginCommit)(
            ::windows_core::Interface::as_raw(self),
            timeoutmilliseconds,
            callback.into_param().abi(),
            &mut result__,
        )
        .and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EndCommit<P0>(&self, context: P0) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::IFabricAsyncOperationContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndCommit)(
            ::windows_core::Interface::as_raw(self),
            context.into_param().abi(),
            &mut result__,
        )
        .map(|| result__)
    }
    pub unsafe fn Rollback(&self) {
        (::windows_core::Interface::vtable(self).Rollback)(::windows_core::Interface::as_raw(self))
    }
}
unsafe impl ::core::marker::Send for IFabricTransaction {}
unsafe impl ::core::marker::Sync for IFabricTransaction {}
#[repr(C)]
pub struct IFabricTransaction_Vtbl {
    pub base__: IFabricTransactionBase_Vtbl,
    pub BeginCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        u32,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndCommit: unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut i64,
    ) -> ::windows_core::HRESULT,
    pub Rollback: unsafe extern "system" fn(*mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(
    IFabricTransactionBase,
    IFabricTransactionBase_Vtbl,
    0x32d656a1_7ad5_47b8_bd66_a2e302626b7e
);
::windows_core::imp::interface_hierarchy!(IFabricTransactionBase, ::windows_core::IUnknown);
impl IFabricTransactionBase {
    pub unsafe fn get_Id(&self) -> *mut ::windows_core::GUID {
        (::windows_core::Interface::vtable(self).get_Id)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn get_IsolationLevel(&self) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL {
        (::windows_core::Interface::vtable(self).get_IsolationLevel)(
            ::windows_core::Interface::as_raw(self),
        )
    }
}
unsafe impl ::core::marker::Send for IFabricTransactionBase {}
unsafe impl ::core::marker::Sync for IFabricTransactionBase {}
#[repr(C)]
pub struct IFabricTransactionBase_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_Id: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> *mut ::windows_core::GUID,
    pub get_IsolationLevel:
        unsafe extern "system" fn(
            *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL,
}
pub const FabricRuntime: ::windows_core::GUID =
    ::windows_core::GUID::from_u128(0xcc53af8c_74cd_11df_ac3e_0024811e3892);
pub type FnFabricMain = ::core::option::Option<
    unsafe extern "system" fn(
        runtime: ::core::option::Option<IFabricRuntime>,
        activationcontext: ::core::option::Option<IFabricCodePackageActivationContext>,
    ) -> ::windows_core::HRESULT,
>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
