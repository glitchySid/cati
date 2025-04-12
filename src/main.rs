use anyhow::Result;
use colored::*;
use std::path::Path;

mod utils;
use utils::format_string_array::vec_to_string_with_newlines;
use utils::get_text::get_text;
use utils::links::extract_links;
use utils::show_bat_output::display_with_bat;

fn main() -> Result<()> {
    let input_image_path = Path::new("test2.png");
    let text = get_text(input_image_path)?;
    let links = extract_links(&text);

    if !links.is_empty() {
        println!("{}", "links found".green().bold().italic());
        // let output = format_links_table(&links);
        let output = vec_to_string_with_newlines(&links);
        if let Err(e) = display_with_bat(&output) {
            eprintln!("Error displaying Rust code: {}", e);
        }
    } else {
        println!(
            "{}",
            "links not found, printing raw text instead"
                .red()
                .bold()
                .italic()
        );
        if let Err(e) = display_with_bat(&text) {
            eprintln!("Error displaying Rust code: {}", e);
        }
    }
    Ok(())
}
