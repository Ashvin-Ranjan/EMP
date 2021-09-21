use crate::constants;
use crate::errors::DecodeError;
use crate::value::Value;
use std::collections::HashMap;
use std::convert::TryInto;

pub mod json;

fn decode_bit(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] != constants::BIT {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() == 1 {
    return Err(DecodeError::EOFError);
  }

  return Ok((Some(Value::Bit(bytes[1] == 1)), &bytes[2..]));
}

fn decode_bool(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] != constants::BOOLEAN {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() == 1 {
    return Err(DecodeError::EOFError);
  }

  return Ok((Some(Value::Boolean(bytes[1] == 1)), &bytes[2..]));
}

fn decode_array(_b: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  let mut emp_array: Vec<Value> = vec![];

  let mut bytes = _b;

  if bytes[0] & 0xf != constants::ARRAY_START {
    return Ok((None, &bytes[1..]));
  }

  let left = bytes[0] >> 4;

  bytes = &bytes[1..];

  if left == 0 {
    while bytes.len() != 0 {
      if bytes[0] == constants::ARRAY_END {
        return Ok((Some(Value::Array(emp_array)), &bytes[1..]));
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
  } else {
    for _ in 0..left {
      if bytes.len() == 0 {
        return Err(DecodeError::EOFError);
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
    return Ok((Some(Value::Array(emp_array)), &bytes));
  }

  return Err(DecodeError::EOFError);
}

fn decode_string(_b: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  let mut emp_string: Vec<u8> = vec![];

  let mut bytes = _b;

  if bytes[0] & 0xf != constants::STRING {
    return Ok((None, &bytes[1..]));
  }

  let left = bytes[0] >> 4;

  bytes = &bytes[1..];

  if left == 0 {
    while bytes.len() != 0 {
      if bytes[0] == constants::STRING {
        let s = match std::str::from_utf8(&emp_string) {
          Ok(s) => s,
          Err(e) => return Err(DecodeError::StringDecodeError(e)),
        };
        return Ok((Some(Value::String(s.to_owned())), &bytes[1..]));
      }

      emp_string.push(bytes[0]);
      bytes = &bytes[1..];
    }
  } else {
    for _ in 0..left {
      if bytes.len() == 0 {
        return Err(DecodeError::EOFError);
      }

      emp_string.push(bytes[0]);
      bytes = &bytes[1..];
    }

    let s = match std::str::from_utf8(&emp_string) {
      Ok(s) => s,
      Err(e) => return Err(DecodeError::StringDecodeError(e)),
    };
    return Ok((Some(Value::String(s.to_owned())), &bytes));
  }

  return Err(DecodeError::EOFError);
}

fn decode_null(bytes: &[u8]) -> (Option<Value>, &[u8]) {
  if bytes[0] != constants::NULL {
    return (None, &bytes[1..]);
  }

  return (Some(Value::Null), &bytes[1..]);
}

fn decode_int32(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] & 0x0F != constants::INT_32 {
    return Ok((None, &bytes[1..]));
  }
  let back: usize = (bytes[0] >> 4).into();

  if bytes.len() < 5 - back {
    return Err(DecodeError::EOFError);
  }
  return Ok((
    Some(Value::Int32(i32::from_be_bytes(
      force_to_length(&bytes[1..5 - back], 4)
        .try_into()
        .expect("Slice with incorrect length"),
    ))),
    &bytes[5 - back..],
  ));
}

fn decode_object(_b: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  let mut emp_object: HashMap<String, Value> = HashMap::new();

  let mut bytes = _b;

  if bytes[0] & 0xf != constants::DICTIONARY_START {
    return Ok((None, &bytes[1..]));
  }

  let left = bytes[0] >> 4;

  bytes = &bytes[1..];

  if left == 0 {
    let mut current_key: Option<String> = None;
    while bytes.len() != 0 {
      if bytes[0] == constants::DICTIONARY_END {
        if let Some(k) = current_key {
          return Err(DecodeError::UnmatchedKeyError(k));
        }
        return Ok((Some(Value::Object(emp_object)), &bytes[1..]));
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
  } else {
    for _ in 0..left {
      let key;

      let val_tokens = decode(bytes);

      match val_tokens {
        Ok((val, b)) => {
          bytes = b;
          if let Value::String(s) = val {
            key = s;
          } else {
            return Err(DecodeError::InvalidKeyError(val));
          }
        }
        Err(e) => return Err(e),
      }

      let val_tokens = decode(bytes);

      match val_tokens {
        Ok((val, b)) => {
          bytes = b;
          emp_object.insert(key, val);
        }
        Err(e) => return Err(e),
      }
    }
    return Ok((Some(Value::Object(emp_object)), &bytes));
  }

  return Err(DecodeError::EOFError);
}

fn decode_float(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] & 0x0F != constants::FLOAT {
    return Ok((None, &bytes[1..]));
  }

  let back: usize = (bytes[0] >> 4).into();

  if bytes.len() < 5 - back {
    return Err(DecodeError::EOFError);
  }

  return Ok((
    Some(Value::Float(f32::from_be_bytes(
      force_to_length(&bytes[1..5 - back], 4)
        .try_into()
        .expect("Slice with incorrect length"),
    ))),
    &bytes[5 - back..],
  ));
}

fn decode_double(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] & 0x0F != constants::DOUBLE {
    return Ok((None, &bytes[1..]));
  }
  let back: usize = (bytes[0] >> 4).into();

  if bytes.len() < 9 - back {
    return Err(DecodeError::EOFError);
  }

  return Ok((
    Some(Value::Double(f64::from_be_bytes(
      force_to_length(&bytes[1..9 - back], 8)
        .try_into()
        .expect("Slice with incorrect length"),
    ))),
    &bytes[9 - back..],
  ));
}

fn decode_int64(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] & 0x0F != constants::INT_64 {
    return Ok((None, &bytes[1..]));
  }

  let back: usize = (bytes[0] >> 4).into();

  if bytes.len() < 9 - back {
    return Err(DecodeError::EOFError);
  }

  return Ok((
    Some(Value::Int64(i64::from_be_bytes(
      force_to_length(&bytes[1..9 - back], 8)
        .try_into()
        .expect("Slice with incorrect length"),
    ))),
    &bytes[9 - back..],
  ));
}

fn decode_int16(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] & 0x0F != constants::INT_16 {
    return Ok((None, &bytes[1..]));
  }

  let back: usize = (bytes[0] >> 4).into();

  if bytes.len() < 3 - back {
    return Err(DecodeError::EOFError);
  }

  return Ok((
    Some(Value::Int16(i16::from_be_bytes(
      force_to_length(&bytes[1..3 - back], 2)
        .try_into()
        .expect("Slice with incorrect length"),
    ))),
    &bytes[3 - back..],
  ));
}

fn decode_int8(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
  if bytes[0] != constants::INT_8 {
    return Ok((None, &bytes[1..]));
  }

  if bytes.len() < 2 {
    return Err(DecodeError::EOFError);
  }

  return Ok((
    Some(Value::Int8(i8::from_be_bytes([bytes[1]]))),
    &bytes[2..],
  ));
}

pub fn decode(_b: &[u8]) -> Result<(Value, &[u8]), DecodeError> {
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

  return Err(DecodeError::UnexpectedByteError(
    bytes[0],
    bytes.len() as u64,
  ));
}

pub fn decode_safe(val: &[u8]) -> Value {
  match decode(val) {
    Ok((c, _)) => return c,
    Err(_) => return Value::Null,
  }
}

fn force_to_length(arr: &[u8], n: u64) -> Vec<u8> {
  let mut zero: Vec<u8> = Vec::new();

  while zero.len() + arr.len() < n as usize {
    zero.push(0);
  }

  let out = [zero, Vec::from(arr)].concat();
  return out;
}
