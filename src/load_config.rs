use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use ygramul::config::ConfigBuilder;
use ygramul::error::Error;

const APP_DIR: &str = "ygramul";
const CONFIG_FILE: &str = "config.toml";
pub(crate) fn load_config() -> Result<ConfigBuilder, Error> {
    let app_dir = get_app_dir()?;
    let config_file = app_dir.join(CONFIG_FILE);
    let config_toml =
        read_to_string(&config_file)
            .map_err(|io_error|
                Error::wrap(config_file.to_string_lossy().to_string(), io_error)
            )?;
    let config = ConfigBuilder::try_from(config_toml.as_str())?;
    Ok(config)
}


fn get_app_dir() -> Result<PathBuf, Error> {
    let app_dir1 =
        env::var_os("XDG_CONFIG_HOME")
            .filter(|os_string| !os_string.is_empty())
            .map(|os_string| PathBuf::from(os_string).join(APP_DIR));
    let app_dir2 =
        home::home_dir()
            .filter(|path| !path.as_os_str().is_empty())
            .map(|path| path.join(".config").join(APP_DIR));
    match (app_dir1, app_dir2) {
        (Some(app_dir1), Some(app_dir2)) => {
            if app_dir1 == app_dir2 {
                ensure_dir_exists(app_dir1)
            } else {
                match (app_dir1.exists(), app_dir2.exists()) {
                    (true, false) => Ok(app_dir1),
                    (false, true) => Ok(app_dir2),
                    (false, false) => {
                        std::fs::create_dir_all(&app_dir1)?;
                        Ok(app_dir1)
                    }
                    (true, true) => {
                        let message = format!(
                            "Two different app directories exist: '{}' and '{}'.",
                            app_dir1.display(),
                            app_dir2.display()
                        );
                        Err(Error::from(message))
                    }
                }
            }
        }
        (Some(app_dir1), None) => ensure_dir_exists(app_dir1),
        (None, Some(app_dir2)) => ensure_dir_exists(app_dir2),
        (None, None) => Err(Error::from("Could not determine app directory."))
    }
}

fn ensure_dir_exists(path: PathBuf) -> Result<PathBuf, Error> {
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}