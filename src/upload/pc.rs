use std::io::{BufReader, Read};
use crate::error::Error;
use crate::neo::Neo;
use crate::upload::cypher::CreateEdgeQueryBuilder;
use crate::upload::UploadRowEater;

pub(crate) fn upload_pc<R: Read>(reader: BufReader<R>, neo: &Neo, row_eater: &mut UploadRowEater,
                                 query_builder: &CreateEdgeQueryBuilder)
                                 -> Result<(), Error> {
    todo!("Implement upload_pc")
}