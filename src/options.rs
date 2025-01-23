use crate::config::{ConfigBuilder, CredsBuilder};

pub enum Options {
    Hello,
    Survey,
    Ping(CredsBuilder)
}

impl Options {
    pub fn to_config(&self) -> ConfigBuilder {
        ConfigBuilder::new()
    }
}