use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

pub struct Creds {
    pub user: String,
    pub password: String
}
pub struct Neo4jConfig {
    pub(crate) uri: String,
    pub(crate) creds: Creds
}
pub struct Config {
    pub(crate) data_dir: PathBuf,
    pub(crate) neo4j: Neo4jConfig,
}

#[derive(Deserialize)]
pub struct CredsBuilder {
    pub user: Option<String>,
    pub password: Option<String>
}

#[derive(Deserialize)]
pub struct Neo4jConfigBuilder {
    uri: Option<String>,
    creds: CredsBuilder,
}
#[derive(Deserialize)]
pub struct ConfigBuilder {
    data_dir: Option<PathBuf>,
    neo4j: Neo4jConfigBuilder,
}

impl CredsBuilder {
    pub fn new() -> CredsBuilder {
        let user: Option<String> = None;
        let password: Option<String> = None;
        CredsBuilder { user, password }
    }
    pub fn build(self) -> Result<Creds, Error> {
        let user = self.user.ok_or(Error::from("No user specified."))?;
        let password = self.password.ok_or(Error::from("No password specified."))?;
        Ok(Creds { user, password })
    }
}
impl Neo4jConfigBuilder {
    pub fn new() -> Neo4jConfigBuilder {
        let uri: Option<String> = None;
        let creds = CredsBuilder::new();
        Neo4jConfigBuilder {
            uri, creds
        }
    }
    pub fn build(self) -> Result<Neo4jConfig, Error> {
        let uri = self.uri.ok_or(Error::from("No URI specified."))?;
        let creds = self.creds.build()?;
        Ok(Neo4jConfig {
            uri, creds
        })
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
}

impl Default for CredsBuilder {
    fn default() -> Self {
        CredsBuilder::new()
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
