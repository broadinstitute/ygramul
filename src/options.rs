use crate::config::ConfigBuilder;

pub enum Options {
    Hello,
    Survey,
}

impl Options {
    pub fn to_config(&self) -> ConfigBuilder {
        ConfigBuilder::new()
    }
}