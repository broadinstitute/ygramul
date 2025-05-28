use std::io::Write;
use crate::error::Error;
use crate::pigean::pgs::PhenoGeneSet;
use crate::tsv::TsvEater;

pub(crate) struct PhenoPgs {
    pub(crate) pgs: PhenoGeneSet,
    pub(crate) beta_uncorrected: f64,
    pub(crate) beta: f64,
}

pub(crate) struct PhenoPgsFile<W: Write> {
    writer: W,
}

impl<W: Write> PhenoPgsFile<W> {
    pub(crate) fn new(mut writer: W) -> Result<Self, crate::error::Error> {
        writeln!(writer, "pheno,pgs,beta_uncorrected,beta")?;
        Ok(PhenoPgsFile { writer })
    }

    pub(crate) fn write_pheno_pgs(
        &mut self,
        pheno: &str,
        item: PhenoPgs,
    ) -> Result<(), crate::error::Error> {
        writeln!(
            self.writer,
            "{},{},{},{}",
            pheno, item.pgs, item.beta_uncorrected, item.beta
        )?;
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
            "beta_uncorrected" => {
                self.beta_uncorrected = value.parse().unwrap_or(f64::NAN)
            }
            "beta" => self.beta = value.parse().unwrap_or(f64::NAN),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let PhenosPgsTsvEater {
            pheno, gene_set, beta_uncorrected, beta
        } = self;
        let gene_set = gene_set.ok_or_else(|| Error::from("Missing gene set"))?;
        let pgs = PhenoGeneSet::new(pheno, gene_set);
        Ok(PhenoPgs { pgs, beta_uncorrected, beta, })
    }
}