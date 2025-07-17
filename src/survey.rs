use std::fs;
use log::{info, warn};
use crate::config::LocalConfig;
use crate::error::Error;
use crate::file_info::{FileInfo, FileInfos};


pub(crate) fn survey(config: &LocalConfig) -> Result<FileInfos, Error>{
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
    let mut file_infos = FileInfos::new();
    let mut n_unrecognized: usize = 0;
    for entry in entries {
        let entry =
            entry.map_err(|io_error| Error::wrap(data_dir.display().to_string(), io_error))?;
        let path = entry.path();
        if path.is_file() {
            match FileInfo::from_path(&path) {
                Err(error) => {
                    warn!("{error}");
                    n_unrecognized += 1;
                }
                Ok(file_info) => {
                    file_infos.add(file_info);
                }
            }
        }
    }
    if n_unrecognized > 0 {
        warn!("{n_unrecognized} files were not recognized and will be ignored.");
    }
    info!("{file_infos}");
    Ok(file_infos)
}

