use crate::config;
use crate::config::{Action, ACTIONS};
use crate::error::Error;
use clap::{command, Arg, ArgMatches, Command};
use std::path::PathBuf;

pub struct Args {
    pub(crate) data_dir: Option<PathBuf>,
    pub(crate) uri: Option<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) file: Option<String>,
    pub(crate) out: Option<String>,
}
pub struct CliOptions {
    pub(crate) action: Option<Action>,
    pub(crate) args: Args,
}
mod args {
    pub(crate) const DATA_DIR: &str = "data-dir";
    pub(crate) const URI: &str = "uri";
    pub(crate) const USER: &str = "user";
    pub(crate) const PASSWORD: &str = "password";
    pub(crate) const FILE: &str = "file";
    pub(crate) const OUT: &str = "out";
}

mod arg_short {
    pub(crate) const DATA_DIR: char = 'd';
    pub(crate) const URI: char = 'l';
    pub(crate) const USER: char = 'u';
    pub(crate) const PASSWORD: char = 'p';
    pub(crate) const FILE: char = 'f';
    pub(crate) const OUT: char = 'o';
}

mod arg_help {
    pub(crate) const DATA_DIR: &str = "The directory containing the data.";
    pub(crate) const URI: &str = "The URI of the Neo4j server.";
    pub(crate) const USER: &str = "The user name for the Neo4j server.";
    pub(crate) const PASSWORD: &str = "The password for the Neo4j server.";
    pub(crate) const FILE: &str = "The input file";
    pub(crate) const OUT: &str = "The output directory";
}

pub fn get_cli_options() -> Result<CliOptions, Error> {
    let matches = add_subcommands(add_args(command!())).get_matches();
    match matches.subcommand() {
        Some((command, sub_matches)) => Action::try_from(command)
            .map_err(|_| Error::from(
                format!("Unknown command: {}. {}", command, known_subcommands())
            ))
            .map(|action| new_options(Some(action), sub_matches)),
        None => Ok(new_options(None, &matches)),
    }
}

fn new_command(name: &'static str, about: &'static str) -> Command {
    add_args(Command::new(name).about(about))
}

fn add_args(command: Command) -> Command {
    command
        .arg(
            new_arg(args::DATA_DIR, arg_short::DATA_DIR, arg_help::DATA_DIR)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(new_arg(args::URI, arg_short::URI, arg_help::URI))
        .arg(new_arg(args::USER, arg_short::USER, arg_help::USER))
        .arg(new_arg(args::PASSWORD, arg_short::PASSWORD, arg_help::PASSWORD))
        .arg(new_arg(args::FILE, arg_short::FILE, arg_help::FILE))
        .arg(new_arg(args::OUT, arg_short::OUT, arg_help::OUT))
}

fn new_arg(name: &'static str, short: char, help: &'static str) -> Arg {
    Arg::new(name).short(short).help(help)
}
fn known_subcommands() -> String {
    format!("Known subcommands are '{}'.", config::all_actions_list())
}

fn add_subcommands(command: Command) -> Command {
    ACTIONS.into_iter().fold(command, |cmd, subcommand| {
        cmd.subcommand(new_command(subcommand.name(), subcommand.about()))
    })
}

fn extract_args(matches: &ArgMatches) -> Args {
    Args {
        data_dir: matches.get_one::<PathBuf>(args::DATA_DIR).cloned(),
        uri: matches.get_one::<String>(args::URI).cloned(),
        user: matches.get_one::<String>(args::USER).cloned(),
        password: matches.get_one::<String>(args::PASSWORD).cloned(),
        file: matches.get_one::<String>(args::FILE).cloned(),
        out: matches.get_one::<String>(args::OUT).cloned(),
    }
}

fn new_options(action: Option<Action>, matches: &ArgMatches) -> CliOptions {
    CliOptions {
        action,
        args: extract_args(matches),
    }
}
