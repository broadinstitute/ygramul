use crate::error::Error;
use crate::neo::Neo;
use crate::upload::{entities, UploadRowEater};
use std::io::{BufReader, Read};
use crate::upload::cypher::{CreateEntityEdgeQueryBuilder, CreatePhenoEdgeQueryBuilder};
use crate::upload::entities::EntityUploadEaterMaker;

mod fields {
    pub const PHENO: &str = "Pheno";
}

const THRESHOLD: f64 = 0.01;
pub(crate) fn upload_pc<R: Read>(key: &[String], reader: BufReader<R>, neo: &Neo,
                                 row_eater: &mut UploadRowEater) -> Result<(), Error> {
    let query_builder = CreatePhenoEdgeQueryBuilder::new();
    let eater_maker = EntityUploadEaterMaker::new(fields::PHENO.to_string());
    entities::upload_rows(key, reader, neo, &query_builder, row_eater, eater_maker, THRESHOLD)?;
    Ok(())
}

