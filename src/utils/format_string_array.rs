use std::cmp;
use std::fmt::Write;

pub fn format_links_table(links: &[String]) -> String {
    if links.is_empty() {
        // Handle empty input - maybe return just headers or an empty string
        return String::from("Serial No. | Link\n-----------|-----"); // Basic header if empty
    }

    // Determine column widths
    // Width for "Serial No." column: based on the largest number or the header length
    let sno_width = cmp::max("Serial No.".len(), links.len().to_string().len());

    // Width for "Link" column: based on the longest link or the header length
    let link_width = cmp::max(
        "Link".len(),
        links.iter().map(|s| s.len()).max().unwrap_or(0), // .unwrap_or(0) handles empty list case if we didn't check earlier
    );

    // Use a String buffer for efficient building
    let mut table = String::new();

    // --- Header ---
    writeln!(
        &mut table,
        "{:>sno_width$} | {:<link_width$}", // Right-align S.No., Left-align Link
        "Serial No.",
        "Link",
        sno_width = sno_width,
        link_width = link_width
    )
    .unwrap(); // write!/writeln! on String shouldn't fail

    // --- Separator ---
    writeln!(
        &mut table,
        "{} | {}",
        "-".repeat(sno_width),
        "-".repeat(link_width)
    )
    .unwrap();

    // --- Data Rows ---
    for (index, link) in links.iter().enumerate() {
        let serial_no = index + 1;
        writeln!(
            &mut table,
            "{:>sno_width$} | {:<link_width$}",
            serial_no,
            link,
            sno_width = sno_width,
            link_width = link_width
        )
        .unwrap();
    }

    table
}
