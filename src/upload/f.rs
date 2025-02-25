use crate::error::Error;
use crate::neo::Neo;
use crate::tsv::{TsvEater, TsvEaterMaker};
use crate::upload::cypher::CreateFactorNodeQueryBuilder;
use crate::upload::UploadRowEater;
use std::io::{BufReader, Read};

const FACTOR: &str = "Factor";
const LABEL: &str = "label";
struct FUploadEaterMaker {}
struct FUploadEater {
    subkey: Option<String>,
    label: Option<String>
}
struct Row {
    subkey: String,
    label: String
}

impl TsvEaterMaker for FUploadEaterMaker {
    type Row = Row;
    type Eater = FUploadEater;
    fn make(&self) -> Self::Eater {
        FUploadEater { subkey: None, label: None }
    }
}

impl TsvEater for FUploadEater {
    type Row = Row;
    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        match name {
            FACTOR => self.subkey = Some(value.to_string()),
            LABEL => self.label = Some(value.to_string()),
            _ => {}
        }
        Ok(())
    }
    fn finish(self) -> Result<Self::Row, Error> {
        let subkey = self.subkey.ok_or_else(|| Error::from("Missing Factor"))?;
        let label = self.label.ok_or_else(|| Error::from("Missing label"))?;
        Ok(Row { subkey, label })
    }
}

pub(crate) fn upload_f<R: Read>(key: &[String], reader: BufReader<R>, neo: &Neo,
                                row_eater: &mut UploadRowEater)
                                 -> Result<(), Error> {
    let eater_maker = FUploadEaterMaker {};
    let tsv_reader = crate::tsv::TsvReader::new(reader, eater_maker)?;
    let query_builder = CreateFactorNodeQueryBuilder::new();
    for row in tsv_reader {
        let row = row?;
        upload_row(key, neo, &query_builder, row_eater, row)?;
    }
    Ok(())
}

fn upload_row(key: &[String], neo: &Neo, query_builder: &CreateFactorNodeQueryBuilder,
              row_eater: &mut UploadRowEater, row: Row)
              -> Result<(), Error> {
    let node_id = format!("{}_{}", key.join("_"), row.subkey);
    let query = query_builder.create_query(&node_id, &row.label);
    println!("f, label: {}", row.label);
    neo.cypher(query, row_eater)?;
    Ok(())
}