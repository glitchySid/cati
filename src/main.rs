use anyhow::Result;

mod utils;
use utils::format_string_array::format_links_table;
use utils::get_text::get_text;
use utils::links::extract_links;

fn main() -> Result<()> {
    let input_image = "test.png";
    let text = get_text(input_image)?;
    let links = extract_links(&text);
    let output = format_links_table(&links);
    println!("{}", output);
    Ok(())
}
