use crate::value;
use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    UnexpectedByteError(u8, u64),
    EOFError,
    UnmatchedKeyError(std::string::String),
    StringDecodeError(std::str::Utf8Error),
    InvalidKeyError(value::Value),
}

#[derive(Debug)]
pub enum ParseError {
    EOFError,
    UnexpectedCharacterError(char),
    UnexpectedTokenError(std::string::String),
    InvalidKeyError(value::Value),
    InvalidNumberError(char),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", display_decode_error(self))
    }
}

fn display_decode_error(val: &DecodeError) -> String {
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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", display_parse_error(self))
    }
}

fn display_parse_error(val: &ParseError) -> String {
    match val {
        ParseError::UnexpectedCharacterError(c) => return format!("Unexpected Character: '{}'", c),
        ParseError::UnexpectedTokenError(c) => return format!("Unexpected Token: \"{}\"", c),
        ParseError::EOFError => return "Unexpected EOF".to_owned(),
        ParseError::InvalidKeyError(k) => return format!("Invalid Key: `{}`", k),
        ParseError::InvalidNumberError(c) => {
            return format!("Could not parse number with signifier: `{}`", c)
        }
    }
}
