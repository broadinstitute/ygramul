use crate::config::ClientConfig;
use crate::error::Error;
use crate::file_info::{FileGroup, FileKind};
use crate::survey::survey;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::neo::{Neo, RowEater};
use crate::upload::f::upload_f;
use crate::upload::gc::upload_gc;
use crate::upload::pc::upload_pc;

mod cypher;
mod pc;
mod gc;
mod f;
mod entities;
mod factor;

pub(crate) struct UploadRowEater {

}

impl UploadRowEater {
    pub(crate) fn new() -> Self {
        UploadRowEater {}
    }
}

impl RowEater for UploadRowEater {
    type Summary = ();
    fn eat(&mut self, _row: neo4rs::Row) -> Result<(), Error> {
        Ok(())
    }
    fn finish(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub(crate) fn upload_data(config: &ClientConfig) -> Result<(), Error> {
    let file_infos = survey(&config.local_config)?;
    let neo = Neo::for_config(&config.neo4j)?;
    let mut row_eater = UploadRowEater::new();
    for (key, group) in file_infos.groups {
        upload_group(&key, &group, config, &neo, &mut row_eater)?
    }
    Ok(())
}

fn upload_group(key: &[String], group: &FileGroup, config: &ClientConfig, neo: &Neo,
                row_eater: &mut UploadRowEater) -> Result<(), Error> {
    for kind in &group.kinds {
        upload_kind(key, *kind, config, neo, row_eater)?
    }
    Ok(())
}

fn upload_kind(key: &[String], kind: FileKind, config: &ClientConfig, neo: &Neo,
               row_eater: &mut UploadRowEater) -> Result<(), Error> {
    let name = kind.create_name(key);
    let path = config.local_config.data_dir.join(&name);
    let file = File::open(&path).map_err(|io_error|
        Error::wrap(path.display().to_string(), io_error)
    )?;
    let reader = BufReader::new(file);
    match kind {
        FileKind::Gss => { ignore_file(&path) }
        FileKind::Gs => { ignore_file(&path) }
        FileKind::F => { upload_f(key, reader, neo, row_eater)? }
        FileKind::GscOut => { ignore_file(&path) }
        FileKind::GscList => { ignore_file(&path) }
        FileKind::Gc => { upload_gc(key, reader, neo, row_eater)? }
        FileKind::Pc => { upload_pc(key, reader, neo, row_eater)? }
        FileKind::Pc1 => { ignore_file(&path) }
        FileKind::Pc2 => { ignore_file(&path) }
        FileKind::Pc3 => { ignore_file(&path) }
        FileKind::PcList => { ignore_file(&path) }
    }
    Ok(())
}

fn ignore_file(path: &Path) {
    log::info!("Ignoring file '{}'.", path.display());
}