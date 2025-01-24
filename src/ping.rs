use crate::config::Config;
use crate::error::Error;

pub(crate) fn ping_neo4j(config: &Config) -> Result<(), Error> {
    let neo = crate::neo::Neo::for_config(&config.neo4j)?;
    todo!("Implement ping_neo4j")
}