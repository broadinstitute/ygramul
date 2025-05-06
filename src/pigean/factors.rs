mod gene_factors;
mod geneset_factors;

use std::fmt::Display;
use std::path::Path;
use log::info;
use crate::config::PigeanConfig;
use crate::error::Error;
use crate::{pigean, s3};

pub(crate) struct Factor {
    pub(crate) name: String,
    pub(crate) pheno: String,
}

enum FileKind {
    Gc,
    Gsac
}

pub struct FileInfo {
    pub(crate) name: String,
    pub(crate) pheno: String,
    kind: FileKind,
}

pub(crate) fn create_bulk_files(config: &PigeanConfig) -> Result<(), Error> {
    info!("Finding all files in {} for factor-gene-genset relations", config.factors_dir);
    let mut factor_gene_files: Vec<FileInfo> = Vec::new();
    let mut factor_geneset_files: Vec<FileInfo> = Vec::new();
    let data_files = s3::collect(&config.factors_dir)?;
    for data_file in data_files {
        match classify_file(&data_file, &config.factors_sub_dir) {
            Some(file_info) => match file_info.kind {
                FileKind::Gc => {
                    factor_gene_files.push(file_info);
                }
                FileKind::Gsac => {
                    factor_geneset_files.push(file_info);
                }
            },
            None => {
                pigean::handle_unclassified_file(&data_file)?;
            }
        }
    }
    info!("Found {} factor-gene files and {} factor-geneset files", factor_gene_files.len(), 
        factor_geneset_files.len());
    let factor_gene_file = Path::new(&config.out).join("factor_gene.csv");
    info!("Writing factor-gene file to {}", factor_gene_file.display());
    gene_factors::add_files(&factor_gene_files, &factor_gene_file)?;
    let factor_geneset_file = Path::new(&config.out).join("factor_geneset.csv");
    info!("Writing factor-geneset file to {}", factor_geneset_file.display());
    geneset_factors::add_files(&factor_geneset_files, &factor_geneset_file)?;
    Ok(())
}

fn classify_file(file: &str, sub_dir: &str) -> Option<FileInfo> {
    if let Some((pheno, sub, local)) = pigean::last_three_parts(file) {
        if sub == sub_dir {
            let pheno = pheno.to_string();
            let kind = match local {
                "gc.out" => FileKind::Gc,
                "gsac.out" => FileKind::Gsac,
                _ => return None,
            };
            Some(FileInfo {
                name: file.to_string(),
                pheno,
                kind,
            })
        } else {
            None
        }
    } else {
        None
    }
}

impl Factor {
    pub(crate) fn new(name: String, pheno: String) -> Self {
        Factor { name, pheno }
    }
}
impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.name, self.pheno)
    }
}