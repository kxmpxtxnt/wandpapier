use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Errors {
    #[error("error while creating directory.")]
    DirCreate(#[from] io::Error),
    #[error("error in libheif-rs.")]
    Hife(#[from] libheif_rs::HeifError),
}