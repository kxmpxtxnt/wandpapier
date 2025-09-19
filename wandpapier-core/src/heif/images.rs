use anyhow::{Ok, Result};
use libheif_rs as heif;
use std::path::PathBuf;
use libheif_rs::{ColorSpace, Image, RgbChroma};
use tracing::{debug};

pub async fn load_images(file: PathBuf) -> Result<(String, Vec<Image>)> {
    let heif = heif::LibHeif::new();
    let file_path = file.as_path().to_string_lossy();
    let file_name = file.file_name().unwrap().to_string_lossy();
    let ctx = heif::HeifContext::read_from_file(file_path.to_string().as_str())?;

    let image_count = ctx.number_of_top_level_images();

    debug!(
        "Found {} top level images in {}",
        image_count,
        file_path.to_string()
    );

    let mut images: Vec<Image> = vec![];

    for handle in ctx.top_level_image_handles() {
        let image = heif.decode(
            &handle,
            ColorSpace::Rgb(RgbChroma::Rgb),
            None,
        )?;

        images.push(image);
    }

    Ok((file_name.to_string(), images))
}
