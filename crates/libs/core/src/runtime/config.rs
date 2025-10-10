// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{WString, error::ErrorCode, strings::StringResult};
use mssf_com::{
    FabricRuntime::IFabricConfigurationPackage,
    FabricTypes::{
        FABRIC_CONFIGURATION_PARAMETER, FABRIC_CONFIGURATION_PARAMETER_EX1,
        FABRIC_CONFIGURATION_SECTION,
    },
};

#[derive(Debug, Clone)]
pub struct ConfigurationPackage {
    com: IFabricConfigurationPackage,
}

pub struct ConfigurationPackageDesc {
    pub name: WString,
    pub service_manifest_name: WString,
    pub service_manifest_version: WString,
    pub version: WString,
}

pub struct ConfigurationSettings {
    pub sections: Vec<ConfigurationSection>,
}

impl From<IFabricConfigurationPackage> for ConfigurationPackage {
    fn from(com: IFabricConfigurationPackage) -> Self {
        Self { com }
    }
}

impl From<ConfigurationPackage> for IFabricConfigurationPackage {
    fn from(value: ConfigurationPackage) -> Self {
        value.com
    }
}

impl ConfigurationPackage {
    pub fn get_description(&self) -> ConfigurationPackageDesc {
        let raw = unsafe { self.com.get_Description().as_ref().unwrap() };

        ConfigurationPackageDesc {
            name: WString::from(raw.Name),
            service_manifest_name: WString::from(raw.ServiceManifestName),
            service_manifest_version: WString::from(raw.ServiceManifestVersion),
            version: WString::from(raw.Version),
        }
    }

    pub fn get_settings(&self) -> ConfigurationSettings {
        let sections = unsafe { self.com.get_Settings().as_ref() }
            .map(|list| {
                unsafe { list.Sections.as_ref() }
                    .map(|l| crate::iter::vec_from_raw_com(l.Count as usize, l.Items))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        ConfigurationSettings { sections }
    }

    pub fn get_path(&self) -> WString {
        let raw = unsafe { self.com.get_Path() };
        WString::from(raw)
    }

    pub fn get_section(&self, section_name: &WString) -> crate::Result<ConfigurationSection> {
        let raw = unsafe { self.com.GetSection(section_name.as_pcwstr()) }?;
        let raw_ref = unsafe { raw.as_ref() };
        match raw_ref {
            Some(c) => Ok(ConfigurationSection::from(c)),
            None => Err(ErrorCode::E_POINTER.into()),
        }
    }

    pub fn get_value(
        &self,
        section_name: &WString,
        parameter_name: &WString,
    ) -> crate::Result<(WString, bool)> {
        let mut is_encrypted: u8 = Default::default();
        let raw = unsafe {
            self.com.GetValue(
                section_name.as_pcwstr(),
                parameter_name.as_pcwstr(),
                std::ptr::addr_of_mut!(is_encrypted),
            )
        }?;
        Ok((WString::from(raw), is_encrypted != 0))
    }

    pub fn decrypt_value(&self, encryptedvalue: &WString) -> crate::Result<WString> {
        let s = unsafe { self.com.DecryptValue(encryptedvalue.as_pcwstr()) }?;
        Ok(StringResult::from(&s).into_inner())
    }
}

pub struct ConfigurationSection {
    pub name: WString,
    pub parameters: Vec<ConfigurationParameter>,
}

impl From<&FABRIC_CONFIGURATION_SECTION> for ConfigurationSection {
    fn from(value: &FABRIC_CONFIGURATION_SECTION) -> Self {
        let parameters = unsafe { value.Parameters.as_ref() }
            .map(|list| crate::iter::vec_from_raw_com(list.Count as usize, list.Items))
            .unwrap_or_default();
        Self {
            name: WString::from(value.Name),
            parameters,
        }
    }
}

#[derive(Debug)]
pub struct ConfigurationParameter {
    pub is_encrypted: bool,
    pub must_overrride: bool,
    pub name: WString,
    pub value: WString,
    pub r#type: WString,
}

impl From<&FABRIC_CONFIGURATION_PARAMETER> for ConfigurationParameter {
    fn from(value: &FABRIC_CONFIGURATION_PARAMETER) -> Self {
        let raw1 = unsafe {
            (value.Reserved as *const FABRIC_CONFIGURATION_PARAMETER_EX1)
                .as_ref()
                .unwrap()
        };
        Self {
            name: WString::from(value.Name),
            is_encrypted: value.IsEncrypted,
            must_overrride: value.MustOverride,
            value: WString::from(value.Value),
            r#type: WString::from(raw1.Type),
        }
    }
}
