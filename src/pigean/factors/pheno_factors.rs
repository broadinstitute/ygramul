use crate::error::Error;
use crate::pigean::factors::{Factor, FileInfo};
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct PhenoFactor {
    factor: String,
    label: String,
    pheno: String,
    any_relevance: f64
}

struct FactorLabelTsvEater {
    pheno: String,
    prefix: Option<String>,
    label: Option<String>,
    any_relevance: f64,
}

impl FactorLabelTsvEater {
    fn new(pheno: String) -> Self {
        FactorLabelTsvEater {
            pheno,
            prefix: None,
            label: None,
            any_relevance: f64::NAN,
        }
    }
}

impl TsvEater for FactorLabelTsvEater {
    type Row = PhenoFactor;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Factor" => self.prefix = Some(value.to_string()),
            "label" => self.label = Some(value.to_string()),
            "any_relevance" => self.any_relevance = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let FactorLabelTsvEater {
            pheno, prefix, label, any_relevance
        } = self;
        let prefix = prefix.ok_or_else(|| Error::from("Missing factor prefix"))?;
        let label = label.ok_or_else(|| Error::from("Missing factor label"))?;
        let factor = Factor::new(prefix, pheno.clone()).to_string();
        Ok(PhenoFactor { factor, label, pheno, any_relevance })
    }
}

struct FactorLabelsTsvEaterMaker {
    pheno: String,
}

impl TsvEaterMaker for FactorLabelsTsvEaterMaker {
    type Row = PhenoFactor;
    type Eater = FactorLabelTsvEater;

    fn make(&self) -> Self::Eater {
        FactorLabelTsvEater::new(self.pheno.clone())
    }
}

impl FactorLabelsTsvEaterMaker {
    pub(crate) fn new(pheno: String) -> Self {
        FactorLabelsTsvEaterMaker { pheno }
    }
}

fn add_file<W: Write>(
    file: &FileInfo,
    writer: &mut csv::Writer<W>,
) -> Result<(), Error> {
    let tsv_eater_maker = FactorLabelsTsvEaterMaker::new(file.pheno.clone());
    let mut tsv_consumer =
        TsvConsumer::new('\t', tsv_eater_maker, |item| {
            writer.serialize(item)?;
            Ok(())
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