use std::str::FromStr;
use crate::error::Error;

pub(crate) enum FileKind {
    Gss
}
pub(crate) struct FileInfo {
    kind: FileKind,
    factors: Vec<u8>,
}

impl FromStr for FileInfo {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = string.split(',').collect();
        todo!()
    }
}