mod pheno_names;

use crate::config::PigeanConfig;
use crate::error::Error;
use crate::s3;

enum FileKind {
    Ggss,
    Gss,
    Gs,
    Ge
}

struct File {
    name: String,
    pheno: String,
    kind: FileKind,
}

pub(crate) fn create_bulk_files(config: &PigeanConfig) -> Result<(), Error> {
    let pheno_names = pheno_names::pheno_names(&config.pheno_names)?;
    let data_files = s3::collect(&config.data_dir)?;
    for data_file in data_files {
        if let Some(file) = classify_file(&data_file, &config.sub_dir) {
            println!("File: {}, pheno: {}", file.name, file.pheno);
        }
    }
    todo!()
}

fn classify_file(file: &str, sub_dir: &str) -> Option<File> {
    if let Some((_, pheno, sub, local)) = rsplit_thrice(file) {
        if sub == sub_dir {
            let pheno = pheno.to_string();
            let file_kind = 
                match local {
                    "ggss.out"       => FileKind::Ggss,
                    "gss.out" => FileKind::Gss,
                    "gs.out" => FileKind::Gs,
                    "ge.out" => FileKind::Ge,
                    _ => return None,
                };
            Some(File { name: file.to_string(), pheno, kind: file_kind })
        } else {
            None
        }
    } else {
        None
    }
}

fn rsplit_thrice(string: &str) -> Option<(&str, &str, &str, &str)> {
    if let Some((prefix, fourth)) = string.rsplit_once('/') {
        if let Some((prefix, third)) = prefix.rsplit_once('/') {
            if let Some((first, second)) = prefix.rsplit_once('/') {
                Some((first, second, third, fourth))
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


