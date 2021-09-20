use crate::constants;
use crate::value::Value;
use std::vec::Vec;

pub mod json;

pub fn encode(val: Value) -> Vec<u8> {
  match val {
    Value::Null => return vec![constants::NULL],
    Value::Bit(b) => return vec![constants::BIT, if b { 1 } else { 0 }],
    Value::Boolean(b) => return vec![constants::BOOLEAN, if b { 1 } else { 0 }],
    Value::Int32(i) => {
      let mut value = vec![constants::INT_32 | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

      let mut first_numb = false;

      for val in i.to_be_bytes() {
        if val != 0 || first_numb {
          value.push(val);
          first_numb = true;
        }
      }

      return value;
    }
    Value::Float(i) => {
      let mut value = vec![constants::FLOAT | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

      let mut first_numb = false;

      for val in i.to_be_bytes() {
        if val != 0 || first_numb {
          value.push(val);
          first_numb = true;
        }
      }

      return value;
    }
    Value::Double(i) => {
      let mut value = vec![constants::DOUBLE | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

      let mut first_numb = false;

      for val in i.to_be_bytes() {
        if val != 0 || first_numb {
          value.push(val);
          first_numb = true;
        }
      }

      return value;
    }
    Value::String(s) => {
      let mut value = vec![constants::STRING];

      for val in s.replace(|c: char| !c.is_ascii(), "").as_bytes() {
        if val.to_owned() != constants::STRING {
          value.push(val.to_owned());
        }
      }

      value.push(constants::STRING);
      return value;
    }
    Value::Array(a) => {
      let mut value = vec![constants::ARRAY_START];
      for val in a {
        for byte in encode(val) {
          value.push(byte);
        }
      }

      value.push(constants::ARRAY_END);

      return value;
    }
    Value::Object(o) => {
      let mut value = vec![constants::DICTIONARY_START];
      for key in o.keys() {
        for byte in encode(Value::String(key.to_owned())) {
          value.push(byte);
        }

        let val = match o.get(key) {
          Some(e) => encode(e.clone()),
          None => vec![constants::NULL],
        };

        for byte in val {
          value.push(byte);
        }
      }

      value.push(constants::DICTIONARY_END);

      return value;
    }
    Value::Int64(i) => {
      let mut value = vec![constants::INT_64 | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

      let mut first_numb = false;

      for val in i.to_be_bytes() {
        if val != 0 || first_numb {
          value.push(val);
          first_numb = true;
        }
      }

      return value;
    }
    Value::Int16(i) => {
      let mut value = vec![constants::INT_16 | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

      let mut first_numb = false;

      for val in i.to_be_bytes() {
        if val != 0 || first_numb {
          value.push(val);
          first_numb = true;
        }
      }

      return value;
    }
    Value::Int8(i) => {
      let mut value = vec![constants::INT_8];

      for val in i.to_be_bytes() {
        value.push(val);
      }

      return value;
    }
  }
}

fn get_leading_zeros(val: Vec<u8>) -> u8 {
  let mut out = 0;
  for v in val {
    if v != 0 {
      return out
    }
    out += 1
  }
  
  return out;
}
