//! Dynamically load SF libs and c functions.
//! SF shared lib provides these functions, and we dynamically load them here so that user of this crate
//! does not need to worry about installing SF lib and linking, which can be complex.
//!

use mssf_com::{
    FabricClient::{IFabricClientConnectionEventHandler, IFabricServiceNotificationEventHandler},
    FabricCommon::{
        IFabricAsyncOperationCallback, IFabricAsyncOperationContext, IFabricStringResult,
    },
    FabricRuntime::IFabricStoreEventHandler,
    FabricTypes::{FABRIC_CLIENT_ROLE, FABRIC_LOCAL_STORE_KIND, FABRIC_REPLICATOR_SETTINGS},
};
use windows_core::{Interface, Param};

lazy_static::lazy_static! {
    static ref LIB_TABLE: LibTable = LibTable::create();
    /// All SF APIs entrypoints needed for mssf.
    /// These APIs are lazy loaded at the first time use after app starts.
    pub static ref API_TABLE: ApiTable = ApiTable::create(&LIB_TABLE);
}

/// Contains all the SF shared libs needs to be loaded for mssf.
pub struct LibTable {
    fabric_runtime: libloading::Library,
    fabric_common: libloading::Library,
    fabric_client: libloading::Library,
}

impl LibTable {
    fn create() -> Self {
        Self {
            fabric_runtime: load_lib("FabricRuntime"),
            fabric_common: load_lib("FabricCommon"),
            fabric_client: load_lib("FabricClient"),
        }
    }
}

fn load_lib(name: &str) -> libloading::Library {
    unsafe { libloading::Library::new(libloading::library_filename(name)) }
        .unwrap_or_else(|e| panic!("cannot load lib {name} :{e}"))
}

