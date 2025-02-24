use std::io::{BufReader, Read};
use crate::error::Error;
use crate::neo::Neo;
use crate::upload::cypher::CreateEdgeQueryBuilder2;
use crate::upload::UploadRowEater;

pub(crate) fn upload_f<R: Read>(reader: BufReader<R>, neo: &Neo, row_eater: &mut UploadRowEater,
                                 query_builder: &CreateEdgeQueryBuilder2)
                                 -> Result<(), Error> {
    todo!("Implement upload_f")
}