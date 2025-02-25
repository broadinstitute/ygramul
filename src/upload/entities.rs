use crate::error::Error;
use crate::neo::Neo;
use crate::tsv::{TsvEater, TsvEaterMaker, TsvReader};
use crate::upload::cypher::CreateEntityEdgeQueryBuilder;
use crate::upload::UploadRowEater;
use std::io::{BufReader, Read};


pub struct EntityUploadEaterMaker {
    entity_class: String,
}

pub(crate) struct EntityUploadEater {
    entity_class: String,
    entity: Option<String>,
    subkeys: Vec<String>,
    weights: Vec<f64>,
}

pub(crate) struct EntityRow {
    entity_class: String,
    entity: String,
    subkeys: Vec<String>,
    weights: Vec<f64>,
}

impl EntityUploadEaterMaker {
    pub fn new(entity_class: String) -> EntityUploadEaterMaker {
        EntityUploadEaterMaker { entity_class }
    }
}

impl TsvEaterMaker for EntityUploadEaterMaker {
    type Row = EntityRow;
    type Eater = EntityUploadEater;
    fn make(&self) -> Self::Eater {
        EntityUploadEater {
            entity_class: self.entity_class.to_string(),
            entity: None,
            subkeys: Vec::new(),
            weights: Vec::new(),
        }
    }
}
impl TsvEater for EntityUploadEater {
    type Row = EntityRow;
    fn field(&mut self, name: &str, value: &str) -> Result<(), Error> {
        if name == self.entity_class {
            self.entity = Some(value.to_string())
        } else if let Some(subkey) = name.strip_prefix("Factor") {
            let weight = value.parse::<f64>()?;
            self.subkeys.push(subkey.to_string());
            self.weights.push(weight);
        }
        Ok(())
    }
    fn finish(self) -> Result<Self::Row, Error> {
        let EntityUploadEater {
            entity_class,
            entity: pheno,
            subkeys,
            weights,
        } = self;
        let entity =
            pheno.ok_or_else(|| Error::from(format!("Missing {}", entity_class)))?;
        Ok(EntityRow {
            entity_class,
            entity,
            subkeys,
            weights,
        })
    }
}

pub fn upload_rows<R: Read, B: CreateEntityEdgeQueryBuilder>(
    key: &[String],
    reader: BufReader<R>,
    neo: &Neo,
    query_builder: &B,
    row_eater: &mut UploadRowEater,
    eater_maker: EntityUploadEaterMaker,
) -> Result<(), Error> {
    let tsv_reader: TsvReader<_, EntityUploadEaterMaker> = TsvReader::new(reader, eater_maker)?;
    for row in tsv_reader {
        upload_row(key, neo, query_builder, row_eater, row?)?;
    }
    Ok(())
}

fn upload_row<B: CreateEntityEdgeQueryBuilder>(
    key: &[String],
    neo: &Neo,
    query_builder: &B,
    row_eater: &mut UploadRowEater,
    row: EntityRow,
) -> Result<(), Error> {
    let key_prefix = key.join("_");
    for (subkey, &weight) in row.subkeys.iter().zip(row.weights.iter()) {
        let factor_id = format!("{}_{}", key_prefix, subkey);
        let query = query_builder.create_query(&row.entity, &factor_id, weight);
        println!("{}", row.entity_class);
        neo.cypher(query, row_eater)?;
    }
    Ok(())
}
