use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
pub fn load(path: Option<String>) -> Result<Config, Box<dyn Error>> {
    Ok(match path {
        Some(path) => toml::from_str(&fs::read_to_string(path)?)?,
        None => toml::from_str(&fs::read_to_string("./config.toml")?)?,
    })
}
