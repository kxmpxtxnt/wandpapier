use libheif_rs::LibHeif;
use wandpapier_core::directories::{images_dir, unpack_dir};
use wandpapier_core::heif::extract::extract_images;
use wandpapier_core::heif::images::load_images;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )?;

    let heif = LibHeif::new();

    for entry in std::fs::read_dir(images_dir()?)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }

        let images = load_images(&heif, images_dir()?.join(path)).await?;

        extract_images(&heif, images, unpack_dir()?)?;
    }

    Ok(())
}
