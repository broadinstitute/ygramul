use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::error::Error;
use crate::pigean::factors::{Factor, FileInfo};
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};

struct FactorLabel {
    factor: Factor,
    label: String,
}

struct FactorLabelsFile<W: Write> {
    writer: W,
}

impl<W: Write> FactorLabelsFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, Error> {
        writeln!(writer, "factor,label")?;
        Ok(FactorLabelsFile { writer })
    }

    fn write_factor_label(&mut self, item: FactorLabel) -> Result<(), Error> {
        writeln!(self.writer, "{},{}", item.factor, item.label)?;
        Ok(())
    }
}

struct FactorLabelTsvEater {
    pheno: String,
    prefix: Option<String>,
    label: Option<String>,
}

impl FactorLabelTsvEater {
    fn new(pheno: String) -> Self {
        FactorLabelTsvEater {
            pheno,
            prefix: None,
            label: None,
        }
    }
}

impl TsvEater for FactorLabelTsvEater {
    type Row = FactorLabel;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Factor" => self.prefix = Some(value.to_string()),
            "label" => self.label = Some(value.to_string()),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let FactorLabelTsvEater { pheno, prefix, label} = self;
        let prefix = prefix.ok_or_else(|| Error::from("Missing factor prefix"))?;
        let label = label.ok_or_else(|| Error::from("Missing factor label"))?;
        let factor = Factor::new(prefix, pheno);
        Ok(FactorLabel { factor, label })
    }
}

struct FactorLabelsTsvEaterMaker {
    pheno: String,
}

impl TsvEaterMaker for FactorLabelsTsvEaterMaker {
    type Row = FactorLabel;
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

fn add_file(
    file: &FileInfo,
    writer: &mut FactorLabelsFile<impl Write>
) -> Result<(), Error> {
    let tsv_eater_maker = FactorLabelsTsvEaterMaker::new(file.pheno.clone());
    let mut tsv_consumer =
        TsvConsumer::new('\t', tsv_eater_maker, |item| {
        writer.write_factor_label(item)
    });
    let file_path = FilePath::from_path(&file.path)?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap("Failed to process file".to_string(), e))?;
    Ok(())
}

pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = FactorLabelsFile::new(File::create(out_file)?)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}