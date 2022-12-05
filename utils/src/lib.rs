use std::fs;

pub fn parse_file(path: &str) -> String {
    fs::read_to_string(path).expect("Cannot find file!")
}

pub fn color_text(text: &str, color: char) -> String {
    let ansi_start: &str = match color {
        'g' => "\u{1b}[32m",
        'b' => "\u{1b}[34m",
        _ => "",
    };
    let ansi_end: &str = "\u{1b}[39m";
    format!("{}{}{}", ansi_start, text, ansi_end)
}
