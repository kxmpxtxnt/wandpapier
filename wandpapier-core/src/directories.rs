use crate::errors::Errors;
use std::path::PathBuf;

fn create_dir_exists(path: PathBuf) -> Result<PathBuf, Errors> {
    if path.exists() {
        return Ok(path);
    }

    match std::fs::create_dir(path.clone()) {
        Ok(_) => Ok(path),
        Err(err) => Err(Errors::from(err))
    }
}

pub fn user_config_dir() -> Result<PathBuf, Errors> {
    create_dir_exists(glib::user_config_dir().join("wandpapier"))
}

pub fn images_dir() -> Result<PathBuf, Errors> {
    create_dir_exists(user_config_dir()?.join("images"))
}

pub fn unpack_dir() -> Result<PathBuf, Errors> {
    create_dir_exists(images_dir()?.join(".unpack"))
}
