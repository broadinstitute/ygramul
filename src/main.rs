mod cli;
mod load_config;

use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, Config as LogConfig, TermLogger, TerminalMode};
use ygramul::error::Error;
use ygramul::execute;

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();
    match run() {
        Ok(_) => { info!("Done!") }
        Err(error) => { error!("Error: {}", error)}
    }
}

fn run() -> Result<(), Error> {
    let options = cli::get_cli_options()?;
    let config = load_config::load_config()?.build()?;
    execute(&options, &config)?;
    Ok(())
}
