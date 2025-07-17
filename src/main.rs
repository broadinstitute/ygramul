mod load_config;

use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, Config as LogConfig, TermLogger, TerminalMode};
use ygramul::error::Error;
use ygramul::execute;
use ygramul::cli::get_cli_options;

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();
    match run() {
        Ok(_) => { info!("Done!") }
        Err(error) => { error!("Error: {error}")}
    }
}

fn run() -> Result<(), Error> {
    let options = get_cli_options()?;
    let config = load_config::load_config()?.with_cli_options(options).build()?;
    execute(&config)?;
    Ok(())
}
