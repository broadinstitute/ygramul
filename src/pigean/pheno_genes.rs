use crate::error::Error;
use crate::pigean::FileInfo;
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub(crate) struct PhenoGene {
    pub(crate) gene: String,
    pub(crate) combined: f64,
    pub(crate) log_bf: f64,
    pub(crate) prior: f64,
}

struct PhenosGenesFile<W: Write> {
    writer: W,
}

impl<W: Write> PhenosGenesFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, Error> {
        writeln!(writer, "pheno,gene,combined,log_bfs,prior")?;
        Ok(PhenosGenesFile { writer })
    }
    fn write_pheno_gene(&mut self, pheno: &str, item: PhenoGene) -> Result<(), Error> {
        writeln!(
            self.writer,
            "{},{},{},{},{}",
            pheno, item.gene, item.combined, item.log_bf, item.prior
        )?;
        Ok(())
    }
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
    consumer: &mut PhenosGenesFile<W>,
) -> Result<(), Error> {
    let mut tsv_consumer = 
        TsvConsumer::new('\t', PhenosGenesTsvEaterMaker {}, |pheno_gene| {
        if pheno_gene.combined > 1.0 {
            consumer.write_pheno_gene(&file.pheno, pheno_gene)
        } else {
            Ok(())
        }
    });
    let file_path = FilePath::from_path(&file.name)?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap("Failed to process file".to_string(), e))?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &PathBuf) -> Result<(), Error> {
    let mut consumer = PhenosGenesFile::new(File::create(out_file)?)?;
    for file in files {
        add_file(file, &mut consumer)?;
    }
    Ok(())
}
