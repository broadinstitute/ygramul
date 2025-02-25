use neo4rs::{Query, query};

const CREATE_FACTOR_NODE: &str = "\
MERGE (n:Factor { id: $id })\n\
SET n += { name: $name }";

const CREATE_GENE_EDGE: &str = "\
MERGE (n1:Gene { id: $gene_id })\n\
MERGE (n2:Factor { id: $factor_id })\n\
MERGE (n1)-[e:IMPACTS]->(n2)\n\
SET e += { weight: $weight }";
const CREATE_PHENO_EDGE: &str = "\
MERGE (n1:Pheno { id: $pheno_id })\n\
MERGE (n2:Factor { id: $factor_id })\n\
MERGE (n1)-[e:CONTROLS]->(n2)\n\
SET e += { weight: $weight }";

pub(crate) struct CreateFactorNodeQueryBuilder {
    query: Query,
}

pub(crate) trait CreateEntityEdgeQueryBuilder {
    fn new() -> Self;
    fn create_query(&self, entity_id: &str, factor_id: &str, weight: f64) -> Query;
}
pub(crate) struct CreateGeneEdgeQueryBuilder {
    query: Query,
}
pub(crate) struct CreatePhenoEdgeQueryBuilder {
    query: Query,
}
impl CreateFactorNodeQueryBuilder {
    pub(crate) fn new() -> Self {
        CreateFactorNodeQueryBuilder {
            query: query(CREATE_FACTOR_NODE),
        }
    }
    pub(crate) fn create_query(&self, id: &str, name: &str) -> Query {
        self.query.clone().param("id", id).param("name", name)
    }
}

impl CreateEntityEdgeQueryBuilder for CreateGeneEdgeQueryBuilder {
    fn new() -> Self {
        CreateGeneEdgeQueryBuilder {
            query: query(CREATE_GENE_EDGE),
        }
    }
    fn create_query(&self, entity_id: &str, factor_id: &str, weight: f64) -> Query {
        self.query
            .clone()
            .param("gene_id", entity_id)
            .param("factor_id", factor_id)
            .param("weight", weight)
    }
}
impl CreateEntityEdgeQueryBuilder for CreatePhenoEdgeQueryBuilder {
    fn new() -> Self {
        CreatePhenoEdgeQueryBuilder {
            query: query(CREATE_PHENO_EDGE),
        }
    }
    fn create_query(&self, entity_id: &str, factor_id: &str, weight: f64) -> Query {
        self.query
            .clone()
            .param("pheno_id", entity_id)
            .param("factor_id", factor_id)
            .param("weight", weight)
    }
}
