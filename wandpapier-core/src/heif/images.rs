use crate::errors::Errors;
use libheif_rs as heif;
use libheif_rs::{ColorSpace, Image, LibHeif, RgbChroma};
use std::path::PathBuf;
use tracing::debug;

pub async fn load_images(heif: &LibHeif, buf: PathBuf) -> Result<(String, Vec<Image>), Errors> {
    let file_path = buf.as_path().to_string_lossy();
    let file_name = buf.to_string_lossy();
    let ctx = heif::HeifContext::read_from_file(file_path.to_string().as_str())?;

    let image_count = ctx.number_of_top_level_images();

    debug!(
        "Found {} top level images in {}",
        image_count,
        file_path.to_string()
    );

    let mut images: Vec<Image> = vec![];

    for handle in ctx.top_level_image_handles() {
        let image = heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
        images.push(image);
    }

    debug!("Encoded {} images successfully.", image_count);

    Ok((file_name.to_string(), images))
}
