use crate::config::LocalConfig;

pub(crate) fn hello(config: &LocalConfig) {
    println!("Data directory: {}", config.data_dir.display())}