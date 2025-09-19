use std::path::PathBuf;
use anyhow::anyhow;

fn get_or_load_dir(path: PathBuf) -> anyhow::Result<PathBuf> {
    if path.exists() {
        return anyhow::Ok(path)
    }

    match std::fs::create_dir(path.clone()) {
        anyhow::Result::Ok(_) => anyhow::Ok(path),
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn user_config_dir() -> anyhow::Result<PathBuf> {
    get_or_load_dir(glib::user_config_dir().join("wandpapier"))
}

pub fn images_dir() -> anyhow::Result<PathBuf> {
    get_or_load_dir(user_config_dir()?.join("images"))
}

pub fn unpack_dir() -> anyhow::Result<PathBuf> {
    get_or_load_dir(images_dir()?.join(".unpack"))
}