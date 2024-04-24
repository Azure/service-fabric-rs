// Integrating service fabric config package with config-rs
//

use config::{ConfigError, Source};
use log::info;

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
        settings
            .sections
            .iter()
            .enumerate()
            .for_each(|(_, section)| {
                let section_name = section.name.to_string();
                info!("Section: {}", section_name);
                section.parameters.iter().enumerate().for_each(|(_, p)| {
                    let param_name = p.name.to_string();
                    let param_val = p.value.to_string();
                    info!("Param: {:?}", param_name);
                    let val = config::Value::new(
                        Some(&uri_origion),
                        config::ValueKind::String(param_val),
                    );
                    // section and param is separated by a dot.
                    res.insert(section_name.clone() + "." + &param_name, val);
                })
            });
        Ok(res)
    }
}
