use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::collections::{BTreeMap, BTreeSet};
use log::{info, warn};
use crate::config::LocalConfig;
use crate::error::Error;
use crate::file_info::{FileInfo, FileKind};

struct FileInfos {
    groups: BTreeMap<Vec<String>, FileGroup>,
    n_files: usize
}

struct FileGroup {
    kinds: BTreeSet<FileKind>
}

impl FileGroup {
    fn new() -> FileGroup { FileGroup { kinds: BTreeSet::new(), } }
    fn add(&mut self, kind: FileKind) { self.kinds.insert(kind); }
}

impl FileInfos {
    fn new() -> FileInfos {
        FileInfos { groups: BTreeMap::new(), n_files: 0 }
    }
    fn add(&mut self, file_info: FileInfo) {
        let FileInfo { kind, factors } = file_info;
        self.groups.entry(factors).or_default().add(kind);
        self.n_files += 1;
    }
}

impl Default for FileGroup {
    fn default() -> Self { FileGroup::new() }
}

impl Display for FileGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut kinds = self.kinds.iter();
        if let Some(kind) = kinds.next() {
            write!(f, "{}", kind)?;
            for kind in kinds {
                write!(f, ", {}", kind)?;
            }
        }
        write!(f, " ({} files)", self.kinds.len())?;
        Ok(())
    }
}

impl Display for FileInfos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (factors, group) in &self.groups {
            writeln!(f, "{}: {}", factors.join("/"), group)?;
        }
        write!(f, "Identified {} data files in {} groups.", self.n_files, self.groups.len())
    }
}

pub(crate) fn survey(config: &LocalConfig) -> Result<(), Error>{
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
                    warn!("{}", error);
                    n_unrecognized += 1;
                }
                Ok(file_info) => {
                    file_infos.add(file_info);
                }
            }
        }
    }
    if n_unrecognized > 0 {
        warn!("{} files were not recognized and will be ignored.", n_unrecognized);
    }
    info!("{}", file_infos);
    Ok(())
}

