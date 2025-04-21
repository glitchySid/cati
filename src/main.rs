use anyhow::Result;
use clap::Parser;
use colored::*;

mod utils;
use utils::cli::{validate_image_path, Args};
use utils::format_string_array::vec_to_string_with_newlines;
use utils::get_text::get_text;
use utils::links::extract_links;
use utils::show_bat_output::display_with_bat;

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Validate and get the absolute path to the image
    let input_image_path = validate_image_path(&args.image)?;
    // println!(
    //     "Processing image: {}",
    //     input_image_path.display().to_string().cyan()
    // );

    // Get text from the image using OCR
    let text = get_text(&input_image_path)?;

    // Extract links from the text
    let links = extract_links(&text);

    // Handle output based on links flag and whether links were found
    if args.links {
        // Links-only mode
        if !links.is_empty() {
            println!("{}", "Links found:".green().bold().italic());
            let output = vec_to_string_with_newlines(&links);
            if let Err(e) = display_with_bat(&output) {
                eprintln!("Error displaying links: {}", e);
            }
        } else {
            println!("{}", "No links found in the image.".yellow().bold());
        }
    } else {
        if let Err(e) = display_with_bat(&text) {
            eprintln!("Error displaying text: {}", e);
        }
    }
    Ok(())
}
