use crate::encode;
use crate::value;
use serde_json;
use std::vec::Vec;

pub fn encode_json(val: serde_json::Value) -> Vec<u8> {
    return encode::encode(value::json::from_json(val));
}
