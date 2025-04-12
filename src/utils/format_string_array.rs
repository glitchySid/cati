pub fn vec_to_string_with_newlines(data: &Vec<String>) -> String {
    let mut result = String::new();
    for item in data {
        result.push_str(item);
        result.push('\n');
    }
    result
}
