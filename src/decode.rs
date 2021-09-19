use crate::constants;
use crate::value::Value;
use std::convert::TryInto;
use std::collections::HashMap;

pub mod json;

fn decode_bit(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::BIT {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() == 1 {
    return Err("Expected Byte, got end of file".to_owned());
  }

  return Ok((Some(Value::Bit(bytes[1] == 1)), &bytes[2..]));
}

fn decode_bool(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::BOOLEAN {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() == 1 {
    return Err("Expected boolean, got end of file".to_owned());
  }

  return Ok((Some(Value::Boolean(bytes[1] == 1)), &bytes[2..]));
}

fn decode_array(_b: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  let mut emp_array: Vec<Value> = vec![];

  let mut bytes = _b;

  if bytes[0] != constants::ARRAY_START {
    return Ok((None, &bytes[1..]));
  }

  bytes = &bytes[1..];

  while bytes.len() != 0 {
    if bytes[0] == constants::ARRAY_END {
      return Ok((Some(Value::Array(emp_array)), &bytes[1..]))
    }

    let val_tokens = decode(bytes);

    match val_tokens {
      Ok((val, b)) => {
        bytes = b;
        emp_array.push(val);
      }
      Err(e) => return Err(e),
    }
  }

  return Err("Expected Array to end before end of file".to_owned());
}

fn decode_string(_b: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  let mut emp_string: Vec<u8> = vec![];

  let mut bytes = _b;

  if bytes[0] != constants::STRING {
    return Ok((None, &bytes[1..]));
  }

  bytes = &bytes[1..];

  while bytes.len() != 0 {
    if bytes[0] == constants::STRING {
      let s = match std::str::from_utf8(&emp_string) {
        Ok(s) => s,
        Err(e) => return Err(format!("Unable to decode string bytes {}", e)),
      };
      return Ok((Some(Value::String(s.to_owned())), &bytes[1..]))
    }

    emp_string.push(bytes[0]);
    bytes = &bytes[1..];
  }

  return Err("Expected String to end before end of file".to_owned());
}

fn decode_null(bytes: &[u8]) -> (Option<Value>, &[u8]) {
  if bytes[0] != constants::NULL {
    return (None, &bytes[1..]);
  }

  return (Some(Value::Null), &bytes[1..]);
}

fn decode_int32(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::INT_32 {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 5 {
    return Err("Expected Int 32, got end of file".to_owned());
  }

  return Ok((Some(Value::Int32(u32::from_be_bytes(bytes[1..5].try_into().expect("Slice with incorrect length")))), &bytes[5..]));
}

fn decode_object(_b: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  let mut emp_object:HashMap<String, Value> = HashMap::new();

  let mut bytes = _b;

  if bytes[0] != constants::DICTIONARY_START {
    return Ok((None, &bytes[1..]));
  }

  bytes = &bytes[1..];

  let mut current_key:Option<String> = None;

  while bytes.len() != 0 {
    if bytes[0] == constants::DICTIONARY_END {
      if current_key != None {
        return Err("Unmatched key".to_owned());
      }
      return Ok((Some(Value::Object(emp_object)), &bytes[1..]))
    }

    let val_tokens = decode(bytes);

    match val_tokens {
      Ok((val, b)) => {
        bytes = b;
        if let Some(c) = current_key.clone() {
          emp_object.insert(c.clone(), val);
          current_key = None;
        } else {
          if let Value::String(s) = val {
            let c = s;
            current_key = Some(c);
          }
        }
      }
      Err(e) => return Err(e),
    }
  }

  return Err("Expected Array to end before end of file".to_owned());
}

fn decode_float(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::FLOAT {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 5 {
    return Err("Expected float, got end of file".to_owned());
  }

  return Ok((Some(Value::Float(f32::from_be_bytes(bytes[1..5].try_into().expect("Slice with incorrect length")))), &bytes[5..]));
}

fn decode_double(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::DOUBLE {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 9 {
    return Err("Expected Double, got end of file".to_owned());
  }

  return Ok((Some(Value::Double(f64::from_be_bytes(bytes[1..9].try_into().expect("Slice with incorrect length")))), &bytes[9..]));
}

fn decode_int64(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::INT_64 {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 9 {
    return Err("Expected Int 64, got end of file".to_owned());
  }

  return Ok((Some(Value::Int64(u64::from_be_bytes(bytes[1..9].try_into().expect("Slice with incorrect length")))), &bytes[9..]));
}

fn decode_int16(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::INT_16 {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 3 {
    return Err("Expected Int 16, got end of file".to_owned());
  }

  return Ok((Some(Value::Int16(u16::from_be_bytes(bytes[1..3].try_into().expect("Slice with incorrect length")))), &bytes[3..]));
}

fn decode_int8(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), String> {
  if bytes[0] != constants::INT_8 {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 2 {
    return Err("Expected Int 8, got end of file".to_owned());
  }

  return Ok((Some(Value::Int8(u8::from_be_bytes([bytes[1]]))), &bytes[2..]));
}

pub fn decode(_b: &[u8]) -> Result<(Value, &[u8]), String> {
  let bytes = _b;

  let emp_bit = decode_bit(bytes);
  match emp_bit {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_bool = decode_bool(bytes);
  match emp_bool {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_array = decode_array(bytes);
  match emp_array {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_string = decode_string(bytes);
  match emp_string {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let (emp_null, b) = decode_null(bytes);
  if let Some(e) = emp_null {
    return Ok((e, b));
  }

  let emp_int32 = decode_int32(bytes);
  match emp_int32 {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_object = decode_object(bytes);
  match emp_object {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_float = decode_float(bytes);
  match emp_float {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_double = decode_double(bytes);
  match emp_double {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_int64 = decode_int64(bytes);
  match emp_int64 {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_int16 = decode_int16(bytes);
  match emp_int16 {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  let emp_int8 = decode_int8(bytes);
  match emp_int8 {
    Ok((val, b)) => {
      if let Some(v) = val {
        return Ok((v, b));
      }
    }
    Err(e) => return Err(e),
  }

  return Err(format!("Unexpected Byte: 0x{:x?}", bytes[0]));
}

pub fn decode_safe(val: &[u8]) -> Value {
  match decode(val) {
    Ok((c, _)) => return c,
    Err(_) => return Value::Null,
  }
}