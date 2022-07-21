#[derive(Debug)]
pub enum Colors {
    Red,
    Blue,
    Green,
    Black,
    Orange,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    Default,

    BgRed,
    BgBlue,
    BgGreen,
    BgBlack,
    BgOrange,
    BgMagenta,
    BgCyan,
    BgLightGray,
    BgDefault,

    ResetDim,
}

impl Colors {
    fn set_color_code(color: &Colors) -> &'static str {
        return match color {
            Colors::Red => "\x1b[91m",
            Colors::Blue => "\x1b[94m",
            Colors::Green => "\x1b[92m",
            Colors::Black => "\x1b[30m",
            Colors::Orange => "\x1b[93m",
            Colors::Magenta => "\x1b[95m",
            Colors::Cyan => "\x1b[96m",
            Colors::LightGray => "\x1b[37m",
            Colors::Default => "\x1b[39m",
            Colors::DarkGray => "\x1b[90m",

            Colors::ResetDim => "\x1b[22m",

            Colors::BgRed => "\x1b[41m",
            Colors::BgBlue => "\x1b[44m",
            Colors::BgGreen => "\x1b[42m",
            Colors::BgBlack => "\x1b[40m",
            Colors::BgOrange => "\x1b[43m",
            Colors::BgMagenta => "\x1b[45m",
            Colors::BgCyan => "\x1b[46m",
            Colors::BgLightGray => "\x1b[47m",
            Colors::BgDefault => "\x1b[49m",
        };
    }
}

pub fn colorize_text(text: &str, color: &Colors) -> String {
    let colored_string = format!("{}{}\x1b[0m", Colors::set_color_code(&color), text,);

    colored_string
}
