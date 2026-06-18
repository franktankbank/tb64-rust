mod core;
mod error;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use clap::{Parser, builder::styling};
use terminal_size::{terminal_size, Width, Height};
use core::{RawBox, print_box, RGB};

const STYLES_RGB: styling::Styles = styling::Styles::styled()
    .header(styling::RgbColor(166, 227, 161).on_default().bold())
    .usage(styling::RgbColor(166, 227, 161).on_default().bold())
    .literal(styling::RgbColor(137, 180, 250).on_default().bold())
    .placeholder(styling::RgbColor(137, 220, 235).on_default())
    .error(styling::RgbColor(243, 139, 168).on_default().bold())
    .invalid(styling::RgbColor(249, 226, 175).on_default().bold())
    .valid(styling::RgbColor(166, 227, 161).on_default().bold());

#[cfg(target_family = "windows")]
use windows_sys::Win32::System::Console::SetConsoleOutputCP;

use error::TB64Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(styles = STYLES_RGB)]
struct Cli {
    /// Optional name to operate on
    input: Option<String>,
}

fn triple_encode(input: String) -> String {
    let mut enc: String = input;
    for _ in 0..3 {
        enc = STANDARD.encode(enc);
    }
    enc
}

fn main() {
    let cli: Cli = Cli::parse();

    match core(cli) {
        Ok(_) => (),
        Err(e) => eprint!("{e}")
    }
}

fn core(cli: Cli) -> Result<(), TB64Error> {
    #[cfg(target_family = "windows")]
    {
        unsafe {
            SetConsoleOutputCP(65001);
        }
    }
    let raw_b: RawBox;

    if let Some(input) = cli.input.as_deref() {
        let enc: String = triple_encode(String::from(input));

        let term_cols: u16 = terminal_size().unwrap_or((Width(64), Height(30))).0.0;

        let length: usize = enc.len() + 7 + 5;
        let width: usize = term_cols as usize - 4;

        if length > width {
            return Err(TB64Error::Size)
        }

        raw_b = RawBox {
            text1: String::from("Encoded"),
            text_color_rgb1: RGB {r: 0, g: 206, b: 209},
            text2: enc.clone(),
            text_color_rgb2: RGB { r: 255, g: 234, b: 0 },
            sep_color_rgb: RGB { r: 255, g: 105, b: 180 },
            box_color_rgb: RGB { r: 124, g: 252, b: 0 },
        };
        print_box(raw_b);
        cli_clipboard::set_contents(enc.to_owned()).unwrap();
    } else {
        return Err(TB64Error::Arg);
    }

    Ok(())
}
