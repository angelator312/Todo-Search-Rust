pub fn from_path_to_name(line: String) -> String {
    if let Some(f) = line.rfind("/") {
        return string_sub_to_end(line, f);
    } else {
        return String::new();
    }
}
///gets the string[start..]
pub fn string_sub_to_end(s: String, start: usize) -> String {
    return String::from(((s[start..]).chars()).as_str());
}
