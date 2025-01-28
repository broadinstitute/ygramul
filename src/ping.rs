use std::time::{Duration, UNIX_EPOCH};
use humantime::format_rfc3339_millis;
use log::info;
use crate::config::Config;
use crate::error::Error;
use crate::neo::RowEater;
use neo4rs::{Query, Row};

const PING_QUERY: &str = "RETURN timestamp()";
const KEY_TIMESTAMP: &str = "timestamp()";
struct PingRowEater {
    timestamp: Option<u64>,
}

impl PingRowEater {
    fn new() -> PingRowEater {
        PingRowEater { timestamp: None }
    }
}

impl RowEater for PingRowEater {
    type Summary = u64;
    fn eat(&mut self, row: Row) -> Result<(), Error> {
        self.timestamp = Some(row.get(KEY_TIMESTAMP)?);
        Ok(())
    }
    fn finish(&mut self) -> Result<u64, Error> {
        let timestamp = self.timestamp.ok_or(Error::from("No timestamp"))?;
        Ok(timestamp)
    }
}

pub(crate) fn ping_neo4j(config: &Config) -> Result<(), Error> {
    let neo = crate::neo::Neo::for_config(&config.neo4j)?;
    let query = Query::new(PING_QUERY.to_string());
    let mut row_eater = PingRowEater::new();
    let timestamp = neo.cypher(query, &mut row_eater)?;
    info!("Neo4j is up and running as of {}",
        format_rfc3339_millis(UNIX_EPOCH + Duration::from_millis(timestamp)));
    Ok(())
}
