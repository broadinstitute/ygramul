use crate::error::Error;
use crate::pigean::phenos::FileInfo;
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

pub(crate) struct PhenoGene {
    pub(crate) gene: String,
    pub(crate) combined: f64,
    pub(crate) log_bf: f64,
    pub(crate) prior: f64,
}

impl PhenoGene {
    fn into_row(self, pheno: &str) -> PhenoGeneRow {
        PhenoGeneRow {
            pheno: pheno.to_string(),
            gene: self.gene,
            combined: self.combined,
            log_bf: self.log_bf,
            prior: self.prior,
        }
    }
}

#[derive(Serialize)]
struct PhenoGeneRow {
    pub(crate) pheno: String,
    pub(crate) gene: String,
    pub(crate) combined: f64,
    pub(crate) log_bf: f64,
    pub(crate) prior: f64,
}

fn write_pheno_gene<W: Write>(writer: &mut csv::Writer<W>, pheno: &str, item: PhenoGene)
    -> Result<(), Error> {
    let row = item.into_row(pheno);
    writer.serialize(row)?;
    Ok(())
}


struct PhenosGenesTsvEater {
    gene: Option<String>,
    combined: f64,
    log_bf: f64,
    prior: f64,
}

impl PhenosGenesTsvEater {
    fn new() -> Self {
        PhenosGenesTsvEater {
            gene: None,
            combined: f64::NAN,
            log_bf: f64::NAN,
            prior: f64::NAN,
        }
    }
}

impl TsvEater for PhenosGenesTsvEater {
    type Row = PhenoGene;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Gene" => self.gene = Some(value.to_string()),
            "combined" => self.combined = value.parse().unwrap_or(f64::NAN),
            "log_bf" => self.log_bf = value.parse().unwrap_or(f64::NAN),
            "prior" => self.prior = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let gene = self.gene.ok_or_else(|| Error::from("Missing gene"))?;
        let combined = self.combined;
        let log_bf = self.log_bf;
        let prior = self.prior;
        Ok(PhenoGene {
            gene,
            combined,
            log_bf,
            prior,
        })
    }
}

struct PhenosGenesTsvEaterMaker {}

impl TsvEaterMaker for PhenosGenesTsvEaterMaker {
    type Row = PhenoGene;
    type Eater = PhenosGenesTsvEater;

    fn make(&self) -> Self::Eater {
        PhenosGenesTsvEater::new()
    }
}
fn add_file<W: Write>(
    file: &FileInfo,
    writer: &mut csv::Writer<W>,
) -> Result<(), Error> {
    let mut tsv_consumer = 
        TsvConsumer::new('\t', PhenosGenesTsvEaterMaker {}, |pheno_gene| {
        if pheno_gene.combined > 1.0 {
            write_pheno_gene(writer, &file.pheno, pheno_gene)
        } else {
            Ok(())
        }
    });
    let file_path = FilePath::from_path(&file.path)?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap("Failed to process file".to_string(), e))?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = csv::Writer::from_path(out_file)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}
