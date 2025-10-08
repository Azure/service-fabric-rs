// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{WString, error::ErrorCode, strings::StringResult};
use mssf_com::{
    FabricRuntime::IFabricConfigurationPackage,
    FabricTypes::{
        FABRIC_CONFIGURATION_PARAMETER, FABRIC_CONFIGURATION_PARAMETER_EX1,
        FABRIC_CONFIGURATION_PARAMETER_LIST, FABRIC_CONFIGURATION_SECTION,
        FABRIC_CONFIGURATION_SECTION_LIST,
    },
};

use crate::iter::{FabricIter, FabricListAccessor};

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
    pub sections: ConfigurationSectionList,
}

// FABRIC_CONFIGURATION_SECTION_LIST
pub struct ConfigurationSectionList {
    com: IFabricConfigurationPackage,
}

type ConfigurationSectionListIter<'a> =
    FabricIter<'a, FABRIC_CONFIGURATION_SECTION, ConfigurationSection, ConfigurationSectionList>;

impl ConfigurationSectionList {
    fn get_section_list_ref(&self) -> &FABRIC_CONFIGURATION_SECTION_LIST {
        let raw = unsafe { self.com.get_Settings().as_ref().unwrap() };
        unsafe { raw.Sections.as_ref().unwrap() }
    }
    pub fn iter(&self) -> ConfigurationSectionListIter<'_> {
        ConfigurationSectionListIter::new(self, self)
    }
}

impl FabricListAccessor<FABRIC_CONFIGURATION_SECTION> for ConfigurationSectionList {
    fn get_count(&self) -> u32 {
        self.get_section_list_ref().Count
    }

    fn get_first_item(&self) -> *const FABRIC_CONFIGURATION_SECTION {
        self.get_section_list_ref().Items
    }
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
        ConfigurationSettings {
            sections: ConfigurationSectionList {
                com: self.com.clone(),
            },
        }
    }

    pub fn get_path(&self) -> WString {
        let raw = unsafe { self.com.get_Path() };
        WString::from(raw)
    }

    pub fn get_section(&self, section_name: &WString) -> crate::Result<ConfigurationSection> {
        let raw = unsafe { self.com.GetSection(section_name.as_pcwstr()) }?;
        let raw_ref = unsafe { raw.as_ref() };
        match raw_ref {
            Some(c) => {
                let mut res = ConfigurationSection::from(c);
                res.owner = Some(self.com.clone());
                Ok(res)
            }
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

// Note: parameter has ptr to raw memory into
// Com obj, but this relationship is not tracked by lifetime,
// So when using config section and parameter list,
// make sure the com obj is still in scope.
// TODO: find a way to make lifetime work.
pub struct ConfigurationSection {
    owner: Option<IFabricConfigurationPackage>,
    pub name: WString,
    pub parameters: ConfigurationParameterList, // Note: the list has no lifetime tracking
}

impl From<&FABRIC_CONFIGURATION_SECTION> for ConfigurationSection {
    fn from(value: &FABRIC_CONFIGURATION_SECTION) -> Self {
        Self {
            owner: None,
            name: WString::from(value.Name),
            parameters: ConfigurationParameterList {
                list: value.Parameters, // TODO: ownership/lifetime escaped here.
            },
        }
    }
}

// FABRIC_CONFIGURATION_PARAMETER_LIST
// TODO: the owner is not accessible.
type ConfigurationParameterListIter<'a> = FabricIter<
    'a,
    FABRIC_CONFIGURATION_PARAMETER,
    ConfigurationParameter,
    ConfigurationParameterList,
>;

pub struct ConfigurationParameterList {
    list: *const FABRIC_CONFIGURATION_PARAMETER_LIST,
}

impl ConfigurationParameterList {
    pub fn iter(&self) -> ConfigurationParameterListIter<'_> {
        ConfigurationParameterListIter::new(self, self)
    }
}

impl FabricListAccessor<FABRIC_CONFIGURATION_PARAMETER> for ConfigurationParameterList {
    fn get_count(&self) -> u32 {
        unsafe { self.list.as_ref().unwrap().Count }
    }

    fn get_first_item(&self) -> *const FABRIC_CONFIGURATION_PARAMETER {
        unsafe { self.list.as_ref().unwrap().Items }
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
