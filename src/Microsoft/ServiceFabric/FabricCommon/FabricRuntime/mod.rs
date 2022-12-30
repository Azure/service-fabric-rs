#[inline]
pub unsafe fn FabricBeginCreateRuntime<'a, P0, P1>(
    riid: *const ::windows::core::GUID,
    exithandler: P0,
    timeoutmilliseconds: u32,
    callback: P1,
) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricProcessExitHandler>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricBeginCreateRuntime(
            riid: *const ::windows::core::GUID,
            exithandler: *mut ::core::ffi::c_void,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricBeginCreateRuntime(
        ::core::mem::transmute(riid),
        exithandler.into().abi(),
        timeoutmilliseconds,
        callback.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<super::IFabricAsyncOperationContext>(result__)
}
#[inline]
pub unsafe fn FabricBeginGetActivationContext<'a, P0>(
    riid: *const ::windows::core::GUID,
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricBeginGetActivationContext(
            riid: *const ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricBeginGetActivationContext(
        ::core::mem::transmute(riid),
        timeoutmilliseconds,
        callback.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<super::IFabricAsyncOperationContext>(result__)
}
#[inline]
pub unsafe fn FabricBeginGetCodePackageActivator<'a, P0>(
    riid: *const ::windows::core::GUID,
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricBeginGetCodePackageActivator(
            riid: *const ::windows::core::GUID,
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricBeginGetCodePackageActivator(
        ::core::mem::transmute(riid),
        timeoutmilliseconds,
        callback.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<super::IFabricAsyncOperationContext>(result__)
}
#[inline]
pub unsafe fn FabricBeginGetNodeContext<'a, P0>(
    timeoutmilliseconds: u32,
    callback: P0,
) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricBeginGetNodeContext(
            timeoutmilliseconds: u32,
            callback: *mut ::core::ffi::c_void,
            context: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricBeginGetNodeContext(
        timeoutmilliseconds,
        callback.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<super::IFabricAsyncOperationContext>(result__)
}
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica<'a, P0, P1>(
    riid: *const ::windows::core::GUID,
    storename: P0,
    partitionid: ::windows::core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStoreEventHandler>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateKeyValueStoreReplica(
            riid: *const ::windows::core::GUID,
            storename: ::windows::core::PCWSTR,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateKeyValueStoreReplica(
        ::core::mem::transmute(riid),
        storename.into(),
        ::core::mem::transmute(partitionid),
        replicaid,
        ::core::mem::transmute(replicatorsettings),
        localstorekind,
        ::core::mem::transmute(localstoresettings),
        storeeventhandler.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica2<'a, P0, P1, P2>(
    riid: *const ::windows::core::GUID,
    storename: P0,
    partitionid: ::windows::core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStoreEventHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricSecondaryEventHandler>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateKeyValueStoreReplica2(
            riid: *const ::windows::core::GUID,
            storename: ::windows::core::PCWSTR,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateKeyValueStoreReplica2(
        ::core::mem::transmute(riid),
        storename.into(),
        ::core::mem::transmute(partitionid),
        replicaid,
        ::core::mem::transmute(replicatorsettings),
        localstorekind,
        ::core::mem::transmute(localstoresettings),
        storeeventhandler.into().abi(),
        secondaryeventhandler.into().abi(),
        notificationmode,
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica3<'a, P0, P1, P2>(
    riid: *const ::windows::core::GUID,
    storename: P0,
    partitionid: ::windows::core::GUID,
    replicaid: i64,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStoreEventHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricSecondaryEventHandler>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateKeyValueStoreReplica3(
            riid: *const ::windows::core::GUID,
            storename: ::windows::core::PCWSTR,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateKeyValueStoreReplica3(
        ::core::mem::transmute(riid),
        storename.into(),
        ::core::mem::transmute(partitionid),
        replicaid,
        ::core::mem::transmute(replicatorsettings),
        localstorekind,
        ::core::mem::transmute(localstoresettings),
        storeeventhandler.into().abi(),
        secondaryeventhandler.into().abi(),
        notificationmode,
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica4<'a, P0, P1, P2>(
    riid: *const ::windows::core::GUID,
    storename: P0,
    partitionid: ::windows::core::GUID,
    replicaid: i64,
    servicename: *const u16,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
    notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStoreEventHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricSecondaryEventHandler>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateKeyValueStoreReplica4(
            riid: *const ::windows::core::GUID,
            storename: ::windows::core::PCWSTR,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            servicename: *const u16,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            notificationmode: super::super::FABRIC_KEY_VALUE_STORE_NOTIFICATION_MODE,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateKeyValueStoreReplica4(
        ::core::mem::transmute(riid),
        storename.into(),
        ::core::mem::transmute(partitionid),
        replicaid,
        ::core::mem::transmute(servicename),
        ::core::mem::transmute(replicatorsettings),
        localstorekind,
        ::core::mem::transmute(localstoresettings),
        storeeventhandler.into().abi(),
        secondaryeventhandler.into().abi(),
        notificationmode,
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricCreateKeyValueStoreReplica5<'a, P0, P1, P2>(
    riid: *const ::windows::core::GUID,
    storename: P0,
    partitionid: ::windows::core::GUID,
    replicaid: i64,
    servicename: *const u16,
    replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    kvssettings: *const super::super::FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS,
    localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
    localstoresettings: *const ::core::ffi::c_void,
    storeeventhandler: P1,
    secondaryeventhandler: P2,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStoreEventHandler>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricSecondaryEventHandler>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateKeyValueStoreReplica5(
            riid: *const ::windows::core::GUID,
            storename: ::windows::core::PCWSTR,
            partitionid: ::windows::core::GUID,
            replicaid: i64,
            servicename: *const u16,
            replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
            kvssettings: *const super::super::FABRIC_KEY_VALUE_STORE_REPLICA_SETTINGS,
            localstorekind: super::super::FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const ::core::ffi::c_void,
            storeeventhandler: *mut ::core::ffi::c_void,
            secondaryeventhandler: *mut ::core::ffi::c_void,
            keyvaluestore: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateKeyValueStoreReplica5(
        ::core::mem::transmute(riid),
        storename.into(),
        ::core::mem::transmute(partitionid),
        replicaid,
        ::core::mem::transmute(servicename),
        ::core::mem::transmute(replicatorsettings),
        ::core::mem::transmute(kvssettings),
        localstorekind,
        ::core::mem::transmute(localstoresettings),
        storeeventhandler.into().abi(),
        secondaryeventhandler.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricCreateRuntime(
    riid: *const ::windows::core::GUID,
) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricCreateRuntime(
            riid: *const ::windows::core::GUID,
            fabricruntime: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricCreateRuntime(
        ::core::mem::transmute(riid),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricEndCreateRuntime<'a, P0>(
    context: P0,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricEndCreateRuntime(
            context: *mut ::core::ffi::c_void,
            fabricruntime: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricEndCreateRuntime(
        context.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricEndGetActivationContext<'a, P0>(
    context: P0,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricEndGetActivationContext(
            context: *mut ::core::ffi::c_void,
            activationcontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricEndGetActivationContext(
        context.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricEndGetCodePackageActivator<'a, P0>(
    context: P0,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricEndGetCodePackageActivator(
            context: *mut ::core::ffi::c_void,
            activator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricEndGetCodePackageActivator(
        context.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricEndGetNodeContext<'a, P0>(
    context: P0,
) -> ::windows::core::Result<*mut ::core::ffi::c_void>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricEndGetNodeContext(
            context: *mut ::core::ffi::c_void,
            nodecontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricEndGetNodeContext(
        context.into().abi(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricGetActivationContext(
    riid: *const ::windows::core::GUID,
) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricGetActivationContext(
            riid: *const ::windows::core::GUID,
            activationcontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricGetActivationContext(
        ::core::mem::transmute(riid),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricGetCodePackageActivator(
    riid: *const ::windows::core::GUID,
) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricGetCodePackageActivator(
            riid: *const ::windows::core::GUID,
            activator: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricGetCodePackageActivator(
        ::core::mem::transmute(riid),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricGetNodeContext() -> ::windows::core::Result<*mut ::core::ffi::c_void> {
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricGetNodeContext(
            nodecontext: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricGetNodeContext(::core::mem::transmute(result__.as_mut_ptr()))
        .from_abi::<*mut ::core::ffi::c_void>(result__)
}
#[inline]
pub unsafe fn FabricLoadEseLocalStoreSettings<'a, P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows::core::Result<IFabricEseLocalStoreSettingsResult>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricLoadEseLocalStoreSettings(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows::core::PCWSTR,
            sectionname: ::windows::core::PCWSTR,
            settings: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricLoadEseLocalStoreSettings(
        codepackageactivationcontext.into().abi(),
        configurationpackagename.into(),
        sectionname.into(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<IFabricEseLocalStoreSettingsResult>(result__)
}
#[inline]
pub unsafe fn FabricLoadReplicatorSettings<'a, P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows::core::Result<IFabricReplicatorSettingsResult>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricLoadReplicatorSettings(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows::core::PCWSTR,
            sectionname: ::windows::core::PCWSTR,
            replicatorsettings: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricLoadReplicatorSettings(
        codepackageactivationcontext.into().abi(),
        configurationpackagename.into(),
        sectionname.into(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<IFabricReplicatorSettingsResult>(result__)
}
#[inline]
pub unsafe fn FabricLoadSecurityCredentials<'a, P0, P1, P2>(
    codepackageactivationcontext: P0,
    configurationpackagename: P1,
    sectionname: P2,
) -> ::windows::core::Result<IFabricSecurityCredentialsResult>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[link(name = "fabricruntime")]
    extern "system" {
        fn FabricLoadSecurityCredentials(
            codepackageactivationcontext: *mut ::core::ffi::c_void,
            configurationpackagename: ::windows::core::PCWSTR,
            sectionname: ::windows::core::PCWSTR,
            securitycredentials: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FabricLoadSecurityCredentials(
        codepackageactivationcontext.into().abi(),
        configurationpackagename.into(),
        sectionname.into(),
        ::core::mem::transmute(result__.as_mut_ptr()),
    )
    .from_abi::<IFabricSecurityCredentialsResult>(result__)
}
#[repr(transparent)]
pub struct IFabricAtomicGroupStateProvider(::windows::core::IUnknown);
impl IFabricAtomicGroupStateProvider {
    pub unsafe fn BeginAtomicGroupCommit<'a, P0>(
        &self,
        atomicgroupid: i64,
        commitsequencenumber: i64,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginAtomicGroupCommit)(
            ::windows::core::Vtable::as_raw(self),
            atomicgroupid,
            commitsequencenumber,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndAtomicGroupCommit<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndAtomicGroupCommit)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginAtomicGroupRollback<'a, P0>(
        &self,
        atomicgroupid: i64,
        rollbackequencenumber: i64,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginAtomicGroupRollback)(
            ::windows::core::Vtable::as_raw(self),
            atomicgroupid,
            rollbackequencenumber,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndAtomicGroupRollback<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndAtomicGroupRollback)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUndoProgress<'a, P0>(
        &self,
        fromcommitsequencenumber: i64,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginUndoProgress)(
            ::windows::core::Vtable::as_raw(self),
            fromcommitsequencenumber,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndUndoProgress<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndUndoProgress)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricAtomicGroupStateProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricAtomicGroupStateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricAtomicGroupStateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricAtomicGroupStateProvider {}
impl ::core::fmt::Debug for IFabricAtomicGroupStateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricAtomicGroupStateProvider")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricAtomicGroupStateProvider {
    type Vtable = IFabricAtomicGroupStateProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricAtomicGroupStateProvider {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2b670953_6148_4f7d_a920_b390de43d913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAtomicGroupStateProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginAtomicGroupCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: i64,
        commitsequencenumber: i64,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndAtomicGroupCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginAtomicGroupRollback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: i64,
        rollbackequencenumber: i64,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndAtomicGroupRollback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginUndoProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fromcommitsequencenumber: i64,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndUndoProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricAtomicGroupStateReplicator(::windows::core::IUnknown);
impl IFabricAtomicGroupStateReplicator {
    pub unsafe fn CreateAtomicGroup(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAtomicGroup)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupOperation<'a, P0, P1>(
        &self,
        atomicgroupid: i64,
        operationdata: P0,
        callback: P1,
        operationsequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricOperationData>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        (::windows::core::Vtable::vtable(self).BeginReplicateAtomicGroupOperation)(
            ::windows::core::Vtable::as_raw(self),
            atomicgroupid,
            operationdata.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(operationsequencenumber),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    pub unsafe fn EndReplicateAtomicGroupOperation<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndReplicateAtomicGroupOperation)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupCommit<'a, P0>(
        &self,
        atomicgroupid: i64,
        callback: P0,
        commitsequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        (::windows::core::Vtable::vtable(self).BeginReplicateAtomicGroupCommit)(
            ::windows::core::Vtable::as_raw(self),
            atomicgroupid,
            callback.into().abi(),
            ::core::mem::transmute(commitsequencenumber),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    pub unsafe fn EndReplicateAtomicGroupCommit<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndReplicateAtomicGroupCommit)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn BeginReplicateAtomicGroupRollback<'a, P0>(
        &self,
        atomicgroupid: i64,
        callback: P0,
        rollbacksequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        (::windows::core::Vtable::vtable(self).BeginReplicateAtomicGroupRollback)(
            ::windows::core::Vtable::as_raw(self),
            atomicgroupid,
            callback.into().abi(),
            ::core::mem::transmute(rollbacksequencenumber),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    pub unsafe fn EndReplicateAtomicGroupRollback<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndReplicateAtomicGroupRollback)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricAtomicGroupStateReplicator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricAtomicGroupStateReplicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricAtomicGroupStateReplicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricAtomicGroupStateReplicator {}
impl ::core::fmt::Debug for IFabricAtomicGroupStateReplicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricAtomicGroupStateReplicator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricAtomicGroupStateReplicator {
    type Vtable = IFabricAtomicGroupStateReplicator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricAtomicGroupStateReplicator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x80d2155c_4fc2_4fde_9696_c2f39b471c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricAtomicGroupStateReplicator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateAtomicGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub BeginReplicateAtomicGroupOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: i64,
        operationdata: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        operationsequencenumber: *mut i64,
        context: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub EndReplicateAtomicGroupOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        operationsequencenumber: *mut i64,
    )
        -> ::windows::core::HRESULT,
    pub BeginReplicateAtomicGroupCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: i64,
        callback: *mut ::core::ffi::c_void,
        commitsequencenumber: *mut i64,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndReplicateAtomicGroupCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        commitsequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub BeginReplicateAtomicGroupRollback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        atomicgroupid: i64,
        callback: *mut ::core::ffi::c_void,
        rollbacksequencenumber: *mut i64,
        context: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub EndReplicateAtomicGroupRollback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        rollbacksequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackage(::windows::core::IUnknown);
impl IFabricCodePackage {
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_Description)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_Path(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_Path)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricCodePackage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricCodePackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackage {}
impl ::core::fmt::Debug for IFabricCodePackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackage {
    type Vtable = IFabricCodePackage_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackage {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x20792b45_4d13_41a4_af13_346e529f00c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Description:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION,
    pub get_Path:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
}
#[repr(transparent)]
pub struct IFabricCodePackage2(::windows::core::IUnknown);
impl IFabricCodePackage2 {
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_CODE_PACKAGE_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).base__.get_Description)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_Path(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).base__.get_Path)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_SetupEntryPointRunAsPolicy(
        &self,
    ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_SetupEntryPointRunAsPolicy)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_EntryPointRunAsPolicy(
        &self,
    ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_EntryPointRunAsPolicy)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackage2,
    ::windows::core::IUnknown,
    IFabricCodePackage
);
impl ::core::clone::Clone for IFabricCodePackage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackage2 {}
impl ::core::fmt::Debug for IFabricCodePackage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackage2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackage2 {
    type Vtable = IFabricCodePackage2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackage2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcdf0a4e6_ad80_4cd6_b67e_e4c002428600);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackage2_Vtbl {
    pub base__: IFabricCodePackage_Vtbl,
    pub get_SetupEntryPointRunAsPolicy:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION,
    pub get_EntryPointRunAsPolicy:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_RUNAS_POLICY_DESCRIPTION,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_ContextId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_CodePackageName)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_CodePackageVersion)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_WorkDirectory)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_LogDirectory)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_TempDirectory)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self).get_ServiceTypes)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self).get_ServiceGroupTypes)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_ApplicationPrincipals)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self).get_ServiceEndpointResources)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext {
    type Vtable = IFabricCodePackageActivationContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x68a971e2_f15f_4d95_a79c_8a257909659e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_ContextId:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_CodePackageName:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_CodePackageVersion:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_WorkDirectory:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_LogDirectory:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_TempDirectory:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_ServiceTypes:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST,
    pub get_ServiceGroupTypes:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST,
    pub get_ApplicationPrincipals:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION,
    pub get_ServiceEndpointResources:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST,
    pub GetServiceEndpointResource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        serviceendpointresourcename: ::windows::core::PCWSTR,
        bufferedvalue: *mut *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION,
    ) -> ::windows::core::HRESULT,
    pub GetCodePackageNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        names: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetConfigurationPackageNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        names: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetDataPackageNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        names: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        codepackagename: ::windows::core::PCWSTR,
        codepackage: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetConfigurationPackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        configpackagename: ::windows::core::PCWSTR,
        configpackage: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetDataPackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        datapackagename: ::windows::core::PCWSTR,
        datapackage: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RegisterCodePackageChangeHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        callbackhandle: *mut i64,
    )
        -> ::windows::core::HRESULT,
    pub UnregisterCodePackageChangeHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callbackhandle: i64,
    )
        -> ::windows::core::HRESULT,
    pub RegisterConfigurationPackageChangeHandler:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
            callbackhandle: *mut i64,
        ) -> ::windows::core::HRESULT,
    pub UnregisterConfigurationPackageChangeHandler:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            callbackhandle: i64,
        ) -> ::windows::core::HRESULT,
    pub RegisterDataPackageChangeHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        callbackhandle: *mut i64,
    )
        -> ::windows::core::HRESULT,
    pub UnregisterDataPackageChangeHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callbackhandle: i64,
    )
        -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext2(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext2 {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).base__.get_ContextId)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_CodePackageName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_CodePackageVersion)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_WorkDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_LogDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_TempDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ServiceTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ServiceGroupTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ApplicationPrincipals)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ServiceEndpointResources)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self).get_ApplicationName)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_ApplicationTypeName)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceManifestName)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceManifestVersion)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext2,
    ::windows::core::IUnknown,
    IFabricCodePackageActivationContext
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext2 {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext2 {
    type Vtable = IFabricCodePackageActivationContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6c83d5c1_1954_4b80_9175_0d0e7c8715c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext2_Vtbl {
    pub base__: IFabricCodePackageActivationContext_Vtbl,
    pub get_ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut u16,
    pub get_ApplicationTypeName:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub GetServiceManifestName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicemanifestname: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetServiceManifestVersion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicemanifestversion: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext3(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext3 {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ContextId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_CodePackageName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_CodePackageVersion)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_WorkDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_LogDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_TempDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ServiceTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ApplicationName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ApplicationTypeName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetServiceManifestName)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetServiceManifestVersion)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportDeployedApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportDeployedServicePackageHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext3,
    ::windows::core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext3 {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext3")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext3 {
    type Vtable = IFabricCodePackageActivationContext3_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext3 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6efee900_f491_4b03_bc5b_3a70de103593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext3_Vtbl {
    pub base__: IFabricCodePackageActivationContext2_Vtbl,
    pub ReportApplicationHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub ReportDeployedApplicationHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub ReportDeployedServicePackageHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    )
        -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext4(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext4 {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ContextId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ApplicationName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .get_ApplicationTypeName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetServiceManifestName)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportDeployedApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportDeployedServicePackageHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext4,
    ::windows::core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext4 {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext4")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext4 {
    type Vtable = IFabricCodePackageActivationContext4_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext4 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x99efebb6_a7b4_4d45_b45e_f191a66eef03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext4_Vtbl {
    pub base__: IFabricCodePackageActivationContext3_Vtbl,
    pub ReportApplicationHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::HRESULT,
    pub ReportDeployedApplicationHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    )
        -> ::windows::core::HRESULT,
    pub ReportDeployedServicePackageHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    )
        -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext5(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext5 {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ContextId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .get_ApplicationTypeName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceManifestName)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportDeployedApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportDeployedServicePackageHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn get_ServiceListenAddress(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_ServiceListenAddress)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_ServicePublishAddress(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_ServicePublishAddress)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext5,
    ::windows::core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3,
    IFabricCodePackageActivationContext4
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext5 {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext5")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext5 {
    type Vtable = IFabricCodePackageActivationContext5_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext5 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfe45387e_8711_4949_ac36_31dc95035513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext5_Vtbl {
    pub base__: IFabricCodePackageActivationContext4_Vtbl,
    pub get_ServiceListenAddress:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_ServicePublishAddress:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivationContext6(::windows::core::IUnknown);
impl IFabricCodePackageActivationContext6 {
    pub unsafe fn get_ContextId(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ContextId)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_CodePackageVersion(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_CodePackageVersion)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_WorkDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_WorkDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_LogDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_LogDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_TempDirectory(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_TempDirectory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceGroupTypes(
        &self,
    ) -> *mut super::super::FABRIC_SERVICE_GROUP_TYPE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceGroupTypes)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationPrincipals(
        &self,
    ) -> *mut super::super::FABRIC_APPLICATION_PRINCIPALS_DESCRIPTION {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationPrincipals)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServiceEndpointResources(
        &self,
    ) -> *mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION_LIST {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .get_ServiceEndpointResources)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceEndpointResource<'a, P0>(
        &self,
        serviceendpointresourcename: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetServiceEndpointResource)(
            ::windows::core::Vtable::as_raw(self),
            serviceendpointresourcename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_ENDPOINT_RESOURCE_DESCRIPTION>(result__)
    }
    pub unsafe fn GetCodePackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCodePackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetConfigurationPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetDataPackageNames(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringListResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetDataPackageNames)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringListResult>(result__)
    }
    pub unsafe fn GetCodePackage<'a, P0>(
        &self,
        codepackagename: P0,
    ) -> ::windows::core::Result<IFabricCodePackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            codepackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricCodePackage>(result__)
    }
    pub unsafe fn GetConfigurationPackage<'a, P0>(
        &self,
        configpackagename: P0,
    ) -> ::windows::core::Result<IFabricConfigurationPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetConfigurationPackage)(
            ::windows::core::Vtable::as_raw(self),
            configpackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricConfigurationPackage>(result__)
    }
    pub unsafe fn GetDataPackage<'a, P0>(
        &self,
        datapackagename: P0,
    ) -> ::windows::core::Result<IFabricDataPackage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetDataPackage)(
            ::windows::core::Vtable::as_raw(self),
            datapackagename.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricDataPackage>(result__)
    }
    pub unsafe fn RegisterCodePackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCodePackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterCodePackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterConfigurationPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricConfigurationPackageChangeHandler>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterConfigurationPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterConfigurationPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn RegisterDataPackageChangeHandler<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackageChangeHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .RegisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterDataPackageChangeHandler(
        &self,
        callbackhandle: i64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UnregisterDataPackageChangeHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
    pub unsafe fn get_ApplicationName(&self) -> *mut u16 {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ApplicationTypeName(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .get_ApplicationTypeName)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetServiceManifestName(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceManifestName)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetServiceManifestVersion(
        &self,
    ) -> ::windows::core::Result<super::IFabricStringResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetServiceManifestVersion)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn ReportApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportDeployedApplicationHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportDeployedServicePackageHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedApplicationHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportDeployedApplicationHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportDeployedServicePackageHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportDeployedServicePackageHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn get_ServiceListenAddress(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ServiceListenAddress)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_ServicePublishAddress(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_ServicePublishAddress)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDirectory<'a, P0>(
        &self,
        logicaldirectoryname: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDirectory)(
            ::windows::core::Vtable::as_raw(self),
            logicaldirectoryname.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricCodePackageActivationContext6,
    ::windows::core::IUnknown,
    IFabricCodePackageActivationContext,
    IFabricCodePackageActivationContext2,
    IFabricCodePackageActivationContext3,
    IFabricCodePackageActivationContext4,
    IFabricCodePackageActivationContext5
);
impl ::core::clone::Clone for IFabricCodePackageActivationContext6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivationContext6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivationContext6 {}
impl ::core::fmt::Debug for IFabricCodePackageActivationContext6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivationContext6")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivationContext6 {
    type Vtable = IFabricCodePackageActivationContext6_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivationContext6 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa5fda9b_472c_45a0_9b60_a374691227a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivationContext6_Vtbl {
    pub base__: IFabricCodePackageActivationContext5_Vtbl,
    pub GetDirectory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        logicaldirectoryname: ::windows::core::PCWSTR,
        directorypath: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageActivator(::windows::core::IUnknown);
impl IFabricCodePackageActivator {
    pub unsafe fn BeginActivateCodePackage<'a, P0>(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        environment: *const super::super::FABRIC_STRING_MAP,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginActivateCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(codepackagenames),
            ::core::mem::transmute(environment),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndActivateCodePackage<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndActivateCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginDeactivateCodePackage<'a, P0>(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        timeoutmilliseconds: u32,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginDeactivateCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(codepackagenames),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndDeactivateCodePackage<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndDeactivateCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn AbortCodePackage(
        &self,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AbortCodePackage)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(codepackagenames),
        )
        .ok()
    }
    pub unsafe fn RegisterCodePackageEventHandler<'a, P0>(
        &self,
        eventhandler: P0,
    ) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageEventHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterCodePackageEventHandler)(
            ::windows::core::Vtable::as_raw(self),
            eventhandler.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u64>(result__)
    }
    pub unsafe fn UnregisterCodePackageEventHandler(
        &self,
        callbackhandle: u64,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterCodePackageEventHandler)(
            ::windows::core::Vtable::as_raw(self),
            callbackhandle,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricCodePackageActivator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricCodePackageActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageActivator {}
impl ::core::fmt::Debug for IFabricCodePackageActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageActivator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageActivator {
    type Vtable = IFabricCodePackageActivator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageActivator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x70be1b10_b259_46fc_b813_0b75720e7183);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageActivator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginActivateCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        environment: *const super::super::FABRIC_STRING_MAP,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndActivateCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginDeactivateCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndDeactivateCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AbortCodePackage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        codepackagenames: *const super::super::FABRIC_STRING_LIST,
    ) -> ::windows::core::HRESULT,
    pub RegisterCodePackageEventHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventhandler: *mut ::core::ffi::c_void,
        callbackhandle: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub UnregisterCodePackageEventHandler: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callbackhandle: u64,
    )
        -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricCodePackageChangeHandler(::windows::core::IUnknown);
