pub fn parse_field(bytes: &[u8]) -> String {
    core::str::from_utf8(bytes)
        .unwrap_or("")
        .trim_end_matches('<')
        .to_string()
}

pub fn parse_str_field(s: &str) -> String {
    s.trim_end_matches('<').to_string()
}
