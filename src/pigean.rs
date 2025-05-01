mod pheno_names;

use crate::config::PigeanConfig;
use crate::error::Error;
use crate::s3;

enum FileKind {
    Ggss,
    Gss,
    Gs,
    Ge,
}

struct FileInfo {
    name: String,
    pheno: String,
    kind: FileKind,
}

pub(crate) fn create_bulk_files(config: &PigeanConfig) -> Result<(), Error> {
    let pheno_names = pheno_names::pheno_names(&config.pheno_names)?;
    let data_files = s3::collect(&config.data_dir)?;
    for data_file in data_files {
        match classify_file(&data_file, &config.sub_dir) {
            Some(file_info) => match file_info.kind {
                FileKind::Ggss => {
                    todo!()
                }
                FileKind::Gss => {
                    todo!()
                }
                FileKind::Gs => {
                    todo!()
                }
                FileKind::Ge => {
                    todo!()
                }
            },
            None => {
                todo!()
            }
        }
    }
    todo!()
}

fn classify_file(file: &str, sub_dir: &str) -> Option<FileInfo> {
    if let Some((pheno, sub, local)) = last_three_parts(file) {
        if sub == sub_dir {
            let pheno = pheno.to_string();
            let file_kind = match local {
                "ggss.out" => FileKind::Ggss,
                "gss.out" => FileKind::Gss,
                "gs.out" => FileKind::Gs,
                "ge.out" => FileKind::Ge,
                _ => return None,
            };
            Some(FileInfo {
                name: file.to_string(),
                pheno,
                kind: file_kind,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn last_three_parts(string: &str) -> Option<(&str, &str, &str)> {
    if let Some((prefix, third)) = string.rsplit_once('/') {
        if let Some((prefix, second)) = prefix.rsplit_once('/') {
            if let Some((_, first)) = prefix.rsplit_once('/') {
                Some((first, second, third))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
