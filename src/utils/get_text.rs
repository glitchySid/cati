use anyhow::Result;
use std::path::Path;
use tesseract::Tesseract;

use super::convert_image::preprocess_image;

pub fn get_text(image_path: &Path) -> Result<String> {
    // Initialize Tesseract for English language
    let mut ocr = Tesseract::new(None, Some("eng"))?;
    
    // Preprocess the image
    let gray_image = preprocess_image(image_path)?;
    
    // Create a temp path for the processed image
    let temp_path = std::env::temp_dir().join("temp_processed.png");
    let temp_path_str = temp_path.to_str().unwrap();
    
    // Save the preprocessed image to a temporary file
    gray_image.save(&temp_path_str)?;
    
    // Set the image for OCR processing
    ocr = ocr.set_image(temp_path_str)?;

    // Retrieve the extracted text
    let text = ocr.get_text()?;
    return Ok(text);
}
