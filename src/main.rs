use anyhow::Result;
use std::path::Path;

mod utils;
use utils::format_string_array::format_links_table;
use utils::get_text::get_text;
use utils::links::extract_links;
use utils::show_bat_output::display_with_bat;

fn main() -> Result<()> {
    let input_image_path = Path::new("test2.png");
    let text = get_text(input_image_path)?;
    let links = extract_links(&text);

    if !links.is_empty() {
        let output = format_links_table(&links);
        println!("links \n{}", output);
    }
    if let Err(e) = display_with_bat(&text) {
        eprintln!("Error displaying Rust code: {}", e);
    }
    Ok(())
}
