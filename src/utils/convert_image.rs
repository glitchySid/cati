use anyhow::{Ok, Result};

pub fn convert_image(image_path: &str) -> Result<()> {
    let image = image::open(image_path)?.grayscale();
    let temp_file = "input.png";
    image.save(temp_file)?;
    Ok(())
}
