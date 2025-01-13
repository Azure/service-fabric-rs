// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricRuntime::{
        IFabricCodePackageActivationContext6, IFabricConfigurationPackageChangeHandler,
    },
    FabricTypes::{FABRIC_HEALTH_INFORMATION, FABRIC_HEALTH_REPORT_SEND_OPTIONS},
};

use crate::{
    strings::WStringWrap,
    types::{EndpointResourceDescription, HealthInformation, HealthReportSendOption},
    Error, WString, PCWSTR,
};

use super::{
    config::ConfigurationPackage,
    package_change::{
        config::{
            ConfigurationPackageChangeCallbackHandle, ConfigurationPackageChangeEventHandlerBridge,
            LambdaConfigurationPackageEventHandler,
        },
        ConfigurationPackageChangeEvent,
    },
};

#[derive(Debug, Clone)]
pub struct CodePackageActivationContext {
    com_impl: IFabricCodePackageActivationContext6,
}

/// Code package information is propagated here from the ActivationContext
/// provided when the service is started. This information can be used to setup
/// log directories and other resources needed which require information
/// about the code package and the local runtime environment.
#[derive(Debug, Clone)]
pub struct CodePackageInfo {
    pub context_id: WString,
    pub code_package_name: WString,
    pub code_package_version: WString,
    pub work_directory: WString,
    pub log_directory: WString,
    pub temp_directory: WString,
    pub application_name: WString,
    pub application_type_name: WString,
    pub service_listen_address: WString,
    pub service_publish_address: WString,
}

impl CodePackageActivationContext {
    pub fn create() -> Result<CodePackageActivationContext, Error> {
        let com = super::get_com_activation_context::<IFabricCodePackageActivationContext6>()?;
        Ok(Self::from(com))
    }

    pub fn get_endpoint_resource(
        &self,
        serviceendpointresourcename: &WString,
    ) -> crate::Result<EndpointResourceDescription> {
        let rs = unsafe {
            self.com_impl.GetServiceEndpointResource(PCWSTR::from_raw(
                serviceendpointresourcename.as_ptr(),
            ))?
        };
        let res_ref = unsafe { rs.as_ref().unwrap() };
        let desc = EndpointResourceDescription::from(res_ref);
        Ok(desc)
    }

    pub fn get_configuration_package(
        &self,
        configpackagename: &WString,
    ) -> crate::Result<ConfigurationPackage> {
        let c = unsafe {
            self.com_impl
                .GetConfigurationPackage(configpackagename.as_pcwstr())
        }?;
        Ok(ConfigurationPackage::from(c))
    }

    pub fn get_code_package_info(&self) -> CodePackageInfo {
        CodePackageInfo {
            context_id: WStringWrap::from(unsafe { self.com_impl.get_ContextId() }).into(),
            code_package_name: WStringWrap::from(unsafe { self.com_impl.get_CodePackageName() })
                .into(),
            code_package_version: WStringWrap::from(unsafe {
                self.com_impl.get_CodePackageVersion()
            })
            .into(),
            work_directory: WStringWrap::from(unsafe { self.com_impl.get_WorkDirectory() }).into(),
            log_directory: WStringWrap::from(unsafe { self.com_impl.get_LogDirectory() }).into(),
            temp_directory: WStringWrap::from(unsafe { self.com_impl.get_TempDirectory() }).into(),
            application_name: WStringWrap::from(PCWSTR(unsafe {
                self.com_impl.get_ApplicationName().0
            }))
            .into(),
            application_type_name: WStringWrap::from(unsafe {
                self.com_impl.get_ApplicationTypeName()
            })
            .into(),
            service_listen_address: WStringWrap::from(unsafe {
                self.com_impl.get_ServiceListenAddress()
            })
            .into(),
            service_publish_address: WStringWrap::from(unsafe {
                self.com_impl.get_ServicePublishAddress()
            })
            .into(),
        }
    }

    /// The health information describes the report details, like the source ID, the property,
    /// the health state and other relevant details. The code package activation context uses an
    /// internal health client to send the reports to the health store. The client optimizes messages to
    /// Health Manager by batching reports per a configured duration (Default: 30 seconds).
    /// If the report has high priority, you can specify send options to send it immediately.
    ///
    /// Possible Errors:
    ///     FABRIC_E_HEALTH_STALE_REPORT:
    ///         HealthReport already exist for the same entity,
    ///         SourceId and Property with same or higher SequenceNumber.
    ///     FABRIC_E_HEALTH_MAX_REPORTS_REACHED:
    ///         HeathClient has reached the maximum number of health reports
    ///         that can accept for processing. More reports will be accepted when progress is done
    ///         with the currently accepted reports. By default, the FabricClient.HealthClient can accept 10000 different reports.
    pub fn report_application_health(
        &self,
        healthinfo: &HealthInformation,
        send_options: Option<&HealthReportSendOption>,
    ) -> crate::Result<()> {
        let raw: FABRIC_HEALTH_INFORMATION = healthinfo.into();
        let send_options = send_options.map(FABRIC_HEALTH_REPORT_SEND_OPTIONS::from);
        let raw_options = match send_options.as_ref() {
            Some(opt) => opt as *const FABRIC_HEALTH_REPORT_SEND_OPTIONS,
            None => std::ptr::null(),
        };
        unsafe { self.com_impl.ReportApplicationHealth2(&raw, raw_options) }
    }

    pub fn get_com(&self) -> IFabricCodePackageActivationContext6 {
        self.com_impl.clone()
    }

    /// Register a configuration package change handler callback
    /// Consider using [`AutoConfigurationPackageChangeCallbackHandle::new`] instead of this directly.
    pub fn register_configuration_package_change_handler<T>(
        &self,
        handler: T,
    ) -> crate::Result<ConfigurationPackageChangeCallbackHandle>
    where
        T: Fn(&ConfigurationPackageChangeEvent) + 'static,
    {
        let lambda_handler = LambdaConfigurationPackageEventHandler::new(handler);
        let bridge = ConfigurationPackageChangeEventHandlerBridge::new(lambda_handler);
        let callback: IFabricConfigurationPackageChangeHandler = bridge.into();
        // SAFETY: bridge implements the required COM interface
        let raw_handle = unsafe {
            self.com_impl
                .RegisterConfigurationPackageChangeHandler(&callback)
        }?;
        // SAFETY: raw_handle is a configuration package change handler id, not some other id.
        Ok(unsafe { ConfigurationPackageChangeCallbackHandle::from(raw_handle) })
    }

    pub fn unregister_configuration_package_change_handler(
        &self,
        handle: ConfigurationPackageChangeCallbackHandle,
    ) -> crate::Result<()> {
        // SAFETY: we assume that only 1 activation context can be
        unsafe {
            self.com_impl
                .UnregisterConfigurationPackageChangeHandler(handle.0)
        }
        .unwrap();
        Ok(())
    }
}

impl From<IFabricCodePackageActivationContext6> for CodePackageActivationContext {
    fn from(value: IFabricCodePackageActivationContext6) -> Self {
        CodePackageActivationContext { com_impl: value }
    }
}
