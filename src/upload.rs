use log::error;
use crate::config::ClientConfig;
use crate::error::Error;
use crate::file_info::{FileGroup, FileKind};
use crate::survey::survey;

pub(crate) fn upload_data(config: &ClientConfig) -> Result<(), Error> {
    let file_infos = survey(&config.local_config)?;
    for (key, group) in file_infos.groups {
        upload_group(&key, &group, config)?
    }
    Ok(())
}

fn upload_group(key: &[String], group: &FileGroup, config: &ClientConfig) -> Result<(), Error> {
    for kind in &group.kinds {
        upload_kind(key, *kind, config)?
    }
    Ok(())
}

fn upload_kind(key: &[String], kind: FileKind, config: &ClientConfig) -> Result<(), Error> {
    let name = kind.create_name(key);
    let path = config.local_config.data_dir.join(&name);
    error!("Upload kind not implemented yet: {}", path.display());
    Ok(())
}