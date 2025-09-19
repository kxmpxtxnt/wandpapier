use crate::errors::Errors;
use libheif_rs as heif;
use libheif_rs::{ColorSpace, Image, LibHeif, RgbChroma};
use std::path::PathBuf;

pub fn extract_thumbnail(heif: &LibHeif, file: PathBuf) -> Result<Image, Errors> {
    let file_name = file.as_path().to_string_lossy();
    let ctx = heif::HeifContext::read_from_file(file_name.to_string().as_str())?;

    let handle = ctx.primary_image_handle()?;

    let image = heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    Ok(image)
}
