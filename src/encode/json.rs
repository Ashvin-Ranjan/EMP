use crate::encode;
use crate::value;
use serde_json;

pub fn encode_json(val: serde_json::Value) -> &'static [u8] {
    return encode::encode(value::json::from_json(val));
}