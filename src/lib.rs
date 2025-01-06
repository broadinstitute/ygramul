use crate::config::Config;
use crate::options::Options;

pub mod options;
pub mod error;
pub mod config;

pub fn execute(options: &Options, config: &Config) {
    match options {
        Options::Hello => println!("Data directory: {}", config.data_dir.display())
    }
}
