use crate::config::Action;
use crate::error::Error;
use clap::{command, Arg, ArgMatches, Command};
use std::path::PathBuf;
use crate::config::action;

pub struct Args {
    pub(crate) data_dir: Option<PathBuf>,
    pub(crate) uri: Option<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) file: Option<String>,
}
pub struct CliOptions {
    pub(crate) action: Option<Action>,
    pub(crate) args: Args,
}
mod about {
    pub(crate) const HELLO: &str = "Prints some config information.";
    pub(crate) const SURVEY: &str = "Surveys the data.";
    pub(crate) const PING: &str = "Pings the Neo4j server.";
    pub(crate) const UPLOAD: &str = "Uploads data to the Neo4j server.";
    pub(crate) const WIPE: &str = "Deletes all data on the Neo4j server.";
    pub(crate) const CAT: &str = "Prints the content of the input file.";
}

mod args {
    pub(crate) const DATA_DIR: &str = "data-dir";
    pub(crate) const URI: &str = "uri";
    pub(crate) const USER: &str = "user";
    pub(crate) const PASSWORD: &str = "password";
    pub(crate) const FILE: &str = "file";
}

mod arg_short {
    pub(crate) const DATA_DIR: char = 'd';
    pub(crate) const URI: char = 'l';
    pub(crate) const USER: char = 'u';
    pub(crate) const PASSWORD: char = 'p';
    pub(crate) const FILE: char = 'f';
}

mod arg_help {
    pub(crate) const DATA_DIR: &str = "The directory containing the data.";
    pub(crate) const URI: &str = "The URI of the Neo4j server.";
    pub(crate) const USER: &str = "The user name for the Neo4j server.";
    pub(crate) const PASSWORD: &str = "The password for the Neo4j server.";
    pub(crate) const FILE: &str = "The input file";
}

pub fn get_cli_options() -> Result<CliOptions, Error> {
    let matches =
        add_args(command!())
        .subcommand(new_command(action::HELLO, about::HELLO))
        .subcommand(new_command(action::SURVEY, about::SURVEY))
            .subcommand(new_command(action::PING, about::PING))
            .subcommand(new_command(action::UPLOAD, about::UPLOAD))
            .subcommand(new_command(action::WIPE, about::WIPE))
            .subcommand(new_command(action::CAT, about::CAT))
        .get_matches();
    match matches.subcommand() {
        Some((action::HELLO, sub_matches)) =>
            Ok(new_options(Some(Action::Hello), sub_matches)),
        Some((action::SURVEY, sub_matches)) =>
            Ok(new_options(Some(Action::Survey), sub_matches)),
        Some((action::PING, sub_matches)) =>
            Ok(new_options(Some(Action::Ping), sub_matches)),
        Some((action::UPLOAD, sub_matches)) =>
            Ok(new_options(Some(Action::Upload), sub_matches)),
        Some((action::WIPE, sub_matches)) =>
            Ok(new_options(Some(Action::Wipe), sub_matches)),
        Some((action::CAT, sub_matches)) =>
            Ok(new_options(Some(Action::Cat), sub_matches)),
        Some((command, _)) =>
            Err(Error::from(
                format!("Unknown command: {}. {}", command, known_subcommands())
            )),
        None => Ok(new_options(None, &matches)),
    }
}

fn new_command(name: &'static str, about: &'static str) -> Command {
    add_args(Command::new(name).about(about))
}

fn add_args(command: Command) -> Command {
    command
        .arg(new_arg(args::DATA_DIR, arg_short::DATA_DIR, arg_help::DATA_DIR)
            .value_parser(clap::value_parser!(PathBuf)))
        .arg(new_arg(args::URI, arg_short::URI, arg_help::URI))
        .arg(new_arg(args::USER, arg_short::USER, arg_help::USER))
        .arg(new_arg(args::PASSWORD, arg_short::PASSWORD, arg_help::PASSWORD))
        .arg(new_arg(args::FILE, arg_short::FILE, arg_help::FILE))
}

fn new_arg(name: &'static str, short: char, help: &'static str) -> Arg {
    Arg::new(name).short(short).help(help)
}
fn known_subcommands() -> String {
    format!("Known subcommands are '{}'.", action::ALL.join("', '"))
}

fn extract_args(matches: &ArgMatches) -> Args {
    Args {
        data_dir: matches.get_one::<PathBuf>(args::DATA_DIR).cloned(),
        uri: matches.get_one::<String>(args::URI).cloned(),
        user: matches.get_one::<String>(args::USER).cloned(),
        password: matches.get_one::<String>(args::PASSWORD).cloned(),
        file: matches.get_one::<String>(args::FILE).cloned(),
    }
}

fn new_options(action: Option<Action>, matches: &ArgMatches) -> CliOptions {
    CliOptions { action, args: extract_args(matches), }
}