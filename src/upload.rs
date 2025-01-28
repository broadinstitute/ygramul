use log::error;
use crate::config::Config;
use crate::error::Error;

pub(crate) fn upload_data(config: &Config) -> Result<(), Error> {
    let data_dir = &config.data_dir;
    if !data_dir.exists() {
        return Err(Error::from(format!("Data directory does not exist: {}",
                                       data_dir.display())));
    }
    error!("Upload not implemented yet.");
    Ok(())
}