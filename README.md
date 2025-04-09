# OCR Text Extractor

A Rust application that extracts text from images using OCR (Optical Character Recognition) and identifies links within the text.

## Features

- Extract text from images using Tesseract OCR
- Convert non-PNG images to PNG format for processing
- Identify URLs/links within the extracted text
- Format and display links in a neat table format

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract)
- English language data for Tesseract

## Installation

1. Clone the repository
2. Install Tesseract OCR on your system
3. Install the English language data for Tesseract
4. Build the project:

```bash
cargo build --release
```

## Usage

Place an image file in the project directory and run the application:

```bash
cargo run
```

By default, the application looks for a file named `test.png`. You can modify the input image path in `src/main.rs`.

## How It Works

1. The application reads an input image
2. If the image is not in PNG format, it automatically converts it
3. Tesseract OCR processes the image and extracts text
4. The application scans the text for URLs using regex
5. Identified links are formatted into a table and displayed

## Project Structure

- `src/main.rs` - Main application entry point
- `src/utils/` - Utility modules:
  - `check_extension.rs` - Image file extension validation
  - `convert_image.rs` - Image format conversion
  - `format_string_array.rs` - Table formatting for display
  - `get_text.rs` - OCR text extraction
  - `links.rs` - URL extraction from text using regex

## Dependencies

- `anyhow` - Error handling
- `tesseract` - OCR functionality
- `image` - Image processing
- `regex` - Regular expression matching

## License

[MIT](LICENSE)