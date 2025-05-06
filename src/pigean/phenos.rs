use std::path::Path;
use log::info;
use crate::config::PigeanConfig;
use crate::error::Error;
use crate::{pigean, s3};
mod pheno_genes;
mod pheno_genesets;

enum FileKind {
    Ggss,
    Gss,
    Gs,
    Ge,
}

pub struct FileInfo {
    pub(crate) name: String,
    pub(crate) pheno: String,
    kind: FileKind,
}

pub fn create_bulk_files(config: &PigeanConfig) -> Result<(), Error> {
    info!("Finding all files in {} for pheno-gene-geneset relations", config.data_dir);
    let mut pheno_gene_files: Vec<FileInfo> = Vec::new();
    let mut pheno_geneset_files: Vec<FileInfo> = Vec::new();
    let data_files = s3::collect(&config.data_dir)?;
    for data_file in data_files {
        match classify_file(&data_file, &config.sub_dir) {
            Some(file_info) => match file_info.kind {
                FileKind::Ggss => {
                    pigean::ignore_file(&file_info.name)?;
                }
                FileKind::Gss => {
                    pheno_geneset_files.push(file_info);
                }
                FileKind::Gs => {
                    pheno_gene_files.push(file_info);
                }
                FileKind::Ge => {
                    pigean::ignore_file(&file_info.name)?;
                }
            },
            None => {
                pigean::handle_unclassified_file(&data_file)?;
            }
        }
    }
    info!("Found {} pheno-gene files and {} pheno-geneset files", pheno_gene_files.len(), 
        pheno_geneset_files.len());
    let pheno_gene_file = Path::new(&config.out).join("pheno_gene.csv");
    info!("Writing pheno-gene file to {}", pheno_gene_file.display());   
    pheno_genes::add_files(&pheno_gene_files, &pheno_gene_file)?;
    let pheno_geneset_file = Path::new(&config.out).join("pheno_geneset.csv");
    info!("Writing pheno-genset file to {}", pheno_geneset_file.display());
    pheno_genesets::add_files(&pheno_geneset_files, &pheno_geneset_file)?;
    info!("Finished writing pheno-gene and pheno-genset files");
    Ok(())
}

fn classify_file(file: &str, sub_dir: &str) -> Option<FileInfo> {
    if let Some((pheno, sub, local)) = pigean::last_three_parts(file) {
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