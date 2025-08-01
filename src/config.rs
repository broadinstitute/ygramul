use std::fmt::Display;
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
    Wipe,
    Cat,
    Ls,
    Bulk,
    Factors,
    TraitGeneSets,
}

pub const ACTIONS: [Action; 10] = [
    Action::Hello,
    Action::Survey,
    Action::Ping,
    Action::Upload,
    Action::Wipe,
    Action::Cat,
    Action::Ls,
    Action::Bulk,
    Action::Factors,
    Action::TraitGeneSets,
];

pub(crate) mod action {
    pub(crate) const HELLO: &str = "hello";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const PING: &str = "ping";
    pub(crate) const UPLOAD: &str = "upload";
    pub(crate) const WIPE: &str = "wipe";
    pub(crate) const CAT: &str = "cat";
    pub(crate) const LS: &str = "ls";
    pub(crate) const BULK: &str = "bulk";
    pub(crate) const FACTORS: &str = "factors";
    pub(crate) const PGS: &str = "pgs";
}

mod about {
    pub(crate) const HELLO: &str = "Prints some config information.";
    pub(crate) const SURVEY: &str = "Surveys the data.";
    pub(crate) const PING: &str = "Pings the Neo4j server.";
    pub(crate) const UPLOAD: &str = "Uploads data to the Neo4j server.";
    pub(crate) const WIPE: &str = "Deletes all data on the Neo4j server.";
    pub(crate) const CAT: &str = "Prints the content of the input file.";
    pub(crate) const LS: &str = "Lists the content of a directory.";
    pub(crate) const BULK: &str = "Creates pheno/genes/gene sets bulk files for PIGEAN.";
    pub(crate) const FACTORS: &str = "Creates factors/genes/gene sets bulk files for PIGEAN.";
    pub(crate) const PGS: &str = "Creates trait-gene sets bulk files for PIGEAN.";
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
    Cat(String),
    Ls(String),
    Bulk(PigeanConfig),
    Factors(PigeanConfig),
    TraitGeneSets(PigeanConfig),
}
pub struct LocalConfig {
    pub(crate) data_dir: PathBuf,
}

pub struct ClientConfig {
    pub(crate) local_config: LocalConfig,
    pub(crate) neo4j: Neo4jConfig,
}

