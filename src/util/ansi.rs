/// ANSI escape sequences
///
/// Library uses these convenience functions and values
/// for doing any terminal ANSI manipulations.

pub static PART_START: &str = "\x1b["; // rem, "\x1b" is a single character
pub static PART_END: &str = "m";

pub static PART_RESET: &str = "0";
pub static PART_BACKGROUND_TRUECOLOR: &str = "48;2";
pub static PART_FOREGROUND_TRUECOLOR: &str = "38;2";

pub static CODE_CLEAR: &str = "\x1b[2J\x1b[H";
pub static CODE_TOP_LEFT: &str = "\x1b[H";

pub static CODE_HIDE_CURSOR: &str = "\x1b[?25l";
pub static CODE_SHOW_CURSOR: &str = "\x1b[?25h";

pub fn move_cursor(col:i32, row:i32) -> String {
    format!("{}{row};{col}H", PART_START, row = row + 1, col = col + 1) // `+1` bc 1-indexed
}

#[inline(always)]
pub fn background_color(r: u8, g: u8, b: u8) -> String {
    format!("{}{};{};{};{}{}", PART_START, PART_BACKGROUND_TRUECOLOR, r, g, b, PART_END)
    // eg, "\x1b[48;2;40;177;249m"
}

pub fn foreground_color(r: u8, g: u8, b: u8) -> String {
    format!("{}{};{};{};{}{}", PART_START, PART_FOREGROUND_TRUECOLOR, r, g, b, PART_END)
}

pub fn make_command_using_part(part: &str) -> String {
    format!("{}{}{}", PART_START, part, PART_END)
}
