use crate::error::Error;
use crate::pigean::phenos::FileInfo;
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

pub(crate) struct PhenoGeneset {
    pub(crate) gene_set: String,
    pub(crate) beta_uncorrected: f64,
    pub(crate) beta: f64,
}

impl PhenoGeneset {
    fn into_row(self, pheno: &str) -> PhenoGenesetRow {
        PhenoGenesetRow {
            pheno: pheno.to_string(),
            gene_set: self.gene_set,
            beta_uncorrected: self.beta_uncorrected,
            beta: self.beta,
        }
    }
}

#[derive(Serialize)]
struct PhenoGenesetRow {
    pub(crate) pheno: String,
    pub(crate) gene_set: String,
    pub(crate) beta_uncorrected: f64,
    pub(crate) beta: f64,
}

fn write_pheno_geneset<W: Write>(
    writer: &mut csv::Writer<W>,
    pheno: &str,
    item: PhenoGeneset,
) -> Result<(), Error> {
    let pheno_geneset_row = item.into_row(pheno);
    writer.serialize(pheno_geneset_row)?;
    Ok(())
}

struct PhenosGenesetTsvEater {
    gene_set: Option<String>,
    beta_uncorrected: f64,
    beta: f64,
}

impl PhenosGenesetTsvEater {
    fn new() -> Self {
        PhenosGenesetTsvEater {
            gene_set: None,
            beta_uncorrected: f64::NAN,
            beta: f64::NAN,
        }
    }
}

impl TsvEater for PhenosGenesetTsvEater {
    type Row = PhenoGeneset;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Gene_Set" => self.gene_set = Some(value.to_string()),
            "beta_uncorrected" => {
                self.beta_uncorrected = value.parse().unwrap_or(f64::NAN)
            }
            "beta" => self.beta = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let gene_set = self
            .gene_set
            .ok_or_else(|| Error::from("Missing GeneSet".to_string()))?;
        Ok(PhenoGeneset {
            gene_set,
            beta_uncorrected: self.beta_uncorrected,
            beta: self.beta,
        })
    }
}

struct PhenosGenesetTsvEaterMaker;

impl TsvEaterMaker for PhenosGenesetTsvEaterMaker {
    type Row = PhenoGeneset;
    type Eater = PhenosGenesetTsvEater;

    fn make(&self) -> Self::Eater {
        PhenosGenesetTsvEater::new()
    }
}

fn add_file<W: Write>(
    file: &FileInfo,
    writer: &mut csv::Writer<W>,
) -> Result<(), Error> {
    let mut tsv_consumer =
        TsvConsumer::new('\t', PhenosGenesetTsvEaterMaker {}, |item| {
            if item.beta_uncorrected > 0.01 {
                write_pheno_geneset(writer, &file.pheno, item)
            } else {
                Ok(())
            }
        });
    let file_path = FilePath::from_path(&file.path)?;
    s3::process_file(&file_path, &mut tsv_consumer)?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = csv::Writer::from_path(out_file)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}