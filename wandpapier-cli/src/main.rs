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

    let images = load_images(images_dir()?.join("minecraft.heic")).await?;

    extract_images(images, unpack_dir()?).await?;

    Ok(())
}
