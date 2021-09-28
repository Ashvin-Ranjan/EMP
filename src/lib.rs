//! # EMP
//!
//! EMP is a bytecode format which is able to pack JSON and NBT data into an
//! even smaller file size, which is useful for storage of large amounts of
//! data.
//!
//! ### Rust structure
//!
//! ```
//! pub enum Value {
//!   Null,
//!   Object(std::collections::HashMap<std::string::String, Value>),
//!   Array(std::vec::Vec<Value>),
//!   String(std::string::String),
//!   Bit(bool),
//!   Boolean(bool),
//!   Int32(i32),
//!   Float(f32),
//!   Double(f64),
//!   Int64(i64),
//!   Int16(i16),
//!   Int8(i8),
//! }
//! ```
//!
//! ### Encoding
//!
//! Using `emp::encode::encode` you can pass in an `emp::value::Value` and get an `std::vec::Vec<u8>` back, this will encode the data in the `Value` into the bytes that you can then write to a file.
//!
//! ### Decoding
//!
//! Using `emp::decode::decode` you can pass in a `&[u8]` and get an `Result<emp::value::Value, emp::errors::DecodeError>` in return.
//!
//! `DecodeError` is an enum that is as so:
//!
//! ```
//! pub enum DecodeError {
//!   UnexpectedByte(u8, u64),
//!   EOFError,
//!   UnmatchedKey(std::string::String),
//!   StringDecodeError(std::str::Utf8Error),
//! }
//! ```
//!
//! You can also use `emp::decode::decode_safe` and pass in the same data to decode the data in the same way but if there is an `Err` it instead returns `emp::value::Value::Null`.
//!
//! ### Parsing
//!
//! Using the `emp::value::parse::from_str` function you can pass in a `&str` to convert it into a `Result<emp::value::Value, emp::errors::ParseError>`. Alternatively you can use `emp::value::parse::from_str_safe` and pass in the same thing to get a `emp::value::Value`, if an error is encounted it returns a `emp::value::Value::Null` instead.
//!
//! `ParseError` is an enum that is as so:
//!
//! ```
//! pub enum ParseError {
//!    EOFError,
//!    UnexpectedCharacterError(char),
//!    UnexpectedTokenError(std::string::String),
//!    InvalidKeyError(value::Value),
//!    InvalidNumberError(char),
//! }
//! ```
//!
//! ### JSON Compatability
//!
//! #### Conversion
//!
//! This crate is compatible with `serde_json`, by using `emp::value::json::from_json` you can convert a `serde_json::Value` to an `emp::value::Value`, vice versa for `emp::value::json::to_json`.
//!
//! #### Encoding
//!
//! Using `emp::encode::json::encode_json` you can encode a `serde_json::Value` directly into `emp` bytecode.
//!
//! #### Decoding
//!
//! Using `emp::decode::json::decode_json` you can decode `emp` bytecode directly into a `serde_json::Value`, this uses `decode_safe` rather than `decode`.
//! You can also use `emp::decode::json::decode_json_unsafe` to get the error instead of a `serde_json::Value::Null`
//!
//! ### String representation
//!
//! Strings: Data in quotes
//! Int32: Regular number
//! Int16: Number with `s` appended at the end
//! Int8: Number with `B` appended at the end
//! Bit: 0 or 1 with `b` appended at the end
//! Int64: Number with `l` appended at the end
//! Boolean: `true` or `false`
//! Null: `null`
//! Array: Values separated by commas in square brackets
//! Object: String then a colon then a Value, separated by commas in curly brackets
//!

pub mod constants;
pub mod decode;
pub mod encode;
pub mod errors;
pub mod value;
