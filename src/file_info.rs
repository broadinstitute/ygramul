use crate::error::Error;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum FileKind {
    Gss,
    Gs,
    F,
    GscOut,
    GscList,
    Gc,
    Pc,
    Pc1,
    Pc2,
    Pc3,
    PcList,
}
pub(crate) struct FileInfo {
    pub(crate) kind: FileKind,
    pub(crate) factors: Vec<String>,
}

impl FileInfo {
    pub(crate) fn from_path(path: &Path) -> Result<Self, Error> {
        match path.file_name() {
            None => Err(unrecognized_path(&path.display())),
            Some(file_name) => match file_name.to_str() {
                None => Err(unrecognized_path(&path.display())),
                Some(string) => string.parse(),
            },
        }
    }
}

fn unrecognized_path<P: Display>(path: &P) -> Error {
    Error::from(format!("Unrecognized file: '{path}'."))
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

impl FileKind {
    pub(crate) fn create_name(&self, factors: &[String]) -> String {
        let mut name = String::new();
        match self {
            FileKind::Gss => name.push_str("gss"),
            FileKind::Gs => name.push_str("gs"),
            FileKind::F => name.push('f'),
            FileKind::GscOut | FileKind::GscList => name.push_str("gsc"),
            FileKind::Gc => name.push_str("gc"),
            FileKind::Pc | FileKind::Pc1 | FileKind::Pc2 | FileKind::Pc3 | FileKind::PcList =>
                name.push_str("pc"),
        }
        name.push_str(".phewas_all_large.");
        for factor in factors {
            name.push_str("Factor");
            name.push_str(factor);
            name.push('.');
        }
        match self {
            FileKind::Gss | FileKind::Gs => name.push_str("temp.txt"),
            FileKind::F | FileKind::GscOut | FileKind::Gc | FileKind::Pc =>
                name.push_str("out"),
            FileKind::GscList | FileKind::PcList => name.push_str("list"),
            FileKind::Pc1 => name.push_str("1.out"),
            FileKind::Pc2 => name.push_str("2.out"),
            FileKind::Pc3 => name.push_str("3.out"),
        }
        name
    }
}

impl Display for FileKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileKind::Gss => write!(f, "GSS"),
            FileKind::Gs => write!(f, "GS"),
            FileKind::F => write!(f, "F"),
            FileKind::GscOut => write!(f, "GSC out"),
            FileKind::GscList => write!(f, "GSC list"),
            FileKind::Gc => write!(f, "GC"),
            FileKind::Pc => write!(f, "PC"),
            FileKind::Pc1 => write!(f, "PC1"),
            FileKind::Pc2 => write!(f, "PC2"),
            FileKind::Pc3 => write!(f, "PC3"),
            FileKind::PcList => write!(f, "PC list"),
        }
    }
}

pub(crate) struct FileInfos {
    pub(crate) groups: BTreeMap<Vec<String>, FileGroup>,
    n_files: usize,
}

pub(crate) struct FileGroup {
    pub(crate) kinds: BTreeSet<FileKind>,
}

impl FileGroup {
    fn new() -> FileGroup {
        FileGroup {
            kinds: BTreeSet::new(),
        }
    }
    fn add(&mut self, kind: FileKind) {
        self.kinds.insert(kind);
    }
}

impl FileInfos {
    pub(crate) fn new() -> FileInfos {
        FileInfos {
            groups: BTreeMap::new(),
            n_files: 0,
        }
    }
    pub(crate) fn add(&mut self, file_info: FileInfo) {
        let FileInfo { kind, factors } = file_info;
        self.groups.entry(factors).or_default().add(kind);
        self.n_files += 1;
    }
}

impl Default for FileGroup {
    fn default() -> Self {
        FileGroup::new()
    }
}

impl Display for FileGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut kinds = self.kinds.iter();
        if let Some(kind) = kinds.next() {
            write!(f, "{kind}")?;
            for kind in kinds {
                write!(f, ", {kind}")?;
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
        write!(
            f,
            "Identified {} data files in {} groups.",
            self.n_files,
            self.groups.len()
        )
    }
}
