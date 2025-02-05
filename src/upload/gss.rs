use crate::error::Error;
use crate::neo::Neo;
use crate::tsv::{Column, TsvReader, Type};
use crate::upload::cypher::{CreateEdgeQueryBuilder, Edge, Node};
use crate::upload::UploadRowEater;
use std::io::{BufReader, Read};

const TRAIT: &str = "Trait";
const GENE_SET: &str = "Gene_Set";
const BETA: &str = "beta";
const BETA_UNCORRECTED: &str = "beta_uncorrected";
pub(crate) fn upload_gss<R: Read>(reader: BufReader<R>, neo: &Neo, row_eater: &mut UploadRowEater,
                                  query_builder: &CreateEdgeQueryBuilder)
    -> Result<(), Error> {
    let cols = vec![
        Column::new(TRAIT, Type::String),
        Column::new(GENE_SET, Type::String),
        Column::new(BETA, Type::Float),
        Column::new(BETA_UNCORRECTED, Type::Float),
    ];
    let tsv_reader = TsvReader::new(reader, cols)?;
    for row in tsv_reader {
        let mut row = row?;
        let the_trait = row.remove_string_or_error(TRAIT)?;
        let gene_set = row.remove_string_or_error(GENE_SET)?;
        let beta = row.remove_float_or_error(BETA)?;
        let beta_uncorrected = row.remove_float_or_error(BETA_UNCORRECTED)?;
        create_items(neo, row_eater, query_builder, the_trait, gene_set, beta, beta_uncorrected)?;
    }
    Ok(())
}

fn create_items(neo: &Neo, row_eater: &mut UploadRowEater, query_builder: &CreateEdgeQueryBuilder,
                the_trait: String, gene_set: String, beta: f64, beta_uncorrected: f64)
    -> Result<(), Error> {
    let trait_node = Node::new("Trait".to_string(), the_trait);
    let gene_set_node = Node::new("GeneSet".to_string(), gene_set);
    let edge =
        Edge::new("MANIFESTS".to_string(), "beta".to_string(), beta,
                  "beta_uncorrected".to_string(), beta_uncorrected);
    let query = query_builder.create_query(&gene_set_node, &edge, &trait_node);
    neo.cypher(query, row_eater)?;
    Ok(())
}
