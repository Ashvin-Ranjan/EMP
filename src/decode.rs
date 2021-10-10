//! Decoding EMP Bytecode

use crate::constants;
use crate::errors::DecodeError;
use crate::value::Value;
use std::collections::HashMap;
use std::convert::TryInto;

pub mod json;
#[macro_use]
mod macros;

fn decode_bit(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    if bytes[0] & 0xf != constants::BIT {
        return Ok((None, &bytes[1..]));
    }

    return Ok((Some(Value::Bit(bytes[0] >> 4 == 1)), &bytes[1..]));
}

fn decode_bool(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    if bytes[0] & 0xf != constants::BOOLEAN {
        return Ok((None, &bytes[1..]));
    }

    return Ok((Some(Value::Boolean(bytes[0] >> 4 == 1)), &bytes[1..]));
}

fn decode_array(mut bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    let mut emp_array: Vec<Value> = vec![];

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

fn decode_string(mut bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    let mut emp_string: Vec<u8> = vec![];

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

fn decode_null(bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    if bytes[0] != constants::NULL {
        return Ok((None, &bytes[1..]));
    }

    return Ok((Some(Value::Null), &bytes[1..]));
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

fn decode_object(mut bytes: &[u8]) -> Result<(Option<Value>, &[u8]), DecodeError> {
    let mut emp_object: HashMap<String, Value> = HashMap::new();

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

/// Decodes a slice of `u8`s into a `(emp::value::Value, &[u8])` tuple
///
/// The `&[u8]` in the tuple is for internal use and should be discorded.
/// If the decoder encounters an error it will return a
/// `emp::errors::DecodeError` error instead.
pub fn decode(bytes: &[u8]) -> Result<(Value, &[u8]), DecodeError> {
    try_decode!(decode_bit, bytes);
    try_decode!(decode_bool, bytes);
    try_decode!(decode_array, bytes);
    try_decode!(decode_string, bytes);
    try_decode!(decode_null, bytes);
    try_decode!(decode_int32, bytes);
    try_decode!(decode_object, bytes);
    try_decode!(decode_float, bytes);
    try_decode!(decode_double, bytes);
    try_decode!(decode_int64, bytes);
    try_decode!(decode_int16, bytes);
    try_decode!(decode_int8, bytes);

    return Err(DecodeError::UnexpectedByteError(
        bytes[0],
        bytes.len() as u64,
    ));
}

/// Decodes a lice of `u8`s into an `emp::value::Value`.
///
/// If it encounters an error it will instead return a `emp::value::Value::Null`
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
