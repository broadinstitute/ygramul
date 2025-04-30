use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use crate::error::Error;
use crate::tsv::{TsvEater, TsvEaterMaker, TsvReader};

mod cols {
    pub(crate) const KEY: &str = "phenotype";
    pub(crate) const NAME: &str = "phenotype_name";
}

struct Entry {
    key: String,
    name: String
}
struct PhenoNamesRowEater {
    key: Option<String>,
    name: Option<String>
}

impl TsvEater for PhenoNamesRowEater {
    type Row = Entry;

    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            cols::KEY => self.key = Some(value.to_string()),
            cols::NAME => self.name = Some(value.to_string()),
            _ => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Row, Error> {
        let key = self.key.ok_or_else(|| Error::from("Missing key"))?;
        let name = self.name.ok_or_else(|| Error::from("Missing name"))?;
        Ok(Entry { key, name })
    }
}

struct PhenoNamesEaterMaker {}

impl TsvEaterMaker for PhenoNamesEaterMaker {
    type Row = Entry;
    type Eater = PhenoNamesRowEater;

    fn make(&self) -> Self::Eater {
        PhenoNamesRowEater { key: None, name: None }
    }
}
pub(crate) fn pheno_names(file: &str) -> Result<BTreeMap<String, String>, Error> {
    let reader = 
        BufReader::new(File::open(file).map_err(|e| Error::wrap(file.to_string(), e))?);
    let separator = ',';
    let tsv_eater_maker = PhenoNamesEaterMaker {};
    let tsv_reader = 
        TsvReader::new(reader, separator, tsv_eater_maker)?;
    let mut pheno_names = BTreeMap::new();
    for row in tsv_reader {
        let row = row?;
        pheno_names.insert(row.key, row.name);
    }
    Ok(pheno_names)
}