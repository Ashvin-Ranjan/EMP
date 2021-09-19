use crate::decode;
use crate::value;
use serde_json;

pub fn decode_json(val: &[u8]) -> serde_json::Value {
    return value::json::to_json(decode::decode_safe(val));
}