pub struct PigeanConfig {
    pub(crate) data_dir: String,
    pub(crate) sub_dir: String,
    pub(crate) factors_dir: String,
    pub(crate) factors_sub_dir: String,
    pub(crate) out: String,
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
    file: Option<String>,
    out: Option<String>,
    pigean: Option<PigeanConfigBuilder>
}
#[derive(Deserialize)]
pub struct PigeanConfigBuilder {
    data_dir: Option<String>,
    sub_dir: Option<String>,
    factors_dir: Option<String>,
    factors_sub_dir: Option<String>,
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
impl PigeanConfigBuilder {
    pub fn new() -> PigeanConfigBuilder {
        let data_dir: Option<String> = None;
        let sub_dir: Option<String> = None;
        let factors_dir: Option<String> = None;
        let factors_sub_dir: Option<String> = None;
        PigeanConfigBuilder { data_dir, sub_dir, factors_dir, factors_sub_dir }
    }
    pub fn build(self, out: String) -> Result<PigeanConfig, Error> {
        let PigeanConfigBuilder {
            data_dir, sub_dir, factors_dir, factors_sub_dir
        } = self;
        let data_dir =
            data_dir.ok_or(Error::from("No PIGEAN data directory specified."))?;
        let sub_dir =
            sub_dir.ok_or(Error::from("No PIGEAN sub directory specified."))?;
        let factors_dir =
            factors_dir.ok_or(Error::from("No PIGEAN factors directory specified."))?;
        let factors_sub_dir =
            factors_sub_dir.ok_or(Error::from("No PIGEAN factors sub directory specified."))?;
        Ok(PigeanConfig {
            data_dir, sub_dir, factors_dir, factors_sub_dir, out
        })
    }
}
impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        let action: Option<Action> = None;
        let data_dir: Option<PathBuf> = None;
        let neo4j = Some(Neo4jConfigBuilder::new());
        let file: Option<String> = None;
        let out: Option<String> = None;
        let pigean = Some(PigeanConfigBuilder::new());
        ConfigBuilder { action, data_dir, neo4j, file, out, pigean }
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
        if let Some(file) = cli_options.args.file {
            builder.file = Some(file);
        }
        if let Some(out) = cli_options.args.out {
            builder.out = Some(out);
        }
        builder
    }
    fn get_action(&self) -> Result<Action, Error> {
        self.action.ok_or(Error::from(
            format!("No action specified. Possible actions are {}.", all_actions_list())
        ))
    }
    pub fn build(self) -> Result<ActionConfig, Error> {
        let action = self.get_action()?;
        match action {
            Action::Hello => {
                let ConfigBuilder { data_dir, .. } = self;
                let data_dir =
                    data_dir.ok_or_else(|| Error::from("No data directory specified."))?;
                Ok(ActionConfig::Hello(LocalConfig { data_dir }))
            }
            Action::Survey => {
                let ConfigBuilder { data_dir, .. } = self;
                let data_dir =
                    data_dir.ok_or_else(|| Error::from("No data directory specified."))?;
                Ok(ActionConfig::Survey(LocalConfig { data_dir }))
            }
            Action::Ping => {
                let ConfigBuilder { data_dir, neo4j, .. }
                    = self;
                let data_dir =
                    data_dir.ok_or_else(|| Error::from("No data directory specified."))?;
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(neo4j)?;
                Ok(ActionConfig::Ping(ClientConfig { local_config, neo4j }))
            }
            Action::Upload => {
                let ConfigBuilder { data_dir, neo4j, .. }
                    = self;
                let data_dir =
                    data_dir.ok_or_else(|| Error::from("No data directory specified."))?;
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(neo4j)?;
                Ok(ActionConfig::Upload(ClientConfig { local_config, neo4j }))
            }
            Action::Wipe => {
                let ConfigBuilder { data_dir, neo4j, .. } 
                    = self;
                let data_dir = 
                    data_dir.ok_or_else(|| Error::from("No data directory specified."))?;
                let local_config = LocalConfig { data_dir };
                let neo4j = neo4j_config(neo4j)?;
                Ok(ActionConfig::Wipe(ClientConfig { local_config, neo4j }))
            }
            Action::Cat => {
                let ConfigBuilder { file, .. } = self;
                let file = file.ok_or_else(|| Error::from("No file specified."))?;
                Ok(ActionConfig::Cat(file))
            }
            Action::Ls => {
                let ConfigBuilder { file, .. } = self;
                let file = file.ok_or_else(|| Error::from("No directory specified."))?;
                Ok(ActionConfig::Ls(file))
            }
            Action::Bulk => {
                let ConfigBuilder { pigean, out, .. } = self;
                let pigean = 
                    pigean.ok_or_else(|| Error::from("No PIGEAN configuration specified."))?;
                let out = 
                    out.ok_or_else(|| Error::from("No output directory specified."))?;
                let pigean = pigean.build(out)?;
                Ok(ActionConfig::Bulk(pigean))
            }
            Action::Factors => {
                let ConfigBuilder { pigean, out, .. } = self;
                let pigean = 
                    pigean.ok_or_else(|| Error::from("No PIGEAN configuration specified."))?;
                let out = 
                    out.ok_or_else(|| Error::from("No output directory specified."))?;
                let pigean = pigean.build(out)?;
                Ok(ActionConfig::Factors(pigean))
            }
            Action::TraitGeneSets => {
                let ConfigBuilder { pigean, out, .. } = self;
                let pigean =
                    pigean.ok_or_else(|| Error::from("No PIGEAN configuration specified."))?;
                let out =
                    out.ok_or_else(|| Error::from("No output directory specified."))?;
                let pigean = pigean.build(out)?;
                Ok(ActionConfig::TraitGeneSets(pigean))
            }
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Hello => write!(f, "{}", action::HELLO),
            Action::Survey => write!(f, "{}", action::SURVEY),
            Action::Ping => write!(f, "{}", action::PING),
            Action::Upload => write!(f, "{}", action::UPLOAD),
            Action::Wipe => write!(f, "{}", action::WIPE),
            Action::Cat => write!(f, "{}", action::CAT),
            Action::Ls => write!(f, "{}", action::LS),
            Action::Bulk => write!(f, "{}", action::BULK),
            Action::Factors => write!(f, "{}", action::FACTORS),
            Action::TraitGeneSets => write!(f, "{}", action::PGS),
        }
    }
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Hello => action::HELLO,
            Action::Survey => action::SURVEY,
            Action::Ping => action::PING,
            Action::Upload => action::UPLOAD,
            Action::Wipe => action::WIPE,
            Action::Cat => action::CAT,
            Action::Ls => action::LS,
            Action::Bulk => action::BULK,
            Action::Factors => action::FACTORS,
            Action::TraitGeneSets => action::PGS,
        }
    }
    pub fn about(&self) -> &'static str {
        match self {
            Action::Hello => about::HELLO,
            Action::Survey => about::SURVEY,
            Action::Ping => about::PING,
            Action::Upload => about::UPLOAD,
            Action::Wipe => about::WIPE,
            Action::Cat => about::CAT,
            Action::Ls => about::LS,
            Action::Bulk => about::BULK,
            Action::Factors => about::FACTORS,
            Action::TraitGeneSets => about::PGS,
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

impl Default for PigeanConfigBuilder {
    fn default() -> Self { PigeanConfigBuilder::new() }
}

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            action::HELLO => Ok(Action::Hello),
            action::SURVEY => Ok(Action::Survey),
            action::PING => Ok(Action::Ping),
            action::UPLOAD => Ok(Action::Upload),
            action::WIPE => Ok(Action::Wipe),
            action::CAT => Ok(Action::Cat),
            action::LS => Ok(Action::Ls),
            action::BULK => Ok(Action::Bulk),
            action::FACTORS => Ok(Action::Factors),
            action::PGS => Ok(Action::TraitGeneSets),
            _ => Err(Error::from(format!("Unknown action: {value }"))),
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

pub(crate) fn all_actions_list() -> String {
    ACTIONS.map(|action| action.to_string()).join(", ")
}
