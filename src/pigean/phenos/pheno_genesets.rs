use crate::error::Error;
use crate::pigean::phenos::FileInfo;
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) struct PhenoGeneset {
    pub(crate) gene_set: String,
    pub(crate) beta_uncorrected: f64,
    pub(crate) beta: f64,
}

pub(crate) struct PhenoGenesetFile<W: Write> {
    writer: W,
}

impl<W: Write> PhenoGenesetFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, Error> {
        writeln!(writer, "pheno,gene_set,beta_uncorrected,beta")?;
        Ok(PhenoGenesetFile { writer })
    }
    fn write_pheno_geneset(
        &mut self,
        pheno: &str,
        item: PhenoGeneset,
    ) -> Result<(), Error> {
        writeln!(
            self.writer,
            "{},{},{},{}",
            pheno, item.gene_set, item.beta_uncorrected, item.beta
        )?;
        Ok(())
    }
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
    consumer: &mut PhenoGenesetFile<W>,
) -> Result<(), Error> {
    let mut tsv_consumer =
        TsvConsumer::new('\t', PhenosGenesetTsvEaterMaker {}, |item| {
            if item.beta_uncorrected > 0.01 {
                consumer.write_pheno_geneset(&file.pheno, item)
            } else {
                Ok(())
            }
        });
    let file_path = FilePath::from_path(&file.name)?;
    s3::process_file(&file_path, &mut tsv_consumer)?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = PhenoGenesetFile::new(File::create(out_file)?)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}