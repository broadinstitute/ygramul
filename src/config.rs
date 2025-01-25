use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;
use crate::cli::CliOptions;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
#[serde(try_from = "&str")]
pub enum Action {
    Hello,
    Survey,
    Ping
}

pub struct Neo4jConfig {
    pub(crate) uri: String,
    pub(crate) user: String,
    pub(crate) password: String
}
pub struct Config {
    pub(crate) action: Action,
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
    action: Option<Action>,
    data_dir: Option<PathBuf>,
    neo4j: Option<Neo4jConfigBuilder>,
}

impl Neo4jConfigBuilder {
    pub fn new() -> Neo4jConfigBuilder {
        let uri: Option<String> = None;
        let user: Option<String> = None;
        let password: Option<String> = None;
        Neo4jConfigBuilder {
            uri, user, password
        }
    }
    pub fn build(self) -> Result<Neo4jConfig, Error> {
        let uri = self.uri.ok_or(Error::from("No URI specified."))?;
        let user = self.user.ok_or(Error::from("No user specified."))?;
        let password = self.password.ok_or(Error::from("No password specified."))?;
        Ok(Neo4jConfig {
            uri, user, password
        })
    }
}
impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        let action: Option<Action> = None;
        let data_dir: Option<PathBuf> = None;
        let neo4j = Some(Neo4jConfigBuilder::new());
        ConfigBuilder { action, data_dir, neo4j }
    }
    pub fn with_cli_options(self, cli_options: CliOptions) -> ConfigBuilder {
        let mut builder = self;
        if let Some(action) = cli_options.action {
            builder.action = Some(action);
        }
        builder
    }
    pub fn build(self) -> Result<Config, Error> {
        let action = self.action.ok_or(Error::from("No action specified."))?;
        let data_dir =
            self.data_dir
            .ok_or(Error::from("No data directory specified."))?;
        let neo4j =
            self.neo4j.ok_or(Error::from("No Neo4j configuration specified."))?.build()?;
        Ok(Config { action, data_dir, neo4j })
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

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "hello" => Ok(Action::Hello),
            "survey" => Ok(Action::Survey),
            "ping" => Ok(Action::Ping),
            _ => Err(Error::from(format!("Unknown action: {}", value))),
        }
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
