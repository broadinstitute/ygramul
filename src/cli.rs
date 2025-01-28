use crate::config::Action;
use crate::error::Error;
use clap::{command, Arg, ArgMatches, Command};
use std::path::PathBuf;

pub struct Args {
    pub(crate) data_dir: Option<PathBuf>,
    pub(crate) uri: Option<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
}
pub struct CliOptions {
    pub(crate) action: Option<Action>,
    pub(crate) args: Args,
}
mod cmds {
    pub(crate) const HELLO: &str = "hello";
    pub(crate) const SURVEY: &str = "survey";
    pub(crate) const PING: &str = "ping";
    pub(crate) const ALL: [&str; 3] = [HELLO, SURVEY, PING];
}

mod about {
    pub(crate) const HELLO: &str = "Prints some config information.";
    pub(crate) const SURVEY: &str = "Surveys the data.";
    pub(crate) const PING: &str = "Pings the Neo4j server.";
}

mod args {
    pub(crate) const DATA_DIR: &str = "data-dir";
    pub(crate) const URI: &str = "uri";
    pub(crate) const USER: &str = "user";
    pub(crate) const PASSWORD: &str = "password";
}

mod arg_short {
    pub(crate) const DATA_DIR: char = 'd';
    pub(crate) const URI: char = 'l';
    pub(crate) const USER: char = 'u';
    pub(crate) const PASSWORD: char = 'p';
}

mod arg_help {
    pub(crate) const DATA_DIR: &str = "The directory containing the data.";
    pub(crate) const URI: &str = "The URI of the Neo4j server.";
    pub(crate) const USER: &str = "The user name for the Neo4j server.";
    pub(crate) const PASSWORD: &str = "The password for the Neo4j server.";
}

pub fn get_cli_options() -> Result<CliOptions, Error> {
    let matches =
        add_args(command!())
        .subcommand(new_command(cmds::HELLO, about::HELLO))
        .subcommand(new_command(cmds::SURVEY, about::SURVEY))
        .subcommand(new_command(cmds::PING, about::PING))
        .get_matches();
    match matches.subcommand() {
        Some((cmds::HELLO, sub_matches)) =>
            Ok(new_options(Some(Action::Hello), sub_matches)),
        Some((cmds::SURVEY, sub_matches)) =>
            Ok(new_options(Some(Action::Survey), sub_matches)),
        Some((cmds::PING, sub_matches)) =>
            Ok(new_options(Some(Action::Ping), sub_matches)),
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
}

fn new_arg(name: &'static str, short: char, help: &'static str) -> Arg {
    Arg::new(name).short(short).help(help)
}
fn known_subcommands() -> String {
    format!("Known subcommands are '{}'.", cmds::ALL.join("', '"))
}

fn extract_args(matches: &clap::ArgMatches) -> Args {
    Args {
        data_dir: matches.get_one::<PathBuf>(args::DATA_DIR).cloned(),
        uri: matches.get_one::<String>(args::URI).cloned(),
        user: matches.get_one::<String>(args::USER).cloned(),
        password: matches.get_one::<String>(args::PASSWORD).cloned(),
    }
}

fn new_options(action: Option<Action>, matches: &ArgMatches) -> CliOptions {
    CliOptions { action, args: extract_args(matches), }
}