use clap::{command, Command};
use ygramul::options::Options;
use ygramul::error::Error;


mod commands {
    pub(crate) const HELLO: &str = "hello";
    pub(crate) const HELLO_ABOUT: &str = "Prints some config information.";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const SURVEY_ABOUT: &str = "Surveys the data.";
    pub(crate) const ALL: [&str; 2] = [HELLO, SURVEY];
}

pub(crate) fn get_cli_options() -> Result<Options, Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new(commands::HELLO).about(commands::HELLO_ABOUT))
        .subcommand(Command::new(commands::SURVEY).about(commands::SURVEY_ABOUT))
        .get_matches();
    match matches.subcommand() {
        Some((commands::HELLO, _)) => Ok(Options::Hello),
        Some((commands::SURVEY, _)) => Ok(Options::Survey),
        Some((command, _)) =>
            Err(Error::from(
                format!("Unknown command: {}. {}", command, known_subcommands())
            )),
        None => Err(Error::from(format!("No command provided. {}", known_subcommands())))
    }
}

fn known_subcommands() -> String {
    format!("Known subcommands are '{}'.", commands::ALL.join("', '"))
}