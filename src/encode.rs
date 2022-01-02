//! Encoding Values into EMP Bytecode

use crate::constants;
use crate::value::Value;
use std::vec::Vec;

pub mod json;

/// Encodes an `emp::value::Value` into a `Vec<u8>` to store in a `.emp` file.
pub fn encode(val: Value) -> Vec<u8> {
    // Most of the compression in this program comes from the fact that all identifier bytes only take up
    // the last 4 bits, which means that the first 4 bits can be used to store information.
    match val {
        // Null values are represented by 0x05 or 0b0101, no compression here.
        Value::Null => return vec![constants::NULL],

        // Bits and booleans both store their value in their identifier, storing it in the 4th bit.
        Value::Bit(b) => return vec![constants::BIT | (if b { 1 } else { 0 } << 4)],
        Value::Boolean(b) => return vec![constants::BOOLEAN | (if b { 1 } else { 0 } << 4)],

        // All number values except for bits and bytes have what I call heading byte optimization.
        //
        // Since all of the values can be represented by 8 bytes or less we use the MSB to store whether the
        // number is negative or not, and we use the remaining 3 bits available to us to store the amout of
        // bytes containing 0x00, which saves on space when these are used to store small values. This does
        // mean that for a 0l there is a byte containing 0x00 which cannot be trimmed off but I have come to
        // the conclusion that it is worth it for the optimization of negative numbers.
        Value::Int64(i) => {
            let bytes = i.abs().to_be_bytes();
            let leading = get_leading_zeros(Vec::from(bytes));
            let mut value =
                vec![constants::INT_64 | leading << 4 | if i < 0 { 0b10000000 } else { 0 }];

            value.append(&mut (bytes[(leading as usize)..bytes.len()]).to_vec());

            return value;
        }
        Value::Int32(i) => {
            let bytes = i.abs().to_be_bytes();
            let leading = get_leading_zeros(Vec::from(bytes));
            let mut value =
                vec![constants::INT_32 | leading << 4 | if i < 0 { 0b10000000 } else { 0 }];

            value.append(&mut (bytes[(leading as usize)..bytes.len()]).to_vec());

            return value;
        }
        Value::Int16(i) => {
            let bytes = i.abs().to_be_bytes();
            let leading = get_leading_zeros(Vec::from(bytes));
            let mut value =
                vec![constants::INT_16 | leading << 4 | if i < 0 { 0b10000000 } else { 0 }];

            value.append(&mut (bytes[(leading as usize)..bytes.len()]).to_vec());

            return value;
        }
        Value::Float(i) => {
            let bytes = i.abs().to_be_bytes();
            let leading = get_leading_zeros(Vec::from(bytes));
            let mut value =
                vec![constants::FLOAT | leading << 4 | if i < 0.0 { 0b10000000 } else { 0 }];

            value.append(&mut (bytes[(leading as usize)..bytes.len()]).to_vec());

            return value;
        }
        Value::Double(i) => {
            let bytes = i.abs().to_be_bytes();
            let leading = get_leading_zeros(Vec::from(bytes));
            let mut value =
                vec![constants::DOUBLE | leading << 4 | if i < 0.0 { 0b10000000 } else { 0 }];

            value.append(&mut (bytes[(leading as usize)..bytes.len()]).to_vec());

            return value;
        }

        // Due to the fact that Int8s are one byte instead it is checked wheter the value is greater than 0
        // and less than 16, if this is met then it will instead store the value of the byte in the first
        // 4 bits of the identifier, saving a byte in total.
        Value::Int8(i) => {
            let mut value =
                vec![constants::INT_8 | if i <= 0x0F && i > 0 { i as u8 } else { 0 } << 4];

            if i > 0xf || i <= 0 {
                for val in i.to_be_bytes() {
                    value.push(val);
                }
            }

            return value;
        }

        // Strings, Arrays, and Objects are similar as they are all lists of values, and so have the same optimization.
        //
        // If their length is below 16, they do not have a closing tag, instead they will store their length in the top
        // 4 bits of their identifier, saving a byte in total.
        //
        // Theorecticially, for strings the first character can be split into 2 parts of 4 bits each and distributed to
        // the top 4 bits of both the opening and closing tags, saving a byte every time, but this would also require
        // the banning of all characters with the lower 4 bits 0b0100
        Value::String(s) => {
            let set_len = s.len() <= 0x0F && s.len() != 0;

            let mut value = vec![constants::STRING | if set_len { s.len() << 4 } else { 0 } as u8];

            for val in s.replace(|c: char| !c.is_ascii(), "").as_bytes() {
                if val.to_owned() != constants::STRING {
                    value.push(val.to_owned());
                }
            }

            if !set_len {
                value.push(constants::STRING);
            }
            return value;
        }
        Value::Array(a) => {
            let set_len = a.len() <= 0x0F && a.len() != 0;

            let mut value =
                vec![constants::ARRAY_START | if set_len { a.len() << 4 } else { 0 } as u8];
            for val in a {
                for byte in encode(val) {
                    value.push(byte);
                }
            }

            if !set_len {
                value.push(constants::ARRAY_END);
            }

            return value;
        }
        Value::Object(o) => {
            let set_len = o.keys().len() <= 0x0F && o.keys().len() != 0;

            let mut value = vec![
                constants::DICTIONARY_START | if set_len { o.keys().len() << 4 } else { 0 } as u8,
            ];
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

            if !set_len {
                value.push(constants::DICTIONARY_END);
            }

            return value;
        }
    }
}

fn get_leading_zeros(val: Vec<u8>) -> u8 {
    let mut out = 0;
    for v in val {
        if v != 0 || out == 7 {
            return out;
        }
        out += 1
    }
    return out;
}
