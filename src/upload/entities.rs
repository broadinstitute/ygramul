use crate::error::Error;
use crate::neo::Neo;
use crate::tsv::{TsvEater, TsvEaterMaker, TsvReader};
use crate::upload::cypher::CreateEntityEdgeQueryBuilder;
use crate::upload::UploadRowEater;
use std::io::{BufReader, Read};
use crate::upload::factor::factor_id;

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
    entity: String,
    subkeys: Vec<String>,
    weights: Vec<f64>,
    weight_max: f64
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
        let weight_max = weights.iter().cloned().fold(0.0, f64::max);
        Ok(EntityRow {
            entity,
            subkeys,
            weights,
            weight_max,
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
    threshold: f64
) -> Result<(), Error> {
    let tsv_reader: TsvReader<_, EntityUploadEaterMaker> = 
        TsvReader::new(reader, '\t', eater_maker)?;
    for row in tsv_reader {
        upload_row(key, neo, query_builder, row_eater, row?, threshold)?;
    }
    Ok(())
}

fn upload_row<B: CreateEntityEdgeQueryBuilder>(
    key: &[String],
    neo: &Neo,
    query_builder: &B,
    row_eater: &mut UploadRowEater,
    row: EntityRow,
    threshold: f64
) -> Result<(), Error> {
    for (subkey, &weight) in row.subkeys.iter().zip(row.weights.iter()) {
        if weight > row.weight_max * threshold {
            let factor_id = factor_id(key, subkey);
            println!("{}, {}, {}", &row.entity, &factor_id, weight);
            let query = query_builder.create_query(&row.entity, &factor_id, weight);
            neo.cypher(query, row_eater)?;
        }
    }
    Ok(())
}