fn load_fn<T>(lib: &'static libloading::Library, name: &str) -> libloading::Symbol<'static, T> {
    unsafe { lib.get(name.as_bytes()) }.unwrap_or_else(|e| panic!("cannot load fn {name} :{e}"))
}

/// Contains all SF APIs loaded from SF libs needed for mssf.
/// More APIs can be added here when mssf needs them.
pub struct ApiTable {
    fabric_get_last_error_message_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(message: *mut *mut core::ffi::c_void) -> crate::HRESULT,
    >,
    fabric_create_client3_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            connectionstringssize: u16,
            connectionstrings: *const windows_core::PCWSTR,
            __midl__fabricclientmodule0002: *mut core::ffi::c_void,
            __midl__fabricclientmodule0003: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            fabricclient: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,
    fabric_create_local_client3_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            __midl__fabricclientmodule0004: *mut core::ffi::c_void,
            __midl__fabricclientmodule0005: *mut core::ffi::c_void,
            iid: *const windows_core::GUID,
            fabricclient: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,

    fabric_create_local_client4_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            __midl__fabricclientmodule0006: *mut core::ffi::c_void,
            __midl__fabricclientmodule0007: *mut core::ffi::c_void,
            clientrole: FABRIC_CLIENT_ROLE,
            iid: *const windows_core::GUID,
            fabricclient: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,

    fabric_create_runtime_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            riid: *const windows_core::GUID,
            fabricruntime: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,

    fabric_get_activation_context_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            riid: *const windows_core::GUID,
            activationcontext: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,

    fabric_begin_get_node_context_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            timeoutmilliseconds: u32,
            callback: *mut core::ffi::c_void,
            context: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,

    fabric_end_get_node_context_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            context: *mut core::ffi::c_void,
            nodecontext: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,
    fabric_get_node_context_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(nodecontext: *mut *mut core::ffi::c_void) -> crate::HRESULT,
    >,
    fabric_create_key_value_store_replica_fn: libloading::Symbol<
        'static,
        unsafe extern "system" fn(
            riid: *const windows_core::GUID,
            storename: windows_core::PCWSTR,
            partitionid: windows_core::GUID,
            replicaid: i64,
            replicatorsettings: *const FABRIC_REPLICATOR_SETTINGS,
            localstorekind: FABRIC_LOCAL_STORE_KIND,
            localstoresettings: *const core::ffi::c_void,
            storeeventhandler: *mut core::ffi::c_void,
            keyvaluestore: *mut *mut core::ffi::c_void,
        ) -> crate::HRESULT,
    >,
}

impl ApiTable {
    fn create(lib_table: &'static LibTable) -> Self {
        Self {
            fabric_get_last_error_message_fn: load_fn(
                &lib_table.fabric_common,
                "FabricGetLastErrorMessage",
            ),
            fabric_create_client3_fn: load_fn(&lib_table.fabric_client, "FabricCreateClient3"),
            fabric_create_local_client3_fn: load_fn(
                &lib_table.fabric_client,
                "FabricCreateLocalClient3",
            ),
            fabric_create_local_client4_fn: load_fn(
                &lib_table.fabric_client,
                "FabricCreateLocalClient4",
            ),
            fabric_create_runtime_fn: load_fn(&lib_table.fabric_runtime, "FabricCreateRuntime"),
            fabric_get_activation_context_fn: load_fn(
                &lib_table.fabric_runtime,
                "FabricGetActivationContext",
            ),
            fabric_begin_get_node_context_fn: load_fn(
                &lib_table.fabric_runtime,
                "FabricBeginGetNodeContext",
            ),
            fabric_end_get_node_context_fn: load_fn(
                &lib_table.fabric_runtime,
                "FabricEndGetNodeContext",
            ),
            fabric_get_node_context_fn: load_fn(&lib_table.fabric_runtime, "FabricGetNodeContext"),
            fabric_create_key_value_store_replica_fn: load_fn(
                &lib_table.fabric_runtime,
                "FabricCreateKeyValueStoreReplica",
            ),
        }
    }

    pub fn fabric_get_last_error_message(&self) -> crate::WinResult<IFabricStringResult> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe { (self.fabric_get_last_error_message_fn)(std::ptr::addr_of_mut!(result)) }.ok()?;
        assert!(!result.is_null());
        Ok(unsafe { IFabricStringResult::from_raw(result) })
    }

    pub fn fabric_create_client3<T: Interface>(
        &self,
        connectionstrings: &[windows_core::PCWSTR],
        service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
        client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
    ) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_create_client3_fn)(
                connectionstrings.len().try_into().unwrap(),
                connectionstrings.as_ptr(),
                service_notification_handler.param().abi(),
                client_connection_handler.param().abi(),
                &T::IID,
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_create_local_client3<T: Interface>(
        &self,
        service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
        client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
    ) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_create_local_client3_fn)(
                service_notification_handler.param().abi(),
                client_connection_handler.param().abi(),
                &T::IID,
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_create_local_client4<T: Interface>(
        &self,
        service_notification_handler: Option<&IFabricServiceNotificationEventHandler>,
        client_connection_handler: Option<&IFabricClientConnectionEventHandler>,
        clientrole: FABRIC_CLIENT_ROLE,
    ) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_create_local_client4_fn)(
                service_notification_handler.param().abi(),
                client_connection_handler.param().abi(),
                clientrole,
                &T::IID,
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_create_runtime<T: Interface>(&self) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe { (self.fabric_create_runtime_fn)(&T::IID, std::ptr::addr_of_mut!(result)) }.ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_get_activation_context<T: Interface>(&self) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe { (self.fabric_get_activation_context_fn)(&T::IID, std::ptr::addr_of_mut!(result)) }
            .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_begin_get_node_context(
        &self,
        timeoutmilliseconds: u32,
        callback: Option<&IFabricAsyncOperationCallback>,
    ) -> crate::WinResult<IFabricAsyncOperationContext> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_begin_get_node_context_fn)(
                timeoutmilliseconds,
                callback.param().abi(),
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { IFabricAsyncOperationContext::from_raw(result) })
    }

    pub fn fabric_end_get_node_context<T: Interface>(
        &self,
        context: Option<&IFabricAsyncOperationContext>,
    ) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_end_get_node_context_fn)(
                context.param().abi(),
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    pub fn fabric_get_node_context<T: Interface>(&self) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe { (self.fabric_get_node_context_fn)(std::ptr::addr_of_mut!(result)) }.ok()?;
        Ok(unsafe { T::from_raw(result) })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fabric_create_key_value_store_replica<T: Interface>(
        &self,
        storename: windows_core::PCWSTR,
        partitionid: windows_core::GUID,
        replicaid: i64,
        replicatorsettings: *const FABRIC_REPLICATOR_SETTINGS,
        localstorekind: FABRIC_LOCAL_STORE_KIND,
        localstoresettings: *const core::ffi::c_void,
        storeeventhandler: Option<&IFabricStoreEventHandler>,
    ) -> crate::WinResult<T> {
        let mut result = std::ptr::null_mut::<core::ffi::c_void>();
        unsafe {
            (self.fabric_create_key_value_store_replica_fn)(
                &T::IID,
                storename,
                partitionid,
                replicaid,
                replicatorsettings,
                localstorekind,
                localstoresettings,
                storeeventhandler.param().abi(),
                std::ptr::addr_of_mut!(result),
            )
        }
        .ok()?;
        Ok(unsafe { T::from_raw(result) })
    }
}
