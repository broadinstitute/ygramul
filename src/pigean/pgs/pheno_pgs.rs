use crate::error::Error;
use crate::pigean::pgs::{FileInfo, PhenoGeneSet};
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) struct PhenoPgs {
    pub(crate) pgs: PhenoGeneSet,
    pub(crate) beta_uncorrected: f64,
    pub(crate) beta: f64,
}

pub(crate) struct PhenoPgsFile<W: Write> {
    writer: W,
}

impl<W: Write> PhenoPgsFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, Error> {
        writeln!(writer, "pheno,pgs,beta_uncorrected,beta")?;
        Ok(PhenoPgsFile { writer })
    }

    pub(crate) fn write_pheno_pgs(
        &mut self, pheno: &str, item: PhenoPgs, min_beta: f64,
    ) -> Result<(), Error> {
        if item.beta > min_beta {
            writeln!(
                self.writer,
                "{},{},{},{}",
                pheno, item.pgs, item.beta_uncorrected, item.beta
            )?;
        }
        Ok(())
    }
}

pub(crate) struct PhenosPgsTsvEater {
    pheno: String,
    gene_set: Option<String>,
    beta_uncorrected: f64,
    beta: f64,
}

impl PhenosPgsTsvEater {
    pub(crate) fn new(pheno: String) -> Self {
        PhenosPgsTsvEater {
            pheno,
            gene_set: None,
            beta_uncorrected: f64::NAN,
            beta: f64::NAN,
        }
    }
}

impl TsvEater for PhenosPgsTsvEater {
    type Row = PhenoPgs;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Gene_Set" => self.gene_set = Some(value.to_string()),
            "beta_uncorrected" => self.beta_uncorrected = value.parse().unwrap_or(f64::NAN),
            "beta" => self.beta = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let PhenosPgsTsvEater {
            pheno, gene_set, beta_uncorrected, beta,
        } = self;
        let gene_set = gene_set.ok_or_else(|| Error::from("Missing gene set"))?;
        let pgs = PhenoGeneSet::new(pheno, gene_set);
        Ok(PhenoPgs { pgs, beta_uncorrected, beta })
    }
}

struct PhenoPgsTsvEaterMaker {
    pheno: String,
}

impl PhenoPgsTsvEaterMaker {
    pub(crate) fn new(pheno: String) -> Self {
        PhenoPgsTsvEaterMaker { pheno }
    }
}

impl TsvEaterMaker for PhenoPgsTsvEaterMaker {
    type Row = PhenoPgs;
    type Eater = PhenosPgsTsvEater;

    fn make(&self) -> Self::Eater {
        PhenosPgsTsvEater::new(self.pheno.clone())
    }
}

fn add_file<W: Write>(file: &FileInfo, writer: &mut PhenoPgsFile<W>) -> Result<(), Error> {
    let tsv_eater_maker = PhenoPgsTsvEaterMaker::new(file.pheno.clone());
    let mut tsv_consumer =
        TsvConsumer::new('\t', tsv_eater_maker, |item| {
            writer.write_pheno_pgs(&file.pheno, item, 0.01)
        });
    let file_path = FilePath::from_path(&file.path)
        .map_err(|e| Error::wrap(format!("Could not use {} as path", file.path), e))?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap(format!("Failed to process {}", file.path), e))?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = PhenoPgsFile::new(File::create(out_file)?)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}
