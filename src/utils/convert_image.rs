use anyhow::{Ok, Result};
use image::{GrayImage, ImageReader};
use imageproc::contrast::{otsu_level, threshold_mut, ThresholdType};
use std::path::Path;

pub fn preprocess_image(image_path: &Path) -> Result<GrayImage> {
    let image = ImageReader::open(image_path)?.decode()?;
    let mut gray_image = image.to_luma8();
    let threshold = otsu_level(&gray_image);
    threshold_mut(&mut gray_image, threshold, ThresholdType::Binary);
    Ok(gray_image)
}
