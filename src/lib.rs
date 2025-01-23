use crate::config::Config;
use crate::error::Error;
use crate::options::Options;

pub mod options;
pub mod error;
pub mod config;
mod survey;
mod hello;
mod file_info;
mod ping;
mod neo;

pub fn execute(options: &Options, config: &Config) -> Result<(), Error> {
    match options {
        Options::Hello => hello::hello(config),
        Options::Survey => survey::survey(config)?,
        Options::Ping => ping::ping_neo4j(config),
    }
    Ok(())
}
