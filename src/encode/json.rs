//! Encoding json data into EMP Bytecode

use crate::encode;
use crate::value;
use serde_json;
use std::vec::Vec;

/// Encodes serde_json data into EMP bytecode
pub fn encode_json(val: serde_json::Value) -> Vec<u8> {
    return encode::encode(value::json::from_json(val));
}
