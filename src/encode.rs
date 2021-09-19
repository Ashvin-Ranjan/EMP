use crate::constants;
use crate::value::Value;
use std::vec::Vec;

pub mod json;

pub fn encode(val: Value) -> &'static [u8] {
  match val {
    Value::Null => return &[constants::NULL],
    Value::Bit(b) => return &[constants::BIT, if b { 1 } else { 0 }],
    Value::Boolean(b) => return &[constants::BOOLEAN, if b { 1 } else { 0 }],
    Value::Int32(i) => {
      let mut value = vec![constants::INT_32];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
    Value::Float(i) => {
      let mut value = vec![constants::FLOAT];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
    Value::Double(i) => {
      let mut value = vec![constants::DOUBLE];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
    Value::String(s) => {
      let mut value = vec![constants::STRING];

      for val in s.replace(|c: char| !c.is_ascii(), "").as_bytes() {
        if val.to_owned() != constants::STRING {
          value.push(val.to_owned());
        }
      }

      value.push(constants::STRING);
      return Vec::as_slice(&value);
    }
    Value::Array(a) => {
      let mut value = vec![constants::ARRAY_START];
      for val in a {
        for byte in encode(val) {
          value.push(byte.to_owned());
        }
      }

      value.push(constants::ARRAY_END);

      return Vec::as_slice(&value);
    }
    Value::Object(o) => {
      let mut value = vec![constants::DICTIONARY_START];
      for key in o.keys() {
        for byte in encode(Value::String(key.to_owned())) {
          value.push(byte.to_owned());
        }

        let val = match o.get(key) {
          Some(e) => encode(e.clone()),
          None => &[constants::NULL],
        };

        for byte in val {
          value.push(byte.to_owned());
        }
      }

      value.push(constants::DICTIONARY_END);

      return &value[..];
    }
    Value::Int64(i) => {
      let mut value = vec![constants::INT_64];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
    Value::Int16(i) => {
      let mut value = vec![constants::INT_16];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
    Value::Int8(i) => {
      let mut value = vec![constants::INT_8];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return Vec::as_slice(&value);
    }
  }
}
