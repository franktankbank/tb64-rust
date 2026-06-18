use crate::core::{RGB, RawBox, make_exception_box};
use std::{fmt, error::Error, io::Error as IoError};

#[derive(Debug)]
pub enum TB64Error {
    Arg,
    Size,
    Io(IoError)
}

impl fmt::Display for TB64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msgs: (String, String) = match self {
            TB64Error::Arg => (String::from("Invalid Argument"), String::from("Please provide exactly one string as an argument")),
            TB64Error::Size => (String::from("Output Size Exceeded"), String::from("Output too large")),
            TB64Error::Io(e) => (String::from("Io Error"), format!("{e}"))
        };

        let raw_b: RawBox = RawBox {
            text1: err_msgs.0,
            text_color_rgb1: RGB { r: 220, g: 20, b: 60 },
            text2: err_msgs.1,
            text_color_rgb2: RGB { r: 255, g: 234, b: 0 },
            sep_color_rgb: RGB { r: 255, g: 127, b: 80 },
            box_color_rgb: RGB { r: 124, g: 252, b: 0 }
        };

        let r#box: String = make_exception_box(raw_b);

        write!(f, "{box}")
    }
}

impl Error for TB64Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TB64Error::Arg => None,
            TB64Error::Size => None,
            TB64Error::Io(e) => Some(e)
        }
    }
}

impl From<IoError> for TB64Error {
    fn from(err: IoError) -> TB64Error {
        TB64Error::Io(err)
    }
}
