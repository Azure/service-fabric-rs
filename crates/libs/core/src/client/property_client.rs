// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![cfg_attr(
    not(feature = "tokio_async"),
    allow(unused_imports, reason = "code configured out")
)]

use std::time::Duration;

#[cfg(feature = "tokio_async")]
use crate::{
    sync::fabric_begin_end_proxy,
    sync::CancellationToken,
    sync::FabricReceiver,
    types::{NameEnumerationResult, Uri},
    types::{PropertyMetadataResult, PropertyValueResult},
    WString,
};
#[cfg(feature = "tokio_async")]
use mssf_com::{
    FabricClient::{
        IFabricNameEnumerationResult, IFabricPropertyBatchResult, IFabricPropertyEnumerationResult,
        IFabricPropertyMetadataResult, IFabricPropertyValueResult,
    },
    FabricTypes::{FABRIC_PROPERTY_BATCH_OPERATION, FABRIC_PUT_CUSTOM_PROPERTY_OPERATION},
};

use mssf_com::FabricClient::IFabricPropertyManagementClient2;

#[derive(Debug, Clone)]
pub struct PropertyManagementClient {
    com: IFabricPropertyManagementClient2,
}

impl From<IFabricPropertyManagementClient2> for PropertyManagementClient {
    fn from(com: IFabricPropertyManagementClient2) -> Self {
        Self { com }
    }
}

impl From<PropertyManagementClient> for IFabricPropertyManagementClient2 {
    fn from(value: PropertyManagementClient) -> Self {
        value.com
    }
}

#[cfg(feature = "tokio_async")]
impl PropertyManagementClient {
    fn create_name_internal(
        &self,
        name: &Uri,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginCreateName(name.as_raw(), timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndCreateName(ctx) },
            cancellation_token,
        )
    }

    fn delete_name_internal(
        &self,
        name: &Uri,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginDeleteName(name.as_raw(), timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndDeleteName(ctx) },
            cancellation_token,
        )
    }

    fn name_exists_internal(
        &self,
        name: &Uri,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<u8>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginNameExists(name.as_raw(), timeout_milliseconds, callback)
            },
            move |ctx| unsafe { com2.EndNameExists(ctx) },
            cancellation_token,
        )
    }

    fn enumerate_sub_names_internal(
        &self,
        name: &Uri,
        prev: Option<&IFabricNameEnumerationResult>,
        recursive: bool,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricNameEnumerationResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginEnumerateSubNames(
                    name.as_raw(),
                    prev,
                    recursive,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndEnumerateSubNames(ctx) },
            cancellation_token,
        )
    }

    fn put_property_binary_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &[u8],
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutPropertyBinary(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    data,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutPropertyBinary(ctx) },
            cancellation_token,
        )
    }

    fn put_property_int64_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        data: i64,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutPropertyInt64(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    data,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutPropertyInt64(ctx) },
            cancellation_token,
        )
    }

    fn put_property_double_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        data: f64,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutPropertyDouble(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    data,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutPropertyDouble(ctx) },
            cancellation_token,
        )
    }

    fn put_property_wstring_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &WString,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutPropertyWString(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    data.as_pcwstr(),
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutPropertyWString(ctx) },
            cancellation_token,
        )
    }

    fn put_property_guid_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &windows_core::GUID,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutPropertyGuid(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    data,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutPropertyGuid(ctx) },
            cancellation_token,
        )
    }

    fn delete_property_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginDeleteProperty(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndDeleteProperty(ctx) },
            cancellation_token,
        )
    }

    fn get_property_metadata_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricPropertyMetadataResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetPropertyMetadata(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndGetPropertyMetadata(ctx) },
            cancellation_token,
        )
    }

    fn get_property_internal(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricPropertyValueResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginGetProperty(
                    name.as_raw(),
                    property_name.as_pcwstr(),
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndGetProperty(ctx) },
            cancellation_token,
        )
    }

    // TODO: implement this
    // Batch operations are not supported yet.
    #[allow(dead_code)]
    fn submit_property_batch_internal(
        &self,
        name: &Uri,
        batch: &[FABRIC_PROPERTY_BATCH_OPERATION],
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<(u32, IFabricPropertyBatchResult)>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginSubmitPropertyBatch(name.as_raw(), batch, timeout_milliseconds, callback)
            },
            move |ctx| unsafe {
                let mut failed_operation_index_in_request = 0;
                let result =
                    com2.EndSubmitPropertyBatch(ctx, &mut failed_operation_index_in_request);
                result.map(|res| (failed_operation_index_in_request, res))
            },
            cancellation_token,
        )
    }

    // TODO: implement this
    #[allow(dead_code)]
    fn enumerate_properties_internal(
        &self,
        name: &Uri,
        include_values: bool,
        prev: Option<&IFabricPropertyEnumerationResult>,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<IFabricPropertyEnumerationResult>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginEnumerateProperties(
                    name.as_raw(),
                    include_values,
                    prev,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndEnumerateProperties(ctx) },
            cancellation_token,
        )
    }

    // TODO: implement this
    #[allow(dead_code)]
    fn put_custom_property_operation_internal(
        &self,
        name: &Uri,
        property_operation: &FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeout_milliseconds: u32,
        cancellation_token: Option<CancellationToken>,
    ) -> FabricReceiver<crate::WinResult<()>> {
        let com1 = &self.com;
        let com2 = self.com.clone();
        fabric_begin_end_proxy(
            move |callback| unsafe {
                com1.BeginPutCustomPropertyOperation(
                    name.as_raw(),
                    property_operation,
                    timeout_milliseconds,
                    callback,
                )
            },
            move |ctx| unsafe { com2.EndPutCustomPropertyOperation(ctx) },
            cancellation_token,
        )
    }
}

