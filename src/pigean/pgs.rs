mod pheno_pgs;
mod gene_pgs;

use std::fmt::{Display, Formatter};
use std::path::Path;
use log::info;
use crate::config::PigeanConfig;
use crate::{pigean, s3};

pub(crate) struct PhenoGeneSet {
    pub(crate) pheno: String,
    pub(crate) gene_set: String,
}

enum FileKind {
    PhenoGeneSet,
    GeneGeneSet
}

pub(crate) struct FileInfo {
    pub(crate) path: String,
    pub(crate) pheno: String,
    kind: FileKind,
}

pub fn create_bulk_files(config: &PigeanConfig) -> Result<(), pigean::Error> {
    info!("Finding all files in {} for pheno-gene-geneset relations", config.data_dir);
    let mut gene_pgs_files: Vec<FileInfo> = Vec::new();
    let mut pheno_pgs_files: Vec<FileInfo> = Vec::new();
    let data_files = s3::collect(&config.data_dir)?;
    for data_file in data_files {
        match classify_file(&data_file, &config.sub_dir) {
            Some(file_info) => match file_info.kind {
                FileKind::PhenoGeneSet => {
                    pheno_pgs_files.push(file_info);
                }
                FileKind::GeneGeneSet => {
                    gene_pgs_files.push(file_info);
                }
            },
            None => {
                pigean::handle_unclassified_file(&data_file)?;
            }
        }
    }
    info!("Found {} pheno-pheno-geneset files and {} gene-pheno-geneset files",
          pheno_pgs_files.len(), gene_pgs_files.len());

    let pheno_pgs_file = Path::new(&config.out).join("pheno_geneset.csv");
    info!("Writing pheno-geneset file to {}", pheno_pgs_file.display());
    pheno_pgs::add_files(&pheno_pgs_files, &pheno_pgs_file)?;

    let gene_pgs_file = Path::new(&config.out).join("gene_geneset.csv");
    info!("Writing pheno-geneset file to {}", pheno_pgs_file.display());
    gene_pgs::add_files(&gene_pgs_files, &gene_pgs_file)?;

    info!("Finished writing pheno-pheno-geneset and gene-pheno-geneset files");
    Ok(())
}

pub(crate) fn classify_file(file: &str, sub_dir: &str) -> Option<FileInfo> {
    if let Some((pheno, sub, local)) = pigean::last_three_parts(file) {
        if sub == sub_dir {
            let pheno = pheno.to_string();
            let file_kind = match local {
                "gss.out" => FileKind::PhenoGeneSet,
                "ggss.out" => FileKind::GeneGeneSet,
                _ => return None,
            };
            Some(FileInfo {
                path: file.to_string(),
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

impl PhenoGeneSet {
    pub fn new(pheno: String, gene_set: String) -> Self {
        PhenoGeneSet { pheno, gene_set }
    }
}

impl Display for PhenoGeneSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.gene_set, self.pheno)
    }
}
