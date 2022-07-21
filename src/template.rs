use crate::utils::colorize::{colorize_text, Colors};

pub struct TermLine {
    pub key: String,
    pub value: String,
    pub color: Colors,
}

pub fn write_to_term(lines: &Vec<TermLine>) {
    for line in lines.iter() {
        let key = colorize_text(&line.key[..], &Colors::DarkGray);
        let value = colorize_text(&line.value[..], &line.color);

        println!("{key}{value}");
    }
}
