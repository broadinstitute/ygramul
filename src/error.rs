use std::fmt::{Debug, Display, Formatter};
use log::SetLoggerError;

mod errors {
    pub(crate) const IO_ERROR: &str = "I/O error";
    pub(crate) const SET_LOGGER_ERROR: &str = "Set logger error";
    pub(crate) const NEO4RS_ERROR: &str = "Neo4rs error";
}
pub struct Error {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    fn new(message: String, source: Option<Box<dyn std::error::Error>>) -> Error {
        Error { message, source }
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
    pub fn wrap<E: std::error::Error + 'static>(message: String, error: E) -> Error {
        Error::new(message, Some(Box::new(error)))
    }
    pub fn approximate_clone(&self) -> Error {
        let message = self.message.clone();
        let source =
            self.source.as_ref().map(|e| sorta_clone(e.as_ref()));
        Error::new(message, source)
    }
}

fn sorta_clone(error: &dyn std::error::Error) -> Box<dyn std::error::Error> {
    let message = error.to_string();
    let source =
        error.source().map(|e| sorta_clone(e));
    Box::new(Error::new(message, source))
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        let mut source = self.source();
        while let Some(e) = source {
            write!(f, ": {}", e)?;
            source = e.source();
        }
        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::new(message, None)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::new(message.to_string(), None)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(errors::IO_ERROR.to_string(), Some(Box::new(error)))
    }
}

impl From<SetLoggerError> for Error {
    fn from(error: SetLoggerError) -> Self {
        Error::new(errors::SET_LOGGER_ERROR.to_string(), Some(Box::new(error)))
    }
}

impl From<neo4rs::Error> for Error {
    fn from(error: neo4rs::Error) -> Self {
        Error::new(errors::NEO4RS_ERROR.to_string(), Some(Box::new(error)))
    }
}

