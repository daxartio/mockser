use std::path::PathBuf;

use config::{Config, ConfigError, Environment};
use log::kv::ToValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_config_port")]
    pub config_port: u16,
    #[serde(default = "default_host")]
    pub host: String,

    pub initial_configs: Option<PathBuf>,
}

fn default_port() -> u16 {
    3000
}

fn default_config_port() -> u16 {
    3001
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

impl Settings {
    pub fn new(prefix: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix(prefix))
            .build()?;

        s.try_deserialize()
    }
}

impl ToValue for Settings {
    fn to_value(&self) -> log::kv::Value {
        log::kv::Value::from_serde(self)
    }
}
