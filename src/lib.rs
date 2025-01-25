use crate::config::{Action, Config};
use crate::error::Error;

pub mod error;
pub mod config;
mod survey;
mod hello;
mod file_info;
mod ping;
mod neo;
pub mod cli;


pub fn execute(config: &Config) -> Result<(), Error> {
    match config.action {
        Action::Hello => hello::hello(config),
        Action::Survey => survey::survey(config)?,
        Action::Ping => ping::ping_neo4j(config)?,
    }
    Ok(())
}
