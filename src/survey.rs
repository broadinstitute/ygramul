use std::fmt::Display;
use std::{fmt, fs};
use log::{info, warn};
use crate::config::Config;
use crate::error::Error;
use crate::file_info::FileInfo;

struct FileInfos {
    n_files: usize
}

struct FilesSummary {
    n_files: usize
}

impl FileInfos {
    fn new() -> FileInfos {
        FileInfos { n_files: 0 }
    }
    fn add(&mut self, file_info: FileInfo) {
        self.n_files += 1;
    }
    fn summary(&self) -> FilesSummary {
        FilesSummary { n_files: self.n_files }
    }
}

impl Display for FilesSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identified {} data files", self.n_files)
    }
}

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
    let mut file_infos = FileInfos::new();
    let mut n_unrecognized: usize = 0;
    for entry in entries {
        let entry =
            entry.map_err(|io_error| Error::wrap(data_dir.display().to_string(), io_error))?;
        if entry.path().is_file() {
            match classify_file(&entry.path()) {
                None => {
                    warn!("Unrecognized file '{}'.", entry.path().display());
                    n_unrecognized += 1;
                }
                Some(file_info) => {
                    file_infos.add(file_info);
                }
            }
        }
    }
    if n_unrecognized > 0 {
        warn!("{} files were not recognized and will be ignored.", n_unrecognized);
    }
    info!("{}", file_infos.summary());
    Ok(())
}

const FACTOR: &str = "Factor";
fn classify_file(file: &std::path::Path) -> Option<FileInfo> {
    None
}

