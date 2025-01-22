use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

pub struct Neo4jConfig {
    pub(crate) uri: String,
    pub(crate) user: String,
    pub(crate) password: String,
}
pub struct Config {
    pub(crate) data_dir: PathBuf,
    pub(crate) neo4j: Neo4jConfig,
}

#[derive(Deserialize)]
pub struct Neo4jConfigBuilder {
    uri: Option<String>,
    user: Option<String>,
    password: Option<String>,
}
#[derive(Deserialize)]
pub struct ConfigBuilder {
    data_dir: Option<PathBuf>,
    neo4j: Neo4jConfigBuilder,
}

impl Neo4jConfigBuilder {
    pub fn new() -> Neo4jConfigBuilder {
        let uri: Option<String> = None;
        let user: Option<String> = None;
        let password: Option<String> = None;
        Neo4jConfigBuilder {
            uri,
            user,
            password,
        }
    }
    pub fn build(self) -> Result<Neo4jConfig, Error> {
        let uri = self.uri.ok_or(Error::from("No URI specified."))?;
        let user = self.user.ok_or(Error::from("No user specified."))?;
        let password = self.password.ok_or(Error::from("No password specified."))?;
        Ok(Neo4jConfig {
            uri,
            user,
            password,
        })
    }
    pub fn with_fallback(&self, fallback: &Neo4jConfigBuilder) -> Neo4jConfigBuilder {
        let uri = fall_back(&self.uri, &fallback.uri);
        let user = fall_back(&self.user, &fallback.user);
        let password = fall_back(&self.password, &fallback.password);
        Neo4jConfigBuilder {
            uri,
            user,
            password,
        }
    }
}
impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        let data_dir: Option<PathBuf> = None;
        let neo4j = Neo4jConfigBuilder::new();
        ConfigBuilder { data_dir, neo4j }
    }
    pub fn build(self) -> Result<Config, Error> {
        let data_dir = self
            .data_dir
            .ok_or(Error::from("No data directory specified."))?;
        let neo4j = self.neo4j.build()?;
        Ok(Config { data_dir, neo4j })
    }
    pub fn with_fallback(&self, fallback: &ConfigBuilder) -> ConfigBuilder {
        let data_dir = fall_back(&self.data_dir, &fallback.data_dir);
        let neo4j = self.neo4j.with_fallback(&fallback.neo4j);
        ConfigBuilder { data_dir, neo4j }
    }
}

fn fall_back<T: Clone>(opt1: &Option<T>, opt2: &Option<T>) -> Option<T> {
    match (opt1, opt2) {
        (Some(_), _) => opt1.clone(),
        _ => opt2.clone(),
    }
}

impl Default for Neo4jConfigBuilder {
    fn default() -> Self {
        Neo4jConfigBuilder::new()
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
            Err(error) => Err(Error::wrap(
                "Failed to parse configuration.".to_string(),
                error,
            )),
        }
    }
}