impl IFabricCodePackageChangeHandler {
    pub unsafe fn OnPackageAdded<'a, P0, P1>(&self, source: P0, codepackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageAdded)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            codepackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<'a, P0, P1>(&self, source: P0, codepackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageRemoved)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            codepackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageModified<'a, P0, P1, P2>(
        &self,
        source: P0,
        previouscodepackage: P1,
        codepackage: P2,
    ) where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackage>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageModified)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            previouscodepackage.into().abi(),
            codepackage.into().abi(),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricCodePackageChangeHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricCodePackageChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageChangeHandler {}
impl ::core::fmt::Debug for IFabricCodePackageChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageChangeHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageChangeHandler {
    type Vtable = IFabricCodePackageChangeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageChangeHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb90d36cd_acb5_427a_b318_3b045981d0cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageChangeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        codepackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        codepackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        previouscodepackage: *mut ::core::ffi::c_void,
        codepackage: *mut ::core::ffi::c_void,
    ),
}
#[repr(transparent)]
pub struct IFabricCodePackageEventHandler(::windows::core::IUnknown);
impl IFabricCodePackageEventHandler {
    pub unsafe fn OnCodePackageEvent<'a, P0>(
        &self,
        source: P0,
        eventdesc: *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
    ) where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivator>>,
    {
        (::windows::core::Vtable::vtable(self).OnCodePackageEvent)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            ::core::mem::transmute(eventdesc),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricCodePackageEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricCodePackageEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricCodePackageEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricCodePackageEventHandler {}
impl ::core::fmt::Debug for IFabricCodePackageEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricCodePackageEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricCodePackageEventHandler {
    type Vtable = IFabricCodePackageEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricCodePackageEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x899e0ca8_16df_458e_8915_d0307b4ab101);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricCodePackageEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCodePackageEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        eventdesc: *const super::super::FABRIC_CODE_PACKAGE_EVENT_DESCRIPTION,
    ),
}
#[repr(transparent)]
pub struct IFabricConfigurationPackage(::windows::core::IUnknown);
impl IFabricConfigurationPackage {
    pub unsafe fn get_Description(
        &self,
    ) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_Description)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_Path(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_Path)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS {
        (::windows::core::Vtable::vtable(self).get_Settings)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSection<'a, P0>(
        &self,
        sectionname: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_CONFIGURATION_SECTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSection)(
            ::windows::core::Vtable::as_raw(self),
            sectionname.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_CONFIGURATION_SECTION>(result__)
    }
    pub unsafe fn GetValue<'a, P0, P1>(
        &self,
        sectionname: P0,
        parametername: P1,
        isencrypted: *mut u8,
        bufferedvalue: *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).GetValue)(
            ::windows::core::Vtable::as_raw(self),
            sectionname.into(),
            parametername.into(),
            ::core::mem::transmute(isencrypted),
            ::core::mem::transmute(bufferedvalue),
        )
        .ok()
    }
    pub unsafe fn DecryptValue<'a, P0>(
        &self,
        encryptedvalue: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DecryptValue)(
            ::windows::core::Vtable::as_raw(self),
            encryptedvalue.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricConfigurationPackage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricConfigurationPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricConfigurationPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricConfigurationPackage {}
impl ::core::fmt::Debug for IFabricConfigurationPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricConfigurationPackage")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricConfigurationPackage {
    type Vtable = IFabricConfigurationPackage_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricConfigurationPackage {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac4c3bfa_2563_46b7_a71d_2dca7b0a8f4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricConfigurationPackage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Description:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION,
    pub get_Path:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub get_Settings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    )
        -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS,
    pub GetSection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sectionname: ::windows::core::PCWSTR,
        bufferedvalue: *mut *mut super::super::FABRIC_CONFIGURATION_SECTION,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sectionname: ::windows::core::PCWSTR,
        parametername: ::windows::core::PCWSTR,
        isencrypted: *mut u8,
        bufferedvalue: *mut ::windows::core::PWSTR,
    ) -> ::windows::core::HRESULT,
    pub DecryptValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        encryptedvalue: ::windows::core::PCWSTR,
        decryptedvalue: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricConfigurationPackage2(::windows::core::IUnknown);
