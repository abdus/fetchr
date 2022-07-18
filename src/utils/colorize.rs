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
}

impl Colors {
    fn set_color_code(color: Colors) -> &'static str {
        return match color {
            Colors::Red => "\x1b[31m",
            Colors::Blue => "\x1b[34m",
            Colors::Green => "\x1b[32m",
            Colors::Black => "\x1b[30m",
            Colors::Orange => "\x1b[33m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::LightGray => "\x1b[37m",
            Colors::Default => "\x1b[39m",

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

pub fn colorize_text(text: &str, color: Colors) {
    println!(
        "{}{}{}{}",
        Colors::set_color_code(color),
        text,
        Colors::set_color_code(Colors::Default),
        Colors::set_color_code(Colors::BgDefault)
    );
}
