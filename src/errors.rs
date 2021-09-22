use crate::value;
use std::fmt;

pub enum DecodeError {
    UnexpectedByteError(u8, u64),
    EOFError,
    UnmatchedKeyError(std::string::String),
    StringDecodeError(std::str::Utf8Error),
    InvalidKeyError(value::Value),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", display(self))
    }
}

fn display(val: &DecodeError) -> String {
    match val {
        DecodeError::UnexpectedByteError(b, l) => {
            return format!("Unexpected Byte: 0x{:x?} at location {}", b, l)
        }
        DecodeError::EOFError => return "Unexpected EOF".to_owned(),
        DecodeError::UnmatchedKeyError(k) => return format!("Unmatched Key: `{}`", k),
        DecodeError::StringDecodeError(e) => return format!("Unable to decode string data: {}", e),
        DecodeError::InvalidKeyError(k) => return format!("Invalid Key: `{}`", k),
    }
}