impl IFabricConfigurationPackage2 {
    pub unsafe fn get_Description(
        &self,
    ) -> *mut super::super::FABRIC_CONFIGURATION_PACKAGE_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).base__.get_Description)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn get_Path(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).base__.get_Path)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_CONFIGURATION_SETTINGS {
        (::windows::core::Vtable::vtable(self).base__.get_Settings)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn GetSection<'a, P0>(
        &self,
        sectionname: P0,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_CONFIGURATION_SECTION>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSection)(
            ::windows::core::Vtable::as_raw(self),
            sectionname.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_CONFIGURATION_SECTION>(result__)
    }
    pub unsafe fn GetValue<'a, P0, P1>(
        &self,
        sectionname: P0,
        parametername: P1,
        isencrypted: *mut u8,
        bufferedvalue: *mut ::windows::core::PWSTR,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetValue)(
            ::windows::core::Vtable::as_raw(self),
            sectionname.into(),
            parametername.into(),
            ::core::mem::transmute(isencrypted),
            ::core::mem::transmute(bufferedvalue),
        )
        .ok()
    }
    pub unsafe fn DecryptValue<'a, P0>(
        &self,
        encryptedvalue: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DecryptValue)(
            ::windows::core::Vtable::as_raw(self),
            encryptedvalue.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn GetValues<'a, P0, P1>(
        &self,
        sectionname: P0,
        parameterprefix: P1,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetValues)(
            ::windows::core::Vtable::as_raw(self),
            sectionname.into(),
            parameterprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricConfigurationPackage2,
    ::windows::core::IUnknown,
    IFabricConfigurationPackage
);
impl ::core::clone::Clone for IFabricConfigurationPackage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricConfigurationPackage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricConfigurationPackage2 {}
impl ::core::fmt::Debug for IFabricConfigurationPackage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricConfigurationPackage2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricConfigurationPackage2 {
    type Vtable = IFabricConfigurationPackage2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricConfigurationPackage2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd3161f31_708a_4f83_91ff_f2af15f74a2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricConfigurationPackage2_Vtbl {
    pub base__: IFabricConfigurationPackage_Vtbl,
    pub GetValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sectionname: ::windows::core::PCWSTR,
        parameterprefix: ::windows::core::PCWSTR,
        bufferedvalue: *mut *mut super::super::FABRIC_CONFIGURATION_PARAMETER_LIST,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricConfigurationPackageChangeHandler(::windows::core::IUnknown);
impl IFabricConfigurationPackageChangeHandler {
    pub unsafe fn OnPackageAdded<'a, P0, P1>(&self, source: P0, configpackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricConfigurationPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageAdded)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            configpackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<'a, P0, P1>(&self, source: P0, configpackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricConfigurationPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageRemoved)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            configpackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageModified<'a, P0, P1, P2>(
        &self,
        source: P0,
        previousconfigpackage: P1,
        configpackage: P2,
    ) where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricConfigurationPackage>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricConfigurationPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageModified)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            previousconfigpackage.into().abi(),
            configpackage.into().abi(),
        )
    }
}
::windows::core::interface_hierarchy!(
    IFabricConfigurationPackageChangeHandler,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricConfigurationPackageChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricConfigurationPackageChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricConfigurationPackageChangeHandler {}
impl ::core::fmt::Debug for IFabricConfigurationPackageChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricConfigurationPackageChangeHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricConfigurationPackageChangeHandler {
    type Vtable = IFabricConfigurationPackageChangeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricConfigurationPackageChangeHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc3954d48_b5ee_4ff4_9bc0_c30f6d0d3a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricConfigurationPackageChangeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        configpackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        configpackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        previousconfigpackage: *mut ::core::ffi::c_void,
        configpackage: *mut ::core::ffi::c_void,
    ),
}
#[repr(transparent)]
pub struct IFabricDataPackage(::windows::core::IUnknown);
impl IFabricDataPackage {
    pub unsafe fn get_Description(&self) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION {
        (::windows::core::Vtable::vtable(self).get_Description)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn get_Path(&self) -> ::windows::core::PWSTR {
        (::windows::core::Vtable::vtable(self).get_Path)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricDataPackage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricDataPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricDataPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricDataPackage {}
impl ::core::fmt::Debug for IFabricDataPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricDataPackage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricDataPackage {
    type Vtable = IFabricDataPackage_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricDataPackage {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xaa67de09_3657_435f_a2f6_b3a17a0a4371);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricDataPackage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Description:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_DATA_PACKAGE_DESCRIPTION,
    pub get_Path:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
}
#[repr(transparent)]
pub struct IFabricDataPackageChangeHandler(::windows::core::IUnknown);
impl IFabricDataPackageChangeHandler {
    pub unsafe fn OnPackageAdded<'a, P0, P1>(&self, source: P0, datapackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageAdded)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            datapackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageRemoved<'a, P0, P1>(&self, source: P0, datapackage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageRemoved)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            datapackage.into().abi(),
        )
    }
    pub unsafe fn OnPackageModified<'a, P0, P1, P2>(
        &self,
        source: P0,
        previousdatapackage: P1,
        datapackage: P2,
    ) where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricCodePackageActivationContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackage>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IFabricDataPackage>>,
    {
        (::windows::core::Vtable::vtable(self).OnPackageModified)(
            ::windows::core::Vtable::as_raw(self),
            source.into().abi(),
            previousdatapackage.into().abi(),
            datapackage.into().abi(),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricDataPackageChangeHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricDataPackageChangeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricDataPackageChangeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricDataPackageChangeHandler {}
impl ::core::fmt::Debug for IFabricDataPackageChangeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricDataPackageChangeHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricDataPackageChangeHandler {
    type Vtable = IFabricDataPackageChangeHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricDataPackageChangeHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8d0a726f_bd17_4b32_807b_be2a8024b2e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricDataPackageChangeHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnPackageAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        datapackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        datapackage: *mut ::core::ffi::c_void,
    ),
    pub OnPackageModified: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
        previousdatapackage: *mut ::core::ffi::c_void,
        datapackage: *mut ::core::ffi::c_void,
    ),
}
#[repr(transparent)]
pub struct IFabricEseLocalStoreSettingsResult(::windows::core::IUnknown);
impl IFabricEseLocalStoreSettingsResult {
    pub unsafe fn get_Settings(&self) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS {
        (::windows::core::Vtable::vtable(self).get_Settings)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricEseLocalStoreSettingsResult,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricEseLocalStoreSettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricEseLocalStoreSettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricEseLocalStoreSettingsResult {}
impl ::core::fmt::Debug for IFabricEseLocalStoreSettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricEseLocalStoreSettingsResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricEseLocalStoreSettingsResult {
    type Vtable = IFabricEseLocalStoreSettingsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricEseLocalStoreSettingsResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xaace77ae_d8e1_4144_b1ee_5ac74fd54f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricEseLocalStoreSettingsResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Settings:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_ESE_LOCAL_STORE_SETTINGS,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreEnumerator(::windows::core::IUnknown);
impl IFabricKeyValueStoreEnumerator {
    pub unsafe fn EnumerateByKey<'a, P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricKeyValueStoreEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricKeyValueStoreEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreEnumerator {}
impl ::core::fmt::Debug for IFabricKeyValueStoreEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreEnumerator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreEnumerator {
    type Vtable = IFabricKeyValueStoreEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreEnumerator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6722b848_15bb_4528_bf54_c7bbe27b6f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumerateByKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateMetadataByKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreEnumerator2(::windows::core::IUnknown);
impl IFabricKeyValueStoreEnumerator2 {
    pub unsafe fn EnumerateByKey<'a, P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0>(
        &self,
        keyprefix: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey2<'a, P0, P1>(
        &self,
        keyprefix: P0,
        strictprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateByKey2)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey2<'a, P0, P1>(
        &self,
        keyprefix: P0,
        strictprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateMetadataByKey2)(
            ::windows::core::Vtable::as_raw(self),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreEnumerator2,
    ::windows::core::IUnknown,
    IFabricKeyValueStoreEnumerator
);
impl ::core::clone::Clone for IFabricKeyValueStoreEnumerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreEnumerator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreEnumerator2 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreEnumerator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreEnumerator2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreEnumerator2 {
    type Vtable = IFabricKeyValueStoreEnumerator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreEnumerator2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x63dfd264_4f2b_4be6_8234_1fa200165fe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreEnumerator_Vtbl,
    pub EnumerateByKey2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateMetadataByKey2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemEnumerator(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemResult> {
        (::windows::core::Vtable::vtable(self).get_Current)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreItemEnumerator,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricKeyValueStoreItemEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemEnumerator {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemEnumerator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemEnumerator {
    type Vtable = IFabricKeyValueStoreItemEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemEnumerator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc202788f_54d3_44a6_8f3c_b4bbfcdb95d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MoveNext:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreItemResult,
    >,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemEnumerator2(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveNext)(::windows::core::Vtable::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreItemResult> {
        (::windows::core::Vtable::vtable(self).base__.get_Current)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryMoveNext)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreItemEnumerator2,
    ::windows::core::IUnknown,
    IFabricKeyValueStoreItemEnumerator
);
impl ::core::clone::Clone for IFabricKeyValueStoreItemEnumerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemEnumerator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemEnumerator2 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemEnumerator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemEnumerator2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemEnumerator2 {
    type Vtable = IFabricKeyValueStoreItemEnumerator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemEnumerator2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xda143bbc_81e1_48cd_afd7_b642bc5b9bfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreItemEnumerator_Vtbl,
    pub TryMoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        success: *mut u8,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemMetadataEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn get_Current(
        &self,
    ) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult> {
        (::windows::core::Vtable::vtable(self).get_Current)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataEnumerator,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricKeyValueStoreItemMetadataEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemMetadataEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemMetadataEnumerator {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemMetadataEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemMetadataEnumerator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemMetadataEnumerator {
    type Vtable = IFabricKeyValueStoreItemMetadataEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemMetadataEnumerator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0bc06aee_fffa_4450_9099_116a5f0e0b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MoveNext:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreItemMetadataResult,
    >,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator2(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemMetadataEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveNext)(::windows::core::Vtable::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn get_Current(
        &self,
    ) -> ::core::option::Option<IFabricKeyValueStoreItemMetadataResult> {
        (::windows::core::Vtable::vtable(self).base__.get_Current)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryMoveNext)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataEnumerator2,
    ::windows::core::IUnknown,
    IFabricKeyValueStoreItemMetadataEnumerator
);
impl ::core::clone::Clone for IFabricKeyValueStoreItemMetadataEnumerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemMetadataEnumerator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemMetadataEnumerator2 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemMetadataEnumerator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemMetadataEnumerator2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemMetadataEnumerator2 {
    type Vtable = IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemMetadataEnumerator2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8803d53e_dd73_40fc_a662_1bfe999419ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemMetadataEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreItemMetadataEnumerator_Vtbl,
    pub TryMoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        success: *mut u8,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemMetadataResult(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemMetadataResult {
    pub unsafe fn get_Metadata(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA {
        (::windows::core::Vtable::vtable(self).get_Metadata)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreItemMetadataResult,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricKeyValueStoreItemMetadataResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemMetadataResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemMetadataResult {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemMetadataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemMetadataResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemMetadataResult {
    type Vtable = IFabricKeyValueStoreItemMetadataResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemMetadataResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17c483a1_69e6_4bdc_a058_54fd4a1839fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemMetadataResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Metadata:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        )
            -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM_METADATA,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreItemResult(::windows::core::IUnknown);
impl IFabricKeyValueStoreItemResult {
    pub unsafe fn get_Item(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM {
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricKeyValueStoreItemResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricKeyValueStoreItemResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreItemResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreItemResult {}
impl ::core::fmt::Debug for IFabricKeyValueStoreItemResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreItemResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreItemResult {
    type Vtable = IFabricKeyValueStoreItemResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreItemResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc1f1c89d_b0b8_44dc_bc97_6c074c1a805e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreItemResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Item: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreNotification(::windows::core::IUnknown);
impl IFabricKeyValueStoreNotification {
    pub unsafe fn get_Item(&self) -> *mut super::super::FABRIC_KEY_VALUE_STORE_ITEM {
        (::windows::core::Vtable::vtable(self).base__.get_Item)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn IsDelete(&self) -> ::windows::Win32::Foundation::BOOLEAN {
        (::windows::core::Vtable::vtable(self).IsDelete)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreNotification,
    ::windows::core::IUnknown,
    IFabricKeyValueStoreItemResult
);
impl ::core::clone::Clone for IFabricKeyValueStoreNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreNotification {}
impl ::core::fmt::Debug for IFabricKeyValueStoreNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreNotification")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreNotification {
    type Vtable = IFabricKeyValueStoreNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreNotification {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcb660aa6_c51e_4f05_9526_93982b550e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreNotification_Vtbl {
    pub base__: IFabricKeyValueStoreItemResult_Vtbl,
    pub IsDelete: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::BOOLEAN,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreNotificationEnumerator(::windows::core::IUnknown);
impl IFabricKeyValueStoreNotificationEnumerator {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreNotification> {
        (::windows::core::Vtable::vtable(self).get_Current)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Reset(&self) {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreNotificationEnumerator,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricKeyValueStoreNotificationEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreNotificationEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreNotificationEnumerator {}
impl ::core::fmt::Debug for IFabricKeyValueStoreNotificationEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreNotificationEnumerator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreNotificationEnumerator {
    type Vtable = IFabricKeyValueStoreNotificationEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreNotificationEnumerator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xef25bc08_be76_43c7_adad_20f01fba3399);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreNotificationEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MoveNext:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<
        IFabricKeyValueStoreNotification,
    >,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreNotificationEnumerator2(::windows::core::IUnknown);
impl IFabricKeyValueStoreNotificationEnumerator2 {
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MoveNext)(::windows::core::Vtable::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn get_Current(&self) -> ::core::option::Option<IFabricKeyValueStoreNotification> {
        (::windows::core::Vtable::vtable(self).base__.get_Current)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn Reset(&self) {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn TryMoveNext(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryMoveNext)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreNotificationEnumerator2,
    ::windows::core::IUnknown,
    IFabricKeyValueStoreNotificationEnumerator
);
impl ::core::clone::Clone for IFabricKeyValueStoreNotificationEnumerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreNotificationEnumerator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreNotificationEnumerator2 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreNotificationEnumerator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreNotificationEnumerator2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreNotificationEnumerator2 {
    type Vtable = IFabricKeyValueStoreNotificationEnumerator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreNotificationEnumerator2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55eec7c6_ae81_407a_b84c_22771d314ac7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreNotificationEnumerator2_Vtbl {
    pub base__: IFabricKeyValueStoreNotificationEnumerator_Vtbl,
    pub TryMoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        success: *mut u8,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self).base__.BeginClose)(
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
        (::windows::core::Vtable::vtable(self).base__.EndClose)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self).base__.Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica {
    type Vtable = IFabricKeyValueStoreReplica_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x97da35c4_38ed_4a2a_8f37_fbeb56382235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica_Vtbl {
    pub base__: IFabricStatefulServiceReplica_Vtbl,
    pub GetCurrentEpoch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        currentepoch: *mut super::super::FABRIC_EPOCH,
    ) -> ::windows::core::HRESULT,
    pub UpdateReplicatorSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::HRESULT,
    pub CreateTransaction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
    ) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        checksequencenumber: i64,
    ) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
        checksequencenumber: i64,
    ) -> ::windows::core::HRESULT,
    pub Get: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetMetadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        result: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub Enumerate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateByKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateMetadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateMetadataByKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica2(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica2 {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .BeginClose)(
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
        (::windows::core::Vtable::vtable(self).base__.base__.EndClose)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.Abort)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn Backup<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Backup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn Restore<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Restore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTransaction2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(settings),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica2,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica2 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica2 {
    type Vtable = IFabricKeyValueStoreReplica2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfef805b2_5aca_4caa_9c51_fb3bd577a792);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica2_Vtbl {
    pub base__: IFabricKeyValueStoreReplica_Vtbl,
    pub Backup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        backupdirectory: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        backupdirectory: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
    pub CreateTransaction2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
        transaction: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica3(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica3 {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EndClose)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn Backup<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Backup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn Restore<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Restore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .CreateTransaction2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(settings),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn BeginBackup<'a, P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStorePostBackupHandler>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginBackup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            backupoption,
            postbackuphandler.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndBackup<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndBackup)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica3,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica3 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica3")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica3 {
    type Vtable = IFabricKeyValueStoreReplica3_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica3 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc1297172_a8aa_4096_bdcc_1ece0c5d8c8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica3_Vtbl {
    pub base__: IFabricKeyValueStoreReplica2_Vtbl,
    pub BeginBackup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        backupdirectory: ::windows::core::PCWSTR,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndBackup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica4(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica4 {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EndClose)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn Backup<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Backup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn Restore<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Restore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .CreateTransaction2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(settings),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn BeginBackup<'a, P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStorePostBackupHandler>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginBackup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            backupoption,
            postbackuphandler.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndBackup<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndBackup)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRestore<'a, P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRestore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRestore<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndRestore)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica4,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica4 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica4")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica4 {
    type Vtable = IFabricKeyValueStoreReplica4_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica4 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xff16d2f1_41a9_4c64_804a_a20bf28c04f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica4_Vtbl {
    pub base__: IFabricKeyValueStoreReplica3_Vtbl,
    pub BeginRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        backupdirectory: ::windows::core::PCWSTR,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndRestore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica5(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica5 {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndClose)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn Backup<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Backup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn Restore<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .Restore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .CreateTransaction2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(settings),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn BeginBackup<'a, P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStorePostBackupHandler>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .BeginBackup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            backupoption,
            postbackuphandler.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndBackup<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EndBackup)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn BeginRestore<'a, P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginRestore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRestore<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndRestore)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn TryAdd<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryAdd)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryRemove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryRemove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryUpdate<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryUpdate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryGet<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryGet)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn TryGetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TryGetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn EnumerateByKey2<'a, P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateByKey2)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey2<'a, P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateMetadataByKey2)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica5,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3,
    IFabricKeyValueStoreReplica4
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica5 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica5")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica5 {
    type Vtable = IFabricKeyValueStoreReplica5_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica5 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x34f2da40_6227_448a_be72_c517b0d69432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica5_Vtbl {
    pub base__: IFabricKeyValueStoreReplica4_Vtbl,
    pub TryAdd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
        added: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub TryRemove: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        checksequencenumber: i64,
        exists: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub TryUpdate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        valuesizeinbytes: i32,
        value: *const u8,
        checksequencenumber: i64,
        exists: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub TryGet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetMetadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        key: ::windows::core::PCWSTR,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateByKey2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EnumerateMetadataByKey2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transaction: *mut ::core::ffi::c_void,
        keyprefix: ::windows::core::PCWSTR,
        strictprefix: ::windows::Win32::Foundation::BOOLEAN,
        result: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricKeyValueStoreReplica6(::windows::core::IUnknown);
impl IFabricKeyValueStoreReplica6 {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndChangeRole)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .BeginClose)(
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
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .EndClose)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .base__
            .Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentEpoch(&self) -> ::windows::core::Result<super::super::FABRIC_EPOCH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetCurrentEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_EPOCH>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction(&self) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn Add<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Add)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn Remove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Remove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Update<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Update)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
        )
        .ok()
    }
    pub unsafe fn Get<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Get)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn GetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .GetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn Contains<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Contains)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn Enumerate<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .Enumerate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadata<'a, P0>(
        &self,
        transaction: P0,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey<'a, P0, P1>(
        &self,
        transaction: P0,
        keyprefix: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .base__
            .EnumerateMetadataByKey)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn Backup<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Backup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn Restore<'a, P0>(&self, backupdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .Restore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
        )
        .ok()
    }
    pub unsafe fn CreateTransaction2(
        &self,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_TRANSACTION_SETTINGS,
    ) -> ::windows::core::Result<IFabricTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .base__
            .CreateTransaction2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(settings),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricTransaction>(result__)
    }
    pub unsafe fn BeginBackup<'a, P0, P1, P2>(
        &self,
        backupdirectory: P0,
        backupoption: super::super::FABRIC_STORE_BACKUP_OPTION,
        postbackuphandler: P1,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStorePostBackupHandler>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .BeginBackup)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            backupoption,
            postbackuphandler.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndBackup<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .EndBackup)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn BeginRestore<'a, P0, P1>(
        &self,
        backupdirectory: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .BeginRestore)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRestore<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .EndRestore)(::windows::core::Vtable::as_raw(self), context.into().abi())
        .ok()
    }
    pub unsafe fn TryAdd<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryAdd)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryRemove<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryRemove)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            checksequencenumber,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryUpdate<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
        value: &[u8],
        checksequencenumber: i64,
    ) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryUpdate)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            value.len() as _,
            ::core::mem::transmute(value.as_ptr()),
            checksequencenumber,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn TryGet<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryGet)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemResult>(result__)
    }
    pub unsafe fn TryGetMetadata<'a, P0, P1>(
        &self,
        transaction: P0,
        key: P1,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryGetMetadata)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            key.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataResult>(result__)
    }
    pub unsafe fn EnumerateByKey2<'a, P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateByKey2)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemEnumerator>(result__)
    }
    pub unsafe fn EnumerateMetadataByKey2<'a, P0, P1, P2>(
        &self,
        transaction: P0,
        keyprefix: P1,
        strictprefix: P2,
    ) -> ::windows::core::Result<IFabricKeyValueStoreItemMetadataEnumerator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricTransactionBase>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::Win32::Foundation::BOOLEAN>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .EnumerateMetadataByKey2)(
            ::windows::core::Vtable::as_raw(self),
            transaction.into().abi(),
            keyprefix.into(),
            strictprefix.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricKeyValueStoreItemMetadataEnumerator>(result__)
    }
    pub unsafe fn BeginRestore2<'a, P0, P1>(
        &self,
        backupdirectory: P0,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRestore2)(
            ::windows::core::Vtable::as_raw(self),
            backupdirectory.into(),
            ::core::mem::transmute(settings),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricKeyValueStoreReplica6,
    ::windows::core::IUnknown,
    IFabricStatefulServiceReplica,
    IFabricKeyValueStoreReplica,
    IFabricKeyValueStoreReplica2,
    IFabricKeyValueStoreReplica3,
    IFabricKeyValueStoreReplica4,
    IFabricKeyValueStoreReplica5
);
impl ::core::clone::Clone for IFabricKeyValueStoreReplica6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricKeyValueStoreReplica6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricKeyValueStoreReplica6 {}
impl ::core::fmt::Debug for IFabricKeyValueStoreReplica6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricKeyValueStoreReplica6")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricKeyValueStoreReplica6 {
    type Vtable = IFabricKeyValueStoreReplica6_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricKeyValueStoreReplica6 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x56e77be1_e81f_4e42_8522_162c2d608184);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricKeyValueStoreReplica6_Vtbl {
    pub base__: IFabricKeyValueStoreReplica5_Vtbl,
    pub BeginRestore2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        backupdirectory: ::windows::core::PCWSTR,
        settings: *const super::super::FABRIC_KEY_VALUE_STORE_RESTORE_SETTINGS,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricNodeContextResult(::windows::core::IUnknown);
impl IFabricNodeContextResult {
    pub unsafe fn get_NodeContext(&self) -> *mut super::super::FABRIC_NODE_CONTEXT {
        (::windows::core::Vtable::vtable(self).get_NodeContext)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
}
::windows::core::interface_hierarchy!(IFabricNodeContextResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricNodeContextResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricNodeContextResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricNodeContextResult {}
impl ::core::fmt::Debug for IFabricNodeContextResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricNodeContextResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricNodeContextResult {
    type Vtable = IFabricNodeContextResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricNodeContextResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0952f885_6f5a_4ed3_abe4_90c403d1e3ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricNodeContextResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_NodeContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> *mut super::super::FABRIC_NODE_CONTEXT,
}
#[repr(transparent)]
pub struct IFabricNodeContextResult2(::windows::core::IUnknown);
impl IFabricNodeContextResult2 {
    pub unsafe fn get_NodeContext(&self) -> *mut super::super::FABRIC_NODE_CONTEXT {
        (::windows::core::Vtable::vtable(self).base__.get_NodeContext)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn GetDirectory<'a, P0>(
        &self,
        logicaldirectoryname: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDirectory)(
            ::windows::core::Vtable::as_raw(self),
            logicaldirectoryname.into(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricNodeContextResult2,
    ::windows::core::IUnknown,
    IFabricNodeContextResult
);
impl ::core::clone::Clone for IFabricNodeContextResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricNodeContextResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricNodeContextResult2 {}
impl ::core::fmt::Debug for IFabricNodeContextResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricNodeContextResult2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricNodeContextResult2 {
    type Vtable = IFabricNodeContextResult2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricNodeContextResult2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x472bf2e1_d617_4b5c_a91d_fabed9ff3550);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricNodeContextResult2_Vtbl {
    pub base__: IFabricNodeContextResult_Vtbl,
    pub GetDirectory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        logicaldirectoryname: ::windows::core::PCWSTR,
        directorypath: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricOperation(::windows::core::IUnknown);
impl IFabricOperation {
    pub unsafe fn get_Metadata(&self) -> *mut super::super::FABRIC_OPERATION_METADATA {
        (::windows::core::Vtable::vtable(self).get_Metadata)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetData(
        &self,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetData)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(count),
            ::core::mem::transmute(buffers),
        )
        .ok()
    }
    pub unsafe fn Acknowledge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Acknowledge)(::windows::core::Vtable::as_raw(self))
            .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricOperation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricOperation {}
impl ::core::fmt::Debug for IFabricOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricOperation {
    type Vtable = IFabricOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricOperation {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf4ad6bfa_e23c_4a48_9617_c099cd59a23a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Metadata: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    )
        -> *mut super::super::FABRIC_OPERATION_METADATA,
    pub GetData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::HRESULT,
    pub Acknowledge:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricOperationData(::windows::core::IUnknown);
impl IFabricOperationData {
    pub unsafe fn GetData(
        &self,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetData)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(count),
            ::core::mem::transmute(buffers),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricOperationData, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricOperationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricOperationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricOperationData {}
impl ::core::fmt::Debug for IFabricOperationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricOperationData")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricOperationData {
    type Vtable = IFabricOperationData_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricOperationData {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbab8ad87_37b7_482a_985d_baf38a785dcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricOperationData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
        buffers: *mut *mut super::super::FABRIC_OPERATION_DATA_BUFFER,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricOperationDataStream(::windows::core::IUnknown);
impl IFabricOperationDataStream {
    pub unsafe fn BeginGetNext<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginGetNext)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndGetNext<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<IFabricOperationData>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndGetNext)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationData>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricOperationDataStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricOperationDataStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricOperationDataStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricOperationDataStream {}
impl ::core::fmt::Debug for IFabricOperationDataStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricOperationDataStream")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricOperationDataStream {
    type Vtable = IFabricOperationDataStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricOperationDataStream {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc4e9084c_be92_49c9_8c18_d44d088c2e32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricOperationDataStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginGetNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndGetNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        operationdata: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricOperationStream(::windows::core::IUnknown);
impl IFabricOperationStream {
    pub unsafe fn BeginGetOperation<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginGetOperation)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndGetOperation<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<IFabricOperation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndGetOperation)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperation>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricOperationStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricOperationStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricOperationStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricOperationStream {}
impl ::core::fmt::Debug for IFabricOperationStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricOperationStream")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricOperationStream {
    type Vtable = IFabricOperationStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricOperationStream {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa98fb97a_d6b0_408a_a878_a9edb09c2587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricOperationStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginGetOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndGetOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        operation: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricOperationStream2(::windows::core::IUnknown);
impl IFabricOperationStream2 {
    pub unsafe fn BeginGetOperation<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .BeginGetOperation)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndGetOperation<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<IFabricOperation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetOperation)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperation>(result__)
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportFault)(
            ::windows::core::Vtable::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricOperationStream2,
    ::windows::core::IUnknown,
    IFabricOperationStream
);
impl ::core::clone::Clone for IFabricOperationStream2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricOperationStream2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricOperationStream2 {}
impl ::core::fmt::Debug for IFabricOperationStream2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricOperationStream2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricOperationStream2 {
    type Vtable = IFabricOperationStream2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricOperationStream2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0930199b_590a_4065_bec9_5f93b6aae086);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricOperationStream2_Vtbl {
    pub base__: IFabricOperationStream_Vtbl,
    pub ReportFault: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricPrimaryReplicator(::windows::core::IUnknown);
impl IFabricPrimaryReplicator {
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
        (::windows::core::Vtable::vtable(self).base__.BeginOpen)(
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
        (::windows::core::Vtable::vtable(self).base__.EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricStringResult>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(epoch),
            role,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUpdateEpoch<'a, P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .BeginUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(epoch),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndUpdateEpoch<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
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
        (::windows::core::Vtable::vtable(self).base__.BeginClose)(
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
        (::windows::core::Vtable::vtable(self).base__.EndClose)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn Abort(&self) {
        (::windows::core::Vtable::vtable(self).base__.Abort)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentProgress(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetCurrentProgress)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn GetCatchUpCapability(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetCatchUpCapability)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn BeginOnDataLoss<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOnDataLoss<'a, P0>(&self, context: P0) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn UpdateCatchUpReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateCatchUpReplicaSetConfiguration)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(currentconfiguration),
            ::core::mem::transmute(previousconfiguration),
        )
        .ok()
    }
    pub unsafe fn BeginWaitForCatchUpQuorum<'a, P0>(
        &self,
        catchupmode: super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginWaitForCatchUpQuorum)(
            ::windows::core::Vtable::as_raw(self),
            catchupmode,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndWaitForCatchUpQuorum<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndWaitForCatchUpQuorum)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn UpdateCurrentReplicaSetConfiguration(
        &self,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateCurrentReplicaSetConfiguration)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(currentconfiguration),
        )
        .ok()
    }
    pub unsafe fn BeginBuildReplica<'a, P0>(
        &self,
        replica: *const super::super::FABRIC_REPLICA_INFORMATION,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginBuildReplica)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replica),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndBuildReplica<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndBuildReplica)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveReplica(&self, replicaid: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveReplica)(
            ::windows::core::Vtable::as_raw(self),
            replicaid,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricPrimaryReplicator,
    ::windows::core::IUnknown,
    IFabricReplicator
);
impl ::core::clone::Clone for IFabricPrimaryReplicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricPrimaryReplicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricPrimaryReplicator {}
impl ::core::fmt::Debug for IFabricPrimaryReplicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricPrimaryReplicator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricPrimaryReplicator {
    type Vtable = IFabricPrimaryReplicator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricPrimaryReplicator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x564e50dd_c3a4_4600_a60e_6658874307ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricPrimaryReplicator_Vtbl {
    pub base__: IFabricReplicator_Vtbl,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        isstatechanged: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub UpdateCatchUpReplicaSetConfiguration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
        previousconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    )
        -> ::windows::core::HRESULT,
    pub BeginWaitForCatchUpQuorum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        catchupmode: super::super::FABRIC_REPLICA_SET_QUORUM_MODE,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndWaitForCatchUpQuorum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UpdateCurrentReplicaSetConfiguration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        currentconfiguration: *const super::super::FABRIC_REPLICA_SET_CONFIGURATION,
    )
        -> ::windows::core::HRESULT,
    pub BeginBuildReplica: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        replica: *const super::super::FABRIC_REPLICA_INFORMATION,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndBuildReplica: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveReplica: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        replicaid: i64,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricProcessExitHandler(::windows::core::IUnknown);
impl IFabricProcessExitHandler {
    pub unsafe fn FabricProcessExited(&self) {
        (::windows::core::Vtable::vtable(self).FabricProcessExited)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricProcessExitHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricProcessExitHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricProcessExitHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricProcessExitHandler {}
impl ::core::fmt::Debug for IFabricProcessExitHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricProcessExitHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricProcessExitHandler {
    type Vtable = IFabricProcessExitHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricProcessExitHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc58d50a2_01f0_4267_bbe7_223b565c1346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricProcessExitHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FabricProcessExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricReplicator(::windows::core::IUnknown);
impl IFabricReplicator {
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
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(epoch),
            role,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUpdateEpoch<'a, P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(epoch),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndUpdateEpoch<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
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
    pub unsafe fn GetCurrentProgress(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentProgress)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn GetCatchUpCapability(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCatchUpCapability)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricReplicator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricReplicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricReplicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricReplicator {}
impl ::core::fmt::Debug for IFabricReplicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricReplicator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricReplicator {
    type Vtable = IFabricReplicator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricReplicator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x067f144a_e5be_4f5e_a181_8b5593e20242);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricReplicator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        replicationaddress: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginChangeRole: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        epoch: *const super::super::FABRIC_EPOCH,
        role: super::super::FABRIC_REPLICA_ROLE,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndChangeRole: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginUpdateEpoch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        epoch: *const super::super::FABRIC_EPOCH,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndUpdateEpoch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
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
    pub GetCurrentProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lastsequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub GetCatchUpCapability: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fromsequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricReplicatorCatchupSpecificQuorum(::windows::core::IUnknown);
impl IFabricReplicatorCatchupSpecificQuorum {}
::windows::core::interface_hierarchy!(
    IFabricReplicatorCatchupSpecificQuorum,
    ::windows::core::IUnknown
);
impl ::core::clone::Clone for IFabricReplicatorCatchupSpecificQuorum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricReplicatorCatchupSpecificQuorum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricReplicatorCatchupSpecificQuorum {}
impl ::core::fmt::Debug for IFabricReplicatorCatchupSpecificQuorum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricReplicatorCatchupSpecificQuorum")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricReplicatorCatchupSpecificQuorum {
    type Vtable = IFabricReplicatorCatchupSpecificQuorum_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricReplicatorCatchupSpecificQuorum {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xaa3116fe_277d_482d_bd16_5366fa405757);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricReplicatorCatchupSpecificQuorum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[repr(transparent)]
pub struct IFabricReplicatorSettingsResult(::windows::core::IUnknown);
impl IFabricReplicatorSettingsResult {
    pub unsafe fn get_ReplicatorSettings(&self) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS {
        (::windows::core::Vtable::vtable(self).get_ReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricReplicatorSettingsResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricReplicatorSettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricReplicatorSettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricReplicatorSettingsResult {}
impl ::core::fmt::Debug for IFabricReplicatorSettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricReplicatorSettingsResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricReplicatorSettingsResult {
    type Vtable = IFabricReplicatorSettingsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricReplicatorSettingsResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x718954f3_dc1e_4060_9806_0cbf36f71051);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricReplicatorSettingsResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_ReplicatorSettings:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_REPLICATOR_SETTINGS,
}
#[repr(transparent)]
pub struct IFabricRuntime(::windows::core::IUnknown);
impl IFabricRuntime {
    pub unsafe fn BeginRegisterStatelessServiceFactory<'a, P0, P1, P2>(
        &self,
        servicetypename: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatelessServiceFactory>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRegisterStatelessServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            factory.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRegisterStatelessServiceFactory<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndRegisterStatelessServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterStatelessServiceFactory<'a, P0, P1>(
        &self,
        servicetypename: P0,
        factory: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatelessServiceFactory>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterStatelessServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            factory.into().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginRegisterStatefulServiceFactory<'a, P0, P1, P2>(
        &self,
        servicetypename: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServiceFactory>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRegisterStatefulServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            factory.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRegisterStatefulServiceFactory<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndRegisterStatefulServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterStatefulServiceFactory<'a, P0, P1>(
        &self,
        servicetypename: P0,
        factory: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServiceFactory>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterStatefulServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            factory.into().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateServiceGroupFactoryBuilder(
        &self,
    ) -> ::windows::core::Result<IFabricServiceGroupFactoryBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateServiceGroupFactoryBuilder)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricServiceGroupFactoryBuilder>(result__)
    }
    pub unsafe fn BeginRegisterServiceGroupFactory<'a, P0, P1, P2>(
        &self,
        groupservicetype: P0,
        factory: P1,
        timeoutmilliseconds: u32,
        callback: P2,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricServiceGroupFactory>>,
        P2: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRegisterServiceGroupFactory)(
            ::windows::core::Vtable::as_raw(self),
            groupservicetype.into(),
            factory.into().abi(),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndRegisterServiceGroupFactory<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndRegisterServiceGroupFactory)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn RegisterServiceGroupFactory<'a, P0, P1>(
        &self,
        groupservicetype: P0,
        factory: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricServiceGroupFactory>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterServiceGroupFactory)(
            ::windows::core::Vtable::as_raw(self),
            groupservicetype.into(),
            factory.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricRuntime, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricRuntime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricRuntime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricRuntime {}
impl ::core::fmt::Debug for IFabricRuntime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricRuntime").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricRuntime {
    type Vtable = IFabricRuntime_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricRuntime {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc53af8e_74cd_11df_ac3e_0024811e3892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricRuntime_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginRegisterStatelessServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub EndRegisterStatelessServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub RegisterStatelessServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginRegisterStatefulServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub EndRegisterStatefulServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub RegisterStatefulServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateServiceGroupFactoryBuilder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        builder: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub BeginRegisterServiceGroupFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        groupservicetype: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub EndRegisterServiceGroupFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RegisterServiceGroupFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        groupservicetype: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricSecondaryEventHandler(::windows::core::IUnknown);
impl IFabricSecondaryEventHandler {
    pub unsafe fn OnCopyComplete<'a, P0>(&self, enumerator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricKeyValueStoreEnumerator>>,
    {
        (::windows::core::Vtable::vtable(self).OnCopyComplete)(
            ::windows::core::Vtable::as_raw(self),
            enumerator.into().abi(),
        )
        .ok()
    }
    pub unsafe fn OnReplicationOperation<'a, P0>(
        &self,
        enumerator: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, IFabricKeyValueStoreNotificationEnumerator>,
        >,
    {
        (::windows::core::Vtable::vtable(self).OnReplicationOperation)(
            ::windows::core::Vtable::as_raw(self),
            enumerator.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricSecondaryEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricSecondaryEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricSecondaryEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricSecondaryEventHandler {}
impl ::core::fmt::Debug for IFabricSecondaryEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricSecondaryEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricSecondaryEventHandler {
    type Vtable = IFabricSecondaryEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricSecondaryEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7d124a7d_258e_49f2_a9b0_e800406103fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricSecondaryEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCopyComplete: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        enumerator: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OnReplicationOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        enumerator: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricSecurityCredentialsResult(::windows::core::IUnknown);
impl IFabricSecurityCredentialsResult {
    pub unsafe fn get_SecurityCredentials(&self) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS {
        (::windows::core::Vtable::vtable(self).get_SecurityCredentials)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
}
::windows::core::interface_hierarchy!(IFabricSecurityCredentialsResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricSecurityCredentialsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricSecurityCredentialsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricSecurityCredentialsResult {}
impl ::core::fmt::Debug for IFabricSecurityCredentialsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricSecurityCredentialsResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricSecurityCredentialsResult {
    type Vtable = IFabricSecurityCredentialsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricSecurityCredentialsResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x049a111d_6a30_48e9_8f69_470760d3efb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricSecurityCredentialsResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_SecurityCredentials:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> *mut super::super::FABRIC_SECURITY_CREDENTIALS,
}
#[repr(transparent)]
pub struct IFabricServiceGroupFactory(::windows::core::IUnknown);
impl IFabricServiceGroupFactory {}
::windows::core::interface_hierarchy!(IFabricServiceGroupFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricServiceGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricServiceGroupFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricServiceGroupFactory {}
impl ::core::fmt::Debug for IFabricServiceGroupFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricServiceGroupFactory")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricServiceGroupFactory {
    type Vtable = IFabricServiceGroupFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricServiceGroupFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3860d61d_1e51_4a65_b109_d93c11311657);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricServiceGroupFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[repr(transparent)]
pub struct IFabricServiceGroupFactoryBuilder(::windows::core::IUnknown);
impl IFabricServiceGroupFactoryBuilder {
    pub unsafe fn AddStatelessServiceFactory<'a, P0, P1>(
        &self,
        memberservicetype: P0,
        factory: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatelessServiceFactory>>,
    {
        (::windows::core::Vtable::vtable(self).AddStatelessServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            memberservicetype.into(),
            factory.into().abi(),
        )
        .ok()
    }
    pub unsafe fn AddStatefulServiceFactory<'a, P0, P1>(
        &self,
        memberservicetype: P0,
        factory: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServiceFactory>>,
    {
        (::windows::core::Vtable::vtable(self).AddStatefulServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            memberservicetype.into(),
            factory.into().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveServiceFactory<'a, P0>(
        &self,
        memberservicetype: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveServiceFactory)(
            ::windows::core::Vtable::as_raw(self),
            memberservicetype.into(),
        )
        .ok()
    }
    pub unsafe fn ToServiceGroupFactory(
        &self,
    ) -> ::windows::core::Result<IFabricServiceGroupFactory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ToServiceGroupFactory)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricServiceGroupFactory>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricServiceGroupFactoryBuilder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricServiceGroupFactoryBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricServiceGroupFactoryBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricServiceGroupFactoryBuilder {}
impl ::core::fmt::Debug for IFabricServiceGroupFactoryBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricServiceGroupFactoryBuilder")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricServiceGroupFactoryBuilder {
    type Vtable = IFabricServiceGroupFactoryBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricServiceGroupFactoryBuilder {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa9fe8b06_19b1_49e6_8911_41d9d9219e1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricServiceGroupFactoryBuilder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddStatelessServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        memberservicetype: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddStatefulServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        memberservicetype: ::windows::core::PCWSTR,
        factory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveServiceFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        memberservicetype: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
    pub ToServiceGroupFactory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        factory: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricServiceGroupPartition(::windows::core::IUnknown);
impl IFabricServiceGroupPartition {
    pub unsafe fn ResolveMember(
        &self,
        name: *const u16,
        riid: *const ::windows::core::GUID,
    ) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ResolveMember)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(name),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut ::core::ffi::c_void>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricServiceGroupPartition, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricServiceGroupPartition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricServiceGroupPartition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricServiceGroupPartition {}
impl ::core::fmt::Debug for IFabricServiceGroupPartition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricServiceGroupPartition")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricServiceGroupPartition {
    type Vtable = IFabricServiceGroupPartition_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricServiceGroupPartition {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2b24299a_7489_467f_8e7f_4507bff73b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricServiceGroupPartition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ResolveMember: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: *const u16,
        riid: *const ::windows::core::GUID,
        member: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStateProvider(::windows::core::IUnknown);
impl IFabricStateProvider {
    pub unsafe fn BeginUpdateEpoch<'a, P0>(
        &self,
        epoch: *const super::super::FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(epoch),
            previousepochlastsequencenumber,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndUpdateEpoch<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        (::windows::core::Vtable::vtable(self).EndUpdateEpoch)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
        )
        .ok()
    }
    pub unsafe fn GetLastCommittedSequenceNumber(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastCommittedSequenceNumber)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn BeginOnDataLoss<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOnDataLoss<'a, P0>(&self, context: P0) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn GetCopyContext(&self) -> ::windows::core::Result<IFabricOperationDataStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCopyContext)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationDataStream>(result__)
    }
    pub unsafe fn GetCopyState<'a, P0>(
        &self,
        uptosequencenumber: i64,
        copycontextstream: P0,
    ) -> ::windows::core::Result<IFabricOperationDataStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricOperationDataStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCopyState)(
            ::windows::core::Vtable::as_raw(self),
            uptosequencenumber,
            copycontextstream.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationDataStream>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricStateProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStateProvider {}
impl ::core::fmt::Debug for IFabricStateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStateProvider")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStateProvider {
    type Vtable = IFabricStateProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStateProvider {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3ebfec79_bd27_43f3_8be8_da38ee723951);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStateProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginUpdateEpoch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        epoch: *const super::super::FABRIC_EPOCH,
        previousepochlastsequencenumber: i64,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndUpdateEpoch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetLastCommittedSequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        isstatechanged: *mut u8,
    ) -> ::windows::core::HRESULT,
    pub GetCopyContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        copycontextstream: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCopyState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uptosequencenumber: i64,
        copycontextstream: *mut ::core::ffi::c_void,
        copystatestream: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStateReplicator(::windows::core::IUnknown);
impl IFabricStateReplicator {
    pub unsafe fn BeginReplicate<'a, P0, P1>(
        &self,
        operationdata: P0,
        callback: P1,
        sequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricOperationData>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        (::windows::core::Vtable::vtable(self).BeginReplicate)(
            ::windows::core::Vtable::as_raw(self),
            operationdata.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(sequencenumber),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    pub unsafe fn EndReplicate<'a, P0>(&self, context: P0) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndReplicate)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn GetReplicationStream(&self) -> ::windows::core::Result<IFabricOperationStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReplicationStream)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationStream>(result__)
    }
    pub unsafe fn GetCopyStream(&self) -> ::windows::core::Result<IFabricOperationStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCopyStream)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationStream>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricStateReplicator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStateReplicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStateReplicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStateReplicator {}
impl ::core::fmt::Debug for IFabricStateReplicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStateReplicator")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStateReplicator {
    type Vtable = IFabricStateReplicator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStateReplicator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x89e9a978_c771_44f2_92e8_3bf271cabe9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStateReplicator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginReplicate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        operationdata: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        sequencenumber: *mut i64,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndReplicate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        sequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub GetReplicationStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCopyStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UpdateReplicatorSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStateReplicator2(::windows::core::IUnknown);
impl IFabricStateReplicator2 {
    pub unsafe fn BeginReplicate<'a, P0, P1>(
        &self,
        operationdata: P0,
        callback: P1,
        sequencenumber: *mut i64,
        context: *mut ::core::option::Option<super::IFabricAsyncOperationContext>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricOperationData>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginReplicate)(
            ::windows::core::Vtable::as_raw(self),
            operationdata.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(sequencenumber),
            ::core::mem::transmute(context),
        )
        .ok()
    }
    pub unsafe fn EndReplicate<'a, P0>(&self, context: P0) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndReplicate)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn GetReplicationStream(&self) -> ::windows::core::Result<IFabricOperationStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetReplicationStream)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationStream>(result__)
    }
    pub unsafe fn GetCopyStream(&self) -> ::windows::core::Result<IFabricOperationStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCopyStream)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricOperationStream>(result__)
    }
    pub unsafe fn UpdateReplicatorSettings(
        &self,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .UpdateReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(replicatorsettings),
        )
        .ok()
    }
    pub unsafe fn GetReplicatorSettings(
        &self,
    ) -> ::windows::core::Result<IFabricReplicatorSettingsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReplicatorSettings)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicatorSettingsResult>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricStateReplicator2,
    ::windows::core::IUnknown,
    IFabricStateReplicator
);
impl ::core::clone::Clone for IFabricStateReplicator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStateReplicator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStateReplicator2 {}
impl ::core::fmt::Debug for IFabricStateReplicator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStateReplicator2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStateReplicator2 {
    type Vtable = IFabricStateReplicator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStateReplicator2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4a28d542_658f_46f9_9bf4_79b7cae25c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStateReplicator2_Vtbl {
    pub base__: IFabricStateReplicator_Vtbl,
    pub GetReplicatorSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        replicatorsettings: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServiceFactory(::windows::core::IUnknown);
impl IFabricStatefulServiceFactory {
    pub unsafe fn CreateReplica<'a, P0>(
        &self,
        servicetypename: P0,
        servicename: *const u16,
        initializationdata: &[u8],
        partitionid: ::windows::core::GUID,
        replicaid: i64,
    ) -> ::windows::core::Result<IFabricStatefulServiceReplica>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReplica)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            ::core::mem::transmute(servicename),
            initializationdata.len() as _,
            ::core::mem::transmute(initializationdata.as_ptr()),
            ::core::mem::transmute(partitionid),
            replicaid,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricStatefulServiceReplica>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricStatefulServiceFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatefulServiceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServiceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServiceFactory {}
impl ::core::fmt::Debug for IFabricStatefulServiceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServiceFactory")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServiceFactory {
    type Vtable = IFabricStatefulServiceFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServiceFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x77ff0c6b_6780_48ec_b4b0_61989327b0f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServiceFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateReplica: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: ::windows::core::GUID,
        replicaid: i64,
        servicereplica: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServicePartition(::windows::core::IUnknown);
impl IFabricStatefulServicePartition {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReadStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWriteStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn CreateReplicator<'a, P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
        statereplicator: *mut ::core::option::Option<IFabricStateReplicator>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStateProvider>>,
    {
        (::windows::core::Vtable::vtable(self).CreateReplicator)(
            ::windows::core::Vtable::as_raw(self),
            stateprovider.into().abi(),
            ::core::mem::transmute(replicatorsettings),
            ::core::mem::transmute(replicator),
            ::core::mem::transmute(statereplicator),
        )
        .ok()
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportFault)(
            ::windows::core::Vtable::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricStatefulServicePartition, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatefulServicePartition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServicePartition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServicePartition {}
impl ::core::fmt::Debug for IFabricStatefulServicePartition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServicePartition")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServicePartition {
    type Vtable = IFabricStatefulServicePartition_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServicePartition {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5beccc37_8655_4f20_bd43_f50691d7cd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServicePartition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPartitionInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bufferedvalue: *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub GetReadStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        readstatus: *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    ) -> ::windows::core::HRESULT,
    pub GetWriteStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        writestatus: *mut super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS,
    ) -> ::windows::core::HRESULT,
    pub CreateReplicator: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stateprovider: *mut ::core::ffi::c_void,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut *mut ::core::ffi::c_void,
        statereplicator: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ReportLoad: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        metriccount: u32,
        metrics: *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows::core::HRESULT,
    pub ReportFault: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServicePartition1(::windows::core::IUnknown);
impl IFabricStatefulServicePartition1 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReadStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWriteStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn CreateReplicator<'a, P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
        statereplicator: *mut ::core::option::Option<IFabricStateReplicator>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStateProvider>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .CreateReplicator)(
            ::windows::core::Vtable::as_raw(self),
            stateprovider.into().abi(),
            ::core::mem::transmute(replicatorsettings),
            ::core::mem::transmute(replicator),
            ::core::mem::transmute(statereplicator),
        )
        .ok()
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportFault)(
            ::windows::core::Vtable::as_raw(self),
            faulttype,
        )
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportMoveCost)(
            ::windows::core::Vtable::as_raw(self),
            movecost,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatefulServicePartition1,
    ::windows::core::IUnknown,
    IFabricStatefulServicePartition
);
impl ::core::clone::Clone for IFabricStatefulServicePartition1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServicePartition1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServicePartition1 {}
impl ::core::fmt::Debug for IFabricStatefulServicePartition1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServicePartition1")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServicePartition1 {
    type Vtable = IFabricStatefulServicePartition1_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServicePartition1 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc9c66f2f_9dff_4c87_bbe4_a08b4c4074cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServicePartition1_Vtbl {
    pub base__: IFabricStatefulServicePartition_Vtbl,
    pub ReportMoveCost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServicePartition2(::windows::core::IUnknown);
impl IFabricStatefulServicePartition2 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetReadStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetWriteStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn CreateReplicator<'a, P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
        statereplicator: *mut ::core::option::Option<IFabricStateReplicator>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStateProvider>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .CreateReplicator)(
            ::windows::core::Vtable::as_raw(self),
            stateprovider.into().abi(),
            ::core::mem::transmute(replicatorsettings),
            ::core::mem::transmute(replicator),
            ::core::mem::transmute(statereplicator),
        )
        .ok()
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportFault)(::windows::core::Vtable::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportMoveCost)(
            ::windows::core::Vtable::as_raw(self),
            movecost,
        )
        .ok()
    }
    pub unsafe fn ReportReplicaHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportReplicaHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportPartitionHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatefulServicePartition2,
    ::windows::core::IUnknown,
    IFabricStatefulServicePartition,
    IFabricStatefulServicePartition1
);
impl ::core::clone::Clone for IFabricStatefulServicePartition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServicePartition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServicePartition2 {}
impl ::core::fmt::Debug for IFabricStatefulServicePartition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServicePartition2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServicePartition2 {
    type Vtable = IFabricStatefulServicePartition2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServicePartition2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdf27b476_fa25_459f_a7d3_87d3eec9c73c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServicePartition2_Vtbl {
    pub base__: IFabricStatefulServicePartition1_Vtbl,
    pub ReportReplicaHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub ReportPartitionHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServicePartition3(::windows::core::IUnknown);
impl IFabricStatefulServicePartition3 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn GetReadStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetReadStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn GetWriteStatus(
        &self,
    ) -> ::windows::core::Result<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetWriteStatus)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::super::FABRIC_SERVICE_PARTITION_ACCESS_STATUS>(result__)
    }
    pub unsafe fn CreateReplicator<'a, P0>(
        &self,
        stateprovider: P0,
        replicatorsettings: *const super::super::FABRIC_REPLICATOR_SETTINGS,
        replicator: *mut ::core::option::Option<IFabricReplicator>,
        statereplicator: *mut ::core::option::Option<IFabricStateReplicator>,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStateProvider>>,
    {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .CreateReplicator)(
            ::windows::core::Vtable::as_raw(self),
            stateprovider.into().abi(),
            ::core::mem::transmute(replicatorsettings),
            ::core::mem::transmute(replicator),
            ::core::mem::transmute(statereplicator),
        )
        .ok()
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportFault)(::windows::core::Vtable::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportMoveCost)(::windows::core::Vtable::as_raw(self), movecost)
        .ok()
    }
    pub unsafe fn ReportReplicaHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportReplicaHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportPartitionHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportReplicaHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportReplicaHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportPartitionHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatefulServicePartition3,
    ::windows::core::IUnknown,
    IFabricStatefulServicePartition,
    IFabricStatefulServicePartition1,
    IFabricStatefulServicePartition2
);
impl ::core::clone::Clone for IFabricStatefulServicePartition3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServicePartition3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServicePartition3 {}
impl ::core::fmt::Debug for IFabricStatefulServicePartition3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServicePartition3")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServicePartition3 {
    type Vtable = IFabricStatefulServicePartition3_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServicePartition3 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x51f1269d_b061_4c1c_96cf_6508cece813b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServicePartition3_Vtbl {
    pub base__: IFabricStatefulServicePartition2_Vtbl,
    pub ReportReplicaHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::HRESULT,
    pub ReportPartitionHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatefulServiceReplica(::windows::core::IUnknown);
impl IFabricStatefulServiceReplica {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatefulServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            openmode,
            partition.into().abi(),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOpen<'a, P0>(&self, context: P0) -> ::windows::core::Result<IFabricReplicator>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndOpen)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricReplicator>(result__)
    }
    pub unsafe fn BeginChangeRole<'a, P0>(
        &self,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginChangeRole)(
            ::windows::core::Vtable::as_raw(self),
            newrole,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndChangeRole<'a, P0>(
        &self,
        context: P0,
    ) -> ::windows::core::Result<super::IFabricStringResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndChangeRole)(
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
::windows::core::interface_hierarchy!(IFabricStatefulServiceReplica, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatefulServiceReplica {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatefulServiceReplica {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatefulServiceReplica {}
impl ::core::fmt::Debug for IFabricStatefulServiceReplica {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatefulServiceReplica")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatefulServiceReplica {
    type Vtable = IFabricStatefulServiceReplica_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatefulServiceReplica {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8ae3be0e_505d_4dc1_ad8f_0cb0f9576b8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatefulServiceReplica_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        openmode: super::super::FABRIC_REPLICA_OPEN_MODE,
        partition: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        replicator: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BeginChangeRole: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newrole: super::super::FABRIC_REPLICA_ROLE,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndChangeRole: unsafe extern "system" fn(
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
pub struct IFabricStatelessServiceFactory(::windows::core::IUnknown);
impl IFabricStatelessServiceFactory {
    pub unsafe fn CreateInstance<'a, P0>(
        &self,
        servicetypename: P0,
        servicename: *const u16,
        initializationdata: &[u8],
        partitionid: ::windows::core::GUID,
        instanceid: i64,
    ) -> ::windows::core::Result<IFabricStatelessServiceInstance>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstance)(
            ::windows::core::Vtable::as_raw(self),
            servicetypename.into(),
            ::core::mem::transmute(servicename),
            initializationdata.len() as _,
            ::core::mem::transmute(initializationdata.as_ptr()),
            ::core::mem::transmute(partitionid),
            instanceid,
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IFabricStatelessServiceInstance>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricStatelessServiceFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatelessServiceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServiceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServiceFactory {}
impl ::core::fmt::Debug for IFabricStatelessServiceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServiceFactory")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServiceFactory {
    type Vtable = IFabricStatelessServiceFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServiceFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc53af8f_74cd_11df_ac3e_0024811e3892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServiceFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        servicetypename: ::windows::core::PCWSTR,
        servicename: *const u16,
        initializationdatalength: u32,
        initializationdata: *const u8,
        partitionid: ::windows::core::GUID,
        instanceid: i64,
        serviceinstance: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatelessServiceInstance(::windows::core::IUnknown);
impl IFabricStatelessServiceInstance {
    pub unsafe fn BeginOpen<'a, P0, P1>(
        &self,
        partition: P0,
        callback: P1,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFabricStatelessServicePartition>>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOpen)(
            ::windows::core::Vtable::as_raw(self),
            partition.into().abi(),
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
::windows::core::interface_hierarchy!(IFabricStatelessServiceInstance, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatelessServiceInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServiceInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServiceInstance {}
impl ::core::fmt::Debug for IFabricStatelessServiceInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServiceInstance")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServiceInstance {
    type Vtable = IFabricStatelessServiceInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServiceInstance {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc53af90_74cd_11df_ac3e_0024811e3892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServiceInstance_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        partition: *mut ::core::ffi::c_void,
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
pub struct IFabricStatelessServicePartition(::windows::core::IUnknown);
impl IFabricStatelessServicePartition {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportFault)(
            ::windows::core::Vtable::as_raw(self),
            faulttype,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFabricStatelessServicePartition, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStatelessServicePartition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServicePartition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServicePartition {}
impl ::core::fmt::Debug for IFabricStatelessServicePartition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServicePartition")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServicePartition {
    type Vtable = IFabricStatelessServicePartition_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServicePartition {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc53af91_74cd_11df_ac3e_0024811e3892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServicePartition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPartitionInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bufferedvalue: *mut *mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub ReportLoad: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        metriccount: u32,
        metrics: *const super::super::FABRIC_LOAD_METRIC,
    ) -> ::windows::core::HRESULT,
    pub ReportFault: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatelessServicePartition1(::windows::core::IUnknown);
impl IFabricStatelessServicePartition1 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportFault)(
            ::windows::core::Vtable::as_raw(self),
            faulttype,
        )
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportMoveCost)(
            ::windows::core::Vtable::as_raw(self),
            movecost,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatelessServicePartition1,
    ::windows::core::IUnknown,
    IFabricStatelessServicePartition
);
impl ::core::clone::Clone for IFabricStatelessServicePartition1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServicePartition1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServicePartition1 {}
impl ::core::fmt::Debug for IFabricStatelessServicePartition1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServicePartition1")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServicePartition1 {
    type Vtable = IFabricStatelessServicePartition1_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServicePartition1 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbf6bb505_7bd0_4371_b6c0_cba319a5e50b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServicePartition1_Vtbl {
    pub base__: IFabricStatelessServicePartition_Vtbl,
    pub ReportMoveCost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatelessServicePartition2(::windows::core::IUnknown);
impl IFabricStatelessServicePartition2 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportFault)(::windows::core::Vtable::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportMoveCost)(
            ::windows::core::Vtable::as_raw(self),
            movecost,
        )
        .ok()
    }
    pub unsafe fn ReportInstanceHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportInstanceHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportPartitionHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatelessServicePartition2,
    ::windows::core::IUnknown,
    IFabricStatelessServicePartition,
    IFabricStatelessServicePartition1
);
impl ::core::clone::Clone for IFabricStatelessServicePartition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServicePartition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServicePartition2 {}
impl ::core::fmt::Debug for IFabricStatelessServicePartition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServicePartition2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServicePartition2 {
    type Vtable = IFabricStatelessServicePartition2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServicePartition2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ff35b6c_9d97_4312_93ad_7f34cbdb4ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServicePartition2_Vtbl {
    pub base__: IFabricStatelessServicePartition1_Vtbl,
    pub ReportInstanceHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
    pub ReportPartitionHealth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStatelessServicePartition3(::windows::core::IUnknown);
impl IFabricStatelessServicePartition3 {
    pub unsafe fn GetPartitionInfo(
        &self,
    ) -> ::windows::core::Result<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .GetPartitionInfo)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<*mut super::super::FABRIC_SERVICE_PARTITION_INFORMATION>(result__)
    }
    pub unsafe fn ReportLoad(
        &self,
        metrics: &[super::super::FABRIC_LOAD_METRIC],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportLoad)(
            ::windows::core::Vtable::as_raw(self),
            metrics.len() as _,
            ::core::mem::transmute(metrics.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn ReportFault(
        &self,
        faulttype: super::super::FABRIC_FAULT_TYPE,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .base__
            .ReportFault)(::windows::core::Vtable::as_raw(self), faulttype)
        .ok()
    }
    pub unsafe fn ReportMoveCost(
        &self,
        movecost: super::super::FABRIC_MOVE_COST,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .base__
            .ReportMoveCost)(::windows::core::Vtable::as_raw(self), movecost)
        .ok()
    }
    pub unsafe fn ReportInstanceHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportInstanceHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .ReportPartitionHealth)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
        )
        .ok()
    }
    pub unsafe fn ReportInstanceHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportInstanceHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
    pub unsafe fn ReportPartitionHealth2(
        &self,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportPartitionHealth2)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(healthinfo),
            ::core::mem::transmute(sendoptions),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IFabricStatelessServicePartition3,
    ::windows::core::IUnknown,
    IFabricStatelessServicePartition,
    IFabricStatelessServicePartition1,
    IFabricStatelessServicePartition2
);
impl ::core::clone::Clone for IFabricStatelessServicePartition3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStatelessServicePartition3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStatelessServicePartition3 {}
impl ::core::fmt::Debug for IFabricStatelessServicePartition3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStatelessServicePartition3")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStatelessServicePartition3 {
    type Vtable = IFabricStatelessServicePartition3_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStatelessServicePartition3 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf2fa2000_70a7_4ed5_9d3e_0b7deca2433f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStatelessServicePartition3_Vtbl {
    pub base__: IFabricStatelessServicePartition2_Vtbl,
    pub ReportInstanceHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::HRESULT,
    pub ReportPartitionHealth2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        healthinfo: *const super::super::FABRIC_HEALTH_INFORMATION,
        sendoptions: *const super::super::FABRIC_HEALTH_REPORT_SEND_OPTIONS,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStoreEventHandler(::windows::core::IUnknown);
impl IFabricStoreEventHandler {
    pub unsafe fn OnDataLoss(&self) {
        (::windows::core::Vtable::vtable(self).OnDataLoss)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IFabricStoreEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStoreEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStoreEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStoreEventHandler {}
impl ::core::fmt::Debug for IFabricStoreEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStoreEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStoreEventHandler {
    type Vtable = IFabricStoreEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStoreEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x220e6da4_985b_4dee_8fe9_77521b838795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStoreEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnDataLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricStoreEventHandler2(::windows::core::IUnknown);
impl IFabricStoreEventHandler2 {
    pub unsafe fn OnDataLoss(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnDataLoss)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn BeginOnDataLoss<'a, P0>(
        &self,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndOnDataLoss<'a, P0>(&self, context: P0) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndOnDataLoss)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
}
::windows::core::interface_hierarchy!(
    IFabricStoreEventHandler2,
    ::windows::core::IUnknown,
    IFabricStoreEventHandler
);
impl ::core::clone::Clone for IFabricStoreEventHandler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStoreEventHandler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStoreEventHandler2 {}
impl ::core::fmt::Debug for IFabricStoreEventHandler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStoreEventHandler2")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStoreEventHandler2 {
    type Vtable = IFabricStoreEventHandler2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStoreEventHandler2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcce4523f_614b_4d6a_98a3_1e197c0213ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStoreEventHandler2_Vtbl {
    pub base__: IFabricStoreEventHandler_Vtbl,
    pub BeginOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndOnDataLoss: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        isstatechanged: *mut u8,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricStorePostBackupHandler(::windows::core::IUnknown);
impl IFabricStorePostBackupHandler {
    pub unsafe fn BeginPostBackup<'a, P0>(
        &self,
        info: *const super::super::FABRIC_STORE_BACKUP_INFO,
        callback: P0,
    ) -> ::windows::core::Result<super::IFabricAsyncOperationContext>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::IFabricAsyncOperationCallback>,
        >,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginPostBackup)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(info),
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndPostBackup<'a, P0>(&self, context: P0) -> ::windows::core::Result<u8>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndPostBackup)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<u8>(result__)
    }
}
::windows::core::interface_hierarchy!(IFabricStorePostBackupHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricStorePostBackupHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricStorePostBackupHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricStorePostBackupHandler {}
impl ::core::fmt::Debug for IFabricStorePostBackupHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricStorePostBackupHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricStorePostBackupHandler {
    type Vtable = IFabricStorePostBackupHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricStorePostBackupHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2af2e8a6_41df_4e32_9d2a_d73a711e652a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricStorePostBackupHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginPostBackup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        info: *const super::super::FABRIC_STORE_BACKUP_INFO,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndPostBackup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        status: *mut u8,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IFabricTransaction(::windows::core::IUnknown);
impl IFabricTransaction {
    pub unsafe fn get_Id(&self) -> *mut ::windows::core::GUID {
        (::windows::core::Vtable::vtable(self).base__.get_Id)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_IsolationLevel(&self) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL {
        (::windows::core::Vtable::vtable(self)
            .base__
            .get_IsolationLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn BeginCommit<'a, P0>(
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
        (::windows::core::Vtable::vtable(self).BeginCommit)(
            ::windows::core::Vtable::as_raw(self),
            timeoutmilliseconds,
            callback.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<super::IFabricAsyncOperationContext>(result__)
    }
    pub unsafe fn EndCommit<'a, P0>(&self, context: P0) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IFabricAsyncOperationContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndCommit)(
            ::windows::core::Vtable::as_raw(self),
            context.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<i64>(result__)
    }
    pub unsafe fn Rollback(&self) {
        (::windows::core::Vtable::vtable(self).Rollback)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(
    IFabricTransaction,
    ::windows::core::IUnknown,
    IFabricTransactionBase
);
impl ::core::clone::Clone for IFabricTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransaction {}
impl ::core::fmt::Debug for IFabricTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransaction {
    type Vtable = IFabricTransaction_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransaction {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x19ee48b4_6d4d_470b_ac1e_2d3996a173c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransaction_Vtbl {
    pub base__: IFabricTransactionBase_Vtbl,
    pub BeginCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        timeoutmilliseconds: u32,
        callback: *mut ::core::ffi::c_void,
        context: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EndCommit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        commitsequencenumber: *mut i64,
    ) -> ::windows::core::HRESULT,
    pub Rollback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IFabricTransactionBase(::windows::core::IUnknown);
impl IFabricTransactionBase {
    pub unsafe fn get_Id(&self) -> *mut ::windows::core::GUID {
        (::windows::core::Vtable::vtable(self).get_Id)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn get_IsolationLevel(&self) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL {
        (::windows::core::Vtable::vtable(self).get_IsolationLevel)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
}
::windows::core::interface_hierarchy!(IFabricTransactionBase, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFabricTransactionBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFabricTransactionBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFabricTransactionBase {}
impl ::core::fmt::Debug for IFabricTransactionBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFabricTransactionBase")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IFabricTransactionBase {
    type Vtable = IFabricTransactionBase_Vtbl;
}
unsafe impl ::windows::core::Interface for IFabricTransactionBase {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32d656a1_7ad5_47b8_bd66_a2e302626b7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFabricTransactionBase_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub get_Id:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows::core::GUID,
    pub get_IsolationLevel:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
        ) -> super::super::FABRIC_TRANSACTION_ISOLATION_LEVEL,
}
pub const FabricRuntime: ::windows::core::GUID =
    ::windows::core::GUID::from_u128(0xcc53af8c_74cd_11df_ac3e_0024811e3892);
pub type FnFabricMain = ::core::option::Option<
    unsafe extern "system" fn(
        runtime: ::core::option::Option<IFabricRuntime>,
        activationcontext: ::core::option::Option<IFabricCodePackageActivationContext>,
    ) -> ::windows::core::HRESULT,
>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
