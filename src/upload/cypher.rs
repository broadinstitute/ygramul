use neo4rs::Query;
use std::collections::BTreeMap;
use log::info;

pub(crate) struct Node {
    pub(crate) label: String,
    pub(crate) id: String,
}

pub(crate) struct Edge {
    pub(crate) label: String,
    pub(crate) props: BTreeMap<String, f64>,
}

impl Node {
    pub(crate) fn new(label: String, id: String) -> Node {
        Node { label, id }
    }
    pub(crate) fn cypher(&self) -> String {
        format!("(n:{} {{id: '{}'}})", encode(&self.label), encode(&self.id))
    }
}

impl Edge {
    pub(crate) fn new(label: String, props: BTreeMap<String, f64>) -> Edge {
        Edge { label: label.to_string(), props }
    }
    pub(crate) fn cypher(&self) -> String {
        let props =
            self.props.iter()
                .map(|(key, value)| format!("{}: {}", encode(key), value))
                .collect::<Vec<String>>().join(", ");
        format!("[:{} {{{}}}]", encode(&self.label), props)
    }
}

pub(crate) fn create_node(node: &Node) -> Query {
    let cypher= format!("CREATE {}", node.cypher());
    info!("{}", cypher);
    Query::new(cypher)
}

pub(crate) fn create_edge(node1: &Node, edge: &Edge, node2: &Node) -> Query {
    let cypher =
        format!("MATCH (node1:{} {{ id: '{}' }}), (node2:{} {{ id: '{}' }}) \n\
        CREATE (node1)-{}->(node2)", encode(&node1.label), encode(&node1.id), encode(&node2.label),
                encode(&node2.id), encode(&edge.cypher()));
    info!("{}", cypher);
    Query::new(cypher)
}

fn encode(string: &str) -> String {
    string.replace("'", "\\'")
}
