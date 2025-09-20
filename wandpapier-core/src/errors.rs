use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("while creating directory.")]
    DirCreate(#[from] io::Error),
    #[error("in libheif-rs.")]
    Hife(#[from] libheif_rs::HeifError),
}