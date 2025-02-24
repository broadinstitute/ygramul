use neo4rs::{query, Query};

pub(crate) struct Node {
    pub(crate) label: String,
    pub(crate) id: String,
}

pub(crate) struct Edge1 {
    pub(crate) label: String,
    pub(crate) key: String,
    pub(crate) value: f64,
}
pub(crate) struct Edge2 {
    pub(crate) label: String,
    pub(crate) key1: String,
    pub(crate) value1: f64,
    pub(crate) key2: String,
    pub(crate) value2: f64,
}

impl Node {
    pub(crate) fn new(label: String, id: String) -> Node {
        Node { label, id }
    }
}

impl Edge1 {
    pub(crate) fn new(label: String, key: String, value: f64) -> Edge1 {
        Edge1 { label, key, value }
    }
}
impl Edge2 {
    pub(crate) fn new(label: String, key1: String, value1: f64, key2: String, value2: f64)
        -> Edge2 {
        Edge2 { label, key1, value1, key2, value2 }
    }
}

const CREATE_EDGE1: &str = "\
MERGE (n1:$label1 {id: $id1})\n\
MERGE (n2:$label2 {id: $id2})\n\
MERGE (n1)-[e:$label_edge]->(n2)\n\
SET e += { $key: $value }";
const CREATE_EDGE2: &str = "\
MERGE (n1:$label1 {id: $id1})\n\
MERGE (n2:$label2 {id: $id2})\n\
MERGE (n1)-[e:$label_edge]->(n2)\n\
SET e += { $key1: $value1, $key2: $value2 }";

pub(crate) struct CreateEdgeQueryBuilder1 {
    query: Query
}
pub(crate) struct CreateEdgeQueryBuilder2 {
    query: Query
}

impl CreateEdgeQueryBuilder1 {
    pub(crate) fn new() -> Self {
        CreateEdgeQueryBuilder1 { query: query(CREATE_EDGE1) }
    }
    pub(crate) fn create_query(&self, node1: &Node, edge: &Edge1, node2: &Node) -> Query {
        self.query.clone()
            .param("label1", &*node1.label)
            .param("id1", &*node1.id)
            .param("label2", &*node2.label)
            .param("id2", &*node2.id)
            .param("label_edge", &*edge.label)
            .param("key", &*edge.key)
            .param("value", edge.value)
    }
}
impl CreateEdgeQueryBuilder2 {
    pub(crate) fn new() -> Self {
        CreateEdgeQueryBuilder2 { query: query(CREATE_EDGE2) }
    }
    pub(crate) fn create_query(&self, node1: &Node, edge: &Edge2, node2: &Node) -> Query {
        self.query.clone()
            .param("label1", &*node1.label)
            .param("id1", &*node1.id)
            .param("label2", &*node2.label)
            .param("id2", &*node2.id)
            .param("label_edge", &*edge.label)
            .param("key1", &*edge.key1)
            .param("value1", edge.value1)
            .param("key2", &*edge.key2)
            .param("value2", edge.value2)
    }
}