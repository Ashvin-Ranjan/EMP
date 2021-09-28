//! Decoding EMP bytecode into json data

use crate::decode;
use crate::errors;
use crate::value;
use serde_json;

/// Decodes EMP bytecode data into serde_json values
///
/// If an error is encountered the function will just return `serde_json::Value::Null`
pub fn decode_json(val: &[u8]) -> serde_json::Value {
    return value::json::to_json(decode::decode_safe(val));
}

/// Decodes EMP bytecode data into serde_json values
///
/// If an error is encountered it will simply return the error
pub fn decode_json_unsafe(val: &[u8]) -> Result<serde_json::Value, errors::DecodeError> {
    return match decode::decode(val) {
        Ok((v, _)) => Ok(value::json::to_json(v)),
        Err(d) => Err(d),
    };
}
