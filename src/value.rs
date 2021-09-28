//! EMP Enum to manipulate in rust

pub mod json;
pub mod parse;

use crate::constants;
use std::fmt;

#[derive(Clone, Debug)]
/// EMP representation.
///
/// Null: `null`
/// Object: A `HashMap` with a `std::string::String` as the key and a `Value`
/// as the object. You cannot use a `Value` as a key because `HashMap` does
/// not derive `Hash`.
/// Array: A `std::vec::Vec` of `Value`s
/// String: A `std::string::String`
/// Bit: A boolean where true means 1 and false means 0
/// Boolean: A boolean
/// All numbers take in their respective types
pub enum Value {
    Null,
    Object(std::collections::HashMap<std::string::String, Value>),
    Array(std::vec::Vec<Value>),
    String(std::string::String),
    Bit(bool),
    Boolean(bool),
    Int32(i32),
    Float(f32),
    Double(f64),
    Int64(i64),
    Int16(i16),
    Int8(i8),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", display(self.clone()))
    }
}

fn display(val: Value) -> String {
    match val {
        Value::String(s) => return format!("\"{}\"", s.replace("\"", "\\\"")),
        Value::Bit(i) => return format!("{}{}", if i { "1" } else { "0" }, constants::EMP_BIT),
        Value::Boolean(i) => return (if i { "true" } else { "false" }).to_owned(),
        Value::Int32(i) => return format!("{}", i),
        Value::Int64(i) => return format!("{}{}", i, constants::EMP_LONG),
        Value::Int16(i) => return format!("{}{}", i, constants::EMP_SHORT),
        Value::Int8(i) => return format!("{}{}", i, constants::EMP_BYTE),
        Value::Float(i) => return format!("{}{}", i, constants::EMP_FLOAT),
        Value::Double(i) => return format!("{}{}", i, constants::EMP_DOUBLE),
        Value::Array(a) => {
            let mut arr = vec![];
            for v in a {
                arr.push(display(v));
            }

            return format!("[{}]", arr.join(", "));
        }
        Value::Object(o) => {
            let mut out = String::from("");

            for key in o.keys() {
                out = format!(
                    "{}, \"{}\": {}",
                    out,
                    key,
                    display(match o.get(key) {
                        Some(s) => s.clone(),
                        None => Value::Null,
                    })
                );
            }
            if out.len() > 2 {
                return format!("{{{}}}", out[2..].to_owned());
            }
            return "{}".to_owned();
        }
        Value::Null => return "null".to_owned(),
    }
}
