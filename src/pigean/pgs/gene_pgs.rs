use crate::error::Error;
use crate::pigean::pgs::{FileInfo, PhenoGeneSet};
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
pub(crate) struct GenePgs {
    pub gene: String,
    pub pgs: String,
    pub beta: f64,
}

pub(crate) fn write_gene_pgs<W: Write>(
    writer: &mut csv::Writer<W>, item: GenePgs, min_beta: f64,
) -> Result<(), Error> {
    if item.beta > min_beta {
        writer.serialize(item)?;
    }
    Ok(())
}

pub(crate) struct GenePgsTsvEater {
    pheno: String,
    gene: Option<String>,
    gene_set: Option<String>,
    beta: f64,
}

impl GenePgsTsvEater {
    pub(crate) fn new(pheno: String) -> Self {
        GenePgsTsvEater {
            pheno,
            gene: None,
            gene_set: None,
            beta: f64::NAN,
        }
    }
}

impl TsvEater for GenePgsTsvEater {
    type Row = GenePgs;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Gene" => self.gene = Some(value.to_string()),
            "gene_set" => self.gene_set = Some(value.to_string()),
            "beta" => self.beta = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let GenePgsTsvEater {
            pheno, gene, gene_set, beta,
        } = self;
        let gene = gene.ok_or_else(|| Error::from("Missing gene"))?;
        let gene_set = gene_set.ok_or_else(|| Error::from("Missing gene set"))?;
        let pgs = PhenoGeneSet::new(pheno, gene_set).to_string();
        Ok(GenePgs { gene, pgs, beta })
    }
}

struct GenePgsTsvEaterMaker {
    pheno: String,
}

impl GenePgsTsvEaterMaker {
    pub(crate) fn new(pheno: String) -> Self {
        GenePgsTsvEaterMaker { pheno }
    }
}

impl TsvEaterMaker for GenePgsTsvEaterMaker {
    type Row = GenePgs;
    type Eater = GenePgsTsvEater;

    fn make(&self) -> Self::Eater {
        GenePgsTsvEater::new(self.pheno.clone())
    }
}

fn add_file<W: Write>(file: &FileInfo, writer: &mut csv::Writer<W>) -> Result<(), Error> {
    let tsv_eater_maker = GenePgsTsvEaterMaker::new(file.pheno.clone());
    let mut tsv_consumer =
        TsvConsumer::new('\t', tsv_eater_maker, |item| {
            write_gene_pgs(writer, item, 0.01)
        });
    let file_path = FilePath::from_path(&file.path)
        .map_err(|e| Error::wrap(format!("Could not use {} as path", file.path), e))?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap(format!("Failed to process {}", file.path), e))?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = csv::Writer::from_path(out_file)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}
