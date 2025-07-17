use std::fmt::Write;

pub struct RawBox {
    pub text1: String,
    pub text_color_hex1: String,
    pub text2: String,
    pub text_color_hex2: String,
    pub sep_color_hex: String,
    pub box_color_hex: String,
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

fn hex_to_rgb(hex_color: String) -> String {
    // Strip '#' from hex color if present
    // let hex: String = if hex_color.starts_with("#") {
    //     hex_color[1..].to_string()
    // } else {
    //     hex_color.to_string()
    // };
    let hex: String = if let Some(stripped) = hex_color.strip_prefix("#") { stripped.to_string() } else { hex_color };
    if hex.len() == 6 {
        // Extract the red, green, and blue components
        let red_hex: &str = &hex[0..2];
        let green_hex: &str = &hex[2..4];
        let blue_hex: &str = &hex[4..6];

        // Convert hex strings to integers
        let r: i32 = i32::from_str_radix(red_hex, 16).unwrap_or(0);
        let g: i32 = i32::from_str_radix(green_hex, 16).unwrap_or(0);
        let b: i32 = i32::from_str_radix(blue_hex, 16).unwrap_or(0);

        format!("\x1b[38;2;{r};{g};{b}m")
    } else {
        eprintln!("Invalid hex color code");
        String::new()
    }
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
    // Initialize box struct
    let box_width: usize = raw_b.text1.len() + raw_b.text2.len() + 5;
    let b: Box = Box {
        text1: raw_b.text1,
        text_color1: hex_to_rgb(raw_b.text_color_hex1),
        text2: raw_b.text2,
        text_color2: hex_to_rgb(raw_b.text_color_hex2),
        sep_color: hex_to_rgb(raw_b.sep_color_hex),
        box_color: hex_to_rgb(raw_b.box_color_hex),
        box_width,
    };

    print!("  {}\u{250F}", b.box_color); // Top-left corner
    for _ in 0..b.box_width {
        print!("\u{2501}"); // Horizontal line
    }
    println!("\u{2513}\x1b[0m"); // Top-right corner

    // Middle line with text
    print!("  {}\u{2503}\x1b[0m ", b.box_color); // Left border
    print!("{}{}\x1b[0m", b.text_color1, b.text1); // First part of text
    print!(" {}\u{2192}\x1b[0m ", b.sep_color); // Separator
    print!("{}{}\x1b[0m", b.text_color2, b.text2); // Second part of text
    println!(" {}\u{2503}\x1b[0m", b.box_color); // Right border

    // Bottom border
    print!("  {}\u{2517}", b.box_color); // Bottom-left corner
    for _ in 0..b.box_width {
        print!("\u{2501}"); // Horizontal line
    }
    println!("\u{251B}\x1b[0m"); // Bottom-right corner
}

pub fn make_exception_box(raw_b: RawBox) -> String {
    // Initialize box struct
    let box_width: usize = raw_b.text1.len() + raw_b.text2.len() + 5;
    let b: Box = Box {
        text1: raw_b.text1,
        text_color1: hex_to_rgb(raw_b.text_color_hex1),
        text2: raw_b.text2,
        text_color2: hex_to_rgb(raw_b.text_color_hex2),
        sep_color: hex_to_rgb(raw_b.sep_color_hex),
        box_color: hex_to_rgb(raw_b.box_color_hex),
        box_width,
    };

    make_exception(b)
}
