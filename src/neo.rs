use crate::error::Error;
use neo4rs::{Graph, Query, Row};
use tokio::runtime::Runtime;
use crate::config::Neo4jConfig;

pub(crate) trait RowEater {
    type Summary;
    fn eat(&mut self, row: Row) -> Result<(), Error>;
    fn finish(&mut self) -> Result<Self::Summary, Error>;
}
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
        Neo::new(&config.uri, &config.user, &config.password)
    }
    pub(crate) fn cypher<E: RowEater>(&self, query: Query, row_eater: &mut E)
        -> Result<E::Summary, Error> {
        self.runtime.block_on(async {
            let mut result = self.graph.execute(query).await?;
            while let Some(row) = result.next().await? {
                row_eater.eat(row)?;
            }
            Ok::<(), Error>(())
        })?;
        row_eater.finish()
    }
}
