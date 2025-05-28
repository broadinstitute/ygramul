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
mod wipe;
mod tsv;
mod s3;
mod pigean;

pub fn execute(config: &ActionConfig) -> Result<(), Error> {
    match config {
        ActionConfig::Hello(config) => hello::hello(config),
        ActionConfig::Survey(config) => {
            survey::survey(config)?;
        }
        ActionConfig::Ping(config) => ping::ping_neo4j(config)?,
        ActionConfig::Upload(config) => upload::upload_data(config)?,
        ActionConfig::Wipe(config) => wipe::wipe(config)?,
        ActionConfig::Cat(config) => s3::cat(config)?,
        ActionConfig::Ls(config) => s3::ls(config)?,
        ActionConfig::Bulk(config) => pigean::phenos::create_bulk_files(config)?,
        ActionConfig::Factors(config) => pigean::factors::create_bulk_files(config)?,
        ActionConfig::TraitGeneSets(config) =>
            pigean::pgs::create_bulk_files(config)?,
    }
    Ok(())
}
