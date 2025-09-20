use crate::errors::Errors;
use libheif_rs as heif;
use libheif_rs::{CompressionFormat, EncoderQuality, HeifContext, LibHeif};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use tracing::debug;

pub async fn extract_images(
    heif: &LibHeif,
    from: (String, Vec<heif::Image>),
    to: PathBuf,
) -> Result<Vec<PathBuf>, Errors> {
    let (file_name, images) = from;

    let file_name_without_ext = Path::new(&file_name).file_stem().unwrap().to_string_lossy();

    let images_path = to.join(file_name_without_ext.to_string());

    if !images_path.exists() {
        std::fs::create_dir(images_path.clone())?;
    }

    let (sender, receiver) = std::sync::mpsc::channel::<PathBuf>();

    images.into_par_iter().enumerate().try_for_each_with(
        sender,
        |sender, (i, image)| -> Result<(), Errors> {
            let store_file_name = format!("{}-{}.jpg", file_name_without_ext, i);
            let write_file = images_path.join(store_file_name);

            if write_file.exists() {
                debug!("Skipping {}", write_file.to_string_lossy());
                let _ = sender.send(write_file);
                return Ok(());
            }

            let mut encoder = heif.encoder_for_format(CompressionFormat::Undefined)?;
            encoder.set_quality(EncoderQuality::LossLess)?;

            let mut ctx = HeifContext::new()?;

            ctx.encode_image(&image, &mut encoder, None)?;

            ctx.write_to_file(write_file.as_path().to_string_lossy().to_string().as_str())?;

            debug!("Successfully created {}.", write_file.to_string_lossy());

            let _ = sender.send(write_file);
            Ok(())
        },
    )?;

    let stored_images = receiver.iter().collect();

    Ok(stored_images)
}
