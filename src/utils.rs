pub fn from_path_to_name(line: String) -> String {
    if let Some(f) = line.rfind("/") {
        return string_sub_to_end(line, f + 1);
    } else {
        return String::new();
    }
}
///gets the string[start..]
pub fn string_sub_to_end(s: String, start: usize) -> String {
    return (s[start..]).to_string();
}
