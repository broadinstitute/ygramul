use std::fs;
use crate::config::Config;
use crate::error::Error;

pub(crate) fn survey(config: &Config) -> Result<(), Error>{
    let data_dir = &config.data_dir;
    if !data_dir.exists() {
        Err(Error::from(
            format!("Data directory '{}' does not exist.", data_dir.display())
        ))?;
    }
    let entries =
        fs::read_dir(data_dir).map_err(|io_error|
            Error::wrap(data_dir.display().to_string(), io_error)
        )?;
    Ok(())
}