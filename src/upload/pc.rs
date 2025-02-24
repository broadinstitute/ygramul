use crate::error::Error;
use crate::neo::Neo;
use crate::upload::UploadRowEater;
use std::io::{BufReader, Read};
use crate::tsv::{TsvEater, TsvReader};
use crate::upload::cypher::{CreateEdgeQueryBuilder1, Node};

struct PcUploadEater {
    pheno: Option<String>,
    subkeys: Vec<String>,
}

struct PcRow {
    pheno: String,
    subkeys: Vec<String>,
}

mod fields {
    pub const PHENO: &str = "Pheno";
}
impl TsvEater for PcUploadEater {
    type Row = PcRow;
    fn new() -> Self {
        PcUploadEater { pheno: None, subkeys: Vec::new() }
    }
    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            fields::PHENO => self.pheno = Some(value.to_string()),
            _ => {
                if let Some(subkey) = name.strip_prefix("Factor") {
                    self.subkeys.push(subkey.to_string());
                }
            }
        }
        Ok(())
    }
    fn finish(self) -> Result<Self::Row, Error> {
        let PcUploadEater { pheno, subkeys } = self;
        let pheno = pheno.ok_or_else(|| Error::from("Missing pheno"))?;
        Ok(PcRow { pheno, subkeys })
    }
}
pub(crate) fn upload_pc<R: Read>(key: &[String], reader: BufReader<R>, neo: &Neo,
                                 row_eater: &mut UploadRowEater) -> Result<(), Error> {
    let mut tsv_reader: TsvReader<_, PcUploadEater> = TsvReader::new(reader)?;
    let query_builder = CreateEdgeQueryBuilder1::new();
    for row in tsv_reader {
        upload_row(key, neo, &query_builder, row_eater, row?)?;
    }
    Ok(())
}

fn upload_row(key: &[String], neo: &Neo, query_builder: &CreateEdgeQueryBuilder1,
              row_eater: &mut UploadRowEater, row: PcRow) -> Result<(), Error> {
    let pheno_node = Node::new("Pheno".to_string(), row.pheno);
    for subkey in row.subkeys {
        let subkey_node = Node::new("Factor".to_string(), subkey);
        let query = query_builder.create_query(&pheno_node, &subkey_node);
        neo.cypher(query, row_eater)?;
    }
    Ok(())
}
