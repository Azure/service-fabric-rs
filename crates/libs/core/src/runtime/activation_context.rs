// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_com::{
    FabricRuntime::IFabricCodePackageActivationContext6,
    FabricTypes::{FABRIC_HEALTH_INFORMATION, FABRIC_HEALTH_REPORT_SEND_OPTIONS},
};

use crate::{
    strings::HSTRINGWrap,
    types::{EndpointResourceDescription, HealthInformation, HealthReportSendOption},
    Error, HSTRING, PCWSTR,
};

use super::config::ConfigurationPackage;

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
    pub context_id: HSTRING,
    pub code_package_name: HSTRING,
    pub code_package_version: HSTRING,
    pub work_directory: HSTRING,
    pub log_directory: HSTRING,
    pub temp_directory: HSTRING,
    pub application_name: HSTRING,
    pub application_type_name: HSTRING,
    pub service_listen_address: HSTRING,
    pub service_publish_address: HSTRING,
}

impl CodePackageActivationContext {
    pub fn create() -> Result<CodePackageActivationContext, Error> {
        let com = super::get_com_activation_context::<IFabricCodePackageActivationContext6>()?;
        Ok(Self::from(com))
    }

    pub fn get_endpoint_resource(
        &self,
        serviceendpointresourcename: &HSTRING,
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
        configpackagename: &HSTRING,
    ) -> crate::Result<ConfigurationPackage> {
        let c = unsafe { self.com_impl.GetConfigurationPackage(configpackagename) }?;
        Ok(ConfigurationPackage::from_com(c))
    }

    pub fn get_code_package_info(&self) -> CodePackageInfo {
        CodePackageInfo {
            context_id: HSTRINGWrap::from(unsafe { self.com_impl.get_ContextId() }).into(),
            code_package_name: HSTRINGWrap::from(unsafe { self.com_impl.get_CodePackageName() })
                .into(),
            code_package_version: HSTRINGWrap::from(unsafe {
                self.com_impl.get_CodePackageVersion()
            })
            .into(),
            work_directory: HSTRINGWrap::from(unsafe { self.com_impl.get_WorkDirectory() }).into(),
            log_directory: HSTRINGWrap::from(unsafe { self.com_impl.get_LogDirectory() }).into(),
            temp_directory: HSTRINGWrap::from(unsafe { self.com_impl.get_TempDirectory() }).into(),
            application_name: HSTRINGWrap::from(PCWSTR(unsafe {
                self.com_impl.get_ApplicationName().0
            }))
            .into(),
            application_type_name: HSTRINGWrap::from(unsafe {
                self.com_impl.get_ApplicationTypeName()
            })
            .into(),
            service_listen_address: HSTRINGWrap::from(unsafe {
                self.com_impl.get_ServiceListenAddress()
            })
            .into(),
            service_publish_address: HSTRINGWrap::from(unsafe {
                self.com_impl.get_ServicePublishAddress()
            })
            .into(),
        }
    }

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
}

impl From<IFabricCodePackageActivationContext6> for CodePackageActivationContext {
    fn from(value: IFabricCodePackageActivationContext6) -> Self {
        CodePackageActivationContext { com_impl: value }
    }
}
