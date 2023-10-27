use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub server: CliConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub url: String,
    pub port: u16,
}