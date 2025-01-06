use std::path::PathBuf;
use serde::Deserialize;
use crate::error::Error;

pub struct Config {
    pub(crate) data_dir: PathBuf
}
#[derive(Deserialize)]
pub struct ConfigBuilder {
    data_dir: Option<PathBuf>
}
impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        let data_dir: Option<PathBuf> = None;
        ConfigBuilder { data_dir }
    }
    pub fn build(self) -> Result<Config, Error> {
        let data_dir =
            self.data_dir.ok_or(Error::from("No data directory specified."))?;
        Ok(Config { data_dir})
    }
    pub fn with_fallback(&self, fallback: &ConfigBuilder) -> ConfigBuilder {
        let data_dir = fall_back(&self.data_dir, &fallback.data_dir);
        ConfigBuilder { data_dir }
    }
}

fn fall_back<T: Clone>(opt1: &Option<T>, opt2: &Option<T>) -> Option<T> {
    match (opt1, opt2) {
        (Some(_), _) => opt1.clone(),
        _ => opt2.clone()
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}

impl TryFrom<&str> for ConfigBuilder {
    type Error = Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match toml::from_str(string) {
            Ok(config) => Ok(config),
            Err(error) =>
                Err(Error::wrap("Failed to parse configuration.".to_string(), error))
        }
    }
}