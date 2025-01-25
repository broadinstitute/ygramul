use clap::{command, Command};
use crate::config::Action;
use crate::error::Error;

pub struct CliOptions {
    pub(crate) action: Option<Action>,
}
mod commands {
    pub(crate) const HELLO: &str = "hello";
    pub(crate) const HELLO_ABOUT: &str = "Prints some config information.";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const SURVEY_ABOUT: &str = "Surveys the data.";
    pub(crate) const PING: &str = "ping";
    pub(crate) const PING_ABOUT: &str = "Pings the Neo4j server.";
    pub(crate) const ALL: [&str; 3] = [HELLO, SURVEY, PING];
}

pub fn get_cli_options() -> Result<CliOptions, Error> {
    let matches = command!()
        .subcommand(Command::new(commands::HELLO).about(commands::HELLO_ABOUT))
        .subcommand(Command::new(commands::SURVEY).about(commands::SURVEY_ABOUT))
        .subcommand(Command::new(commands::PING).about(commands::PING_ABOUT))
        .get_matches();
    match matches.subcommand() {
        Some((commands::HELLO, _)) => Ok(CliOptions { action: Some(Action::Hello) }),
        Some((commands::SURVEY, _)) => Ok(CliOptions { action: Some(Action::Survey) }),
        Some((commands::PING, _)) => Ok(CliOptions { action: Some(Action::Ping) }),
        Some((command, _)) =>
            Err(Error::from(
                format!("Unknown command: {}. {}", command, known_subcommands())
            )),
        None => Ok(CliOptions { action: None }),
    }
}

fn known_subcommands() -> String {
    format!("Known subcommands are '{}'.", commands::ALL.join("', '"))
}