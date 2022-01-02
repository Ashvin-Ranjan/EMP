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
        // Since all of the values can be represented by 8 bytes or less we can use the 4 bits available to
        // us to store the amout of bytes containing 0x00, which saves on space when these are used to store
        // small values.
        //
        // Ideally in the future I would like to have it so the trailing bits are accounted for too, using the
        // fact that I can maybe get away with counting 0b0000 to 0b1000 as the amount of heading bits and
        // 0b1001 - 0b1111 as the trailing bits and choosing which one to give depending on the space it would
        // save. This would be especially useful for negative numbers.
        Value::Int64(i) => {
            let mut value =
                vec![constants::INT_64 | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

            let mut first_numb = false;

            for val in i.to_be_bytes() {
                if val != 0 || first_numb {
                    value.push(val);
                    first_numb = true;
                }
            }

            return value;
        }
        Value::Int32(i) => {
            let mut value = vec![
                constants::INT_32
                    | get_leading_zeros(Vec::from(i.abs().to_be_bytes())) << 4
                    | if i < 0 { 0xC0 } else { 0 },
            ];

            let mut first_numb = false;

            for val in i.abs().to_be_bytes() {
                if val != 0 || first_numb {
                    value.push(val);
                    first_numb = true;
                }
            }

            return value;
        }
        Value::Int16(i) => {
            let mut value =
                vec![constants::INT_16 | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

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
            let mut value =
                vec![constants::FLOAT | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

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
            let mut value =
                vec![constants::DOUBLE | get_leading_zeros(Vec::from(i.to_be_bytes())) << 4];

            let mut first_numb = false;

            for val in i.to_be_bytes() {
                if val != 0 || first_numb {
                    value.push(val);
                    first_numb = true;
                }
            }

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
