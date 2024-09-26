// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::BOOLEAN;
use crate::{error::FabricErrorCode, HSTRING};
use mssf_com::{
    FabricRuntime::IFabricConfigurationPackage,
    FabricTypes::{
        FABRIC_CONFIGURATION_PARAMETER, FABRIC_CONFIGURATION_PARAMETER_EX1,
        FABRIC_CONFIGURATION_PARAMETER_LIST, FABRIC_CONFIGURATION_SECTION,
        FABRIC_CONFIGURATION_SECTION_LIST,
    },
};

use crate::{
    iter::{FabricIter, FabricListAccessor},
    strings::HSTRINGWrap,
};

#[derive(Debug, Clone)]
pub struct ConfigurationPackage {
    com: IFabricConfigurationPackage,
}

pub struct ConfigurationPackageDesc {
    pub name: HSTRING,
    pub service_manifest_name: HSTRING,
    pub service_manifest_version: HSTRING,
    pub version: HSTRING,
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
    pub fn iter(&self) -> ConfigurationSectionListIter {
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

impl ConfigurationPackage {
    pub fn from_com(com: IFabricConfigurationPackage) -> Self {
        Self { com }
    }

    pub fn get_description(&self) -> ConfigurationPackageDesc {
        let raw = unsafe { self.com.get_Description().as_ref().unwrap() };

        ConfigurationPackageDesc {
            name: HSTRINGWrap::from(raw.Name).into(),
            service_manifest_name: HSTRINGWrap::from(raw.ServiceManifestName).into(),
            service_manifest_version: HSTRINGWrap::from(raw.ServiceManifestVersion).into(),
            version: HSTRINGWrap::from(raw.Version).into(),
        }
    }

    pub fn get_settings(&self) -> ConfigurationSettings {
        ConfigurationSettings {
            sections: ConfigurationSectionList {
                com: self.com.clone(),
            },
        }
    }

    pub fn get_path(&self) -> HSTRING {
        let raw = unsafe { self.com.get_Path() };
        HSTRINGWrap::from(raw).into()
    }

    pub fn get_section(&self, section_name: &HSTRING) -> crate::Result<ConfigurationSection> {
        let raw = unsafe { self.com.GetSection(section_name) }?;
        let raw_ref = unsafe { raw.as_ref() };
        match raw_ref {
            Some(c) => {
                let mut res = ConfigurationSection::from(c);
                res.owner = Some(self.com.clone());
                Ok(res)
            }
            None => Err(FabricErrorCode::ArgumentNull.into()),
        }
    }

    pub fn get_value(
        &self,
        section_name: &HSTRING,
        parameter_name: &HSTRING,
    ) -> crate::Result<(HSTRING, bool)> {
        let mut is_encrypted: BOOLEAN = Default::default();
        let raw = unsafe {
            self.com.GetValue(
                section_name,
                parameter_name,
                std::ptr::addr_of_mut!(is_encrypted.0),
            )
        }?;
        Ok((HSTRINGWrap::from(raw).into(), is_encrypted.as_bool()))
    }

    pub fn decrypt_value(&self, encryptedvalue: &HSTRING) -> windows_core::Result<HSTRING> {
        let s = unsafe { self.com.DecryptValue(encryptedvalue) }?;
        Ok(HSTRINGWrap::from(&s).into())
    }
}

// Note: parameter has ptr to raw memory into
// Com obj, but this relationship is not tracked by lifetime,
// So when using config section and parameter list,
// make sure the com obj is still in scope.
// TODO: find a way to make lifetime work.
pub struct ConfigurationSection {
    owner: Option<IFabricConfigurationPackage>,
    pub name: HSTRING,
    pub parameters: ConfigurationParameterList, // Note: the list has no lifetime tracking
}

impl From<&FABRIC_CONFIGURATION_SECTION> for ConfigurationSection {
    fn from(value: &FABRIC_CONFIGURATION_SECTION) -> Self {
        Self {
            owner: None,
            name: HSTRINGWrap::from(value.Name).into(),
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
    pub fn iter(&self) -> ConfigurationParameterListIter {
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
    pub name: HSTRING,
    pub value: HSTRING,
    pub r#type: HSTRING,
}

impl From<&FABRIC_CONFIGURATION_PARAMETER> for ConfigurationParameter {
    fn from(value: &FABRIC_CONFIGURATION_PARAMETER) -> Self {
        let raw1 = unsafe {
            (value.Reserved as *const FABRIC_CONFIGURATION_PARAMETER_EX1)
                .as_ref()
                .unwrap()
        };
        Self {
            name: HSTRINGWrap::from(value.Name).into(),
            is_encrypted: value.IsEncrypted.as_bool(),
            must_overrride: value.MustOverride.as_bool(),
            value: HSTRINGWrap::from(value.Value).into(),
            r#type: HSTRINGWrap::from(raw1.Type).into(),
        }
    }
}
