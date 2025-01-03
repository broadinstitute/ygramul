use crate::options::Options;

pub mod options;
pub mod error;

pub fn execute(options: &Options) {
    match options {
        Options::Hello => println!("Hello!")
    }
}
