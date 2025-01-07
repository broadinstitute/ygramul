use crate::config::Config;
use crate::error::Error;
use crate::options::Options;

pub mod options;
pub mod error;
pub mod config;
mod survey;
mod hello;

pub fn execute(options: &Options, config: &Config) -> Result<(), Error> {
    match options {
        Options::Hello => hello::hello(config),
        Options::Survey => survey::survey(config)?,
    }
    Ok(())
}
