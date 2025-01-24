use crate::error::Error;
use neo4rs::Graph;
use tokio::runtime::Runtime;
use crate::config::Neo4jConfig;

pub(crate) struct Neo {
    runtime: Runtime,
    graph: Graph,
}

impl Neo {
    pub(crate) fn new(uri: &str, user: &str, password: &str) -> Result<Neo, Error> {
        let runtime = Runtime::new()?;
        let graph = runtime.block_on(async { Graph::new(uri, user, password).await })?;
        Ok(Neo { runtime, graph })
    }
    pub(crate) fn for_config(config: &Neo4jConfig) -> Result<Neo, Error> {
        Neo::new(&config.uri, &config.creds.user, &config.creds.password)
    }
}
