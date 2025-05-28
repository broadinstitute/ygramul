pub(crate) mod phenos;
pub(crate) mod factors;
pub(crate) mod pgs;

use crate::error::Error;
fn last_three_parts(string: &str) -> Option<(&str, &str, &str)> {
    if let Some((prefix, third)) = string.rsplit_once('/') {
        if let Some((prefix, second)) = prefix.rsplit_once('/') {
            if let Some((_, first)) = prefix.rsplit_once('/') {
                Some((first, second, third))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn handle_unclassified_file(file: &str) -> Result<(), Error> {
    ignore_file(file)
}

fn ignore_file(_file: &str) -> Result<(), Error> {
    Ok(())
}
