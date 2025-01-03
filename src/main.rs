mod cli;

use ygramul::error::Error;
use ygramul::execute;

fn main() {
    match run2() {
        Ok(_) => { println!("Done!") }
        Err(error) => { println!("Error: {}", error)}
    }
}

fn run2() -> Result<(), Error> {
    let options = cli::get_cli_options()?;
    execute(&options);
    Ok(())
}
