use crate::constants;
use crate::value::Value;

fn decode_bit(_b: &[u8]) -> Result<(Option<Value>, &[u8]), (&'static str, u8)> {
  let bytes = _b;
  if bytes[0] != constants::BIT {
    return Ok((None, &bytes[1..bytes.len()]));
  }

  if bytes.len() == 1 {
    return Err(("Expected Byte, got end of file", 0));
  }

  return Ok((Some(Value::Bit(bytes[1] == 1)), &bytes[2..bytes.len()]));
}

fn decode_bool(_b: &[u8]) -> Result<(Option<Value>, &[u8]), (&'static str, u8)> {
  let bytes = _b;
  if bytes[0] != constants::BOOLEAN {
    return Ok((None, &bytes[1..bytes.len()]));
  }

  if bytes.len() == 1 {
    return Err(("Expected boolean, got end of file", 0));
  }

  return Ok((Some(Value::Boolean(bytes[1] == 1)), &bytes[2..bytes.len()]));
}

pub fn decode(_b: &[u8]) -> Result<(Value, &[u8]), (&'static str, u8)> {
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

  return Err(("Unexpected Byte: ", bytes[0]));
}
