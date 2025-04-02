// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// Integrating service fabric config package with config-rs.
// Wraps the SF ConfigurationPackage as a Source for config-rs.
// config-rs can load from SF and user can use all higher level
// features of config-rs.

use config::{ConfigError, Source};

use crate::runtime::config::ConfigurationPackage;
pub use config::Config;

/// Integrate with config-rs
/// Example:
/// let source = FabricConfigSource::new(config);
/// let s = config::Config::builder()
/// .add_source(source)
/// .build()?
///
#[derive(Debug, Clone)]
pub struct FabricConfigSource {
    inner: ConfigurationPackage,
}

impl FabricConfigSource {
    pub fn new(c: ConfigurationPackage) -> Self {
        Self { inner: c }
    }
}

impl Source for FabricConfigSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let uri_origion = String::from("fabric source");
        let mut res = config::Map::new();
        let settings = self.inner.get_settings();
        settings.sections.iter().for_each(|section| {
            let section_name = section.name.to_string();
            section.parameters.iter().for_each(|p| {
                let param_name = p.name.to_string();
                let param_val = p.value.to_string();
                #[cfg(feature="tracing")]
                tracing::debug!("Section: {} Param: {} Val: {}", section_name, param_name, param_val);
                let val =
                    config::Value::new(Some(&uri_origion), config::ValueKind::String(param_val));
                // section and param is separated by a dot.
                res.insert(section_name.clone() + "." + &param_name, val);
            })
        });
        Ok(res)
    }
}
