use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::binaries::BinariesConfig;
use crate::config::desktop::{GSettingsConfig, GreetdConfig, MetaConfig};
use crate::config::packages::PackagesConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdaterConfig {
    #[serde(default)]
    pub meta: MetaConfig,
    #[serde(default)]
    pub packages: PackagesConfig,
    #[serde(default)]
    pub binaries: BinariesConfig,
    #[serde(default)]
    pub gsettings: GSettingsConfig,
    #[serde(default)]
    pub mime_defaults: HashMap<String, String>,
    #[serde(default)]
    pub greetd: GreetdConfig,
}
