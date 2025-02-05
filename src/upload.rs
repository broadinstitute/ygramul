use crate::config::ClientConfig;
use crate::error::Error;
use crate::file_info::{FileGroup, FileKind};
use crate::survey::survey;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::neo::{Neo, RowEater};
use crate::upload::cypher::CreateEdgeQueryBuilder;
use crate::upload::gss::upload_gss;

mod gss;
mod cypher;

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
    let query_builder = CreateEdgeQueryBuilder::new();
    match kind {
        FileKind::Gss => { upload_gss(reader, neo, row_eater, &query_builder)? }
        FileKind::Gs => { kind_not_implemented(&path)? }
        FileKind::F => { kind_not_implemented(&path)? }
        FileKind::GscOut => { kind_not_implemented(&path)? }
        FileKind::GscList => { kind_not_implemented(&path)? }
        FileKind::Gc => { kind_not_implemented(&path)? }
        FileKind::Pc => { kind_not_implemented(&path)? }
        FileKind::Pc1 => { kind_not_implemented(&path)? }
        FileKind::Pc2 => { kind_not_implemented(&path)? }
        FileKind::Pc3 => { kind_not_implemented(&path)? }
        FileKind::PcList => { kind_not_implemented(&path)? }
    }
    Ok(())
}

fn kind_not_implemented(path: &Path) -> Result<(), Error> {
    Err(Error::from(
        format!("Reading this file is not implemented: '{}'.", path.display())
    ))
}