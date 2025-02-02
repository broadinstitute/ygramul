use crate::config::ActionConfig;
use crate::error::Error;

pub mod error;
pub mod config;
mod survey;
mod hello;
mod file_info;
mod ping;
mod neo;
pub mod cli;
mod upload;

pub fn execute(config: &ActionConfig) -> Result<(), Error> {
    match config {
        ActionConfig::Hello(config) => hello::hello(config),
        ActionConfig::Survey(config) => survey::survey(config)?,
        ActionConfig::Ping(config) => ping::ping_neo4j(config)?,
        ActionConfig::Upload(config) => upload::upload_data(config)?,
    }
    Ok(())
}
