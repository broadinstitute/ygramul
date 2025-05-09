use std::fs::File;
use std::io::Write;
use crate::error::Error;
use crate::pigean::factors::{Factor, FileInfo};
use std::path::Path;
use crate::s3;
use crate::s3::FilePath;
use crate::tsv::{TsvConsumer, TsvEater, TsvEaterMaker};

struct GeneFactor {
    factor: Factor,
    gene: String,
    weight: f64,
}

struct GeneFactorsFile<W: Write> {
    writer: W,
}

impl<W: Write> GeneFactorsFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, Error> {
        writeln!(writer, "factor,gene,weight")?;
        Ok(GeneFactorsFile { writer })
    }

    fn write_gene_factor(&mut self, item: GeneFactor) -> Result<(), Error> {
        writeln!(self.writer, "{},{},{}", item.factor, item.gene, item.weight)?;
        Ok(())
    }
    fn write_gene_factors(
        &mut self,
        gene_factors: Vec<GeneFactor>,
        min_weight: f64
    ) -> Result<(), Error> {
        for gene_factor in gene_factors.into_iter() {
            if gene_factor.weight > min_weight {
                self.write_gene_factor(gene_factor)?
            }
        }
        Ok(())
    }
}

struct FactorWeight {
    prefix: String,
    weight: f64,
}

struct GeneFactorsTsvEater {
    pheno: String,
    gene: Option<String>,
    factor_weights: Vec<FactorWeight>,
}

impl GeneFactorsTsvEater {
    fn new(pheno: String) -> Self {
        GeneFactorsTsvEater {
            pheno,
            gene: None,
            factor_weights: Vec::new(),
        }
    }
}

impl TsvEater for GeneFactorsTsvEater {
    type Row = Vec<GeneFactor>;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            "Gene" => self.gene = Some(value.to_string()),
            col => {
                if col.starts_with("Factor") {
                    let weight = value.parse().unwrap_or(f64::NAN);
                    self.factor_weights.push(FactorWeight {
                        prefix: col.to_string(),
                        weight,
                    });
                }
            }
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let gene = self.gene.ok_or(Error::from("No gene specified"))?;
        let gene_factors = self.factor_weights.into_iter().map(|fw| {
            let FactorWeight { prefix, weight } = fw;
            let factor = Factor::new(prefix, self.pheno.clone());
            GeneFactor {
                factor,
                gene: gene.clone(),
                weight,
            }
        }).collect();
        Ok(gene_factors)
    }
}

struct GeneFactorsTsvEaterMaker {
    pheno: String,
}

impl TsvEaterMaker for GeneFactorsTsvEaterMaker {
    type Row = Vec<GeneFactor>;
    type Eater = GeneFactorsTsvEater;
    fn make(&self) -> Self::Eater {
        GeneFactorsTsvEater::new(self.pheno.clone())
    }
}

fn add_file<W: Write>(
    file: &FileInfo,
    writer: &mut GeneFactorsFile<W>,
) -> Result<(), Error> {
    let tsv_eater_maker = GeneFactorsTsvEaterMaker { pheno: file.pheno.clone() };
    let mut tsv_consumer =
        TsvConsumer::new('\t', tsv_eater_maker, |gene_factors| {
            writer.write_gene_factors(gene_factors, 0.01)
        });
    let file_path = FilePath::from_path(&file.path)?;
    s3::process_file(&file_path, &mut tsv_consumer)
        .map_err(|e| Error::wrap("Failed to process file".to_string(), e))?;
    Ok(())
}
pub(crate) fn add_files(files: &[FileInfo], out_file: &Path) -> Result<(), Error> {
    let mut writer = GeneFactorsFile::new(File::create(out_file)?)?;
    for file in files {
        add_file(file, &mut writer)?;
    }
    Ok(())
}