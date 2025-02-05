use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;
use crate::cli::CliOptions;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
#[serde(try_from = "&str")]
pub enum Action {
    Hello,
    Survey,
    Ping,
    Upload,
    Wipe
}

pub struct Neo4jConfig {
    pub(crate) uri: String,
    pub(crate) user: String,
    pub(crate) password: String
}
pub enum ActionConfig {
    Hello(LocalConfig),
    Survey(LocalConfig),
    Ping(ClientConfig),
    Upload(ClientConfig),
    Wipe(ClientConfig),
}
pub struct LocalConfig {
    pub(crate) data_dir: PathBuf,
}

pub struct ClientConfig {
    pub(crate) local_config: LocalConfig,
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
        let uri = self.uri.ok_or(Error::from("No URI (neo4j/uri) )specified."))?;
        let user = self.user.ok_or(Error::from("No user (neo4j/user) specified."))?;
        let password =
            self.password.ok_or(Error::from("No password (neo4j/password) specified."))?;
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
    pub fn neo4j_mut(&mut self) -> &mut Neo4jConfigBuilder {
        self.neo4j.get_or_insert_with(Neo4jConfigBuilder::new)
    }
    pub fn with_cli_options(self, cli_options: CliOptions) -> ConfigBuilder {
        let mut builder = self;
        if let Some(action) = cli_options.action {
            builder.action = Some(action);
        }
        if let Some(data_dir) = cli_options.args.data_dir {
            builder.data_dir = Some(data_dir);
        }
        if let Some(uri) = cli_options.args.uri {
            builder.neo4j_mut().uri = Some(uri);
        }
        if let Some(user) = cli_options.args.user {
            builder.neo4j_mut().user = Some(user);
        }
        if let Some(password) = cli_options.args.password {
            builder.neo4j_mut().password = Some(password);
        }
        builder
    }
    pub fn build(self) -> Result<ActionConfig, Error> {
        let action = self.action.ok_or(Error::from("No action specified."))?;
        let data_dir =
            self.data_dir
            .ok_or(Error::from("No data directory (data_dir) specified."))?;
        match action {
            Action::Hello => {
                Ok(ActionConfig::Hello(LocalConfig { data_dir }))
            }
            Action::Survey => {
                Ok(ActionConfig::Survey(LocalConfig { data_dir }))
            }
            Action::Ping => {
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(self.neo4j)?;
                Ok(ActionConfig::Ping(ClientConfig { local_config, neo4j }))
            }
            Action::Upload => {
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(self.neo4j)?;
                Ok(ActionConfig::Upload(ClientConfig { local_config, neo4j }))
            }
            Action::Wipe => {
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(self.neo4j)?;
                Ok(ActionConfig::Wipe(ClientConfig { local_config, neo4j }))
            }
        }
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
            "upload" => Ok(Action::Upload),
            "wipe" => Ok(Action::Wipe),
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

fn neo4j_config(builder: Option<Neo4jConfigBuilder>) -> Result<Neo4jConfig, Error> {
    builder.ok_or(Error::from("No Neo4j configuration (neo4j) specified."))?.build()
}
