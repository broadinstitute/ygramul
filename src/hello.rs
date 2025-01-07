use crate::config::Config;

pub(crate) fn hello(config: &Config) {
    println!("Data directory: {}", config.data_dir.display())}