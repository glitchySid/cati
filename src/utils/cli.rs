use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

/// CLI Arguments parser for OCR link extractor
#[derive(Parser, Debug)]
#[clap(author, version, about = "Extract links from images using OCR")]
pub struct Args {
    /// Path to the input image
    #[clap(short, long)]
    pub image: String,
}

/// Validates the image path and returns the absolute path
pub fn validate_image_path(image_path: &str) -> Result<PathBuf> {
    // First try to open the file as-is
    let path = Path::new(image_path);

    if path.exists() {
        // If the path exists as provided, convert to absolute
        let abs_path = path
            .canonicalize()
            .with_context(|| format!("Failed to get absolute path for '{}'", image_path))?;
        return Ok(abs_path);
    }

    // If not found, try with the current working directory
    let current_dir =
        std::env::current_dir().with_context(|| "Failed to get current working directory")?;
    let combined_path = current_dir.join(image_path);

    if combined_path.exists() {
        let abs_path = combined_path.canonicalize().with_context(|| {
            format!(
                "Failed to get absolute path for '{}'",
                combined_path.display()
            )
        })?;
        return Ok(abs_path);
    }

    // If we get here, the file doesn't exist
    anyhow::bail!("Image file not found: '{}'", image_path)
}
