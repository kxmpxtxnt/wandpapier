pub mod heif;

use anyhow::{anyhow, Ok, Result};
use std::path::PathBuf;

fn get_or_load_dir(path: PathBuf) -> Result<PathBuf> {
    if path.exists() {
        return Ok(path)
    }

    match std::fs::create_dir(path.clone()) {
        Result::Ok(_) => Ok(path),
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn user_config_dir() -> Result<PathBuf> {
    get_or_load_dir(glib::user_config_dir().join("wandpapier"))
}

pub fn images_dir() -> Result<PathBuf> {
    get_or_load_dir(user_config_dir()?.join("images"))
}

pub fn unpack_dir() -> Result<PathBuf> {
    get_or_load_dir(images_dir()?.join(".unpack"))
}