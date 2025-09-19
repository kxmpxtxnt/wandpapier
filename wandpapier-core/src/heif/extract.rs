use crate::errors::Errors;
use libheif_rs as heif;
use libheif_rs::{CompressionFormat, EncoderQuality, HeifContext};
use std::path::{Path, PathBuf};
use tracing::debug;

pub fn extract_images(
    from: (String, Vec<heif::Image>),
    to: PathBuf,
) -> Result<Vec<PathBuf>, Errors> {
    let mut stored_images = vec![];

    let heif = heif::LibHeif::new();
    let (file_name, images) = from;

    let file_name_without_ext = Path::new(&file_name).file_stem().unwrap().to_string_lossy();

    let images_path = to.join(file_name_without_ext.to_string());

    if !images_path.exists() {
        std::fs::create_dir(images_path.clone())?;
    }

    let mut encoder = heif.encoder_for_format(CompressionFormat::Undefined)?;
    encoder.set_quality(EncoderQuality::LossLess)?;

    for (i, image) in images.iter().enumerate() {
        let store_file_name = format!("{}-{}.jpg", file_name_without_ext, i);
        let write_file = images_path.join(store_file_name);

        if write_file.exists() {
            debug!("Skipping {}", write_file.to_string_lossy());
            continue;
        }

        let mut ctx = HeifContext::new()?;

        ctx.encode_image(image, &mut encoder, None)?;

        ctx.write_to_file(write_file.as_path().to_string_lossy().to_string().as_str())?;

        stored_images.push(write_file.clone());

        debug!("Successfully created {}.", write_file.to_string_lossy())
    }

    Ok(stored_images)
}
