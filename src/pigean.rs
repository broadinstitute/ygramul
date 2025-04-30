mod pheno_names;

use crate::config::PigeanConfig;
use crate::error::Error;

pub(crate) fn create_bulk_files(config: &PigeanConfig) -> Result<(), Error> {
    let pheno_names = pheno_names::pheno_names(&config.pheno_names)?;
    for (key, name) in pheno_names {
        println!("{}: {}", key, name);
    }
    todo!()
}