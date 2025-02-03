use crate::config::ActionConfig;
use crate::error::Error;

pub mod cli;
pub mod config;
pub mod error;
mod file_info;
mod hello;
mod neo;
mod ping;
mod survey;
mod upload;

pub fn execute(config: &ActionConfig) -> Result<(), Error> {
    match config {
        ActionConfig::Hello(config) => hello::hello(config),
        ActionConfig::Survey(config) => {
            survey::survey(config)?;
        }
        ActionConfig::Ping(config) => ping::ping_neo4j(config)?,
        ActionConfig::Upload(config) => upload::upload_data(config)?,
    }
    Ok(())
}
