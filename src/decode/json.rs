use crate::decode;
use crate::value;
use crate::errors;
use serde_json;

pub fn decode_json(val: &[u8]) -> serde_json::Value {
  return value::json::to_json(decode::decode_safe(val));
}

pub fn decode_json_unsafe(val: &[u8]) -> Result<serde_json::Value, errors::DecodeError> {
  return match decode::decode(val) {
    Ok((v, _)) => Ok(value::json::to_json(v)),
    Err(d) => Err(d),
  }
}