use std::fmt::Write;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct RawBox {
    pub text1: String,
    pub text_color_rgb1: RGB,
    pub text2: String,
    pub text_color_rgb2: RGB,
    pub sep_color_rgb: RGB,
    pub box_color_rgb: RGB,
}

struct Box {
    text1: String,
    text_color1: String,
    text2: String,
    text_color2: String,
    sep_color: String,
    box_color: String,
    box_width: usize,
}

fn rgb_to_str(rgb: RGB) -> String {
    format!("\x1b[38;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
}

fn make_exception(b: Box) -> String {
    let mut message: String = "".to_string();
    write!(&mut message, "  {}\u{250F}", b.box_color).unwrap(); // Top-left corner
    for _ in 0..b.box_width {
        write!(&mut message, "\u{2501}").unwrap(); // Horizontal line
    }
    writeln!(&mut message, "\u{2513}\x1b[0m").unwrap(); // Top-right corner

    // Middle line with text
    write!(&mut message, "  {}\u{2503}\x1b[0m ", b.box_color).unwrap(); // Left border
    write!(&mut message, "{}{}\x1b[0m", b.text_color1, b.text1).unwrap(); // First part of text
    write!(&mut message, " {}\u{2192}\x1b[0m ", b.sep_color).unwrap(); // Separator
    write!(&mut message, "{}{}\x1b[0m", b.text_color2, b.text2).unwrap(); // Second part of text
    writeln!(&mut message, " {}\u{2503}\x1b[0m", b.box_color).unwrap(); // Right border

    // Bottom border
    write!(&mut message, "  {}\u{2517}", b.box_color).unwrap(); // Bottom-left corner
    for _ in 0..b.box_width {
        write!(&mut message, "\u{2501}").unwrap(); // Horizontal line
    }
    writeln!(&mut message, "\u{251B}\x1b[0m").unwrap(); // Bottom-right corner

    message
}

pub fn print_box(raw_b: RawBox) {
    let mut msg: String = "".to_string();

    // Initialize box struct
    let box_width: usize = raw_b.text1.len() + raw_b.text2.len() + 5;
    let b: Box = Box {
        text1: raw_b.text1,
        text_color1: rgb_to_str(raw_b.text_color_rgb1),
        text2: raw_b.text2,
        text_color2: rgb_to_str(raw_b.text_color_rgb2),
        sep_color: rgb_to_str(raw_b.sep_color_rgb),
        box_color: rgb_to_str(raw_b.box_color_rgb),
        box_width,
    };

    write!(&mut msg, "  {}\u{250F}", b.box_color).unwrap(); // Top-left corner
    for _ in 0..b.box_width {
        write!(&mut msg, "\u{2501}").unwrap(); // Horizontal line
    }
    writeln!(&mut msg, "\u{2513}\x1b[0m").unwrap(); // Top-right corner

    // Middle line with text
    write!(&mut msg, "  {}\u{2503}\x1b[0m ", b.box_color).unwrap(); // Left border
    write!(&mut msg, "{}{}\x1b[0m", b.text_color1, b.text1).unwrap(); // First part of text
    write!(&mut msg, " {}\u{2192}\x1b[0m ", b.sep_color).unwrap(); // Separator
    write!(&mut msg, "{}{}\x1b[0m", b.text_color2, b.text2).unwrap(); // Second part of text
    writeln!(&mut msg, " {}\u{2503}\x1b[0m", b.box_color).unwrap(); // Right border

    // Bottom border
    write!(&mut msg, "  {}\u{2517}", b.box_color).unwrap(); // Bottom-left corner
    for _ in 0..b.box_width {
        write!(&mut msg, "\u{2501}").unwrap(); // Horizontal line
    }
    writeln!(&mut msg, "\u{251B}\x1b[0m").unwrap(); // Bottom-right corner

    print!("{msg}");
}

pub fn make_exception_box(raw_b: RawBox) -> String {
    // Initialize box struct
    let box_width: usize = raw_b.text1.len() + raw_b.text2.len() + 5;
    let b: Box = Box {
        text1: raw_b.text1,
        text_color1: rgb_to_str(raw_b.text_color_rgb1),
        text2: raw_b.text2,
        text_color2: rgb_to_str(raw_b.text_color_rgb2),
        sep_color: rgb_to_str(raw_b.sep_color_rgb),
        box_color: rgb_to_str(raw_b.box_color_rgb),
        box_width,
    };

    make_exception(b)
}
