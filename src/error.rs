use crate::core::{RawBox, make_exception_box};
use std::fmt;

#[derive(Clone, Copy)]
pub enum Errors {
    Arg,
    Size,
}

impl Errors {
    fn to_i32(self) -> i32 {
        match self {
            Errors::Arg => 400,
            Errors::Size => 414,
        }
    }
}
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::Arg => write!(f, "Invalid Argument"),
            Errors::Size => write!(f, "Output Size Exceeded"),
        }
    }
}

pub struct TB64Error {
    pub code: Errors,
}

fn error(code: Errors) -> String {
    let text1: String = match code {
        Errors::Arg => String::from("Invalid Argument"),
        Errors::Size => String::from("Output Size Exceeded"),
    };
    let text2: String = match code {
        Errors::Arg => String::from("Please provide exactly one string as an argument"),
        Errors::Size => String::from("Output too large"),
    };
    let raw_b: RawBox = RawBox {
        text1,
        text_color_hex1: String::from("#DC143C"),
        text2,
        text_color_hex2: String::from("#FFEA00"),
        sep_color_hex: String::from("#FF7F50"),
        box_color_hex: String::from("#7CFC00"),
    };

    make_exception_box(raw_b)
}

impl fmt::Display for TB64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg: String = match self.code {
            Errors::Arg => error(Errors::Arg),
            Errors::Size => error(Errors::Size),
        };

        write!(f, "{err_msg}")
    }
}

impl fmt::Debug for TB64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TB64Error {{ code: {}, message: {} }}",
            self.code.to_i32(),
            self.code
        )
    }
}
