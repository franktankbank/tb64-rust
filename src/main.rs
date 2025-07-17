mod core;
mod error;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use clap::Parser;
use core::{RawBox, print_box};
#[cfg(target_family = "windows")]
use windows_sys::Win32::System::Console::SetConsoleOutputCP;
cfg_if::cfg_if! {
    if #[cfg(target_family = "unix")] {
        use nix::libc::{ioctl, TIOCGWINSZ, winsize};
        use std::os::unix::io::AsRawFd;
        use std::io::stdout;
    }
}
use error::{Errors, TB64Error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
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
    #[cfg(target_family = "windows")]
    {
        unsafe {
            SetConsoleOutputCP(65001);
        }
    }
    let cli: Cli = Cli::parse();
    let raw_b: RawBox;

    if let Some(input) = cli.input.as_deref() {
        let enc: String = triple_encode(String::from(input));

        #[cfg(target_family = "unix")]
        {
            match main_unix(enc.clone()) {
                Err(e) => {
                    eprint!("{e}");
                    return;
                }
                _ => (),
            }
        }
        #[cfg(target_family = "windows")]
        {
            if let Err(e) = main_win32(enc.clone()) {
                eprint!("{e}");
                return;
            }
        }

        raw_b = RawBox {
            text1: String::from("Encoded"),
            text_color_hex1: String::from("#00CED1"),
            text2: enc.clone(),
            text_color_hex2: String::from("#FFEA00"),
            sep_color_hex: String::from("#FF69B4"),
            box_color_hex: String::from("#7CFC00"),
        };
        print_box(raw_b);
        cli_clipboard::set_contents(enc.to_owned()).unwrap();
    } else if let Err(e) = invalid_arg() { eprint!("{e}") }
}

fn invalid_arg() -> Result<(), TB64Error> {
    Err(TB64Error { code: Errors::Arg })
}

#[cfg(target_family = "unix")]
fn main_unix(enc: String) -> Result<(), TB64Error> {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let fd: i32 = stdout().as_raw_fd();

    // Use unsafe block because ioctl involves raw file descriptors
    unsafe {
        // Call ioctl to get terminal size
        // Retrieve terminal size via `ioctl` (TIOCGWINSZ)
        ioctl(fd, TIOCGWINSZ, &mut w);
    }

    // Calculate the width by subtracting 4 from the number of columns
    let width: usize = w.ws_col as usize - 4;
    let length: usize = enc.len() + 7 + 5;
    if length > width {
        Err(TB64Error { code: Errors::Size })
    } else {
        Ok(())
    }
}
#[cfg(target_family = "windows")]
fn main_win32(enc: String) -> Result<(), TB64Error> {
    if enc.len() > 64 {
        Err(TB64Error { code: Errors::Size })
    } else {
        Ok(())
    }
}
