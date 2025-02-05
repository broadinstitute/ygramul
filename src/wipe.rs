use log::{error, info};
use neo4rs::query;
use crate::config::ClientConfig;
use crate::error::Error;
use crate::neo::RowEater;

struct WipeRowEater {

}

impl WipeRowEater {
    pub(crate) fn new() -> WipeRowEater {
        WipeRowEater {}
    }
}

impl RowEater for WipeRowEater {
    type Summary = ();
    fn eat(&mut self, row: neo4rs::Row) -> Result<(), Error> {
        info!("{:?}", row);
        Ok(())
    }
    fn finish(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub(crate) fn wipe(config: &ClientConfig) -> Result<(), Error> {
    let neo = crate::neo::Neo::for_config(&config.neo4j)?;
    let mut n_nodes: i64 = 1;
    let mut n_retries: usize = 0;
    const MAX_RETRIES: usize = 10;
    loop {
        let query_base = query("MATCH (n) LIMIT $n_nodes DETACH DELETE n");
        let query = query_base.param("n_nodes", n_nodes);
        let mut row_eater = WipeRowEater::new();
        info!("Wiping {} nodes", n_nodes);
        match neo.cypher(query, &mut row_eater) {
            Ok(_) => {
                info!("Wiped {} nodes", n_nodes);
                n_retries = 0;
                n_nodes += 1 + n_nodes / 10;
            }
            Err(_) => {
                error!("Failed to wipe {} nodes", n_nodes);
                n_nodes -= 1 + n_nodes / 10;
                if n_nodes < 1 {
                    n_nodes = 1;
                    n_retries += 1;
                    if n_retries >= MAX_RETRIES {
                        break;
                    }
                }
            }
        };
    }
    Ok(())
}