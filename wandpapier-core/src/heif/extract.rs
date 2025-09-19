use libheif_rs as heif;
use libheif_rs::{CompressionFormat, EncoderQuality, HeifContext};
use std::path::{Path, PathBuf};
use crate::errors::Errors;

pub fn extract_images(from: (String, Vec<heif::Image>), to: PathBuf) -> Result<Vec<PathBuf>, Errors> {
    let mut stored_images = vec![];

    let heif = heif::LibHeif::new();
    let (file_name, images) = from;

    let file_name_without_ext = Path::new(&file_name).file_stem().unwrap().to_string_lossy();

    let images_path = to.join(file_name_without_ext.to_string());

    std::fs::create_dir(images_path.clone())?;

    for (i, image) in images.iter().enumerate() {
        let mut ctx = HeifContext::new()?;

        let mut encoder = heif.encoder_for_format(CompressionFormat::Undefined)?;

        encoder.set_quality(EncoderQuality::LossLess)?;

        ctx.encode_image(image, &mut encoder, None)?;

        let store_file_name = format!("{}-{}.jpg", file_name_without_ext, i);
        let write_file = images_path.join(store_file_name);

        ctx.write_to_file(write_file.as_path().to_string_lossy().to_string().as_str())?;

        stored_images.push(write_file);
    }

    Ok(stored_images)
}