#[cfg(feature = "tokio_async")]
impl PropertyManagementClient {
    /// Creates a SF name in Naming Service.
    /// Provisioned app and service will automatically create names:
    /// For example, fabric:/myapp/mysvc service will create 2 names in the same hierarchy:
    /// - fabric:/myapp
    /// - fabric:/myapp/mysvc
    /// 
    /// One can create names not related to any app or service as well.
    pub async fn create_name(
        &self,
        name: &Uri,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.create_name_internal(
            name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Deletes a SF name from Naming Service.
    /// All properties needs to be deleted first before this call,
    /// otherwise it will fail.
    pub async fn delete_name(
        &self,
        name: &Uri,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.delete_name_internal(
            name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Checks if a SF name exists in Naming Service.
    pub async fn name_exists(
        &self,
        name: &Uri,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<bool> {
        self.name_exists_internal(
            name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await?
        .map_err(|e| e.into())
        .map(|exist| exist != 0)
    }

    /// Enumerates sub-names of a SF name in Naming Service.
    /// For example, if you have a name `fabric:/myapp`,
    /// it will return all sub-names like:
    /// - fabric:/myapp/mysvc1
    pub async fn enumerate_sub_names(
        &self,
        name: &Uri,
        prev: Option<&NameEnumerationResult>,
        recursive: bool,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<NameEnumerationResult> {
        self.enumerate_sub_names_internal(
            name,
            prev.map(|x| x.as_com()),
            recursive,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await?
        .map_err(|e| e.into())
        .map(NameEnumerationResult::from_com)
    }

    /// Put a binary property to a SF name.
    pub async fn put_property_binary(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &[u8],
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.put_property_binary_internal(
            name,
            property_name,
            data,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Put a double property to a SF name.
    pub async fn put_property_double(
        &self,
        name: &Uri,
        property_name: &WString,
        data: f64,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.put_property_double_internal(
            name,
            property_name,
            data,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Put an int64 property to a SF name.
    pub async fn put_property_int64(
        &self,
        name: &Uri,
        property_name: &WString,
        data: i64,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.put_property_int64_internal(
            name,
            property_name,
            data,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Put a wstring property to a SF name.
    pub async fn put_property_wstring(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &WString,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.put_property_wstring_internal(
            name,
            property_name,
            data,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Put a GUID property to a SF name.
    pub async fn put_property_guid(
        &self,
        name: &Uri,
        property_name: &WString,
        data: &windows_core::GUID,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.put_property_guid_internal(
            name,
            property_name,
            data,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Deletes a property from a SF name.
    pub async fn delete_property(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<()> {
        self.delete_property_internal(
            name,
            property_name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await??;
        Ok(())
    }

    /// Gets metadata of a property from a SF name.
    pub async fn get_property_metadata(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<PropertyMetadataResult> {
        self.get_property_metadata_internal(
            name,
            property_name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await?
        .map_err(|e| e.into())
        .map(PropertyMetadataResult::from_com)
    }

    /// Gets a property value from a SF name.
    pub async fn get_property(
        &self,
        name: &Uri,
        property_name: &WString,
        timeout: Duration,
        cancellation_token: Option<crate::sync::CancellationToken>,
    ) -> crate::Result<PropertyValueResult> {
        self.get_property_internal(
            name,
            property_name,
            timeout.as_millis().try_into().unwrap(),
            cancellation_token,
        )
        .await?
        .map_err(|e| e.into())
        .map(PropertyValueResult::from_com)
    }
}
