use regex::Regex;

pub fn extract_links(text: &str) -> Vec<String> {
    let re = Regex::new(r"(https?://\S+)").unwrap();
    re.find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect()
}
