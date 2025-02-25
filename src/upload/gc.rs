use crate::error::Error;
use crate::neo::Neo;
use crate::upload::entities::EntityUploadEaterMaker;
use crate::upload::{entities, UploadRowEater};
use std::io::{BufReader, Read};
use crate::upload::cypher::{CreateEntityEdgeQueryBuilder, CreateGeneEdgeQueryBuilder};

mod fields {
    pub const GENE: &str = "Gene";
}
pub(crate) fn upload_gc<R: Read>(key: &[String], reader: BufReader<R>, neo: &Neo,
                                 row_eater: &mut UploadRowEater) -> Result<(), Error> {
    let query_builder = CreateGeneEdgeQueryBuilder::new();
    let eater_maker = EntityUploadEaterMaker::new(fields::GENE.to_string());
    entities::upload_rows(key, reader, neo, &query_builder, row_eater, eater_maker)?;
    Ok(())
}


