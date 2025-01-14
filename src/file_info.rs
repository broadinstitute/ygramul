use crate::error::Error;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

pub(crate) enum FileKind {
    Gss, Gs, F, GscOut, GscList, Gc, Pc, Pc1, Pc2, Pc3, PcList
}
pub(crate) struct FileInfo {
    kind: FileKind,
    factors: Vec<String>,
}

impl FileInfo {
    pub(crate) fn from_path(path: &Path) -> Result<Self, Error> {
        match path.file_name() {
            None => Err(unrecognized_path(&path.display())),
            Some(file_name) => {
                match file_name.to_str() {
                    None => Err(unrecognized_path(&path.display())),
                    Some(string) => string.parse(),
                }
            }
        }
    }
}

fn unrecognized_path<P: Display>(path: &P) -> Error {
    Error::from(format!("Unrecognized file: '{}'.", path))
}

impl FromStr for FileInfo {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut circumfixes: Vec<&str> = Vec::new();
        let mut factors: Vec<String> = Vec::new();
        for part in string.split('.') {
            if let Some(factor) = part.strip_prefix("Factor") {
                factors.push(factor.to_string());
            } else {
                circumfixes.push(part);
            }
        }
        let kind = match circumfixes.as_slice() {
            ["gss", "phewas_all_large", "temp", "txt"] => Ok(FileKind::Gss),
            ["gs", "phewas_all_large", "temp", "txt"] => Ok(FileKind::Gs),
            ["f", "phewas_all_large", "out"] => Ok(FileKind::F),
            ["gsc", "phewas_all_large", "out"] => Ok(FileKind::GscOut),
            ["gsc", "phewas_all_large", "list"] => Ok(FileKind::GscList),
            ["gc", "phewas_all_large", "out"] => Ok(FileKind::Gc),
            ["pc", "phewas_all_large", "out"] => Ok(FileKind::Pc),
            ["pc", "phewas_all_large", "1", "out"] => Ok(FileKind::Pc1),
            ["pc", "phewas_all_large", "2", "out"] => Ok(FileKind::Pc2),
            ["pc", "phewas_all_large", "3", "out"] => Ok(FileKind::Pc3),
            ["pc", "phewas_all_large", "list"] => Ok(FileKind::PcList),
            _ => Err(unrecognized_path(&string)),
        }?;
        Ok(FileInfo { kind, factors })
    }
}
