use anyhow::Result;
use tesseract::Tesseract;

use super::check_extension::is_png;
use super::convert_image::convert_image;

pub fn get_text(image_path: &str) -> Result<String> {
    let mut input_image = image_path; // Initialize Tesseract for English language
    let mut ocr = Tesseract::new(None, Some("eng"))?;
    if !is_png(input_image) {
        convert_image(input_image)?;
        input_image = "input.png";
    }
    ocr = ocr.set_image(input_image)?;

    // Retrieve the extracted text
    let text = ocr.get_text()?;
    return Ok(text);
}
